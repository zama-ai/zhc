use std::{
    sync::{Arc, atomic::Ordering},
    thread::JoinHandle,
};
use tfhe::{ServerKey, core_crypto::prelude::*, shortint::{atomic_pattern::AtomicPatternServerKey, server_key::ShortintBootstrappingKey}};
use zhc::pipeline::scheduler::vm::VmExecutionPlan;
use zhc_utils::{Dumpable, SafeAs};

use crate::{
    params::VMParams, profiling, run::Run, state::State, topo::Topology, val::{Value, ValueMut}, worker::Worker
};

#[derive(Debug, Clone, Copy)]
pub struct Profile {
    pub exec_nanos: u64,
    pub spin_nanos: u64,
    pub wall_nanos: u64,
    pub n_threads: usize,
}

impl Profile {
    pub fn available_nanos(&self) -> u64 {
        self.wall_nanos * self.n_threads as u64
    }

    pub fn utilization(&self) -> f64 {
        self.exec_nanos as f64 / self.available_nanos() as f64
    }

    pub fn spin_fraction(&self) -> f64 {
        self.spin_nanos as f64 / self.available_nanos() as f64
    }

    pub fn parked_fraction(&self) -> f64 {
        1.0 - self.utilization() - self.spin_fraction()
    }
}

impl Dumpable for Profile {
    fn dump_to_string(&self) -> String {
        let ms = |nanos: u64| nanos as f64 / 1e6;
        format!(
            "╔══════════════════════════════════════════════════════════════════════════════
║ VM Profile
║──────────────────────────────────────────────────────────────────────────────
║   Threads   : {}
║   Wall      : {:.3} ms
║   Available : {:.3} ms  (wall × threads)
║──────────────────────────────────────────────────────────────────────────────
║   Exec      : {:>10.3} ms   ({:>5.1}%)
║   Spin      : {:>10.3} ms   ({:>5.1}%)
║   Parked    : {:>10.3} ms   ({:>5.1}%)
╚══════════════════════════════════════════════════════════════════════════════",
            self.n_threads,
            ms(self.wall_nanos),
            ms(self.available_nanos()),
            ms(self.exec_nanos),
            100.0 * self.utilization(),
            ms(self.spin_nanos),
            100.0 * self.spin_fraction(),
            ms(self.available_nanos()) - ms(self.exec_nanos) - ms(self.spin_nanos),
            100.0 * self.parked_fraction(),
        )
    }
}

#[allow(unused)]
pub struct VM {
    params: VMParams,
    state: Arc<State>,
    fft: Fft,
    threads: Vec<JoinHandle<()>>,
    topo: Topology,
}

impl VM {
    pub fn new(params: &VMParams) -> Self {
        let n_workers = params.n_threads;
        let fft = Fft::new(PolynomialSize(params.bsk_polynomial_size));
        let topo = Topology::detect();
        let state = State::new(&params, n_workers, &topo);
        let core_ids = core_affinity::get_core_ids();
        let threads = (0..n_workers)
            .map(|tid| {
                let state = state.clone();
                let params = params.clone();
                let core_id = core_ids.as_ref().and_then(|ids| ids.get(tid).copied());
                let node = core_id.map_or(0, |c| topo.node_of(c.id));
                let fft =
                    unsafe { std::mem::transmute::<FftView<'_>, FftView<'static>>(fft.as_view()) };
                let mut pbs_buffers = ComputationBuffers::new();
                pbs_buffers.resize(
                    programmable_bootstrap_lwe_ciphertext_mem_optimized_requirement::<u64>(
                        GlweSize(params.bsk_glwe_dim + 1),
                        PolynomialSize(params.bsk_polynomial_size),
                        fft,
                    ).unaligned_bytes_required(),
                );
                std::thread::spawn(move || {
                    Worker {
                        tid: tid.sas(),
                        state,
                        params,
                        fft,
                        pbs_buffers,
                        n_threads: n_workers.sas(),
                        spin_time: std::time::Duration::ZERO,
                        exec_time: std::time::Duration::ZERO,
                        core_id,
                        node,
                    }
                    .run();
                })
            })
            .collect();
        VM {
            params: params.to_owned(),
            threads,
            fft,
            state,
            topo,
        }
    }

    pub fn execute(&mut self, plan: &VmExecutionPlan, inputs: &[Value], outputs: &mut [ValueMut]) {
        profiling::interval_begin("Execution", 0);
        assert!(!self.state.bsk.is_empty());
        assert!(
            plan.nregs <= self.params.regf_size,
            "plan needs {} registers but the register file holds only {}",
            plan.nregs,
            self.params.regf_size
        );
        let mut run = Run::generate(plan, inputs, outputs);
        self.state
            .run
            .store(&mut run as *mut Run, Ordering::Release);

        self.state.vm_barrier.wait(); // Open barrier.
        let t = std::time::Instant::now();
        self.state.vm_barrier.wait(); // Wait for completion.
        self.state
            .wall_nanos
            .fetch_add(t.elapsed().as_nanos() as u64, Ordering::Relaxed);
        profiling::interval_end("Execution", 0);
    }

    pub fn reset_profile(&self) {
        self.state.spin_nanos.store(0, Ordering::Relaxed);
        self.state.exec_nanos.store(0, Ordering::Relaxed);
        self.state.wall_nanos.store(0, Ordering::Relaxed);
    }

    pub fn profile(&self) -> Profile {
        Profile {
            spin_nanos: self.state.spin_nanos.load(Ordering::Relaxed),
            exec_nanos: self.state.exec_nanos.load(Ordering::Relaxed),
            wall_nanos: self.state.wall_nanos.load(Ordering::Relaxed),
            n_threads: self.params.n_threads,
        }
    }

    pub fn set_server_key(&mut self, sk: ServerKey) {
        let key = sk.into_raw_parts().0.into_raw_parts();
        let AtomicPatternServerKey::KeySwitch32(ap) = key.atomic_pattern else {unreachable!()};

        let ksk = ap.key_switching_key;
        let ksk_src = ksk.as_view().into_container();
        assert_eq!(ksk_src.len(), self.params.ksk_alloc_size());

        let ShortintBootstrappingKey::Classic { bsk, .. } = ap.bootstrapping_key else {unreachable!()};
        let bsk_src = bsk.data();
        assert_eq!(bsk_src.len(), self.params.bsk_alloc_size());

        let (ksk_len, bsk_len) = (self.params.ksk_alloc_size(), self.params.bsk_alloc_size());
        for node in 0..self.topo.n_nodes() {
            let ksk_dst = self.state.ksk[node] as usize;
            let bsk_dst = self.state.bsk[node] as usize;
            crate::topo::run_on_cpu(self.topo.representative_cpu(node), || {
                unsafe {
                    std::slice::from_raw_parts_mut(ksk_dst as *mut u32, ksk_len)
                        .clone_from_slice(ksk_src);
                    std::slice::from_raw_parts_mut(bsk_dst as *mut c64, bsk_len)
                        .clone_from_slice(&bsk_src);
                }
            });
        }
    }
}

impl Drop for VM {
    fn drop(&mut self) {
        self.state.drop.store(true, Ordering::Release);
        self.state.vm_barrier.wait();
        self.threads.drain(..).for_each(|jh| match jh.join() {
            Ok(_) => (),
            Err(_) => println!("Error occured while joining VM worker thread."),
        });
    }
}
