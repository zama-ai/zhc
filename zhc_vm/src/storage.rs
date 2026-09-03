#![allow(dead_code)]

use tfhe::core_crypto::prelude::{Fft, c64};
use zhc_config::vm::VmConfig;
use zhc_utils::topology::Topology;

pub struct Storage {
    pub bsk: Allocated<c64>,
    pub ksk: Allocated<u32>,
    pub luts: Allocated<u64>,
    pub reg: Allocated<u64>,
    pub fft: Fft,
}

impl Storage {
    pub fn new(config: &VmConfig, topo: &Topology) -> Self {
        let bsk = Allocated::<c64>::alloc(config.bsk_alloc_size());
        let ksk = Allocated::<u32>::alloc(config.ksk_alloc_size());
        // Accumulators are written by `Vm::load_luts` from the registry of each plan.
        let luts = Allocated::<u64>::alloc(config.lut_registry_alloc_size());
        let local_reg_size = config.register_alloc_size() / topo.n_memories();
        let reg = Allocated::<u64>::alloc(local_reg_size);
        unsafe {
            let s = std::slice::from_raw_parts_mut(reg.ptr, reg.len);
            for (i, v) in s.iter_mut().enumerate() {
                *v = (i as u64)
                    .wrapping_mul(6364136223846793005)
                    .wrapping_add(1442695040888963407);
            }
        }
        let fft = Fft::new(tfhe::shortint::prelude::PolynomialSize(
            config.bsk_polynomial_size,
        ));
        Storage {
            bsk,
            ksk,
            luts,
            reg,
            fft,
        }
    }
}

impl Drop for Storage {
    fn drop(&mut self) {
        unsafe {
            self.reg.dealloc();
            self.luts.dealloc();
            self.bsk.dealloc();
            self.ksk.dealloc();
        }
    }
}

unsafe impl Send for Storage {}
unsafe impl Sync for Storage {}

pub(crate) enum AllocKind {
    Normal,
    Huge,
}

pub(crate) struct Allocated<T> {
    pub ptr: *mut T,
    pub len: usize,
    pub kind: AllocKind,
}

#[cfg(target_os = "linux")]
const HUGE_2M: usize = 2 * 1024 * 1024;

impl<T> Allocated<T> {
    #[cfg(target_os = "linux")]
    fn alloc(len: usize) -> Allocated<T> {
        if let Some(alloc) = Self::alloc_huge(len) {
            alloc
        } else {
            eprintln!(
                "WARNING: MAP_HUGETLB mmap failed — is the 2 MiB pool reserved \
                 (vm.nr_hugepages) and large enough on this node? {} \
                 Falling back to normal allocation.",
                std::io::Error::last_os_error(),
            );
            Self::alloc_normal(len)
        }
    }

    #[cfg(not(target_os = "linux"))]
    fn alloc(len: usize) -> Allocated<T> {
        Self::alloc_normal(len)
    }

    #[cfg(target_os = "linux")]
    fn alloc_huge(len: usize) -> Option<Allocated<T>> {
        let bytes = (len * std::mem::size_of::<T>()).next_multiple_of(HUGE_2M);
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
        if p == libc::MAP_FAILED {
            return None;
        }
        let mut off = 0;
        while off < bytes {
            unsafe { (p as *mut u8).add(off).write_volatile(0) };
            off += HUGE_2M;
        }
        Some(Allocated {
            ptr: p as *mut T,
            len,
            kind: AllocKind::Huge,
        })
    }

    fn alloc_normal(len: usize) -> Allocated<T> {
        let layout = std::alloc::Layout::array::<T>(len).unwrap();
        let ptr = unsafe { std::alloc::alloc_zeroed(layout) as *mut T };
        assert!(!ptr.is_null(), "alloc_zeroed failed");
        Allocated {
            ptr,
            len,
            kind: AllocKind::Normal,
        }
    }

    #[cfg(target_os = "linux")]
    unsafe fn dealloc(&mut self) {
        match self.kind {
            AllocKind::Normal => {
                let layout = std::alloc::Layout::array::<T>(self.len).unwrap();
                unsafe { std::alloc::dealloc(self.ptr as *mut u8, layout) };
            }
            AllocKind::Huge => {
                let bytes = (self.len * std::mem::size_of::<T>()).next_multiple_of(HUGE_2M);
                let ret = unsafe { libc::munmap(self.ptr as *mut libc::c_void, bytes) };
                assert!(
                    ret == 0,
                    "munmap failed: {}",
                    std::io::Error::last_os_error()
                );
            }
        }
    }

    #[cfg(not(target_os = "linux"))]
    unsafe fn dealloc(&mut self) {
        let layout = std::alloc::Layout::array::<T>(self.len).unwrap();
        unsafe { std::alloc::dealloc(self.ptr as *mut u8, layout) };
    }
}
