


# macros 宏



## 场景


宏的主要作用是避免我们创建大量结构相同的脚手架代码。

## 宏 分类
Rust 中宏分为两大类：对代码模板做简单替换的声明宏（declarative macro）和可以深度定制和生成代码的过程宏（procedural macro）



### 声明式宏
在Rust中，应用最广泛的一种宏就是声明式宏，类似于模式匹配的写法，将传入的 Rust 代码与预先指定的模式进行比较，在不同模式下生成不同的代码。

比如像 vec![]、println!、以及 info!，它们都是声明宏。

声明宏可以用 macro_rules! 来描述




### 过程宏

过程宏有三类
- 派生宏（derive macro）：借助 #[derive] 属性标签，它可以用在 struct 和 enum 上。比如 #[derive(Debug)] 为我们的数据结构提供 Debug trait 的实现、#[derive(Serialize, Deserialize)]为我们的数据结构提供 serde 相关 trait 的实现。
- 属性宏(Attribute-like macro) 本身就是一个标签，可以作用于任何地方。
- 函数宏(Function-like macro) 看上去像函数，但是作用在 token 上，即把token作为函数参数


