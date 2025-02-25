use core::cell::Cell;

pub struct MiHeap {
    /// The thread id of the heap
    pub thread_id: usize,
}

impl MiHeap {
    pub const fn default() -> Self {
        Self {
            thread_id: 0,
        }
    }

    pub fn alloc(&self, size: usize) -> *mut u8 {
        todo!()
    }
}

pub static mut MI_HEAP_MAIN: MiHeap = MiHeap::default();
pub static MI_HEAP_EMPTY: MiHeap = MiHeap::default();

thread_local! {
    pub static _MI_HEAP_DEFAULT: Cell<*const MiHeap> = Cell::new(&MI_HEAP_EMPTY as *const MiHeap);
}

pub fn get_heap_default() -> *mut MiHeap {
    _MI_HEAP_DEFAULT.with(|h| {h.get()}) as *mut MiHeap
}

pub fn set_heap_default(heap: *const MiHeap) {
    _MI_HEAP_DEFAULT.with(|h| {h.set(heap)});
}