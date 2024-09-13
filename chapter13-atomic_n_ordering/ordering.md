<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
**Table of Contents**  *generated with [DocToc](https://github.com/thlorenz/doctoc)*

- [数据访问](#%E6%95%B0%E6%8D%AE%E8%AE%BF%E9%97%AE)
- [指令重排](#%E6%8C%87%E4%BB%A4%E9%87%8D%E6%8E%92)
- [ordering](#ordering)
  - [Ordering 的可见性](#ordering-%E7%9A%84%E5%8F%AF%E8%A7%81%E6%80%A7)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

## 数据访问

假设要实现一个计数功能，每次对变量执行+1的操作。CPU执行的时候就需要顺序执行三个操作

1. 从内存中读取变量的值
2. 加1
3. 写回到内存中

如果有两个线程同时操作了这个值，这三个操作可能是交叉的，导致结果不正确

## 指令重排

```rust
fn f(a: &mut i32, b: &mut i32) {
    *a += 1;
    *b += 1;
    *a += 1;
}
```

交给操作系统编译执行，但很可能你得到的是这样的

```rust
fn f(a: &mut i32, b: &mut i32) {
    *a += 2;
    *b += 1;
}
```

**只要不影响程序语义，指令可以重排执行以优化，即不按代码顺序执行。**

单线程下这样问题可能还不大，但如果多线程下，同一线程下多条原子指令，也是会有指令重排的可能，数据竞争很有可能发生，就是说加了原子操作也无法确定数据操作顺序

。

## ordering

Rust用于的内存访问顺序（memory order）的Ordering基本和`C++ 20`的内存排序的保持一致

```rust
#[stable(feature = "rust1", since = "1.0.0")]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[non_exhaustive]
#[rustc_diagnostic_item = "Ordering"]
pub enum Ordering {
    Relaxed,
    Release,
    Acquire,
    AcqRel,
    SeqCst,
}
```

- Relaxed，这是最宽松的规则，它对编译器和 CPU 不做任何限制，可以乱序执行。
- Release，适用于写数据操作
    - 当前线程不能有其他的读或写被 reorder 在 store 之后当前写入后的结果对其他线程的同一数据 Acquire 读取操作是可见的
- Acquire，适用于读取数据操作
    - 当前线程不能有其他的读或写被 reorder 在 load 之前其他线程的同一数据已发生的 Release 写入操作都是对其可见的
- AcqRel ,是 Acquire 和 Release 的结合
    - 一般用在 fetch_xxx 上，比如你要对一个 atomic 自增 1，你希望这个操作之前和之后的读取或写入操作不会被乱序，并且操作的结果对其它线程可见。
- SeqCst,除了 AcqRel 的保证外，它还保证所有线程看到的所有 SeqCst 操作的顺序是一致的。

### Ordering 的可见性

[案例一](./src/ordering1.rs)