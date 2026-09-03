//! An experimental virtual machine for executing Fully Homomorphic Encryption (FHE) programs.
//!
//! `zhc_vm` provides [`Vm`], a multi-threaded execution engine that runs compiled FHE operation
//! plans against encrypted data. The VM manages a pool of worker threads pinned to hardware
//! cores, each with NUMA-local copies of the cryptographic material (bootstrap key, keyswitch
//! key, and lookup tables), so that FHE operations execute with minimal cross-socket traffic.
//! The lookup tables come from the plan itself: every
//! [`VmExecutionPlan`](zhc::prelude::VmExecutionPlan) carries the registry of the tables it uses,
//! and the VM loads it on first execution.
//!
//! # Getting Started
//!
//! The typical workflow is:
//!
//! 1. Build a [`VmConfig`](zhc_config::vm::VmConfig) from your TFHE parameter set using the
//!    [`VmConfigExt`] extension trait.
//! 2. Create a [`Vm`] instance, which spawns and pins worker threads.
//! 3. Install the server key with [`Vm::set_server_key`] — this replicates the bootstrap and
//!    keyswitch keys into every NUMA-local storage.
//! 4. Compile a [`VmExecutionPlan`](zhc::prelude::VmExecutionPlan) from a pipeline, then call
//!    [`Vm::execute`] to run it on ciphertext inputs. The plan's lookup tables are loaded into the
//!    VM on the first call.
//!
//! ```rust,no_run
//! # use tfhe::integer::RadixCiphertext;
//! # use tfhe::shortint::parameters::v1_6::V1_6_PARAM_MESSAGE_2_CARRY_2_KS32_PBS_TUNIFORM_2M128;
//! # use zhc::{builder::CiphertextSpec, prelude::{Pipeline, compat::Iop}};
//! # use zhc_config::vm::VmConfig;
//! # use zhc_utils::svec;
//! use zhc_vm::{Value, ValueMut, Vm, VmConfigExt};
//!
//! // 1. Configure the VM from a TFHE parameter set.
//! let params = V1_6_PARAM_MESSAGE_2_CARRY_2_KS32_PBS_TUNIFORM_2M128;
//! let config = VmConfig::from_ks32_params(params, 256);
//!
//! // 2. Create the VM (spawns worker threads pinned to available cores).
//! let mut vm = Vm::new(&config, None);
//!
//! // 3. Generate keys and install the server key.
//! let (ck, sk) = tfhe::generate_keys(
//!     tfhe::ConfigBuilder::with_custom_parameters(params),
//! );
//! vm.set_server_key(sk);
//!
//! // 4. Compile and execute a plan.
//! let builder = Iop::Add.to_builder(CiphertextSpec::new(64, 2, 2));
//! let plan = Pipeline::new()
//!     .with_builder(builder)
//!     .with_vm_config(config)
//!     .into_vm_execution_plan();
//!
//! let ck = ck.into_raw_parts().0;
//! let lhs = ck.encrypt_radix(42u64, 64 / 2);
//! let rhs = ck.encrypt_radix(58u64, 64 / 2);
//! let mut result = ck.encrypt_radix(0u64, 64 / 2);
//!
//! vm.execute(
//!     &plan,
//!     svec![
//!         Value::FheUint(&lhs as *const RadixCiphertext),
//!         Value::FheUint(&rhs as *const RadixCiphertext),
//!     ].as_slice(),
//!     svec![ValueMut::FheUint(&mut result as *mut RadixCiphertext)].as_mut_slice(),
//! );
//!
//! let decrypted: u64 = ck.decrypt_radix(&result);
//! assert_eq!(decrypted, 100);
//! ```

mod crypto;
mod ids;
mod lut;
mod params;
mod run;
mod state;
mod statistics;
mod storage;
mod val;
mod vm;
mod worker;

use crypto::*;
use ids::*;
use lut::*;
use run::*;
use state::*;
use storage::*;
use worker::*;

pub use params::*;
pub use statistics::*;
pub use val::*;
pub use vm::*;

#[cfg(test)]
mod test {
    use super::*;
    use tfhe::{
        integer::RadixCiphertext,
        shortint::parameters::v1_6::V1_6_PARAM_MESSAGE_2_CARRY_2_KS32_PBS_TUNIFORM_2M128,
    };
    use zhc::{
        builder::CiphertextSpec,
        prelude::{Pipeline, compat::Iop},
    };
    use zhc_config::vm::VmConfig;
    use zhc_utils::svec;

    #[test]
    fn smoke() {
        let builder = Iop::Add.to_builder(CiphertextSpec::new(64, 2, 2));
        let p = V1_6_PARAM_MESSAGE_2_CARRY_2_KS32_PBS_TUNIFORM_2M128;
        let config = VmConfig::from_ks32_params(p, 256);
        let mut vm = Vm::new(&config, None);
        let plan = Pipeline::new()
            .with_builder(builder)
            .with_vm_config(config)
            .into_vm_execution_plan();
        let (ck, sk) = tfhe::generate_keys(tfhe::ConfigBuilder::with_custom_parameters(p));
        vm.set_server_key(sk);
        let ck = ck.into_raw_parts().0;
        let lhs = ck.encrypt_radix(99u64, 64 / 2);
        let rhs = ck.encrypt_radix(99u64, 64 / 2);
        let mut oup = ck.encrypt_radix(99u64, 64 / 2);
        for _ in 0..10 {
            vm.execute(
                &plan,
                svec![
                    Value::FheUint(&lhs as *const RadixCiphertext),
                    Value::FheUint(&rhs as *const RadixCiphertext),
                ]
                .as_slice(),
                svec![ValueMut::FheUint(&mut oup as *mut RadixCiphertext)].as_mut_slice(),
            );
        }
        let val: u64 = ck.decrypt_radix(&oup);
        assert_eq!(val, 198);
    }
}
