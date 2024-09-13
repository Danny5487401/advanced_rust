<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
**Table of Contents**  *generated with [DocToc](https://github.com/thlorenz/doctoc)*

- [**rust 背包客**](#rust-%E8%83%8C%E5%8C%85%E5%AE%A2)
  - [第一章：log 日志](#%E7%AC%AC%E4%B8%80%E7%AB%A0log-%E6%97%A5%E5%BF%97)
  - [第二章：feature 条件编译](#%E7%AC%AC%E4%BA%8C%E7%AB%A0feature-%E6%9D%A1%E4%BB%B6%E7%BC%96%E8%AF%91)
  - [第三章：所有权](#%E7%AC%AC%E4%B8%89%E7%AB%A0%E6%89%80%E6%9C%89%E6%9D%83)
  - [第四章：生命周期](#%E7%AC%AC%E5%9B%9B%E7%AB%A0%E7%94%9F%E5%91%BD%E5%91%A8%E6%9C%9F)
  - [第五章：内存模型](#%E7%AC%AC%E4%BA%94%E7%AB%A0%E5%86%85%E5%AD%98%E6%A8%A1%E5%9E%8B)
  - [第六章：类型](#%E7%AC%AC%E5%85%AD%E7%AB%A0%E7%B1%BB%E5%9E%8B)
  - [第七章：指针](#%E7%AC%AC%E4%B8%83%E7%AB%A0%E6%8C%87%E9%92%88)
  - [第八章：集合容器](#%E7%AC%AC%E5%85%AB%E7%AB%A0%E9%9B%86%E5%90%88%E5%AE%B9%E5%99%A8)
  - [第九章：hash](#%E7%AC%AC%E4%B9%9D%E7%AB%A0hash)
  - [第十章：error 错误处理](#%E7%AC%AC%E5%8D%81%E7%AB%A0error-%E9%94%99%E8%AF%AF%E5%A4%84%E7%90%86)
  - [第十一章：闭包](#%E7%AC%AC%E5%8D%81%E4%B8%80%E7%AB%A0%E9%97%AD%E5%8C%85)
  - [第十二章：unsafe](#%E7%AC%AC%E5%8D%81%E4%BA%8C%E7%AB%A0unsafe)
  - [第十三章：atomic and ordering](#%E7%AC%AC%E5%8D%81%E4%B8%89%E7%AB%A0atomic-and-ordering)
  - [第十四章：并发处理](#%E7%AC%AC%E5%8D%81%E5%9B%9B%E7%AB%A0%E5%B9%B6%E5%8F%91%E5%A4%84%E7%90%86)
  - [第十五章：宏编程](#%E7%AC%AC%E5%8D%81%E4%BA%94%E7%AB%A0%E5%AE%8F%E7%BC%96%E7%A8%8B)
  - [参考](#%E5%8F%82%E8%80%83)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

# **rust 背包客**

![rust logo](rust-logo.png)

[cargo workspace来管理多个package](workspace.md)

## [第一章：log 日志](chapter01-log/log.md)

- 1 env_logger 记录日志
- 2 tracing 记录日志

## [第二章：feature 条件编译](chapter02-feature/feature.md)

## [第三章：所有权](chapter03-ownership/ownership.md)

- 1 COPY 浅拷贝: 赋值或传参会优先使用 Copy 语义
    - [1.1 哪些结构可以 Copy、哪些不可以 Copy](chapter03-ownership/src/ownership2-copy.rs)
- 2 Move 转移所有权：无 Copy,赋值或者传参会导致值 Move
- 3 Borrow 借用（通过 & 或者 &mut）
- 4 多个所有者
    - 4.1 非线程安全 Rc（Reference counter 只读引用计数器）
    - [4.2 非线程安全 RefCell 实现内部可变性](chapter03-ownership/src/ownership1-borrow-mut)
    - 4.3 线程安全 Arc（Atomic reference counter）
    - 4.4 线程安全 Mutex 和 RwLock 实现内部可变性
- [5 Clone 数据深拷贝](chapter03-ownership/src/ownership3-clone.rs)

## [第四章：生命周期](chapter04-lifecycle/lifecycle.md)

- 1 静态生命周期 和 动态生命周期
- [2 生命周期标注(lifetime specifier)](chapter04-lifecycle/src/lifecycle1.rs)

## [第五章：内存模型](chapter05-memory/memory.md)

- [1 enum、Option 以及 Result 的布局](chapter05-memory/src/memory1-enum.rs)
- 2 move and copy 内存模型

## [第六章：类型](chapter06-type/type.md)

- 1 参数多态 (parametric polymorphism）
    - [1.1 泛型参数 R 的数据结构体,使用时再限制](chapter06-type/src/type1-paramiter.rs)
    - 1.2 泛型函数
- 2 特设多态 (adhoc polymorphism)
    - [2.1 带关联类型的 trait:把错误类型延迟到 trait 实现时才决定](chapter06-type/src/type3-related-trait.rs)
- 3 子类型多态（subtype polymorphism）
    - [3.1 静态分派 (static dispatching): 使用泛型函数](chapter06-type/src/type4-child.rs)
    - [3.2 动态分派 (dynamic dispatching): 使用 trait object](chapter06-type/src/type5-dynamic-dispatch.rs)
- [4 AsRef 类型转换](chapter06-type/src/type6-asref.rs)
- [5 Deref解引用运算符（*）的重载:直接访问 Vec<T> 的方法](chapter06-type/src/type7-deref.rs)

## [第七章：指针](chapter07-pointer/pointer.md)

- 1 COW(Clone on write 写时克隆) 智能指针
- 2 mutexGuard 智能指针

## [第八章：集合容器](chapter08-vec/vec.md)

- [1 动态数组 Vector 的 切片引用&[T] 和 数组[T; n] 的 切片引用&[T] 对比](chapter08-vec/src/vec1-slice.rs)
- [2 动态数组 Vector 和 数组 [T; n] 如何转化成 &[T]](chapter08-vec/src/vec2-slice.rs)
- [3 String、&String 转换成 &str](chapter08-vec/src/vec3-string-slice.rs)

## [第九章：hash](chapter09-hash/hash.md)

- [1 hashmap cap 扩容和缩容](chapter09-hash/src/hash1-cap.rs)
- 2 HashSet
- 3 BTreeMap

## [第十章：error 错误处理](chapter10-error/error.md)

- [1 使用 ? 进行传播错误](chapter10-error/src/error1-transfer.rs)

## [第十一章：闭包](chapter11-closure/closure.md)

- [1 闭包的大小跟参数、局部变量都无关，只跟捕获的变量有关](chapter11-closure/src/closure1-size.rs)

## [第十二章：unsafe](chapter12-unsafe/unsafe.md)

- [1 开发者还是调用者保证内存安全](chapter12-unsafe/src/unsafe1-caller.rs)

## [第十三章：atomic and ordering](chapter13-atomic_n_ordering/ordering.md)

- 1 验证 Ordering 的可见性：两线程并发增加值
    - 容易大于 1000000
    - 最多只能 1000000

## 第十四章：并发处理

- [1 Condvar 条件变量](chapter14-concurrecy/src/01_convar.md)
- [2 Future](chapter14-concurrecy/src/02_future.md)
    - 2.1 await 来执行 future，或则使用一个 executor 来执行 future
    - 2.2 将值固定到栈上
    - 2.3 将值固定到堆上
    - 2.4 将固定住的 Future 变为 Unpin
- [3 tokio](chapter14-concurrecy/src/03_tokio.md)
    - 3.1 #[tokio::main] 的作用
    - 3.2 消息通道: mpsc 多生产者，单消费者模式
    - 3.3 消息通道: watch 单生产者、多消费者

## [第十五章：宏编程](chapter15-macro/macro.md)

- 1 声明宏（declarative macro）
    - [1.1 vec! 实现](chapter15-macro/examples/macro1-declarativemacro.rs)
    - [1.2 声明宏的 hygiene:宏内部代码和宏上下文相互影响](chapter15-macro/examples/macro2-declarativemacro-hygiene.rs)
- 2 过程宏（procedural macro）
    - 2.1 派生宏（derive macro）
    - 2.2 属性宏（attribute macro)

## 参考

- [Rust语言圣经](https://github.com/sunface/rust-course)
- [Rust 程序设计语言](https://rustwiki.org/zh-CN/book/title-page.html)
- [Rust Practice](https://github.com/sunface/rust-by-practice)
- [rust-by-example](https://github.com/rust-lang/rust-by-example)
- [Rust 秘典(死灵书)](https://github.com/rust-lang-cn/nomicon-zh-Hans)
- [陈天 · Rust 编程第一课](https://time.geekbang.org/column/intro/100085301?tab=catalog)