// Simplified version of the mimalloc
//! # mimalloc-rs
use core::alloc::{GlobalAlloc, Layout};
use heap::{get_heap_default, set_heap_default, MI_HEAP_MAIN};
use os::{get_pool_default, set_pool_default, MI_POOL_MAIN};
use ctor::ctor;
use libc_print::libc_println;
use utils::mi_thread_id;

mod os;
mod utils;
mod constants;
mod heap;

/// A mimalloc global allocator.
#[derive(Default)]
pub struct MiAllocator;

unsafe impl GlobalAlloc for MiAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        (*get_pool_default()).alloc(layout)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        (*get_pool_default()).dealloc(ptr, layout)
    }
    unsafe fn alloc_zeroed(&self, _layout: Layout) -> *mut u8 {
        let ptr = self.alloc(_layout);
        if !ptr.is_null() {
            core::ptr::write_bytes(ptr, 0, _layout.size());
        }
        ptr
    }
    unsafe fn realloc(&self, _ptr: *mut u8, _layout: Layout, _new_size: usize) -> *mut u8 {
        todo!()
    }
}

extern "C" {
    pub fn atexit(f: extern "C" fn()) -> i32;
}

extern "C" fn mi_process_done() {
    libc_println!("process is shutting down");
}

#[ctor]
fn mi_process_init() {
    // Initialize the mimalloc pool
    libc_println!("Initializing mimalloc pool");

    unsafe {
        set_pool_default(&raw const MI_POOL_MAIN);
        set_heap_default(&raw const MI_HEAP_MAIN);
        (*get_heap_default()).thread_id = mi_thread_id();
        atexit(mi_process_done);
    }
}