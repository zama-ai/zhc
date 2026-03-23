use std::fmt::Debug;
use std::mem::MaybeUninit;

use zhc_crypto::integer_semantics::{
    CiphertextBlockSpec, EmulatedCiphertextBlock, EmulatedPlaintextBlock,
    EmulatedPlaintextBlockStorage, lut::LookupCheck,
};
use zhc_ir::interpretation::{Interpretable, Interpretation, InterpretsTo};
use zhc_utils::small::SmallVec;
use zhc_utils::{FastMap, svec};

use crate::hpulang::LutId;
use crate::ioplang::{Lut1Def, Lut2Def, Lut4Def, Lut8Def};

use super::{Argument, DopTypeSystem};

/// Interpretation domain for DOP programs.
///
/// DOP uses context-threading: all data flows through inline
/// [`Argument`] operands, not SSA values. The single `Ctx` variant
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

impl Interpretation for DopValue {}

impl InterpretsTo<DopValue> for DopTypeSystem {
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
    pub registers: SmallVec<MaybeUninit<EmulatedCiphertextBlock>>,
    /// Heap memory, keyed by heap slot address.
    pub heap: FastMap<usize, EmulatedCiphertextBlock>,
    /// I/O memory, keyed by I/O slot address.
    pub io: FastMap<usize, EmulatedCiphertextBlock>,
    /// Reverse LUT table: LutId → Lut1Def (for PBS / PBS_F).
    pub lut1_table: FastMap<LutId, Lut1Def>,
    /// Reverse LUT table: LutId → Lut2Def (for PBS_ML2 / PBS_ML2_F).
    pub lut2_table: FastMap<LutId, Lut2Def>,
    /// Reverse LUT table: LutId → Lut4Def (for PBS_ML4 / PBS_ML4_F).
    pub lut4_table: FastMap<LutId, Lut4Def>,
    /// Reverse LUT table: LutId → Lut8Def (for PBS_ML8 / PBS_ML8_F).
    pub lut8_table: FastMap<LutId, Lut8Def>,
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
    pub fn new(spec: CiphertextBlockSpec, num_registers: usize) -> Self {
        Self {
            spec,
            registers: svec![MaybeUninit::uninit(); num_registers],
            heap: FastMap::default(),
            io: FastMap::default(),
            lut1_table: FastMap::default(),
            lut2_table: FastMap::default(),
            lut4_table: FastMap::default(),
            lut8_table: FastMap::default(),
            sources: FastMap::default(),
            destinations: FastMap::default(),
            pt_sources: FastMap::default(),
        }
    }

    /// Reads a ciphertext block from the machine state.
    fn read_ct(&self, arg: &Argument) -> EmulatedCiphertextBlock {
        match arg {
            // SAFETY: execution order guarantees the slot was written before read.
            Argument::CtReg { addr, .. } => unsafe { self.registers[*addr].assume_init() },
            Argument::CtHeap { addr } => self
                .heap
                .get(addr)
                .unwrap_or_else(|| panic!("Heap CT_H({addr}) not populated"))
                .clone(),
            Argument::CtIo { addr } => self
                .io
                .get(addr)
                .unwrap_or_else(|| panic!("I/O CT_IO({addr}) not populated"))
                .clone(),
            Argument::CtSrcVar { id, block } => self
                .sources
                .get(&(*id, *block))
                .unwrap_or_else(|| panic!("Source TC({id}, {block}) not populated"))
                .clone(),
            _ => panic!("Expected ciphertext argument, got {arg:?}"),
        }
    }

    /// Writes a ciphertext block to the machine state.
    fn write_ct(&mut self, arg: &Argument, val: EmulatedCiphertextBlock) {
        match arg {
            Argument::CtReg { addr, .. } => self.registers[*addr] = MaybeUninit::new(val),
            Argument::CtHeap { addr } => {
                self.heap.insert(*addr, val);
            }
            Argument::CtIo { addr } => {
                self.io.insert(*addr, val);
            }
            Argument::CtDstVar { id, block } => {
                self.destinations.insert((*id, *block), val);
            }
            _ => panic!("Expected ciphertext destination, got {arg:?}"),
        }
    }

