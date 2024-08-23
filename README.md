# **rust 背包客**

![rust logo](rust-logo.png)

[cargo workspace来管理多个package](workspace.md)

## [第三章：所有权](chapter03-ownership/ownership.md)

- 1 COPY 浅拷贝: 赋值或传参会优先使用 Copy 语义
- 2 Move 转移所有权：无 Copy,赋值或者传参会导致值 Move
- 3 Borrow 借用（通过 & 或者 &mut）
- 4 多个所有者
    - 4.1 非线程安全 Rc（Reference counter 只读引用计数器）
    - [4.2 非线程安全 RefCell 实现内部可变性](chapter03-ownership/src/ownership1.rs)
    - 4.3 线程安全 Arc（Atomic reference counter）
    - 4.4 线程安全 Mutex 和 RwLock 实现内部可变性

## [第四章：生命周期](chapter04-lifecycle/lifecycle.md)

- 1 静态生命周期 和 动态生命周期
- [2 生命周期标注(lifetime specifier)](chapter04-lifecycle/src/lifecycle1.rs)

## [第五章：内存模型](chapter05-memory/memory.md)

- [1 enum、Option 以及 Result 的布局](chapter05-memory/src/memory1-enum.rs)
- 2 move and copy 内存模型

## 参考

- [Rust语言圣经](https://github.com/sunface/rust-course)
- [Rust 程序设计语言](https://rustwiki.org/zh-CN/book/title-page.html)
- [Rust Practice](https://github.com/sunface/rust-by-practice)
- [rust-by-example](https://github.com/rust-lang/rust-by-example)
- [陈天 · Rust 编程第一课](https://time.geekbang.org/column/intro/100085301?tab=catalog)