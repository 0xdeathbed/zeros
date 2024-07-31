# zeros
Operating System or Kernel in Rust


## 1. Minimal Kernel

Creating a minimal rust binary which does can be run without underlaying operating system, Also known as "freestanding" or "bare-metal" executable. Meaning losing ability to use `Rust Standard Library` but still able to use most of rust features eg., `iterators`,`closures`,`pattern matching`,`Enums (including Option and Result)`,`String formating`,`ownership system`,etc.

Our kernel is should run on bare-metal `x86-64` system. So we need to compile to a suitable target triple. And target should not have any underlying operating system. We can get this suitable triple `x86-64-unknown-none` by using rustup.

```bash
cargo build --target x86_64-unknown-none # --relase
  
```
