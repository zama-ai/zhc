use std::{
    sync::{Arc, atomic::Ordering},
    thread::JoinHandle,
};
use tfhe::{
    ServerKey,
    core_crypto::prelude::*,
    shortint::{atomic_pattern::AtomicPatternServerKey, server_key::ShortintBootstrappingKey},
};
use zhc::prelude::VmExecutionPlan;
use zhc_config::vm::VmConfig;
use zhc_crypto::integer_semantics::lut::LutRegistry;
use zhc_profiling::{interval_begin, interval_end};
use zhc_utils::{BiMap, SafeAs, topology::Topology};

use super::*;

/// A multi-threaded virtual machine for executing FHE operation plans.
///
/// The VM spawns one worker thread per available CPU core, pins each to its core, and
/// allocates NUMA-local storage for bootstrap keys (BSK), keyswitch keys (KSK), lookup
/// tables (LUTs), and ciphertext registers. Once a server key is installed via
/// [`set_server_key`](Self::set_server_key), the VM can execute any number of
/// [`VmExecutionPlan`]s through [`execute`](Self::execute).
///
/// The lookup tables are those of the plan being executed: each plan carries the registry
/// of the tables its bootstrappings refer to, and [`execute`](Self::execute) loads them into
/// every storage the first time it meets a new registry. Running plans that share the same
/// registry back to back therefore costs no reload.
///
/// Workers synchronize via barrier: calling [`execute`](Self::execute) releases all workers
/// to process their assigned bytecode slice, then blocks until every worker finishes. This
/// means a single [`Vm`] instance is not meant to be shared across threads — it is driven
/// sequentially by the caller.
///
/// Dropping the VM signals all worker threads to exit and joins them.
#[allow(unused)]
pub struct Vm {
    config: VmConfig,
    topo: Topology,
    state: Arc<State>,
    threads: Vec<JoinHandle<()>>,
    loaded_luts: Option<LutRegistry>,
}

impl Vm {
    /// Creates a new VM with the given configuration and hardware topology.
    ///
    /// If no `topology` is provided, the VM detects the machine topology automatically.
    /// The register file is partitioned evenly across NUMA memory domains, and one worker
    /// thread is spawned per processor in the topology. Each worker pre-allocates its PBS
    /// computation buffers on its local NUMA node.
    ///
    /// On Linux the BSK and KSK buffers are backed by 2 MiB huge pages. Make sure the
    /// huge-page pool is large enough (`vm.nr_hugepages`) or the allocation will panic.
    ///
    /// # Panics
    ///
    /// Panics if the topology is not completely available (all listed processors and memories
    /// must be online and accessible to the process).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_config::vm::VmConfig;
    /// # use tfhe::shortint::parameters::v1_6::V1_6_PARAM_MESSAGE_2_CARRY_2_KS32_PBS_TUNIFORM_2M128;
    /// use zhc_vm::{Vm, VmConfigExt};
    ///
    /// let config = VmConfig::from_ks32_params(
    ///     V1_6_PARAM_MESSAGE_2_CARRY_2_KS32_PBS_TUNIFORM_2M128,
    ///     256,
    /// );
    /// let vm = Vm::new(&config, None);
    /// ```
    pub fn new(config: &VmConfig, topology: Option<&Topology>) -> Self {
        let topo = topology
            .map(|t| t.clone())
            .unwrap_or_else(|| Topology::detect_topology());
        assert!(
            topo.is_completely_available(),
            "The topology is not completely available to the VM"
        );

        let mut mem_map = BiMap::new();
        for (i, m) in topo.iter_all_memories().enumerate() {
            mem_map.insert(m.get_data().index, StorageId(i.sas()));
        }

        let state = State::new(&config, &topo);

        let workers = topo
            .iter_all_processors()
            .enumerate()
            .map(|(wid, processor)| {
                let wid = WorkerId(wid.sas());
                let sid = mem_map
                    .get_dom(&processor.get_closest_memory().get_data().index)
                    .unwrap()
                    .clone();
                let state = state.clone();
                let config = config.clone();
                let pbs_buffers = processor.run_on(|| {
                    let fft = Fft::new(PolynomialSize(config.bsk_polynomial_size));
                    let mut pbs_buffers = ComputationBuffers::new();
                    pbs_buffers.resize(
                        programmable_bootstrap_lwe_ciphertext_mem_optimized_requirement::<u64>(
                            GlweSize(config.bsk_glwe_dim + 1),
                            PolynomialSize(config.bsk_polynomial_size),
                            fft.as_view(),
                        )
                        .unaligned_bytes_required(),
                    );
                    pbs_buffers
                });

                let memory = processor.get_closest_memory().get_data().clone();
                let processor = processor.get_data().clone();
                std::thread::spawn(move || {
                    Worker {
                        wid,
                        sid,
                        state,
                        config,
                        pbs_buffers,
                        spin_time: std::time::Duration::ZERO,
                        exec_time: std::time::Duration::ZERO,
                        processor,
                        memory,
                    }
                    .run();
                })
            })
            .collect();
        Vm {
            config: config.to_owned(),
            threads: workers,
            state,
            topo,
            loaded_luts: None,
        }
    }

