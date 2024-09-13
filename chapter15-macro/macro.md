<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
**Table of Contents**  *generated with [DocToc](https://github.com/thlorenz/doctoc)*

- [macro 宏](#macro-%E5%AE%8F)
  - [声明宏（declarative macro）](#%E5%A3%B0%E6%98%8E%E5%AE%8Fdeclarative-macro)
    - [声明宏的卫生性 hygiene](#%E5%A3%B0%E6%98%8E%E5%AE%8F%E7%9A%84%E5%8D%AB%E7%94%9F%E6%80%A7-hygiene)
  - [过程宏（procedural macro）](#%E8%BF%87%E7%A8%8B%E5%AE%8Fprocedural-macro)
    - [派生宏（derive macro 可推导宏）](#%E6%B4%BE%E7%94%9F%E5%AE%8Fderive-macro-%E5%8F%AF%E6%8E%A8%E5%AF%BC%E5%AE%8F)
    - [属性宏（attribute macro)](#%E5%B1%9E%E6%80%A7%E5%AE%8Fattribute-macro)
      - [第三方实现 --> #[tokio::main]](#%E7%AC%AC%E4%B8%89%E6%96%B9%E5%AE%9E%E7%8E%B0----tokiomain)
    - [函数宏（function-like macro）](#%E5%87%BD%E6%95%B0%E5%AE%8Ffunction-like-macro)
  - [其他编程语言常见的元编程方式](#%E5%85%B6%E4%BB%96%E7%BC%96%E7%A8%8B%E8%AF%AD%E8%A8%80%E5%B8%B8%E8%A7%81%E7%9A%84%E5%85%83%E7%BC%96%E7%A8%8B%E6%96%B9%E5%BC%8F)
  - [工具](#%E5%B7%A5%E5%85%B7)
    - [cargo-expand](#cargo-expand)
    - [syn](#syn)
    - [quote](#quote)
  - [参考](#%E5%8F%82%E8%80%83)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

# macro 宏

元编程可以让开发者将原生语言写的代码作为数据输入，经过自定义的逻辑，重新输出为新的代码并作为整体代码的一部分。这个过程一般在编译时期完成（对于编译型语言来说），所以让人觉得这是一种神奇的
“黑魔法

宏就两大类：对代码模板做简单替换的声明宏（declarative macro）、可以深度定制和生成代码的过程宏（procedural macro）。

宏调用有三种等价的形式：marco!(xx), marcro![xxx], macro!{xx}。惯例是：

- 函数传参调用场景使用 () 形式，如 println!();
- 字面量初始化使用 [] 形式，如 vec![0; 4];

## 声明宏（declarative macro）

声明宏可以用 macro_rules! 来描述,比如像 vec![]、println!、以及 info!，它们都是声明宏。

声明式宏类似于 match 匹配。它可以将表达式的结果与多个模式进行匹配。一旦匹配成功，那么该模式相关联的代码将被展开。和 match
不同的是，宏里的值是一段 rust 源代码。所有这些都发生在编译期，并没有运行期的性能损耗

```rust
#[cfg(all(not(no_global_oom_handling), not(test)))]
#[macro_export]
#[stable(feature = "rust1", since = "1.0.0")]
#[rustc_diagnostic_item = "vec_macro"]
#[allow_internal_unstable(rustc_attrs, liballoc_internals)]
macro_rules! vec {
    () => (
        $crate::__rust_force_expr!($crate::vec::Vec::new())
    );
    // 匹配到以 ; 分隔的两个表达式，; 左边的表达式的值将被捕获匹配到 $elem，; 右边的表达式的值将被捕获匹配到 $n
    ($elem:expr; $n:expr) => (
        $crate::__rust_force_expr!($crate::vec::from_elem($elem, $n))
    );
    ($($x:expr),+ $(,)?) => (
        $crate::__rust_force_expr!(<[_]>::into_vec(
            // This rustc_box is not required, but it produces a dramatic improvement in compile
            // time when constructing arrays with many elements.
            #[rustc_box]
            $crate::boxed::Box::new([$($x),+])
        ))
    );
}
```

- $crate 是一个特殊的元变量，用来指代当前 crate
- 条件捕获的参数使用 $ 开头的标识符来声明
- #[macro_export]标签是用来声明：只要 use 了这个crate，就可以使用该宏。同时包含被 export 出的宏的模块，在声明时必须放在前面，否则靠前的模块里找不到这些宏

macro_rules! 的基本结构

```shell
macro_rules! $ name {
  $ rule0;
  $ rule1;
  //...
  $ ruleN;
}
```

每一条 rule 其实就是模式匹配和代码扩展生成：

```shell
( $matcher ) => { $expansion };
```

类似 vec![0; 10] 的功能时,0; 10 ，其中 ; 左边是元素初始值 0，; 右边是个数 10。那么匹配似乎可以为：

```shell
($elem ; $n) => { ... }
```

描述是不精确的，我们还需要加上捕获方式，即捕获的是一个表达式

```shell
($elem:expr ; $n:expr) => { ... }
```

```rust
let v = vec![1, 2, 3];
```

先看 $matcher 部分，即 1, 2, 3。像这种需要匹配一系列 token 的模式，我们需要使用宏里的重复匹配模式。比如要想匹配 1,2,3，可以写成：

```rust
( $ ( $ elem:expr ), * ) => { ... }
```

即 $(...),* 模式，而 (...) 则是和上节中变量捕获的方式是一样的，即 $elem:expr。, 表示为分隔符，* 表示匹配 0 或者多次.

为参数明确类型，哪些类型可用也整理在这里了：

- item，比如一个函数、结构体、模块等。
- block，代码块。比如一系列由花括号包裹的表达式和语句。
- stmt，语句。比如一个赋值语句。
- pat，模式。
- expr，表达式。刚才的例子使用过了。ty，类型。比如 Vec。
- ident，标识符。比如一个变量名。path，路径。比如：foo、::std::mem::replace、transmute::<_, int>。meta，元数据。一般是在 #[...]
  和 #![...]  属性内部的数据。
- tt，单个的 token 树(一个独立的 token 或一系列在匹配完整的定界符 ()、[] 或 {} 中的 token）。
- vis，可能为空的一个 Visibility 修饰符。比如 pub、pub(crate)

### 声明宏的卫生性 hygiene

宏的卫生性，其实说的就是宏在上下文工作不影响或不受周围环境的影响。或者换句话来说，就是宏的调用是没有 side effect。对于
macro_rules!，
它是部分卫生的（partially hygienic）。我们目前阶段可以不用太关注 macro_rules! 在哪些场景是 “不卫生” 的，而是了解一下
macro_rules! 是如何在大多数场景做到 “卫生” 的

```rust
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use chapter15_macro::make_local;
fn main() {
    let local = 42;
    let local = 0;
    match (&local, &42) {
        (left_val, right_val) => {
            if !(*left_val == *right_val) {
                let kind = ::core::panicking::AssertKind::Eq;
                ::core::panicking::assert_failed(
                    kind,
                    &*left_val,
                    &*right_val,
                    ::core::option::Option::None,
                );
            }
        }
    };
}
```

Rust 看来，macro_rules 中的 local 和 main() 里的 local 分别有着不同的颜色，所以不会将其混淆

## 过程宏（procedural macro）

过程宏分为三种：

- 函数宏（function-like macro）：custom!(…) 看起来像函数的宏，但在编译期进行处理。比如 sqlx 里的 query 宏，它内部展开出一个
  expand_query
  函数宏。你可能想象不到，看上去一个简单的 query 处理，内部有多么庞大的代码结构。
- 属性宏（attribute macro）：#[CustomAttribute]可以在其他代码块上添加属性，为代码块提供更多功能。比如 rocket 的 get / put
  等路由属性，#[tokio::main] 来引入 runtime。
- 派生宏（derive macro 可推导宏）：为 derive 属性添加新的功能,一般用来为 struct/enum/union 实现特定的
  trait。这是我们平时使用最多的宏，比如 #[derive(Debug)] 为我们的数据结构提供
  Debug trait
  的实现、#[derive(Serialize, Deserialize)]为我们的数据结构提供 serde 相关 trait 的实现

它更像函数，他接受一些代码作为参数输入，然后对他们进行加工，生成新的代码，他不是在做声明式宏那样的模式匹配

不能在原始的crate中直接写过程式宏，需要把过程式宏放到一个单独的crate中（以后可能会消除这种约定）。定义过程式宏的方法如下

```rust
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {}

```

在单独的 crate package 中定义过程宏的原因:

proc macro 定义需要先被编译器编译为 host 架构类型，后续编译使用它的代码时，编译器才能 dlopen 和执行它们来为 target 架构生成代码；
非过程宏 crate 需要被边翼卫 target 架构类型，然后才能被和其它 target 架构的动态库链接；

需要引入proc_macro 这个 crate，然后标签是用来声明它是哪种过程式宏的，接着就是一个函数定义，函数接受 TokenStream，返回
TokenStream。TokenStream 类型就定义在 proc_macro 包中，表示 token 序列。

除了标准库 proc_macro 中的这个包，还可以使用proc_macro2 包，使用 proc_macro2::TokenStream::from() 和 proc_macro::
TokenStream::from()
可以很便捷地在两个包的类型间进行转换。
使用 proc_macro2 的好处是可以在过程宏外部使用 proc_macro2 的类型，相反 proc_macro 中的类型只可以在过程宏的上下文中使用。
且 proc_macro2 写出的宏更容易编写测试代码。

### 派生宏（derive macro 可推导宏）

派生宏可以自动生成实现特定trait的代码，减少手动实现的繁琐性。

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}
```

#[derive(Debug)]是一个派生宏，它告诉Rust编译器为Person结构体自动生成Debug trait的实现

### 属性宏（attribute macro)

在Rust中，属性宏是一种特殊的宏，它允许开发者在代码上方添加自定义的属性，并在编译期间对代码进行处理。属性宏使用proc_macro_attribute属性来定义

```rust
extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn attribute_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    // 宏的处理逻辑
    // ...
}

```

使用proc_macro_attribute属性来定义了一个名为attribute_macro的属性宏。
属性宏接受两个TokenStream参数：attr表示属性的输入，item表示应用该属性的代码块。在宏的处理逻辑中，我们可以根据attr和item对代码进行定制化处理，并返回一个TokenStream作为输出。

#### 第三方实现 --> #[tokio::main]

```rust
//  tokio-macros-2.4.0/src/entry.rs
fn parse_knobs(mut input: ItemFn, is_test: bool, config: FinalConfig) -> TokenStream {
    // ...

    let mut rt = match config.flavor {
        RuntimeFlavor::CurrentThread => quote_spanned! {last_stmt_start_span=>
            #crate_path::runtime::Builder::new_current_thread()
        },
        RuntimeFlavor::Threaded => quote_spanned! {last_stmt_start_span=>
            #crate_path::runtime::Builder::new_multi_thread()
        },
    };
    // ... 

    let generated_attrs = if is_test {
        quote! {
            #[::core::prelude::v1::test]
        }
    } else {
        quote! {}
    };

    let body_ident = quote! { body };
    let last_block = quote_spanned! {last_stmt_end_span=>
        #[allow(clippy::expect_used, clippy::diverging_sub_expression)]
        {
            return #rt
                .enable_all()
                .build()
                .expect("Failed building the Runtime")
                .block_on(#body_ident);
        }
    };

    let body = input.body();

    // .. 

    input.into_tokens(generated_attrs, body, last_block)
}
```

### 函数宏（function-like macro）

## 其他编程语言常见的元编程方式

- Go 的 ast 包和 go generate 机制：Go 没有显示提供元编程的相应机制，转而提供了一些相对不那么优雅的机制来实现类似于元编程的效果。比如
  ast 包可以暴露 Go 程序的语法树，从而让开发者可在编译时期对源代码进行修改或者根据模版生成其他类型代码。
- C++ 的 Template 编程：据说 C++ 的 Template 编程是图灵完备的，可在编译时期完成很多让人瞠目结舌的逻辑。由于 C++ 的 Template
  编程非常复杂且难以掌握，所以易用性非常差。
- C 语言的宏：这估计是大多数程序员对于宏的最初体验。个人觉得， C 语言中的宏本质上是发生在预处理过程的文本替换，是一种非常简单原始的元编程机制。而正是这种原始能力，导致
  C 语言的宏结合编译器的各种扩展充满了各种奇技淫巧，可读性和可调试性都非常差，而且稍不小心就很容易写出错误的宏

## 工具

### cargo-expand

https://github.com/dtolnay/cargo-expand

一个Rust cargo子命令扩展，通过简单的 cargo expand 命令，你可以获取当前项目中所有源码经过宏展开后的结果

```shell
# 直接安装 cargo-expand 插件
$ cargo install cargo-expand
```

### syn

syn 是一个对 TokenStream 解析的库，它提供了丰富的数据结构，对语法树中遇到的各种 Rust 语法都有支持。

比如一个 Struct 结构，在 TokenStream 中，看到的就是一系列 TokenTree，而通过 syn 解析后，struct
的各种属性以及它的各个字段，都有明确的类型。这样，我们可以很方便地通过模式匹配来选择合适的类型进行对应的处理。

### quote

quote 是一个特殊的原语，它把代码转换成可以操作的数据（代码即数据）

## 参考

- [Rust 的声明宏机制](https://www.cnblogs.com/RioTian/p/18130417)
- [Rust宏及声明式宏项目MacroKata](https://forsworns.github.io/zh/blogs/20210224/)