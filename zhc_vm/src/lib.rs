mod crypto;
mod lut;
mod params;
mod reg;
mod run;
mod state;
mod val;
mod vm;
mod worker;
mod topo;
mod profiling;

pub use reg::*;
pub use crypto::*;
pub use run::*;
pub use state::*;
pub use worker::*;

pub use params::VMParams;
pub use val::{Value, ValueMut};
pub use vm::VM;

#[cfg(test)]
mod test {
    use super::*;
    use tfhe::integer::RadixCiphertext;
    use tfhe::shortint::parameters::current_params::V1_6_PARAM_MESSAGE_2_CARRY_2_KS32_PBS_TUNIFORM_2M128;
    use zhc::{builder::CiphertextSpec, pipeline::compat::Iop, prelude::BuilderExt};
    use zhc_utils::{SafeAs, svec};

    #[test]
    fn test_vm() {
        let circuit = Iop::Add.to_builder(CiphertextSpec::new(64, 2, 2));
        let p = V1_6_PARAM_MESSAGE_2_CARRY_2_KS32_PBS_TUNIFORM_2M128;
        let params = VMParams::from_ks32_params(p, 256, None);
        let plan = circuit.get_vm_plan(params.n_threads.sas());
        use zhc_utils::Dumpable;
        let (ck, sk) = tfhe::generate_keys(tfhe::ConfigBuilder::with_custom_parameters(p));
        let mut vm = VM::new(&params);
        vm.set_server_key(sk);
        let ck = ck.into_raw_parts().0;
        let lhs = ck.encrypt_radix(99u64, 64 / 2);
        let rhs = ck.encrypt_radix(99u64, 64 / 2);
        let mut oup = ck.encrypt_radix(99u64, 64 / 2);
        for _ in 0..10{
            vm.execute(
                &plan,
                svec![
                    Value::FheUint(&lhs as *const RadixCiphertext),
                    Value::FheUint(&rhs as *const RadixCiphertext),
                ].as_slice(),
                svec![ValueMut::FheUint(&mut oup as *mut RadixCiphertext)].as_mut_slice(),
            );
        }
        let val: u64 = ck.decrypt_radix(&oup);
        val.dump_and_wait();
    }
}
