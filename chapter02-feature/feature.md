<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
**Table of Contents**  *generated with [DocToc](https://github.com/thlorenz/doctoc)*

- [条件编译 Features](#%E6%9D%A1%E4%BB%B6%E7%BC%96%E8%AF%91-features)
  - [作用](#%E4%BD%9C%E7%94%A8)
  - [参考](#%E5%8F%82%E8%80%83)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

# 条件编译 Features

Cargo Feature 是非常强大的机制，可以为大家提供条件编译和可选依赖的高级特性

## 作用

**控制代码流程**:
例如想在某些情况下启动软件后只执行打印b的逻辑，而不执行打印a的逻辑，只需要不编译 println!("a"); 就可以实现

**减少编译任务量**:
我们经常会遇到在使用第三方库的时候，他很大，导致编译的时间比较长，这时候就可以把自己用不到的 feature 给关掉，尽可能地去减少代码编译时间

**隐藏不稳定代码**:
比如我们在编写代码的时候，经常会有还没有完成，但是正在编辑的功能，就可以 用一个 unstable 的 feature
去包含这些代码，这样在实际使用的时候，就不会用到这些代码。

**实验性的代码生成或优化**:
在某些情况下，编译器团队可能会为 Rustc 提供实验性的优化或代码生成策略，而这些可以通过特性标志来启用

## 参考

- [如何使用 feature 控制代码逻辑](https://juejin.cn/post/7288561954486403111)