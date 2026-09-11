use std::fmt::Debug;

use zhc_crypto::integer_semantics::lut::{LutId, LutRegistry};
use zhc_crypto::integer_semantics::{
    CiphertextBlockSpec, EmulatedCiphertextBlock, EmulatedPlaintextBlock,
    EmulatedPlaintextBlockStorage, lut::LookupCheck,
};
use zhc_ir::evaluation::{Evaluable, EvaluatesTo, Evaluation};
use zhc_utils::small::SmallVec;
use zhc_utils::{FastMap, SafeAs, svec};

use super::{CtMem, CtReg, DopTypeSystem, LutRef, PtArg};

/// Interpretation domain for DOP programs.
///
/// DOP uses context-threading: all data flows through inline
/// operand fields, not SSA values. The single `Ctx` variant
/// serves as an ordering token shuttled through the IR framework.
#[derive(Clone, Hash, PartialEq, Eq)]
pub enum DopValue {
    Ctx,
}

impl Debug for DopValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ctx")
    }
}

impl Evaluation for DopValue {}

impl EvaluatesTo<DopValue> for DopTypeSystem {
    fn type_of(_val: &DopValue) -> Self {
        DopTypeSystem::Ctx(0)
    }
}

/// Machine state for DOP program interpretation.
///
/// Simulates the HPU register file, heap, I/O memory, and LUT tables.
/// All data manipulation happens as side-effects on this context;
/// the IR's SSA values only carry opaque context tokens.
///
/// The caller must populate `heap` / `io` with initial ciphertext
/// blocks and fill the LUT tables before interpretation. The register
/// file starts empty and is populated by `LD` and ALU/PBS
/// instructions.
pub struct DopInterpreterContext {
    pub spec: CiphertextBlockSpec,
    /// Fixed-size register file. Slots start uninitialized; the
    /// execution order guarantees all reads follow a prior write.
    pub registers: SmallVec<Option<EmulatedCiphertextBlock>>,
    /// Heap memory, keyed by heap slot address.
    pub heap: FastMap<usize, EmulatedCiphertextBlock>,
    /// I/O memory, keyed by I/O slot address.
    pub io: FastMap<usize, EmulatedCiphertextBlock>,
    /// Lut registry
    pub lut_reg: LutRegistry,
    /// Symbolic ciphertext sources (unpatched stream), keyed by (id, block).
    pub sources: FastMap<(usize, usize), EmulatedCiphertextBlock>,
    /// Symbolic ciphertext destinations (unpatched stream), keyed by (id, block).
    pub destinations: FastMap<(usize, usize), EmulatedCiphertextBlock>,
    /// Symbolic plaintext sources (unpatched stream), keyed by (id, block).
    pub pt_sources: FastMap<(usize, usize), EmulatedPlaintextBlock>,
}

impl Debug for DopInterpreterContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DopInterpreterContext")
            .field("spec", &self.spec)
            .field("num_registers", &self.registers.len())
            .field("heap_len", &self.heap.len())
            .field("io_len", &self.io.len())
            .finish()
    }
}

impl DopInterpreterContext {
    /// Creates a new context with the given spec and a register file
    /// of `num_registers` uninitialized slots.
    pub fn new(spec: CiphertextBlockSpec, num_registers: usize, lut_reg: &LutRegistry) -> Self {
        Self {
            spec,
            registers: svec![Some(spec.random()); num_registers],
            heap: FastMap::default(),
            io: FastMap::default(),
            lut_reg: lut_reg.clone(),
            sources: FastMap::default(),
            destinations: FastMap::default(),
            pt_sources: FastMap::default(),
        }
    }

    /// Reads a ciphertext block from a register.
    fn read_ct_reg(&self, arg: &CtReg) -> EmulatedCiphertextBlock {
        // SAFETY: execution order guarantees the slot was written before read.
        self.registers[arg.addr].unwrap()
    }

    /// Writes a ciphertext block to a register.
    fn write_ct_reg(&mut self, arg: &CtReg, val: EmulatedCiphertextBlock) {
        self.registers[arg.addr] = Some(val);
    }

    /// Reads a ciphertext block from memory (heap, I/O, or an unpatched source template).
    ///
    /// # Panics
    ///
    /// Panics on `CtMem::Dst`: a destination template is only ever written to, never read from.
    fn read_ct_mem(&self, arg: &CtMem) -> EmulatedCiphertextBlock {
        match arg {
            CtMem::Heap(inner) => self
                .heap
                .get(&inner.addr)
                .unwrap_or_else(|| panic!("Heap CT_H({}) not populated", inner.addr))
                .clone(),
            CtMem::Io(inner) => self
                .io
                .get(&inner.addr)
                .unwrap_or_else(|| panic!("I/O CT_IO({}) not populated", inner.addr))
                .clone(),
            CtMem::Src(inner) => self
                .sources
                .get(&(inner.id, inner.block))
                .unwrap_or_else(|| panic!("Source TC({}, {}) not populated", inner.id, inner.block))
                .clone(),
            CtMem::Dst(_) => panic!("Expected a readable memory location, got {arg:?}"),
        }
    }

    /// Writes a ciphertext block to memory (heap, I/O, or an unpatched destination template).
    ///
    /// # Panics
    ///
    /// Panics on `CtMem::Src`: a source template is only ever read from, never written to.
    fn write_ct_mem(&mut self, arg: &CtMem, val: EmulatedCiphertextBlock) {
        match arg {
            CtMem::Heap(inner) => {
                self.heap.insert(inner.addr, val);
            }
            CtMem::Io(inner) => {
                self.io.insert(inner.addr, val);
            }
            CtMem::Dst(inner) => {
                self.destinations.insert((inner.id, inner.block), val);
            }
            CtMem::Src(_) => panic!("Expected a writable memory location, got {arg:?}"),
        }
    }

