//! C representations of the plain-data types crossing the ABI, the opaque handles with their
//! constructors and destructors, and the status code returned by every function.

use std::ffi::{CStr, c_char};
use std::panic::{AssertUnwindSafe, catch_unwind};
use zhc_builder::{
    BoolCiphertext, CiphertextBlock, CiphertextBlockSpec, Flavor, IntegerCiphertext,
    IntegerPlaintext, IrKind, LookupCheck, Lut1, Lut2, Lut4, Lut8, PlaintextBlock,
};
use zhc_config::{hpu::HpuConfig, multi_hpu::MultiHpuConfig, vm::VmConfig};
use zhc_pipeline::{HpuMetrics, MultiHpuMetrics};
use zhc_utils::{
    files::{FileHandle, PerfettoTrace},
    units::{Cycle, MHz},
};

// ---------------------------------------------------------------------------------------------
// Status
// ---------------------------------------------------------------------------------------------

/// Return code of every function of the C API.
#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum zhc_status {
    /// The call succeeded.
    Ok = 0,
    /// The bound Rust method panicked. The message was printed by the panic hook.
    Panic = 1,
    /// A pointer argument was null.
    NullPointer = 2,
    /// An argument was rejected before calling the bound Rust method.
    InvalidArgument = 3,
    /// The bound Rust method returned an I/O error.
    Io = 4,
}

pub(crate) type Fallible<T = ()> = Result<T, zhc_status>;

/// Runs `f`, converting a panic into [`zhc_status::Panic`] instead of aborting the process.
pub(crate) fn guard(f: impl FnOnce() -> Fallible) -> zhc_status {
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(Ok(())) => zhc_status::Ok,
        Ok(Err(status)) => status,
        Err(_) => zhc_status::Panic,
    }
}

// ---------------------------------------------------------------------------------------------
// Plain data
// ---------------------------------------------------------------------------------------------

/// Binding of [`CiphertextBlockSpec`].
#[repr(C)]
#[derive(Clone, Copy)]
pub struct zhc_ciphertext_block_spec {
    pub carry: u8,
    pub message: u8,
}

impl From<zhc_ciphertext_block_spec> for CiphertextBlockSpec {
    fn from(spec: zhc_ciphertext_block_spec) -> Self {
        CiphertextBlockSpec(spec.carry, spec.message)
    }
}

impl From<CiphertextBlockSpec> for zhc_ciphertext_block_spec {
    fn from(spec: CiphertextBlockSpec) -> Self {
        zhc_ciphertext_block_spec {
            carry: spec.0,
            message: spec.1,
        }
    }
}

/// Binding of [`Flavor`].
#[repr(C)]
#[derive(Clone, Copy)]
pub enum zhc_flavor {
    Protect,
    Temper,
    Wrapping,
}

impl From<zhc_flavor> for Flavor {
    fn from(flavor: zhc_flavor) -> Self {
        match flavor {
            zhc_flavor::Protect => Flavor::Protect,
            zhc_flavor::Temper => Flavor::Temper,
            zhc_flavor::Wrapping => Flavor::Wrapping,
        }
    }
}

/// Binding of [`LookupCheck`].
#[repr(C)]
#[derive(Clone, Copy)]
pub struct zhc_lookup_check {
    pub allow_input_padding: bool,
    pub allow_index_bits: bool,
    pub allow_output_padding: bool,
}

impl From<zhc_lookup_check> for LookupCheck {
    fn from(check: zhc_lookup_check) -> Self {
        LookupCheck {
            allow_input_padding: check.allow_input_padding,
            allow_index_bits: check.allow_index_bits,
            allow_output_padding: check.allow_output_padding,
        }
    }
}

/// Binding of [`IrKind`].
#[repr(C)]
#[derive(Clone, Copy)]
pub enum zhc_ir_kind {
    Original,
    Optimized,
}

