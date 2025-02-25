# Initial project setup

## 项目介绍

为什么要写一个内存分配器呢？这是因为我想要学习一下内存分配器的实现，同时也想要了解一下Rust的底层实现。不需要有任何的畏难情绪，在这个系列中，基于微软开源的mimalloc库V1.0，我们将会一步一步实现一个完整可用的简化mimalloc-rs库。

## 项目结构

```c
mimalloc-rs
├── examples
│   └── helloworld
├── src
│   ├── lib.rs
│   └── os.rs
└── tests
    └── lib.rs
├── Cargo.toml
├── README.md
```

这就是最开始的项目结构，我们将会在`src`目录下实现我们的内存分配器，在`examples`目录下提供一些示例，`tests`目录下编写测试用例。

## 最简单的内存分配器

首先，现代操作系统提供了一些内存分配的系统调用，例如`mmap`和`munmap`，那么我们可以通过这些系统调用来实现一个最简单的内存分配器，完整代码在[这里](TODO)。

对于`mmap` 和 `munmap` 系统调用，我们可以通过`libc`库来调用封装好的API接口，首先在`Cargo.toml`中添加依赖：

```toml
[dependencies]
libc = "*"
```

然后在`os.rs`中实现`mmap`和`munmap`函数。

那么如何在Rust中使用我们的内存分配器呢？我们可以通过`#[global_allocator]`属性来指定我们的内存分配器，然后在`lib.rs`中实现`GlobalAlloc` trait。

```rust
use core::alloc::{GlobalAlloc, Layout};
pub struct MiAllocator;
unsafe impl GlobalAlloc for MiAllocator{
    ...Your implementation...
}
#[global_allocator]
static GLOBAL: MiAllocator = MiAllocator;
```

在`examples`目录下，我们可以编写一个简单的示例程序来测试我们的内存分配器。

```rust
#[global_allocator]
static GLOBAL: MiAllocator = MiAllocator {};
fn main() {
    println!("Hello, world!");
}
```

由于println!宏会调用内存分配器，所以我们可以通过这个示例程序来测试我们的内存分配器是否正常工作。但这也让我们在实现内存分配器时遇到了一个问题，我们无法使用println!宏来打印调试信息，因为println!宏会调用内存分配器，而内存分配器又会调用println!宏，这样就会导致递归调用，最终导致栈溢出。幸运的是，我们可以使用`libc-print`库来打印调试信息，这个库不会调用内存分配器。

```toml
[dependencies]
libc-print = "*"
```

## 总结

这是一个非常简单的内存分配器，但是它已经可以完成一些基本的内存分配和释放操作了。但是这个内存分配器存在一个很大的问题，那就是只要分配一个内存块，就会调用一次`mmap`系统调用，这样会导致很大的开销。在接下来的文章中，我们将会逐步完善这个内存分配器。

## 思考

1. `mmap`和`munmap`系统调用的实现原理是什么？
2. 为什么要使用`mmap`和`munmap`系统调用来实现内存分配器，是否可以使用sbrk系统调用来实现？
3. `#[global_allocator]`是如何实现自定义全局内存分配器的？
4. 为什么println!宏会调用内存分配器?以及`libc-print`库是如何避免这个问题的？
