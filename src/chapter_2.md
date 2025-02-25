# OS level abstraction and memory pool

在上一篇文章中，我们实现了一个最简单的内存分配器，但是这个内存分配器每次进行malloc的时候都会调用`mmap`系统调用，这样频繁的系统调用，会显著降低性能。为了解决这个问题，我们需要在操作系统层面进行抽象，引入一个内存池，这样我们就可以一次性分配一大块内存，然后在用户层面进行内存分配，这样就可以减少系统调用的次数。

## OS level abstraction

首先，我们将上篇Blog中关于`mmap`和`munmap`的系统调用封装成一个OS层面的抽象，这样我们就可以在用户层面直接调用这个抽象，而不用关心具体的系统调用。

```rust
pub struct MiOsAPI;
impl MiOsAPI {
   pub fn mi_mmap(size: usize) -> *mut u8;
   pub fn mi_munmap(addr: *mut u8, size: usize) -> bool;
   pub fn mi_mmap_aligned(size: usize, alignment: usize) -> *mut u8;
}
```

其中`mi_mmap_aligned`函数是一个对`mi_mmap`的封装，它可以指定内存的对齐方式。

## 内存池

接下来，我们将实现一个简单的内存池，内存池的实现如下：

```rust
pub struct MiOsPool {
    pub pool: *mut u8,
    pub pool_available: usize,
}
```

`MiOsPool`结构体中包含了一个指向内存池的指针`pool`，以及内存池的大小`pool_available`。在`MiOsPool`中，我们采用的是一个线性分配的方式，即每次分配内存的时候，我们都会从内存池的头部开始分配，然后将内存池的指针向后移动。当内存池的内存为0的时候，我们以`MI_SEGMENT_SIZE=0x400000`对齐的方式向内核申请一块大小为`MI_POOL_SIZE=0x2800000`的内存（10个`MI_SEGMENT_SIZE`的大小）。在当前的实验中，为了简单起见，每次的内存分配也都是`MI_SEGMENT_SIZE`对齐的。具体的实现如下：

```rust
pub unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
   if self.pool_available == 0 {
         self.new_pool();
   }
   // if we can't allocate a new pool, directly return
   if self.pool.is_null() { return null_mut();}
   // now, for simplicity, we just align the layout to the page size
   let layout = layout.align_to(MI_SEGMENT_SIZE).unwrap();
   self.pool_available -= layout.size();
   let p = self.pool;
   self.pool = self.pool.add(layout.size());
   p
}
```

在`new_pool`函数中，我们向内核申请了一块大小为`MI_POOL_SIZE`的内存，其中内存以`MI_SEGMENT_SIZE`对齐。

最后，通过`static mut`关键字，我们在全局定义一个主内存池，这样我们就可以在`MiAllocator`中直接调用这个内存池，实现内存的分配。

```rust
static mut MI_POOL_MAIN: MiOsPool = MiOsPool::new();
```

## 总结

在这篇Blog中，我们实现了一个OS层面的内存抽象，以及一个简单的内存池。目前的内存池只是一个简单的线性分配，且每次分配的内存都是以`MI_SEGMENT_SIZE`对齐的。这样的内存分配方式虽然简单，但在多线程下存在`MI_POOL_MAIN`的竞争问题，以及`MI_SEGMENT_SIZE`对齐的内存分配粒度过大，会导致内存的浪费的问题。在后续的Blog中，我们将会解决这些问题，同时引入更多的内存分配策略。

## 思考

1. 通过`mmap`分配如此大的内存会有什么问题？Linux内核是如何处理这种大内存的分配的？(Hint: `mmap`分配的内存是虚拟内存，而不是物理内存)
