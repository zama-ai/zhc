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
    pub use zhc_langs::ioplang::{Lut1Def, Lut2Def};
    pub use zhc_pipeline::*;
    pub use zhc_utils::{Dumpable, topology::Topology};

    // pub trait BuilderExt {
    //     fn
    // }
}

#[cfg(test)]
mod test {
    #[test]
    fn brrrrrr() {}
}
