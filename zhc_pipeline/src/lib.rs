//! Pipeline infrastructure for HPU compilation.
//!
//! This crate provides the core compilation pipeline that transforms high-level
//! integer operations into executable device operations for HPU hardware. The
//! pipeline consists of translation from IOP language to HPU language,
//! operation scheduling, register allocation, and final code generation.

use allocator::allocate_registers;
// use batcher::batch;
// use scheduler::schedule;
use translation_table::{DOpRepr, generate_translation_table};
use zhc_builder::if_then_else;
use zhc_builder::if_then_zero;
use zhc_builder::{cmp_eq, cmp_gt, cmp_gte, cmp_lt, cmp_lte, cmp_neq};
use zhc_ir::cse::eliminate_common_subexpressions;
use zhc_ir::dce::eliminate_dead_code;
use zhc_langs::ioplang::cut_transfers;
use zhc_langs::ioplang::eliminate_aliases;

pub mod allocator;
pub mod batch_scheduler;
// pub mod batcher;
pub mod interpreter;
pub mod latency;
// pub mod scheduler;
pub mod translation;
pub mod translation_table;

use zhc_langs::ioplang::isolate_subgraphs;
pub use zhc_sim::hpu::HpuConfig;
pub use zhc_sim::{Cycle, MHz};

/// Iops supported by the pipeline.
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Iop {
    CmpGt,
    CmpGte,
    CmpLt,
    CmpLte,
    CmpEq,
    CmpNeq,
    IfThenElse,
    IfThenZero,
}

pub use zhc_builder::CiphertextSpec;

use crate::batch_scheduler::batch_schedule;
use crate::translation::lower_iop_to_hpu;

fn hpu_pipeline(hpu_config: &HpuConfig, spec: CiphertextSpec, iop: Iop) -> Vec<DOpRepr> {
    let mut ir = match iop {
        Iop::CmpGt => cmp_gt(spec).into_ir(),
        Iop::CmpGte => cmp_gte(spec).into_ir(),
        Iop::CmpLt => cmp_lt(spec).into_ir(),
        Iop::CmpLte => cmp_lte(spec).into_ir(),
        Iop::CmpEq => cmp_eq(spec).into_ir(),
        Iop::CmpNeq => cmp_neq(spec).into_ir(),
        Iop::IfThenElse => if_then_else(spec).into_ir(),
        Iop::IfThenZero => if_then_zero(spec).into_ir(),
    };
    eliminate_aliases(&mut ir);
    eliminate_dead_code(&mut ir);
    eliminate_common_subexpressions(&mut ir);
    let unscheduled = lower_iop_to_hpu(&ir);
    let batched = batch_schedule(&unscheduled, &hpu_config);
    let allocated = allocate_registers(&batched, &hpu_config);
    generate_translation_table(&allocated)
}

fn multi_hpu_pipeline(hpu_config: &HpuConfig, spec: CiphertextSpec, iop: Iop) -> Vec<Vec<DOpRepr>> {
    let mut ir = match iop {
        Iop::CmpGt => cmp_gt(spec).into_ir(),
        Iop::CmpGte => cmp_gte(spec).into_ir(),
        Iop::CmpLt => cmp_lt(spec).into_ir(),
        Iop::CmpLte => cmp_lte(spec).into_ir(),
        Iop::CmpEq => cmp_eq(spec).into_ir(),
        Iop::CmpNeq => cmp_neq(spec).into_ir(),
        Iop::IfThenElse => if_then_else(spec).into_ir(),
        Iop::IfThenZero => if_then_zero(spec).into_ir(),
    };
    eliminate_aliases(&mut ir);
    eliminate_dead_code(&mut ir);
    eliminate_common_subexpressions(&mut ir);
    cut_transfers(&mut ir);
    let components = isolate_subgraphs(&ir);
    let mut output = Vec::new();
    for comp in components.into_iter() {
        let unscheduled = lower_iop_to_hpu(&comp);
        let batched = batch_schedule(&unscheduled, &hpu_config);
        let allocated = allocate_registers(&batched, &hpu_config);
        output.push(generate_translation_table(&allocated));
    }
    output
}

/// Generates a translation table for the specified operation configuration.
///
/// Takes the HPU hardware configuration in `hpu_config`, integer arithmetic
/// configuration in `integer_config`, and the desired operation `iop` to
/// produce an hex stream.
pub fn get_translation_table(
    hpu_config: &HpuConfig,
    spec: CiphertextSpec,
    iop: Iop,
) -> Vec<DOpRepr> {
    hpu_pipeline(hpu_config, spec, iop)
}

#[cfg(test)]
mod test;
