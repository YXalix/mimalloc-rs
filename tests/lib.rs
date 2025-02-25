//! tests/lib.rs
use mimalloc_rs::MiAllocator;

#[global_allocator]
static GLOBAL: MiAllocator = MiAllocator {};

#[test]
fn hello_world() {
    println!("Hello, world!");
}