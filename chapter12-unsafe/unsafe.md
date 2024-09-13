<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
**Table of Contents**  *generated with [DocToc](https://github.com/thlorenz/doctoc)*

- [unsafe](#unsafe)
  - [使用 unsafe Rust 的原因](#%E4%BD%BF%E7%94%A8-unsafe-rust-%E7%9A%84%E5%8E%9F%E5%9B%A0)
  - [标准库中 unsafe 的函数](#%E6%A0%87%E5%87%86%E5%BA%93%E4%B8%AD-unsafe-%E7%9A%84%E5%87%BD%E6%95%B0)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

# unsafe

## 使用 unsafe Rust 的原因

- 计算机硬件本身是 unsafe 的，比如操作 IO 访问外设，或者使用汇编指令进行特殊操作（操作 GPU 或者使用 SSE
  指令集）。这样的操作，编译器是无法保证内存安全
- 当 Rust 要访问其它语言比如 C/C++ 的库，因为它们并不满足 Rust 的安全性要求，这种跨语言的 FFI（Foreign Function
  Interface），也是 unsafe 的
- 使用 unsafe Rust 纯粹是为了性能。比如略过边界检查、使用未初始化内存等。这样的 unsafe 我们要尽量不用

## 标准库中 unsafe 的函数

距离

- slice::get_unchecked，它不会检查传入索引的有效性，允许违反内存安全的规则。
- mem::transmute将一些数据重新解释为给定的类型，绕过类型安全的规则（详见conversions）。
- 每一个指向一个 Sized 类型的原始指针都有一个offset方法，如果传递的偏移量不在“界内”，则该调用是未定义行为。
- 所有 FFI（外部函数接口 Foreign Function Interface）函数的调用都是unsafe的，因为 Rust 编译器无法检查其他语言的操作。