impl From<zhc_ir_kind> for IrKind {
    fn from(kind: zhc_ir_kind) -> Self {
        match kind {
            zhc_ir_kind::Original => IrKind::Original,
            zhc_ir_kind::Optimized => IrKind::Optimized,
        }
    }
}

/// Binding of [`HpuConfig`]. Field for field; the `MHz` and `Cycle` newtypes are flattened.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct zhc_hpu_config {
    pub freq: usize,
    pub isc_depth: usize,
    pub isc_query_period: usize,
    pub mem_fifo_capacity: usize,
    pub mem_read_latency: usize,
    pub mem_write_latency: usize,
    pub alu_fifo_capacity: usize,
    pub alu_read_latency: usize,
    pub alu_write_latency: usize,
    pub pbs_fifo_capacity: usize,
    pub pbs_memory_capacity: usize,
    pub pbs_min_batch_size: usize,
    pub pbs_max_batch_size: usize,
    pub pbs_timeout: usize,
    pub pbs_load_unload_latency: usize,
    pub pbs_processing_latency_a: usize,
    pub pbs_processing_latency_b: usize,
    pub pbs_processing_latency_m: usize,
    pub regf_size: usize,
    pub heap_size: usize,
}

impl From<zhc_hpu_config> for HpuConfig {
    fn from(c: zhc_hpu_config) -> Self {
        HpuConfig {
            freq: MHz(c.freq),
            isc_depth: c.isc_depth,
            isc_query_period: Cycle(c.isc_query_period),
            mem_fifo_capacity: c.mem_fifo_capacity,
            mem_read_latency: c.mem_read_latency,
            mem_write_latency: c.mem_write_latency,
            alu_fifo_capacity: c.alu_fifo_capacity,
            alu_read_latency: c.alu_read_latency,
            alu_write_latency: c.alu_write_latency,
            pbs_fifo_capacity: c.pbs_fifo_capacity,
            pbs_memory_capacity: c.pbs_memory_capacity,
            pbs_min_batch_size: c.pbs_min_batch_size,
            pbs_max_batch_size: c.pbs_max_batch_size,
            pbs_timeout: Cycle(c.pbs_timeout),
            pbs_load_unload_latency: c.pbs_load_unload_latency,
            pbs_processing_latency_a: c.pbs_processing_latency_a,
            pbs_processing_latency_b: c.pbs_processing_latency_b,
            pbs_processing_latency_m: c.pbs_processing_latency_m,
            regf_size: c.regf_size,
            heap_size: c.heap_size,
        }
    }
}

impl From<HpuConfig> for zhc_hpu_config {
    fn from(c: HpuConfig) -> Self {
        zhc_hpu_config {
            freq: c.freq.0,
            isc_depth: c.isc_depth,
            isc_query_period: c.isc_query_period.0,
            mem_fifo_capacity: c.mem_fifo_capacity,
            mem_read_latency: c.mem_read_latency,
            mem_write_latency: c.mem_write_latency,
            alu_fifo_capacity: c.alu_fifo_capacity,
            alu_read_latency: c.alu_read_latency,
            alu_write_latency: c.alu_write_latency,
            pbs_fifo_capacity: c.pbs_fifo_capacity,
            pbs_memory_capacity: c.pbs_memory_capacity,
            pbs_min_batch_size: c.pbs_min_batch_size,
            pbs_max_batch_size: c.pbs_max_batch_size,
            pbs_timeout: c.pbs_timeout.0,
            pbs_load_unload_latency: c.pbs_load_unload_latency,
            pbs_processing_latency_a: c.pbs_processing_latency_a,
            pbs_processing_latency_b: c.pbs_processing_latency_b,
            pbs_processing_latency_m: c.pbs_processing_latency_m,
            regf_size: c.regf_size,
            heap_size: c.heap_size,
        }
    }
}

/// Binding of [`MultiHpuConfig`].
#[repr(C)]
#[derive(Clone, Copy)]
pub struct zhc_multi_hpu_config {
    pub hpu_config: zhc_hpu_config,
    pub n_hpus: u8,
}

