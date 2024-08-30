<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
**Table of Contents**  *generated with [DocToc](https://github.com/thlorenz/doctoc)*

- [unsafe](#unsafe)
  - [使用 unsafe Rust 的原因](#%E4%BD%BF%E7%94%A8-unsafe-rust-%E7%9A%84%E5%8E%9F%E5%9B%A0)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

# unsafe

## 使用 unsafe Rust 的原因

- 计算机硬件本身是 unsafe 的，比如操作 IO 访问外设，或者使用汇编指令进行特殊操作（操作 GPU 或者使用 SSE
  指令集）。这样的操作，编译器是无法保证内存安全
- 当 Rust 要访问其它语言比如 C/C++ 的库，因为它们并不满足 Rust 的安全性要求，这种跨语言的 FFI（Foreign Function
  Interface），也是 unsafe 的
- 使用 unsafe Rust 纯粹是为了性能。比如略过边界检查、使用未初始化内存等。这样的 unsafe 我们要尽量不用