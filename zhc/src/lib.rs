pub use zhc_builder as builder;
pub use zhc_config as config;
pub use zhc_crypto as crypto;
pub use zhc_ir as ir;
pub use zhc_langs as langs;
pub use zhc_pipeline as pipeline;
pub use zhc_sim as sim;
pub use zhc_utils as utils;

pub mod compat;
mod pipeline_ext;

pub use pipeline_ext::PipelineExt;

pub mod prelude {
    pub use super::PipelineExt;
    pub use super::compat::*;
    pub use zhc_builder::Builder;
    pub use zhc_config::*;
    pub use zhc_crypto::integer_semantics::CiphertextBlockSpec;
    pub use zhc_langs::ioplang::IopValue;
    pub use zhc_langs::ioplang::{Lut1Def, Lut2Def};
    pub use zhc_pipeline::*;
    pub use zhc_utils::{Dumpable, topology::Topology};
}

#[cfg(test)]
mod test {
    use zhc_builder::{CiphertextBlockSpec, add};
    use zhc_config::hpu::HpuConfig;
    use zhc_langs::tasklang::coarsen_ioplang;
    use zhc_pipeline::Pipeline;
    use zhc_utils::{Dumpable, iter::CollectInVec};

    #[test]
    fn brrrrrr() {
        let BLOCK_SPEC = CiphertextBlockSpec(2, 2);
        let ir = add(BLOCK_SPEC.ciphertext_spec(16)).optimize_ir();
        ir.draw_to_html(None).open();
        coarsen_ioplang(&ir).annotated_output().draw_to_html(None).open();
    }
}
