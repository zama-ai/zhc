//! Bindings of [`zhc_pipeline::Pipeline`].
//!
//! The Rust `with_*` setters consume the pipeline and return it. The C bindings expose their
//! in-place `set_*` twins instead, so a failed call leaves the pipeline untouched.

use crate::builder::zhc_builder;
use crate::types::*;
use ::zhc::PipelineExt;
use ::zhc_pipeline::Pipeline;

pub struct zhc_pipeline(Pipeline);

/// Shorthand: dereferences the pipeline handle to the wrapped [`Pipeline`].
unsafe fn p<'a>(pipeline: *mut zhc_pipeline) -> Fallible<&'a mut Pipeline> {
    Ok(&mut unsafe { deref_mut(pipeline) }?.0)
}

// ---------------------------------------------------------------------------------------------
// Construction
// ---------------------------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_pipeline_new(out: *mut *mut zhc_pipeline) -> zhc_status {
    guard(|| unsafe { write_handle(out, zhc_pipeline(Pipeline::new())) })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_pipeline_free(pipeline: *mut zhc_pipeline) {
    unsafe { free(pipeline) }
}

// ---------------------------------------------------------------------------------------------
// Inputs
// ---------------------------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_pipeline_set_builder(
    pipeline: *mut zhc_pipeline,
    builder: *const zhc_builder,
) -> zhc_status {
    guard(|| {
        let builder = unsafe { deref(builder) }?.0.clone();
        unsafe { p(pipeline) }?.set_builder(builder);
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_pipeline_set_ciphertext_block_spec(
    pipeline: *mut zhc_pipeline,
    spec: zhc_ciphertext_block_spec,
) -> zhc_status {
    guard(|| {
        unsafe { p(pipeline) }?.set_ciphertext_block_spec(spec.into());
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_pipeline_set_hpu_config(
    pipeline: *mut zhc_pipeline,
    config: zhc_hpu_config,
) -> zhc_status {
    guard(|| {
        unsafe { p(pipeline) }?.set_hpu_config(config.into());
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_pipeline_set_multi_hpu_config(
    pipeline: *mut zhc_pipeline,
    config: zhc_multi_hpu_config,
) -> zhc_status {
    guard(|| {
        unsafe { p(pipeline) }?.set_multi_hpu_config(config.into());
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_pipeline_set_vm_config(
    pipeline: *mut zhc_pipeline,
    config: zhc_vm_config,
) -> zhc_status {
    guard(|| {
        unsafe { p(pipeline) }?.set_vm_config(config.into());
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_pipeline_set_legacy_hpu_scheduler(
    pipeline: *mut zhc_pipeline,
) -> zhc_status {
    guard(|| {
        unsafe { p(pipeline) }?.set_legacy_hpu_scheduler();
        Ok(())
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_pipeline_set_trace_hpu_events(
    pipeline: *mut zhc_pipeline,
) -> zhc_status {
    guard(|| {
        unsafe { p(pipeline) }?.set_trace_hpu_events();
        Ok(())
    })
}

// ---------------------------------------------------------------------------------------------
// Artifacts: inputs read back
// ---------------------------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_pipeline_get_ciphertext_block_spec(
    pipeline: *mut zhc_pipeline,
    out: *mut zhc_ciphertext_block_spec,
) -> zhc_status {
    guard(|| {
        let r = *unsafe { p(pipeline) }?.get_ciphertext_block_spec();
        unsafe { write(out, r.into()) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_pipeline_get_hpu_config(
    pipeline: *mut zhc_pipeline,
    out: *mut zhc_hpu_config,
) -> zhc_status {
    guard(|| {
        let r = unsafe { p(pipeline) }?.get_hpu_config().clone();
        unsafe { write(out, r.into()) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_pipeline_get_multi_hpu_config(
    pipeline: *mut zhc_pipeline,
    out: *mut zhc_multi_hpu_config,
) -> zhc_status {
    guard(|| {
        let r = unsafe { p(pipeline) }?.get_multi_hpu_config().clone();
        unsafe { write(out, r.into()) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_pipeline_get_vm_config(
    pipeline: *mut zhc_pipeline,
    out: *mut zhc_vm_config,
) -> zhc_status {
    guard(|| {
        let r = unsafe { p(pipeline) }?.get_vm_config().clone();
        unsafe { write(out, r.into()) }
    })
}

// ---------------------------------------------------------------------------------------------
// Artifacts: numbers
// ---------------------------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_pipeline_get_fingerprint(
    pipeline: *mut zhc_pipeline,
    out: *mut u64,
) -> zhc_status {
    guard(|| {
        let r = unsafe { p(pipeline) }?.get_fingerprint();
        unsafe { write(out, r.0) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_pipeline_get_hpu_metrics(
    pipeline: *mut zhc_pipeline,
    out: *mut zhc_hpu_metrics,
) -> zhc_status {
    guard(|| {
        let r = unsafe { p(pipeline) }?.get_hpu_metrics().into();
        unsafe { write(out, r) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_pipeline_get_multi_hpu_metrics(
    pipeline: *mut zhc_pipeline,
    out: *mut zhc_multi_hpu_metrics,
) -> zhc_status {
    guard(|| {
        let r = unsafe { p(pipeline) }?.get_multi_hpu_metrics().into();
        unsafe { write(out, r) }
    })
}

// ---------------------------------------------------------------------------------------------
// Artifacts: files
// ---------------------------------------------------------------------------------------------

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_pipeline_draw_state(
    pipeline: *mut zhc_pipeline,
    out: *mut *mut zhc_file_handle,
) -> zhc_status {
    guard(|| {
        let r = unsafe { p(pipeline) }?.draw_state();
        unsafe { write_handle(out, zhc_file_handle(r)) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_pipeline_get_slack_drawing(
    pipeline: *mut zhc_pipeline,
    out: *mut *mut zhc_file_handle,
) -> zhc_status {
    guard(|| {
        let r = unsafe { p(pipeline) }?.get_slack_drawing().clone();
        unsafe { write_handle(out, zhc_file_handle(r)) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_pipeline_get_hpu_assembly(
    pipeline: *mut zhc_pipeline,
    out: *mut *mut zhc_file_handle,
) -> zhc_status {
    guard(|| {
        let r = unsafe { p(pipeline) }?.get_hpu_assembly().clone();
        unsafe { write_handle(out, zhc_file_handle(r)) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_pipeline_get_hpu_trace(
    pipeline: *mut zhc_pipeline,
    out: *mut *mut zhc_perfetto_trace,
) -> zhc_status {
    guard(|| {
        let r = unsafe { p(pipeline) }?.get_hpu_trace().clone();
        unsafe { write_handle(out, zhc_perfetto_trace(r)) }
    })
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn zhc_pipeline_get_multi_hpu_trace(
    pipeline: *mut zhc_pipeline,
    out: *mut *mut zhc_perfetto_trace,
) -> zhc_status {
    guard(|| {
        let r = unsafe { p(pipeline) }?.get_multi_hpu_trace().clone();
        unsafe { write_handle(out, zhc_perfetto_trace(r)) }
    })
}
