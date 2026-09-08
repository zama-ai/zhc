use zhc_config::hpu::HpuConfig;
use zhc_crypto::integer_semantics::lut::LutRegistry;
use zhc_ir::{AnnIR, IR};
use zhc_langs::{doplang::DopLang, hpulang::HpuLang};
use zhc_utils::SafeAs;

mod allocator;
mod batch_map;
mod heap;
mod live_range;
mod register_file;
mod register_state;
mod translator;
mod value_state;

/// Allocates physical registers to values in the scheduled IR.
///
/// Takes a scheduled intermediate representation `ir` containing HPU operations
/// and the hardware configuration `config` to produce a new IR in the device
/// operation language with physical register assignments for all values.
pub fn allocate_registers(
    ir: &IR<HpuLang>,
    config: &HpuConfig,
    lut_reg: &LutRegistry,
) -> IR<DopLang> {
    let allocator = allocator::Allocator::init(
        ir,
        config.regf_size,
        config.isc_depth.sas(),
        config.heap_size,
    );
    let allocation = allocator.allocate_registers();
    let annir = AnnIR::new(ir, allocation, ir.filled_valmap(()));
    translator::translate(&annir, &lut_reg)
}