impl From<zhc_multi_hpu_config> for MultiHpuConfig {
    fn from(c: zhc_multi_hpu_config) -> Self {
        MultiHpuConfig {
            hpu_config: c.hpu_config.into(),
            n_hpus: c.n_hpus,
        }
    }
}

impl From<MultiHpuConfig> for zhc_multi_hpu_config {
    fn from(c: MultiHpuConfig) -> Self {
        zhc_multi_hpu_config {
            hpu_config: c.hpu_config.into(),
            n_hpus: c.n_hpus,
        }
    }
}

/// Binding of [`VmConfig`].
#[repr(C)]
#[derive(Clone, Copy)]
pub struct zhc_vm_config {
    pub lwe_dim: usize,
    pub bsk_polynomial_size: usize,
    pub bsk_glwe_dim: usize,
    pub bsk_dec_levels: usize,
    pub bsk_dec_base_log: usize,
    pub ksk_dec_levels: usize,
    pub ksk_dec_base_log: usize,
    pub delta: usize,
    pub carry_size: usize,
    pub message_size: usize,
    pub regf_size: usize,
}

impl From<zhc_vm_config> for VmConfig {
    fn from(c: zhc_vm_config) -> Self {
        VmConfig {
            lwe_dim: c.lwe_dim,
            bsk_polynomial_size: c.bsk_polynomial_size,
            bsk_glwe_dim: c.bsk_glwe_dim,
            bsk_dec_levels: c.bsk_dec_levels,
            bsk_dec_base_log: c.bsk_dec_base_log,
            ksk_dec_levels: c.ksk_dec_levels,
            ksk_dec_base_log: c.ksk_dec_base_log,
            delta: c.delta,
            carry_size: c.carry_size,
            message_size: c.message_size,
            regf_size: c.regf_size,
        }
    }
}

impl From<VmConfig> for zhc_vm_config {
    fn from(c: VmConfig) -> Self {
        zhc_vm_config {
            lwe_dim: c.lwe_dim,
            bsk_polynomial_size: c.bsk_polynomial_size,
            bsk_glwe_dim: c.bsk_glwe_dim,
            bsk_dec_levels: c.bsk_dec_levels,
            bsk_dec_base_log: c.bsk_dec_base_log,
            ksk_dec_levels: c.ksk_dec_levels,
            ksk_dec_base_log: c.ksk_dec_base_log,
            delta: c.delta,
            carry_size: c.carry_size,
            message_size: c.message_size,
            regf_size: c.regf_size,
        }
    }
}

/// Binding of [`HpuMetrics`]. The `Microseconds` newtype is flattened to `f64`. The
/// `batch_stats` histogram is not exposed.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct zhc_hpu_metrics {
    pub latency: f64,
    pub lower_bound: f64,
    pub batching_overhead: f64,
    pub starvation: f64,
    pub batch_count: usize,
    pub slots_filled: usize,
    pub slots_total: usize,
    pub timeout_launches: u16,
}

impl From<&HpuMetrics> for zhc_hpu_metrics {
    fn from(m: &HpuMetrics) -> Self {
        zhc_hpu_metrics {
            latency: m.latency.0,
            lower_bound: m.lower_bound.0,
            batching_overhead: m.batching_overhead.0,
            starvation: m.starvation.0,
            batch_count: m.batch_count,
            slots_filled: m.slots_filled,
            slots_total: m.slots_total,
            timeout_launches: m.timeout_launches,
        }
    }
}

/// Binding of [`MultiHpuMetrics`]. The `Microseconds` newtype is flattened to `f64`.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct zhc_multi_hpu_metrics {
    pub latency: f64,
}

