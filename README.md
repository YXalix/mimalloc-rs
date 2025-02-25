# Mimalloc-Rust

> Mimalloc：Free List Sharding in Action  
> [论文](https://www.microsoft.com/en-us/research/uploads/prod/2019/06/mimalloc-tr-v1.pdf)
> [源码](https://github.com/microsoft/mimalloc)

## 问题背景

现代内存分配器必须平衡众多同时存在的需求，包括性能、安全性、并行性和特定于应用程序的需求(具体取决于它们的使用环境)。分配器的一个越来越多的用例是作为语言的后端实现，例如 Swift、Python、Java这些语言使用引用计数来自动释放对象，并且通常分配许多**小的且生命周期短的**对象。

## 创新点

- 主要思想是对空闲列表进行分片，其中堆被分成多个页(通常为64KiB)，具有更好的空间局部性
- 再次分割FreeList，分为Free List与Local Free List，并进一步减少Fast Path的条件判断
- 使用单独的线程空闲列表来存放其他线程释放的内存，避免锁竞争

## TODO
