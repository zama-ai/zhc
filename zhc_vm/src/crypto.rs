use tfhe::{
    core_crypto::prelude::{
        FftView, FourierLweBootstrapKey, GlweCiphertextMutView, GlweCiphertextView,
        LweCiphertextMutView, LweCiphertextView, MonomialDegree, PodStack, UnsignedTorus, c64,
        extract_lwe_sample_from_glwe_ciphertext, lwe_ciphertext_centered_binary_modulus_switch,
    },
    prelude::CastInto,
};

pub const CACHELINE_ALIGN: usize = {
    #[cfg(any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "powerpc64",
    ))]
    {
        128
    }
    #[cfg(any(
        target_arch = "arm",
        target_arch = "mips",
        target_arch = "mips64",
        target_arch = "riscv64",
    ))]
    {
        32
    }
    #[cfg(target_arch = "s390x")]
    {
        256
    }
    #[cfg(not(any(
        target_arch = "x86_64",
        target_arch = "aarch64",
        target_arch = "powerpc64",
        target_arch = "arm",
        target_arch = "mips",
        target_arch = "mips64",
        target_arch = "riscv64",
        target_arch = "s390x",
    )))]
    {
        64
    }
};

pub trait BootstrapExt {
    fn bootstrap_ml<InputScalar, OutputScalar>(
        self,
        lwe_out: &mut [LweCiphertextMutView<'_, OutputScalar>],
        lwe_in: LweCiphertextView<'_, InputScalar>,
        accumulator: GlweCiphertextView<'_, OutputScalar>,
        fft: FftView<'_>,
        stack: &mut PodStack,
    ) where
        InputScalar: UnsignedTorus + CastInto<usize>,
        OutputScalar: UnsignedTorus;
}

impl<'a> BootstrapExt for FourierLweBootstrapKey<&'a [c64]> {
    fn bootstrap_ml<InputScalar, OutputScalar>(
        self,
        lwe_out: &mut [LweCiphertextMutView<'_, OutputScalar>],
        lwe_in: LweCiphertextView<'_, InputScalar>,
        accumulator: GlweCiphertextView<'_, OutputScalar>,
        fft: FftView<'_>,
        stack: &mut PodStack,
    ) where
        InputScalar: UnsignedTorus + CastInto<usize>,
        OutputScalar: UnsignedTorus,
    {
        assert!(lwe_in.ciphertext_modulus().is_power_of_two());
        assert!(
            lwe_out
                .iter()
                .all(|a| a.ciphertext_modulus().is_power_of_two())
        );
        assert!(
            lwe_out
                .iter()
                .all(|a| a.ciphertext_modulus() == accumulator.ciphertext_modulus())
        );

        let (local_accumulator_data, stack) =
            stack.collect_aligned(CACHELINE_ALIGN, accumulator.as_ref().iter().copied());
        let mut local_accumulator = GlweCiphertextMutView::from_container(
            local_accumulator_data,
            accumulator.polynomial_size(),
            accumulator.ciphertext_modulus(),
        );

        let log_modulus = accumulator
            .polynomial_size()
            .to_blind_rotation_input_modulus_log();

        let msed = lwe_ciphertext_centered_binary_modulus_switch(lwe_in.as_view(), log_modulus);

        self.blind_rotate_assign(local_accumulator.as_mut_view(), &msed, fft, stack);

        let chunk = accumulator.polynomial_size().0 / lwe_out.len();
        for i in 0..lwe_out.len() {
            extract_lwe_sample_from_glwe_ciphertext(
                &local_accumulator,
                &mut lwe_out[i],
                MonomialDegree(i * chunk),
            );
        }
    }
}
