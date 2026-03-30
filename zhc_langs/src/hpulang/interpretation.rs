use std::fmt::Debug;

use zhc_crypto::integer_semantics::{
    CiphertextBlockSpec, EmulatedCiphertextBlock, EmulatedCiphertextBlockStorage,
    EmulatedPlaintextBlock, EmulatedPlaintextBlockStorage, lut::LookupCheck,
};
use zhc_ir::interpretation::{Interpretable, Interpretation, InterpretsTo, interpret_ir};
use zhc_utils::iter::CollectInSmallVec;
use zhc_utils::small::SmallVec;
use zhc_utils::{FastMap, svec};

use crate::hpulang::{HpuTypeSystem, LutId, TDstId, TImmId, TSrcId};
use crate::ioplang::{Lut1Def, Lut2Def};

/// Interpretation domain for HPU programs.
///
/// Each variant corresponds to a [`HpuTypeSystem`]
/// type. `CtRegister` and `CtHeap` both wrap an
/// [`EmulatedCiphertextBlock`]; the distinction is purely locational.
#[derive(Clone, Hash, PartialEq, Eq)]
pub enum HpuValue {
    CtRegister(EmulatedCiphertextBlock),
    PtImmediate(EmulatedPlaintextBlock),
    CtHeap(EmulatedCiphertextBlock),
}

impl HpuValue {
    pub fn unwrap_ct_register(self) -> EmulatedCiphertextBlock {
        match self {
            Self::CtRegister(v) => v,
            _ => panic!("Expected CtRegister, got {:?}", self),
        }
    }

    pub fn unwrap_pt_immediate(self) -> EmulatedPlaintextBlock {
        match self {
            Self::PtImmediate(v) => v,
            _ => panic!("Expected PtImmediate, got {:?}", self),
        }
    }

    pub fn unwrap_ct_heap(self) -> EmulatedCiphertextBlock {
        match self {
            Self::CtHeap(v) => v,
            _ => panic!("Expected CtHeap, got {:?}", self),
        }
    }
}

impl Debug for HpuValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CtRegister(a) => a.fmt(f),
            Self::PtImmediate(a) => a.fmt(f),
            Self::CtHeap(a) => a.fmt(f),
        }
    }
}

impl Interpretation for HpuValue {}

impl InterpretsTo<HpuValue> for HpuTypeSystem {
    fn type_of(val: &HpuValue) -> Self {
        match val {
            HpuValue::CtRegister(..) => Self::CtRegister,
            HpuValue::PtImmediate(..) => Self::PtImmediate,
            HpuValue::CtHeap(..) => Self::CtHeap,
        }
    }
}

/// Execution context for HPU program interpretation.
///
/// Holds cryptographic parameters (`spec`), block-level I/O maps, and
/// reverse LUT tables for PBS resolution. The caller must populate
/// `sources`, `immediates`, and LUT tables before interpretation.
///
/// `destinations` is populated during interpretation by `DstSt`
/// instructions. `batch_args` / `batch_rets` are managed internally
/// during recursive `Batch` interpretation.
#[derive(Debug)]
pub struct HpuInterpreterContext {
    pub spec: CiphertextBlockSpec,
    /// Ciphertext block inputs, keyed by source identifier.
    pub sources: FastMap<TSrcId, EmulatedCiphertextBlock>,
    /// Ciphertext block outputs, populated during interpretation.
    pub destinations: FastMap<TDstId, EmulatedCiphertextBlock>,
    /// Plaintext block inputs, keyed by immediate identifier.
    pub immediates: FastMap<TImmId, EmulatedPlaintextBlock>,
    /// Reverse LUT table: LutId → Lut1Def (for Pbs/PbsF).
    pub lut1_table: FastMap<LutId, Lut1Def>,
    /// Reverse LUT table: LutId → Lut2Def (for Pbs2/Pbs2F).
    pub lut2_table: FastMap<LutId, Lut2Def>,
    // Lut4/Lut8 tables omitted: the corresponding enums are uninhabited.
    /// Batch argument state for nested `Batch` interpretation.
    batch_args: FastMap<u8, HpuValue>,
    /// Batch return state for nested `Batch` interpretation.
    batch_rets: FastMap<u8, HpuValue>,
}