    /// Executes an FHE operation plan against the provided ciphertext inputs and outputs.
    ///
    /// The `plan` describes the bytecode each worker will execute, including inter-operation
    /// dependencies enforced through atomic lock counters. The `inputs` slice supplies the
    /// encrypted (or plaintext) operands that the plan reads from, and `outputs` receives
    /// the encrypted results. The correspondence between slice positions and the plan's
    /// input/output indices is determined at compilation time by the pipeline.
    ///
    /// Before running, the lookup tables of the plan are loaded into every storage, unless
    /// the previous plan already used the same registry. Loading builds one accumulator per
    /// table and copies it on a processor local to each memory node.
    ///
    /// This method blocks until every worker has finished its assigned bytecode. The plan
    /// can be reused across multiple calls, but the VM must not be called concurrently from
    /// multiple threads.
    ///
    /// # Panics
    ///
    /// Panics if the plan requires more registers than the VM's register file can hold
    /// (configured via `regf_size` in [`VmConfig`](zhc_config::vm::VmConfig)), if it uses
    /// more lookup tables than the registry holds
    /// ([`LUTS_REGISTRY_SIZE`](zhc_config::vm::LUTS_REGISTRY_SIZE)), or if one of its tables
    /// was built for another block spec than the VM's.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use tfhe::integer::RadixCiphertext;
    /// # use tfhe::shortint::parameters::v1_6::V1_6_PARAM_MESSAGE_2_CARRY_2_KS32_PBS_TUNIFORM_2M128;
    /// # use zhc::{builder::CiphertextSpec, prelude::{Pipeline, compat::Iop}};
    /// # use zhc_config::vm::VmConfig;
    /// # use zhc_utils::svec;
    /// # use zhc_vm::{Value, ValueMut, Vm, VmConfigExt};
    /// # let params = V1_6_PARAM_MESSAGE_2_CARRY_2_KS32_PBS_TUNIFORM_2M128;
    /// # let config = VmConfig::from_ks32_params(params, 256);
    /// # let mut vm = Vm::new(&config, None);
    /// # let (ck, sk) = tfhe::generate_keys(tfhe::ConfigBuilder::with_custom_parameters(params));
    /// # vm.set_server_key(sk);
    /// # let ck = ck.into_raw_parts().0;
    /// let plan = Pipeline::new()
    ///     .with_builder(Iop::Add.to_builder(CiphertextSpec::new(64, 2, 2)))
    ///     .with_vm_config(config)
    ///     .into_vm_execution_plan();
    ///
    /// let lhs = ck.encrypt_radix(42u64, 32);
    /// let rhs = ck.encrypt_radix(58u64, 32);
    /// let mut out = ck.encrypt_radix(0u64, 32);
    ///
    /// vm.execute(
    ///     &plan,
    ///     svec![
    ///         Value::FheUint(&lhs as *const RadixCiphertext),
    ///         Value::FheUint(&rhs as *const RadixCiphertext),
    ///     ].as_slice(),
    ///     svec![ValueMut::FheUint(&mut out as *mut RadixCiphertext)].as_mut_slice(),
    /// );
    /// ```
    pub fn execute(&mut self, plan: &VmExecutionPlan, inputs: &[Value], outputs: &mut [ValueMut]) {
        interval_begin(c"Execution", 0);
        assert!(
            plan.nregs <= self.config.regf_size,
            "plan needs {} registers but the register file holds only {}",
            plan.nregs,
            self.config.regf_size
        );
        if self.loaded_luts.as_ref() != Some(&plan.lut_reg) {
            self.load_luts(&plan.lut_reg);
        }
        let mut run = Run::generate(plan, inputs, outputs);
        self.state
            .run
            .store(&mut run as *mut Run, Ordering::Release);

        self.state.barrier.wait(); // Open barrier.
        let t = std::time::Instant::now();
        self.state.barrier.wait(); // Wait for completion.
        self.state
            .wall_nanos
            .fetch_add(t.elapsed().as_nanos() as u64, Ordering::Relaxed);
        interval_end(c"Execution", 0);
    }

    /// Builds the accumulators of `lut_reg` and copies them into every storage.
    ///
    /// The accumulators are built once, then copied on a processor local to each memory node
    /// so the pages are faulted on the right NUMA domain.
    fn load_luts(&mut self, lut_reg: &LutRegistry) {
        interval_begin(c"LoadLuts", 0);
        let len = self.config.lut_registry_alloc_size();
        let mut accumulators = vec![0u64; len];
        build_registry(&self.config, lut_reg, &mut accumulators);
        for (sid, mem) in self
            .topo
            .iter_all_memories()
            .enumerate()
            .map(|(i, m)| (StorageId(i.sas()), m))
        {
            let storage = self.state.storages.get(sid).unwrap();
            let processor = mem.iter_associated_processors().next().unwrap();
            processor.run_on(|| unsafe {
                std::slice::from_raw_parts_mut(storage.luts.ptr, len)
                    .clone_from_slice(&accumulators);
            });
        }
        self.loaded_luts = Some(lut_reg.clone());
        interval_end(c"LoadLuts", 0);
    }

