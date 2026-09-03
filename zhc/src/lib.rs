pub use zhc_builder as builder;
pub use zhc_config as config;
pub use zhc_crypto as crypto;
pub use zhc_ir as ir;
pub use zhc_langs as langs;
pub use zhc_pipeline as pipeline;
pub use zhc_sim as sim;
pub use zhc_utils as utils;

pub mod prelude {
    pub use zhc_builder::Builder;
    pub use zhc_config::*;
    pub use zhc_crypto::integer_semantics::CiphertextBlockSpec;
    pub use zhc_langs::ioplang::IopValue;
    use zhc_langs::ioplang::analyze_noise;
    pub use zhc_langs::ioplang::{Lut1Def, Lut2Def};
    pub use zhc_pipeline::*;
    pub use zhc_utils::{Dumpable, topology::Topology};

    pub trait BuilderExt {
        fn dump_noise_budget(&self);
    }

    impl BuilderExt for Builder {
        fn dump_noise_budget(&self) {
            analyze_noise(&*self.ir(), &self.spec().matching_plaintext_block_spec()).dump();
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn brrrrrr() {}
}