    /// Builds a plaintext block from an inline constant or symbolic template argument.
    fn read_pt(&self, arg: &PtArg) -> EmulatedPlaintextBlock {
        match arg {
            PtArg::Const(inner) => self
                .spec
                .complete_plaintext_block_spec()
                .from_message(inner.val.sas::<EmulatedPlaintextBlockStorage>()),
            PtArg::Var(inner) => self
                .pt_sources
                .get(&(inner.id, inner.block))
                .unwrap_or_else(|| panic!("Plaintext TI({}, {}) not populated", inner.id, inner.block))
                .clone(),
        }
    }

    /// Resolves a `LutRef` to its registry `LutId`.
    fn resolve_lut(arg: &LutRef) -> LutId {
        LutId(arg.id)
    }
}

impl Evaluable<DopValue> for super::DopInstructionSet {
    type Context = DopInterpreterContext;

    fn eval(
        &self,
        context: &mut Self::Context,
        _arguments: SmallVec<&DopValue>,
    ) -> SmallVec<DopValue> {
        use super::DopInstructionSet::*;
        match self {
            // ── ALU: register arithmetic ─────────────────────────────
            ADD { dst, src1, src2 } => {
                let left = context.read_ct_reg(src1);
                let right = context.read_ct_reg(src2);
                context.write_ct_reg(dst, left.wrapping_add(right));
                svec![DopValue::Ctx]
            }
            SUB { dst, src1, src2 } => {
                let left = context.read_ct_reg(src1);
                let right = context.read_ct_reg(src2);
                context.write_ct_reg(dst, left.wrapping_sub(right));
                svec![DopValue::Ctx]
            }
            MAC {
                dst,
                src1,
                src2,
                cst,
            } => {
                // dst = src1 * cst + src2
                let left = context.read_ct_reg(src1);
                let right = context.read_ct_reg(src2);
                let PtArg::Const(cst) = cst else {
                    unreachable!()
                };
                assert!(cst.val.is_power_of_two());
                context.write_ct_reg(
                    dst,
                    left.wrapping_shl(cst.val.ilog2().sas()).wrapping_add(right),
                );
                svec![DopValue::Ctx]
            }
            ADDS { dst, src, cst } => {
                let ct = context.read_ct_reg(src);
                let pt = context.read_pt(cst);
                context.write_ct_reg(dst, ct.wrapping_add_pt(pt));
                svec![DopValue::Ctx]
            }
            SUBS { dst, src, cst } => {
                let ct = context.read_ct_reg(src);
                let pt = context.read_pt(cst);
                context.write_ct_reg(dst, ct.wrapping_sub_pt(pt));
                svec![DopValue::Ctx]
            }
            SSUB { dst, src, cst } => {
                let pt = context.read_pt(cst);
                let ct = context.read_ct_reg(src);
                context.write_ct_reg(dst, pt.wrapping_sub_ct(ct));
                svec![DopValue::Ctx]
            }
            MULS { dst, src, cst } => {
                let ct = context.read_ct_reg(src);
                let pt = context.read_pt(cst);
                context.write_ct_reg(dst, ct.wrapping_mul_pt(pt));
                svec![DopValue::Ctx]
            }

            // ── Memory: load / store ─────────────────────────────────
            LD { dst, src } => {
                let ct = context.read_ct_mem(src);
                context.write_ct_reg(dst, ct);
                svec![DopValue::Ctx]
            }
            ST { dst, src } => {
                let ct = context.read_ct_reg(src);
                context.write_ct_mem(dst, ct);
                svec![DopValue::Ctx]
            }

            // ── PBS: single output (regular + flush) ─────────────────
            PBS { dst, src, lut } | PBS_F { dst, src, lut } => {
                let ct = context.read_ct_reg(src);
                let lut_id = DopInterpreterContext::resolve_lut(lut);
                let lut_def = context.lut_reg.get_l1(&lut_id);
                context.write_ct_reg(dst, lut_def.lookup(ct, LookupCheck::AllowBothPadding));
                svec![DopValue::Ctx]
            }

            // ── PBS: 2-output many-LUT ───────────────────────────────
            PBS_ML2 { dst, src, lut } | PBS_ML2_F { dst, src, lut } => {
                let ct = context.read_ct_reg(src);
                let lut_id = DopInterpreterContext::resolve_lut(lut);
                let lut_def = context.lut_reg.get_l2(&lut_id);
                let (ct0, ct1) = lut_def.lookup(ct, LookupCheck::AllowOutputPadding);
                // Write to consecutive registers from the aligned base.
                let base = dst.addr & dst.mask;
                context.registers[base] = Some(ct0);
                context.registers[base + 1] = Some(ct1);
                svec![DopValue::Ctx]
            }

            // ── PBS: 4-output many-LUT ───────────────────────────────
            PBS_ML4 { .. } | PBS_ML4_F { .. } => {
                panic!("PBS_ML4 interpretation not implementd.")
            }

            // ── PBS: 8-output many-LUT ───────────────────────────────
            PBS_ML8 { .. } | PBS_ML8_F { .. } => {
                panic!("PBS_ML8 interpretation not implementd.")
            }

            // ── Control ──────────────────────────────────────────────
            _START => svec![DopValue::Ctx],
            _END => svec![],
            SYNC => svec![DopValue::Ctx],
            WAIT { .. } | NOTIFY { .. } | LD_B2B { .. } => panic!("Multi-HPU not supported yet."),
        }
    }
}