impl From<&MultiHpuMetrics> for zhc_multi_hpu_metrics {
    fn from(m: &MultiHpuMetrics) -> Self {
        zhc_multi_hpu_metrics {
            latency: m.latency.0,
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_hpu_config_default(out: *mut zhc_hpu_config) -> zhc_status {
    guard(|| unsafe { write(out, HpuConfig::default().into()) })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_multi_hpu_config_default(
    out: *mut zhc_multi_hpu_config,
) -> zhc_status {
    guard(|| unsafe { write(out, MultiHpuConfig::default().into()) })
}

// ---------------------------------------------------------------------------------------------
// Opaque handles
// ---------------------------------------------------------------------------------------------

pub struct zhc_ciphertext_block(pub(crate) CiphertextBlock);
pub struct zhc_plaintext_block(pub(crate) PlaintextBlock);
pub struct zhc_bool_ciphertext(pub(crate) BoolCiphertext);
pub struct zhc_integer_ciphertext(pub(crate) IntegerCiphertext);
pub struct zhc_integer_plaintext(pub(crate) IntegerPlaintext);

pub struct zhc_lut1(pub(crate) Lut1);
pub struct zhc_lut2(pub(crate) Lut2);
pub struct zhc_lut4(pub(crate) Lut4);
pub struct zhc_lut8(pub(crate) Lut8);

pub struct zhc_file_handle(pub(crate) FileHandle);
pub struct zhc_perfetto_trace(pub(crate) PerfettoTrace);

// ---------------------------------------------------------------------------------------------
// Pointer helpers
// ---------------------------------------------------------------------------------------------

/// Dereferences a `*const` handle to a reference to the wrapped Rust value.
///
/// # Safety
///
/// `ptr` must be null or point to a live handle.
pub(crate) unsafe fn deref<'a, T>(ptr: *const T) -> Fallible<&'a T> {
    if ptr.is_null() {
        return Err(zhc_status::NullPointer);
    }
    Ok(unsafe { &*ptr })
}

/// Dereferences a `*mut` handle to a mutable reference to the wrapped Rust value.
///
/// # Safety
///
/// `ptr` must be null or point to a live handle that is not aliased during the call.
pub(crate) unsafe fn deref_mut<'a, T>(ptr: *mut T) -> Fallible<&'a mut T> {
    if ptr.is_null() {
        return Err(zhc_status::NullPointer);
    }
    Ok(unsafe { &mut *ptr })
}

/// Writes a plain value to the caller-provided `out` location.
///
/// # Safety
///
/// `out` must be null or point to writable memory for a `T`.
pub(crate) unsafe fn write<T>(out: *mut T, value: T) -> Fallible {
    if out.is_null() {
        return Err(zhc_status::NullPointer);
    }
    unsafe { out.write(value) };
    Ok(())
}

/// Moves a value to the heap and writes the resulting handle to the caller-provided `out`
/// location.
///
/// # Safety
///
/// `out` must be null or point to writable memory for a `*mut T`.
pub(crate) unsafe fn write_handle<T>(out: *mut *mut T, value: T) -> Fallible {
    unsafe { write(out, Box::into_raw(Box::new(value))) }
}

/// Reads a C string argument.
///
/// # Safety
///
/// `s` must be null or point to a null-terminated string.
pub(crate) unsafe fn c_str(s: *const c_char) -> Fallible<String> {
    unsafe { deref(s) }?;
    Ok(unsafe { CStr::from_ptr(s) }.to_string_lossy().into_owned())
}

/// Frees a handle previously written by [`write_handle`]. Null is a no-op.
///
/// # Safety
///
/// `ptr` must be null or a handle written by [`write_handle`] that has not been freed yet.
pub(crate) unsafe fn free<T>(ptr: *mut T) {
    if !ptr.is_null() {
        drop(unsafe { Box::from_raw(ptr) });
    }
}

// ---------------------------------------------------------------------------------------------
// Handle destructors
// ---------------------------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_ciphertext_block_free(ptr: *mut zhc_ciphertext_block) {
    unsafe { free(ptr) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_plaintext_block_free(ptr: *mut zhc_plaintext_block) {
    unsafe { free(ptr) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_bool_ciphertext_free(ptr: *mut zhc_bool_ciphertext) {
    unsafe { free(ptr) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_integer_ciphertext_free(ptr: *mut zhc_integer_ciphertext) {
    unsafe { free(ptr) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_integer_plaintext_free(ptr: *mut zhc_integer_plaintext) {
    unsafe { free(ptr) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_lut1_free(ptr: *mut zhc_lut1) {
    unsafe { free(ptr) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_lut2_free(ptr: *mut zhc_lut2) {
    unsafe { free(ptr) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_lut4_free(ptr: *mut zhc_lut4) {
    unsafe { free(ptr) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_lut8_free(ptr: *mut zhc_lut8) {
    unsafe { free(ptr) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_file_handle_free(ptr: *mut zhc_file_handle) {
    unsafe { free(ptr) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_perfetto_trace_free(ptr: *mut zhc_perfetto_trace) {
    unsafe { free(ptr) }
}

// ---------------------------------------------------------------------------------------------
// File handles
// ---------------------------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_file_handle_open(handle: *const zhc_file_handle) -> zhc_status {
    guard(|| {
        unsafe { deref(handle) }?
            .0
            .open()
            .map_err(|_| zhc_status::Io)
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_file_handle_move_to(
    handle: *mut zhc_file_handle,
    path: *const c_char,
) -> zhc_status {
    guard(|| {
        let path = unsafe { c_str(path) }?;
        unsafe { deref_mut(handle) }?
            .0
            .move_to(path)
            .map_err(|_| zhc_status::Io)
    })
}

// ---------------------------------------------------------------------------------------------
// Perfetto traces
// ---------------------------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_perfetto_trace_open(trace: *const zhc_perfetto_trace) -> zhc_status {
    guard(|| {
        unsafe { deref(trace) }?
            .0
            .open()
            .map_err(|_| zhc_status::Io)
    })
}

// ---------------------------------------------------------------------------------------------
// Lookup table constructors
// ---------------------------------------------------------------------------------------------
//
// The Rust constructors take one closure per output. C has no closures, so each closure is
// replaced by a table: `table[i]` is the output data bits for input data value `i`. Every table
// has `len` entries, where `len` is the size of the input space of one sub-table, that is
// `2^(data_size - k)` for a `2^k`-output LUT.

/// Number of entries a sub-table must have for a LUT with `2^k` outputs.
fn expected_len(spec: CiphertextBlockSpec, k: u8) -> Fallible<usize> {
    let data_size = spec.data_size();
    if data_size < k {
        return Err(zhc_status::InvalidArgument);
    }
    Ok(1usize << (data_size - k))
}

/// Validates a table and returns it as a slice.
///
/// # Safety
///
/// `table` must be null or point to `len` readable `u16` values.
unsafe fn table<'a>(
    spec: CiphertextBlockSpec,
    table: *const u16,
    len: usize,
    expected: usize,
) -> Fallible<&'a [u16]> {
    if table.is_null() {
        return Err(zhc_status::NullPointer);
    }
    if len != expected {
        return Err(zhc_status::InvalidArgument);
    }
    let table = unsafe { std::slice::from_raw_parts(table, len) };
    // Entries may set the padding bit: builtin tables such as `ReduceCarryPad` and
    // `ExtractPropGroup3` do, and are then read back by a padding or wrapping lookup.
    if table.iter().any(|&v| v > spec.complete_mask()) {
        return Err(zhc_status::InvalidArgument);
    }
    Ok(table)
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_lut1_new(
    name: *const c_char,
    spec: zhc_ciphertext_block_spec,
    table_1: *const u16,
    len: usize,
    out: *mut *mut zhc_lut1,
) -> zhc_status {
    guard(|| {
        let spec: CiphertextBlockSpec = spec.into();
        let name = unsafe { c_str(name) }?;
        let expected = expected_len(spec, 0)?;
        let t1 = unsafe { table(spec, table_1, len, expected) }?;
        let lut = Lut1::from_fn(name, spec, |b| {
            spec.from_complete(t1[b.raw_data_bits() as usize])
        });
        unsafe { write_handle(out, zhc_lut1(lut)) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_lut2_new(
    name: *const c_char,
    spec: zhc_ciphertext_block_spec,
    table_1: *const u16,
    table_2: *const u16,
    len: usize,
    out: *mut *mut zhc_lut2,
) -> zhc_status {
    guard(|| {
        let spec: CiphertextBlockSpec = spec.into();
        let name = unsafe { c_str(name) }?;
        let expected = expected_len(spec, 1)?;
        let t1 = unsafe { table(spec, table_1, len, expected) }?;
        let t2 = unsafe { table(spec, table_2, len, expected) }?;
        let lut = Lut2::from_fn(
            name,
            spec,
            |b| spec.from_complete(t1[b.raw_data_bits() as usize]),
            |b| spec.from_complete(t2[b.raw_data_bits() as usize]),
        );
        unsafe { write_handle(out, zhc_lut2(lut)) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_lut4_new(
    name: *const c_char,
    spec: zhc_ciphertext_block_spec,
    table_1: *const u16,
    table_2: *const u16,
    table_3: *const u16,
    table_4: *const u16,
    len: usize,
    out: *mut *mut zhc_lut4,
) -> zhc_status {
    guard(|| {
        let spec: CiphertextBlockSpec = spec.into();
        let name = unsafe { c_str(name) }?;
        let expected = expected_len(spec, 2)?;
        let t1 = unsafe { table(spec, table_1, len, expected) }?;
        let t2 = unsafe { table(spec, table_2, len, expected) }?;
        let t3 = unsafe { table(spec, table_3, len, expected) }?;
        let t4 = unsafe { table(spec, table_4, len, expected) }?;
        let lut = Lut4::from_fn(
            name,
            spec,
            |b| spec.from_complete(t1[b.raw_data_bits() as usize]),
            |b| spec.from_complete(t2[b.raw_data_bits() as usize]),
            |b| spec.from_complete(t3[b.raw_data_bits() as usize]),
            |b| spec.from_complete(t4[b.raw_data_bits() as usize]),
        );
        unsafe { write_handle(out, zhc_lut4(lut)) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_lut8_new(
    name: *const c_char,
    spec: zhc_ciphertext_block_spec,
    table_1: *const u16,
    table_2: *const u16,
    table_3: *const u16,
    table_4: *const u16,
    table_5: *const u16,
    table_6: *const u16,
    table_7: *const u16,
    table_8: *const u16,
    len: usize,
    out: *mut *mut zhc_lut8,
) -> zhc_status {
    guard(|| {
        let spec: CiphertextBlockSpec = spec.into();
        let name = unsafe { c_str(name) }?;
        let expected = expected_len(spec, 3)?;
        let t1 = unsafe { table(spec, table_1, len, expected) }?;
        let t2 = unsafe { table(spec, table_2, len, expected) }?;
        let t3 = unsafe { table(spec, table_3, len, expected) }?;
        let t4 = unsafe { table(spec, table_4, len, expected) }?;
        let t5 = unsafe { table(spec, table_5, len, expected) }?;
        let t6 = unsafe { table(spec, table_6, len, expected) }?;
        let t7 = unsafe { table(spec, table_7, len, expected) }?;
        let t8 = unsafe { table(spec, table_8, len, expected) }?;
        let lut = Lut8::from_fn(
            name,
            spec,
            |b| spec.from_complete(t1[b.raw_data_bits() as usize]),
            |b| spec.from_complete(t2[b.raw_data_bits() as usize]),
            |b| spec.from_complete(t3[b.raw_data_bits() as usize]),
            |b| spec.from_complete(t4[b.raw_data_bits() as usize]),
            |b| spec.from_complete(t5[b.raw_data_bits() as usize]),
            |b| spec.from_complete(t6[b.raw_data_bits() as usize]),
            |b| spec.from_complete(t7[b.raw_data_bits() as usize]),
            |b| spec.from_complete(t8[b.raw_data_bits() as usize]),
        );
        unsafe { write_handle(out, zhc_lut8(lut)) }
    })
}