impl HpuInterpreterContext {
    /// Creates a new context with the given spec and empty I/O maps.
    pub fn new(spec: CiphertextBlockSpec) -> Self {
        Self {
            spec,
            sources: FastMap::default(),
            destinations: FastMap::default(),
            immediates: FastMap::default(),
            lut1_table: FastMap::default(),
            lut2_table: FastMap::default(),
            batch_args: FastMap::default(),
            batch_rets: FastMap::default(),
        }
    }
}

impl Interpretable<HpuValue> for super::HpuInstructionSet {
    type Context = HpuInterpreterContext;

    fn interpret(
        &self,
        context: &mut Self::Context,
        arguments: SmallVec<HpuValue>,
    ) -> SmallVec<HpuValue> {
        use super::HpuInstructionSet::*;
        match self {
            // ── Memory transfer ──────────────────────────────────────
            SrcLd { from } => {
                let ct = context
                    .sources
                    .get(from)
                    .unwrap_or_else(|| panic!("Source {from} missing from context"));
                svec![HpuValue::CtRegister(ct.clone())]
            }
            DstSt { to } => {
                assert!(
                    !context.destinations.contains_key(to),
                    "Destination {to} already written"
                );
                context
                    .destinations
                    .insert(to.clone(), arguments[0].clone().unwrap_ct_register());
                svec![]
            }
            ImmLd { from } => {
                let pt = context
                    .immediates
                    .get(from)
                    .unwrap_or_else(|| panic!("Immediate {from} missing from context"));
                svec![HpuValue::PtImmediate(pt.clone())]
            }

            // ── Constants ────────────────────────────────────────────
            CstCt { cst } => {
                svec![HpuValue::CtRegister(
                    context
                        .spec
                        .from_message(cst.0 as EmulatedCiphertextBlockStorage)
                )]
            }

            // ── Register arithmetic (ct, ct) ─────────────────────────
            AddCt => {
                let left = arguments[0].clone().unwrap_ct_register();
                let right = arguments[1].clone().unwrap_ct_register();
                svec![HpuValue::CtRegister(left.wrapping_add(right))]
            }
            SubCt => {
                let left = arguments[0].clone().unwrap_ct_register();
                let right = arguments[1].clone().unwrap_ct_register();
                svec![HpuValue::CtRegister(left.wrapping_sub(right))]
            }
            Mac { cst } => {
                // src1 * cst + src2
                let left = arguments[0].clone().unwrap_ct_register();
                let right = arguments[1].clone().unwrap_ct_register();
                assert_eq!(cst.0, 2u8.pow(left.spec().message_size() as u32));
                svec![HpuValue::CtRegister(
                    left.wrapping_shl(left.spec().message_size())
                        .wrapping_add(right)
                )]
            }

            // ── Register arithmetic (ct, pt) ─────────────────────────
            AddPt => {
                let ct = arguments[0].clone().unwrap_ct_register();
                let pt = arguments[1].clone().unwrap_pt_immediate();
                svec![HpuValue::CtRegister(ct.wrapping_add_pt(pt))]
            }
            SubPt => {
                let ct = arguments[0].clone().unwrap_ct_register();
                let pt = arguments[1].clone().unwrap_pt_immediate();
                svec![HpuValue::CtRegister(ct.wrapping_sub_pt(pt))]
            }
            PtSub => {
                let pt = arguments[0].clone().unwrap_pt_immediate();
                let ct = arguments[1].clone().unwrap_ct_register();
                svec![HpuValue::CtRegister(pt.wrapping_sub_ct(ct))]
            }
            MulPt => {
                let ct = arguments[0].clone().unwrap_ct_register();
                let pt = arguments[1].clone().unwrap_pt_immediate();
                svec![HpuValue::CtRegister(ct.wrapping_mul(pt))]
            }

            // ── Inline constant arithmetic ───────────────────────────
            AddCst { cst } => {
                let ct = arguments[0].clone().unwrap_ct_register();
                let pt = context
                    .spec
                    .complete_plaintext_block_spec()
                    .from_message(cst.0 as EmulatedPlaintextBlockStorage);
                svec![HpuValue::CtRegister(ct.wrapping_add_pt(pt))]
            }
            SubCst { cst } => {
                let ct = arguments[0].clone().unwrap_ct_register();
                let pt = context
                    .spec
                    .complete_plaintext_block_spec()
                    .from_message(cst.0 as EmulatedPlaintextBlockStorage);
                svec![HpuValue::CtRegister(ct.wrapping_sub_pt(pt))]
            }
            CstSub { cst } => {
                let ct = arguments[0].clone().unwrap_ct_register();
                let pt = context
                    .spec
                    .complete_plaintext_block_spec()
                    .from_message(cst.0 as EmulatedPlaintextBlockStorage);
                svec![HpuValue::CtRegister(pt.wrapping_sub_ct(ct))]
            }
            MulCst { cst } => {
                let ct = arguments[0].clone().unwrap_ct_register();
                let pt = context
                    .spec
                    .complete_plaintext_block_spec()
                    .from_message(cst.0 as EmulatedPlaintextBlockStorage);
                svec![HpuValue::CtRegister(ct.wrapping_mul(pt))]
            }

            // ── PBS (regular + flush, semantically identical) ────────
            Pbs { lut } | PbsF { lut } => {
                let ct = arguments[0].clone().unwrap_ct_register();
                let lut_def = context
                    .lut1_table
                    .get(lut)
                    .unwrap_or_else(|| panic!("Lut1 {lut} missing from context"));
                svec![HpuValue::CtRegister(
                    lut_def.lookup(ct, LookupCheck::AllowBothPadding)
                )]
            }
            Pbs2 { lut } | Pbs2F { lut } => {
                let ct = arguments[0].clone().unwrap_ct_register();
                let lut_def = context
                    .lut2_table
                    .get(lut)
                    .unwrap_or_else(|| panic!("Lut2 {lut} missing from context"));
                let (ct0, ct1) = lut_def.lookup(ct);
                svec![HpuValue::CtRegister(ct0), HpuValue::CtRegister(ct1)]
            }
            Pbs4 { .. } | Pbs4F { .. } => {
                panic!("Pbs4 interpretation not yet supported (Lut4Def is uninhabited)")
            }
            Pbs8 { .. } | Pbs8F { .. } => {
                panic!("Pbs8 interpretation not yet supported (Lut8Def is uninhabited)")
            }

            // ── Batching ─────────────────────────────────────────────
            BatchArg { pos, .. } => {
                let val = context
                    .batch_args
                    .get(pos)
                    .unwrap_or_else(|| panic!("BatchArg {pos} missing from batch context"));
                svec![val.clone()]
            }
            BatchRet { pos, .. } => {
                assert!(
                    !context.batch_rets.contains_key(pos),
                    "BatchRet {pos} already written"
                );
                context.batch_rets.insert(*pos, arguments[0].clone());
                svec![]
            }
            Batch { block } => {
                // Save parent batch state for nesting.
                let saved_args = std::mem::take(&mut context.batch_args);
                let saved_rets = std::mem::take(&mut context.batch_rets);

                // Collect BatchArg positions sorted, then populate batch_args.
                let mut arg_positions: SmallVec<u8> = block
                    .walk_ops_linear()
                    .filter_map(|op| match op.get_instruction() {
                        BatchArg { pos, .. } => Some(pos),
                        _ => None,
                    })
                    .cosvec();
                arg_positions.sort_unstable();
                for (val, pos) in arguments.iter().zip(arg_positions.iter()) {
                    context.batch_args.insert(*pos, val.clone());
                }

                // Recursively interpret the nested block.
                interpret_ir::<super::HpuLang, HpuValue>(block, context)
                    .expect("Batch interpretation failed");

                // Collect BatchRet values in sorted position order.
                let mut ret_positions: SmallVec<u8> = block
                    .walk_ops_linear()
                    .filter_map(|op| match op.get_instruction() {
                        BatchRet { pos, .. } => Some(pos),
                        _ => None,
                    })
                    .cosvec();
                ret_positions.sort_unstable();
                let returns: SmallVec<HpuValue> = ret_positions
                    .iter()
                    .map(|pos| {
                        context
                            .batch_rets
                            .remove(pos)
                            .unwrap_or_else(|| panic!("BatchRet {pos} not produced"))
                    })
                    .cosvec();

                // Restore parent batch state.
                context.batch_args = saved_args;
                context.batch_rets = saved_rets;

                returns
            }
        }
    }
}
