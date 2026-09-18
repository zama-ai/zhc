//! Regression tests for `Pipeline`'s lazy-evaluation cache invalidation.
//!
//! `Pipeline` memoizes every artifact it computes, keyed by the step that produced it, so that
//! calling a `get_*` method twice in a row does no work the second time. A `with_*` setter called
//! after an artifact was already pulled must invalidate that artifact (and everything computed
//! from it), or the pipeline keeps handing back a result that no longer matches its own inputs.

use zhc_builder::{CiphertextSpec, add};
use zhc_config::hpu::{HpuConfig, PhysicalConfig};
use zhc_crypto::integer_semantics::{CiphertextBlockSpec, lut::LutId};
use zhc_pipeline::Pipeline;

#[test]
fn hpu_stream_reflects_lut_relocation_set_after_first_pull() {
    let ir = add(CiphertextSpec::new(16, 2, 2)).optimize_ir();
    let config = HpuConfig::from(PhysicalConfig::gaussian_64b_fast());

    let mut pipeline = Pipeline::new()
        .with_unchecked_ioplang(ir)
        .with_ciphertext_block_spec(CiphertextBlockSpec(2, 2))
        .with_hpu_config(config);

    // Pull `hpu_stream` once, with no relocation set (the default, identity mapping).
    let before = pipeline.get_hpu_stream().clone();

    let n_luts = pipeline.get_lut_registry().iter_luts().count();
    assert!(
        n_luts > 1,
        "test needs at least two distinct LUTs to build a non-identity relocation"
    );
    let relocation: Vec<LutId> = (0..n_luts).map(|i| LutId((i + 1) % n_luts)).collect();

    // Setting the relocation *after* `hpu_stream` was already pulled must still be reflected the
    // next time it's pulled, not served from a stale cache entry computed before the relocation
    // existed.
    let mut pipeline = pipeline.with_hpu_lut_relocation(relocation);
    let after = pipeline.get_hpu_stream();

    assert_ne!(
        &before, after,
        "get_hpu_stream() returned a stale, pre-relocation result"
    );
}
