use super::{utils::*, constants::*};
use core::ptr::null_mut;
use core::alloc::Layout;
use core::cell::Cell;

const PAGE_SIZE: usize = 4096;
const MMAP_FLAG: i32 = libc::MAP_PRIVATE | libc::MAP_ANONYMOUS | libc::MAP_NORESERVE;
const MMAP_PROT: i32 = libc::PROT_READ | libc::PROT_WRITE;

pub struct MiOsPool {
    pub pool: *mut u8,
    pub pool_available: usize,
}

pub static mut MI_POOL_MAIN: MiOsPool = MiOsPool::default();
pub static MI_POOL_EMPTY: MiOsPool = MiOsPool::default();

thread_local! {
    pub static _MI_POOL_DEFAULT: Cell<*const MiOsPool> = Cell::new(&MI_POOL_EMPTY as *const MiOsPool);
}

pub fn get_pool_default() -> *mut MiOsPool {
    _MI_POOL_DEFAULT.with(|p| {p.get()}) as *mut MiOsPool
}

pub fn set_pool_default(pool: *const MiOsPool) {
    _MI_POOL_DEFAULT.with(|p| {p.set(pool)});
}

unsafe impl Sync for MiOsPool {}

impl MiOsPool {
    pub const fn default() -> Self {
        Self { 
            pool: null_mut(), 
            pool_available: 0 
        }
    }
    pub unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
        if self.pool_available == 0 {
            self.new_pool();
        }
        // if we can't allocate a new pool, directly return
        if self.pool.is_null() { return null_mut();}
        // now, for simplicity, we just align the layout to the segment size
        let sz = mi_align_up(layout.size(), MI_SEGMENT_SIZE);
        self.pool_available -= sz;
        let p = self.pool;
        self.pool = self.pool.add(sz);
        p
    }
    pub unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        MiOsAPI::mi_munmap(ptr, layout.size());
    }    

    fn new_pool(&mut self) {
        self.pool = MiOsAPI::mi_mmap_aligned(MI_POOL_SIZE, MI_POOL_ALIGNMENT);
        self.pool_available = if self.pool.is_null() { 0 } else { MI_POOL_SIZE };
    }
}


pub struct MiOsAPI;

impl MiOsAPI {
    // mmap a memory region
    pub fn mi_mmap(size: usize) -> *mut u8 {
        if size == 0 {return null_mut();}
        let p = unsafe {
                libc::mmap( null_mut(), size, MMAP_PROT, MMAP_FLAG, -1, 0)
            };
        if p == libc::MAP_FAILED {
            null_mut()
        } else {
            p as *mut u8
        }
    }

    // unmap a memory region
    pub fn mi_munmap(addr: *mut u8, size: usize) -> bool {
        if addr.is_null() || size == 0 {return true;}
        unsafe { libc::munmap(addr as *mut _, size) != -1 }
    }

    // mmap a memory region with alignment
    pub fn mi_mmap_aligned(size: usize, alignment: usize) -> *mut u8 {
        let alloc_size = size + alignment;
        // avoid overflow
        assert!(size < alloc_size, "size + alignment overflow");
        let p = Self::mi_mmap(alloc_size);
        if p.is_null() {return p;}
        let aligned_p = mi_align_up(p as usize, alignment);
        // |<-pre->|<-         mid         ->|<-  post ->|
        // p   aligned_p          aligned_p + midsize
        let pre_size = aligned_p - p as usize;
        let mid_size = mi_align_up(size, PAGE_SIZE);
        let post_size = alloc_size - mid_size - pre_size;
        if pre_size > 0 { Self::mi_munmap(p, pre_size); }
        if post_size > 0 { Self::mi_munmap((aligned_p + mid_size) as *mut u8, post_size); }
        aligned_p as *mut u8
    }
}