use allocator::allocate_registers;
use scheduler::schedule;
use translation_table::{generate_translation_table, DOpRepr};
use hpuc_builder::iops::cmp::{cmp_eq, cmp_gt, cmp_gte, cmp_lt, cmp_lte, cmp_neq};
use hpuc_ir::translation::Translator;
use translation::IoplangToHpulang;

pub mod scheduler;
pub mod allocator;
pub mod translation;
pub mod latency;
pub mod translation_table;

pub use hpuc_builder::builder::IntegerConfig;
pub use hpuc_sim::hpu::HpuConfig;
pub use hpuc_sim::{MHz, Cycle};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Iop {
    CmpGt,
    CmpGte,
    CmpLt,
    CmpLte,
    CmpEq,
    CmpNeq
}

fn pipeline(hpu_config: &HpuConfig, integer_config: &IntegerConfig, iop: Iop) -> Vec<DOpRepr> {
    let ir = match iop {
        Iop::CmpGt => cmp_gt(integer_config),
        Iop::CmpGte => cmp_gte(integer_config),
        Iop::CmpLt => cmp_lt(integer_config),
        Iop::CmpLte => cmp_lte(integer_config),
        Iop::CmpEq => cmp_eq(integer_config),
        Iop::CmpNeq => cmp_neq(integer_config),
    };
    let unscheduled = IoplangToHpulang.translate(&ir);
    let scheduled = schedule(&unscheduled, hpu_config);
    let allocated = allocate_registers(&scheduled, &hpu_config);
    generate_translation_table(&allocated)
}

pub fn get_translation_table(hpu_config: &HpuConfig, integer_config: &IntegerConfig, iop: Iop) -> Vec<DOpRepr> {
    pipeline(hpu_config, integer_config, iop)
}

#[cfg(test)]
mod test;
