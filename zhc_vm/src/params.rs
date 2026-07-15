use tfhe::shortint::parameters::{ClassicPBSParameters, KeySwitch32PBSParameters};

use crate::lut::N_LUTS;

#[derive(Debug, Clone)]
pub struct VMParams {
    pub lwe_dim: usize,
    pub regf_size: usize,
    pub bsk_polynomial_size: usize,
    pub bsk_glwe_dim: usize,
    pub bsk_dec_levels: usize,
    pub bsk_dec_base_log: usize,
    pub ksk_dec_levels: usize,
    pub ksk_dec_base_log: usize,
    pub delta: usize,
    pub carry_size: usize,
    pub message_size: usize,
    pub n_threads: usize
}

impl VMParams {
    pub fn from_params(p: ClassicPBSParameters, regf_size: usize, n_threads: Option<usize>) -> Self {
        let msg_bits = p.message_modulus.0.ilog2() as usize;
        let carry_bits = p.carry_modulus.0.ilog2() as usize;
        let n_threads = n_threads.unwrap_or(std::thread::available_parallelism().unwrap().get());
        VMParams {
            lwe_dim: p.lwe_dimension.0,
            bsk_polynomial_size: p.polynomial_size.0,
            bsk_glwe_dim: p.glwe_dimension.0,
            bsk_dec_levels: p.pbs_level.0,
            bsk_dec_base_log: p.pbs_base_log.0,
            ksk_dec_levels: p.ks_level.0,
            ksk_dec_base_log: p.ks_base_log.0,
            delta: 1 << (64 - msg_bits - carry_bits - 1),
            message_size: msg_bits,
            carry_size: carry_bits,
            regf_size,
            n_threads
        }
    }

    /// KS32 variant: the keyswitch operates in a 32-bit modulus (u32 KSK and
    /// post-keyswitch ciphertext), while the message-carrying "big" ciphertext
    /// stays 64-bit — hence `delta` is unchanged.
    pub fn from_ks32_params(
        p: KeySwitch32PBSParameters,
        regf_size: usize,
        n_threads: Option<usize>,
    ) -> Self {
        let msg_bits = p.message_modulus.0.ilog2() as usize;
        let carry_bits = p.carry_modulus.0.ilog2() as usize;
        let n_threads = n_threads.unwrap_or(std::thread::available_parallelism().unwrap().get());
        VMParams {
            lwe_dim: p.lwe_dimension.0,
            bsk_polynomial_size: p.polynomial_size.0,
            bsk_glwe_dim: p.glwe_dimension.0,
            bsk_dec_levels: p.pbs_level.0,
            bsk_dec_base_log: p.pbs_base_log.0,
            ksk_dec_levels: p.ks_level.0,
            ksk_dec_base_log: p.ks_base_log.0,
            delta: 1 << (64 - msg_bits - carry_bits - 1),
            message_size: msg_bits,
            carry_size: carry_bits,
            regf_size,
            n_threads,
        }
    }

    pub fn big_ciphertext_size(&self) -> usize {
        self.bsk_glwe_dim * self.bsk_polynomial_size + 1
    }

    pub fn small_ciphertext_size(&self) -> usize {
        self.lwe_dim + 1
    }

    pub fn register_alloc_size(&self) -> usize {
        self.big_ciphertext_size() * self.regf_size
    }

    pub fn ksk_alloc_size(&self) -> usize {
        self.bsk_glwe_dim * self.bsk_polynomial_size * self.ksk_dec_levels * (self.lwe_dim + 1)
    }

    pub fn bsk_alloc_size(&self) -> usize {
        self.lwe_dim
            * self.bsk_dec_levels
            * (self.bsk_glwe_dim + 1)
            * (self.bsk_glwe_dim + 1)
            * self.bsk_polynomial_size
            / 2
    }

    pub fn lut_registry_alloc_size(&self) -> usize {
        N_LUTS * self.lut_alloc_size()
    }

    pub fn lut_alloc_size(&self) -> usize {
        (self.bsk_glwe_dim + 1) * self.bsk_polynomial_size
    }
}
