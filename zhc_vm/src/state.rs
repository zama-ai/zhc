use std::{
    ptr::null_mut,
    sync::{
        Arc, Barrier,
        atomic::{AtomicBool, AtomicPtr, AtomicU64},
    },
};
use crate::{params::VMParams, run::Run, topo::{self, Topology}};
use tfhe::core_crypto::prelude::c64;

#[cfg(target_os = "linux")]
const HUGE_2M: usize = 2 * 1024 * 1024;

#[cfg(target_os = "linux")]
unsafe fn alloc_local<T>(n: usize) -> *mut T {
    let bytes = (n * std::mem::size_of::<T>()).next_multiple_of(HUGE_2M);
    let p = unsafe {
        libc::mmap(
            std::ptr::null_mut(),
            bytes,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS | libc::MAP_HUGETLB | libc::MAP_HUGE_2MB,
            -1,
            0,
        )
    };
    assert!(
        p != libc::MAP_FAILED,
        "MAP_HUGETLB mmap of {bytes} B failed — is the 2 MiB pool reserved \
         (vm.nr_hugepages) and large enough on this node? {}",
        std::io::Error::last_os_error(),
    );
    let mut off = 0;
    while off < bytes {
        unsafe { (p as *mut u8).add(off).write_volatile(0) };
        off += HUGE_2M;
    }
    p as *mut T
}

#[cfg(not(target_os = "linux"))]
unsafe fn alloc_local<T>(n: usize) -> *mut T {
    let layout = std::alloc::Layout::array::<T>(n).unwrap();
    unsafe { std::alloc::alloc_zeroed(layout) as *mut T }
}

pub struct State {
    pub run: AtomicPtr<Run>,
    pub vm_barrier: Barrier,
    pub lockstep_barrier: Barrier,
    pub bsk: Vec<*mut c64>,
    pub ksk: Vec<*mut u32>,
    pub lut_registry: Vec<*const u64>,
    pub register: *mut u64,
    pub params: VMParams,
    pub drop: AtomicBool,
    pub spin_nanos: AtomicU64,
    pub exec_nanos: AtomicU64,
    pub wall_nanos: AtomicU64,
}

impl State {
    pub fn new(params: &VMParams, n_threads: usize, topo: &Topology) -> Arc<Self> {
        let run = AtomicPtr::new(null_mut());
        let vm_barrier = Barrier::new(n_threads + 1);
        let lockstep_barrier = Barrier::new(n_threads);
        let drop = AtomicBool::new(false);

        let mut bsk = Vec::with_capacity(topo.n_nodes());
        let mut ksk = Vec::with_capacity(topo.n_nodes());
        let mut lut_registry = Vec::with_capacity(topo.n_nodes());
        for node in 0..topo.n_nodes() {
            let (b, k, l) = topo::run_on_cpu(topo.representative_cpu(node), || {
                let b = unsafe { alloc_local::<c64>(params.bsk_alloc_size()) } as usize;
                let k = unsafe { alloc_local::<u32>(params.ksk_alloc_size()) } as usize;
                let mut l = vec![0u64; params.lut_registry_alloc_size()];
                crate::lut::build_registry(params, &mut l);
                (b, k, l)
            });
            bsk.push(b as *mut c64);
            ksk.push(k as *mut u32);
            lut_registry.push(l.leak().as_ptr());
        }

        let register = {
            let v: Vec<u64> = (0..params.register_alloc_size() as u64)
                .map(|i| i.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407))
                .collect();
            v.leak().as_mut_ptr()
        };
        Arc::new(State {
            run,
            vm_barrier,
            lockstep_barrier,
            bsk,
            ksk,
            lut_registry,
            register,
            drop,
            spin_nanos: AtomicU64::new(0),
            exec_nanos: AtomicU64::new(0),
            wall_nanos: AtomicU64::new(0),
            params: params.to_owned(),
        })
    }
}

impl Drop for State {
    fn drop(&mut self) {
        unsafe {
            let _register = Vec::from_raw_parts(
                self.register,
                self.params.register_alloc_size(),
                self.params.register_alloc_size(),
            );
        }
        self.register = null_mut();
    }
}

unsafe impl Send for State {}
unsafe impl Sync for State {}
