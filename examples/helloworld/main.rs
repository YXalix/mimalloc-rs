//! examples/hello.rs
use mimalloc_rs::MiAllocator;
use std::string::String;

#[global_allocator]
static GLOBAL: MiAllocator = MiAllocator {};

fn main() {
    let _s = String::from("hello");
    println!("Hello, world!");
}