    /// Builds a plaintext block from an inline constant argument.
    fn read_pt(&self, arg: &Argument) -> EmulatedPlaintextBlock {
        match arg {
            Argument::PtConst { val } => self
                .spec
                .matching_plaintext_block_spec()
                .from_message(*val as EmulatedPlaintextBlockStorage),
            Argument::PtSrcVar { id, block } => self
                .pt_sources
                .get(&(*id, *block))
                .unwrap_or_else(|| panic!("Plaintext TI({id}, {block}) not populated"))
                .clone(),
            _ => panic!("Expected plaintext argument, got {arg:?}"),
        }
    }

    /// Extracts a LutId from an argument.
    fn resolve_lut_id(arg: &Argument) -> LutId {
        match arg {
            Argument::LutId { id } => LutId(*id),
            _ => panic!("Expected LutId, got {arg:?}"),
        }
    }
}

impl Interpretable<DopValue> for super::DopInstructionSet {
    type Context = DopInterpreterContext;

    fn interpret(
        &self,
        context: &mut Self::Context,
        _arguments: SmallVec<DopValue>,
    ) -> SmallVec<DopValue> {
        use super::DopInstructionSet::*;
        match self {
            // ── ALU: register arithmetic ─────────────────────────────
            ADD { dst, src1, src2 } => {
                let left = context.read_ct(src1);
                let right = context.read_ct(src2);
                context.write_ct(dst, left.wrapping_add(right));
                svec![DopValue::Ctx]
            }
            SUB { dst, src1, src2 } => {
                let left = context.read_ct(src1);
                let right = context.read_ct(src2);
                context.write_ct(dst, left.wrapping_sub(right));
                svec![DopValue::Ctx]
            }
            MAC {
                dst, src1, src2, ..
            } => {
                // dst = src1 * cst + src2
                let left = context.read_ct(src1);
                let right = context.read_ct(src2);
                context.write_ct(
                    dst,
                    left.wrapping_shl(left.spec().message_size())
                        .wrapping_add(right),
                );
                svec![DopValue::Ctx]
            }
            ADDS { dst, src, cst } => {
                let ct = context.read_ct(src);
                let pt = context.read_pt(cst);
                context.write_ct(dst, ct.wrapping_add_pt(pt));
                svec![DopValue::Ctx]
            }
            SUBS { dst, src, cst } => {
                let ct = context.read_ct(src);
                let pt = context.read_pt(cst);
                context.write_ct(dst, ct.wrapping_sub_pt(pt));
                svec![DopValue::Ctx]
            }
            SSUB { dst, src, cst } => {
                let pt = context.read_pt(cst);
                let ct = context.read_ct(src);
                context.write_ct(dst, pt.wrapping_sub_ct(ct));
                svec![DopValue::Ctx]
            }
            MULS { dst, src, cst } => {
                let ct = context.read_ct(src);
                let pt = context.read_pt(cst);
                context.write_ct(dst, ct.wrapping_mul(pt));
                svec![DopValue::Ctx]
            }

            // ── Memory: load / store ─────────────────────────────────
            LD { dst, src } => {
                let ct = context.read_ct(src);
                context.write_ct(dst, ct);
                svec![DopValue::Ctx]
            }
            ST { dst, src } => {
                let ct = context.read_ct(src);
                context.write_ct(dst, ct);
                svec![DopValue::Ctx]
            }

            // ── PBS: single output (regular + flush) ─────────────────
            PBS { dst, src, lut } | PBS_F { dst, src, lut } => {
                let ct = context.read_ct(src);
                let lut_id = DopInterpreterContext::resolve_lut_id(lut);
                let lut_def = context
                    .lut1_table
                    .get(&lut_id)
                    .unwrap_or_else(|| panic!("Lut1 {lut_id} missing from context"));
                context.write_ct(dst, lut_def.lookup(ct, LookupCheck::AllowBothPadding));
                svec![DopValue::Ctx]
            }

            // ── PBS: 2-output many-LUT ───────────────────────────────
            PBS_ML2 { dst, src, lut } | PBS_ML2_F { dst, src, lut } => {
                let ct = context.read_ct(src);
                let lut_id = DopInterpreterContext::resolve_lut_id(lut);
                let lut_def = context
                    .lut2_table
                    .get(&lut_id)
                    .unwrap_or_else(|| panic!("Lut2 {lut_id} missing from context"));
                let (ct0, ct1) = lut_def.lookup(ct);
                // Write to consecutive registers from the aligned base.
                let Argument::CtReg { addr, mask } = dst else {
                    panic!("PBS_ML2 dst must be CtReg, got {dst:?}");
                };
                let base = addr & mask;
                context.registers[base] = MaybeUninit::new(ct0);
                context.registers[base + 1] = MaybeUninit::new(ct1);
                svec![DopValue::Ctx]
            }

            // ── PBS: 4-output many-LUT ───────────────────────────────
            PBS_ML4 { dst, src, lut } | PBS_ML4_F { dst, src, lut } => {
                let ct = context.read_ct(src);
                let lut_id = DopInterpreterContext::resolve_lut_id(lut);
                let lut_def = context
                    .lut4_table
                    .get(&lut_id)
                    .unwrap_or_else(|| panic!("Lut4 {lut_id} missing from context"));
                let (ct0, ct1, ct2, ct3) = lut_def.lookup(ct);
                let Argument::CtReg { addr, mask } = dst else {
                    panic!("PBS_ML4 dst must be CtReg, got {dst:?}");
                };
                let base = addr & mask;
                context.registers[base] = MaybeUninit::new(ct0);
                context.registers[base + 1] = MaybeUninit::new(ct1);
                context.registers[base + 2] = MaybeUninit::new(ct2);
                context.registers[base + 3] = MaybeUninit::new(ct3);
                svec![DopValue::Ctx]
            }

            // ── PBS: 8-output many-LUT ───────────────────────────────
            PBS_ML8 { dst, src, lut } | PBS_ML8_F { dst, src, lut } => {
                let ct = context.read_ct(src);
                let lut_id = DopInterpreterContext::resolve_lut_id(lut);
                let lut_def = context
                    .lut8_table
                    .get(&lut_id)
                    .unwrap_or_else(|| panic!("Lut8 {lut_id} missing from context"));
                let (ct0, ct1, ct2, ct3, ct4, ct5, ct6, ct7) = lut_def.lookup(ct);
                let Argument::CtReg { addr, mask } = dst else {
                    panic!("PBS_ML8 dst must be CtReg, got {dst:?}");
                };
                let base = addr & mask;
                context.registers[base] = MaybeUninit::new(ct0);
                context.registers[base + 1] = MaybeUninit::new(ct1);
                context.registers[base + 2] = MaybeUninit::new(ct2);
                context.registers[base + 3] = MaybeUninit::new(ct3);
                context.registers[base + 4] = MaybeUninit::new(ct4);
                context.registers[base + 5] = MaybeUninit::new(ct5);
                context.registers[base + 6] = MaybeUninit::new(ct6);
                context.registers[base + 7] = MaybeUninit::new(ct7);
                svec![DopValue::Ctx]
            }

            // ── Control ──────────────────────────────────────────────
            _INIT => svec![DopValue::Ctx],
            SYNC => svec![],
            WAIT { .. } | NOTIFY { .. } | LD_B2B { .. } => panic!("Multi-HPU not supported yet."),
        }
    }
}