    /// Resets all accumulated execution statistics to zero.
    ///
    /// Call this before a measurement window to isolate the timings of subsequent
    /// [`execute`](Self::execute) calls. Counters are updated with relaxed atomics, so
    /// this is safe to call between executions without additional synchronization.
    pub fn reset_statistics(&self) {
        self.state.spin_nanos.store(0, Ordering::Relaxed);
        self.state.exec_nanos.store(0, Ordering::Relaxed);
        self.state.wall_nanos.store(0, Ordering::Relaxed);
    }

    /// Returns a snapshot of the accumulated execution statistics.
    ///
    /// The returned [`Statistics`] captures wall-clock time, per-worker execution time,
    /// and spin-wait time aggregated across all [`execute`](Self::execute) calls since the
    /// last [`reset_statistics`](Self::reset_statistics) (or since VM creation).
    pub fn get_statistics(&self) -> Statistics {
        Statistics {
            spin_nanos: self.state.spin_nanos.load(Ordering::Relaxed),
            exec_nanos: self.state.exec_nanos.load(Ordering::Relaxed),
            wall_nanos: self.state.wall_nanos.load(Ordering::Relaxed),
            n_workers: self.topo.n_processors(),
        }
    }

    /// Returns the configuration this VM was created with.
    pub fn get_config(&self) -> &VmConfig {
        &self.config
    }

    /// Returns the hardware topology the VM is bound to.
    ///
    /// The topology describes the processors and memory domains in use. You can query it
    /// for the number of active workers via `n_processors()`.
    pub fn get_topology(&self) -> &Topology {
        &self.topo
    }

    /// Installs a TFHE server key, distributing cryptographic material to every storage.
    ///
    /// The server key is decomposed into its bootstrap key (BSK) and keyswitch key (KSK).
    /// Each component is then copied into every NUMA-local storage so that workers can
    /// access the keys without cross-socket memory traffic. The copy is performed on a
    /// processor local to the target memory node, ensuring the pages are faulted on the
    /// correct NUMA domain.
    ///
    /// This must be called before any [`execute`](Self::execute) call. The VM currently
    /// only supports `KeySwitch32`-style atomic-pattern server keys; other key layouts
    /// will cause an unreachable panic.
    ///
    /// # Panics
    ///
    /// Panics if the key dimensions do not match the VM configuration (i.e. the KSK or
    /// BSK allocation size differs from what the config expects).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use zhc_config::vm::VmConfig;
    /// # use tfhe::shortint::parameters::v1_6::V1_6_PARAM_MESSAGE_2_CARRY_2_KS32_PBS_TUNIFORM_2M128;
    /// # use zhc_vm::{Vm, VmConfigExt};
    /// # let params = V1_6_PARAM_MESSAGE_2_CARRY_2_KS32_PBS_TUNIFORM_2M128;
    /// # let config = VmConfig::from_ks32_params(params, 256);
    /// # let mut vm = Vm::new(&config, None);
    /// let (_client_key, server_key) = tfhe::generate_keys(
    ///     tfhe::ConfigBuilder::with_custom_parameters(params),
    /// );
    /// vm.set_server_key(server_key);
    /// ```
    pub fn set_server_key(&mut self, sk: ServerKey) {
        let key = sk.into_raw_parts().0.into_raw_parts();
        let AtomicPatternServerKey::KeySwitch32(ap) = key.atomic_pattern else {
            unreachable!()
        };

        let ksk = ap.key_switching_key;
        let ksk_src = ksk.as_view().into_container();
        assert_eq!(ksk_src.len(), self.config.ksk_alloc_size());

        let ShortintBootstrappingKey::Classic { bsk, .. } = ap.bootstrapping_key else {
            unreachable!()
        };
        let bsk_src = bsk.data();
        assert_eq!(bsk_src.len(), self.config.bsk_alloc_size());

        let (ksk_len, bsk_len) = (self.config.ksk_alloc_size(), self.config.bsk_alloc_size());
        for (sid, mem) in self
            .topo
            .iter_all_memories()
            .enumerate()
            .map(|(i, m)| (StorageId(i.sas()), m))
        {
            let storage = self.state.storages.get(sid).unwrap();
            let processor = mem.iter_associated_processors().next().unwrap();
            processor.run_on(|| unsafe {
                std::slice::from_raw_parts_mut(storage.ksk.ptr, ksk_len).clone_from_slice(ksk_src);
                std::slice::from_raw_parts_mut(storage.bsk.ptr, bsk_len).clone_from_slice(&bsk_src);
            });
        }
    }
}

impl Drop for Vm {
    fn drop(&mut self) {
        self.state.drop.store(true, Ordering::Release);
        self.state.barrier.wait();
        self.threads.drain(..).for_each(|jh| match jh.join() {
            Ok(_) => (),
            Err(_) => println!("Error occured while joining VM worker thread."),
        });
    }
}
