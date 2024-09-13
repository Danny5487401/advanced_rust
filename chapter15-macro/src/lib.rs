// 引入 proc_macro crate，用于创建过程宏
extern crate proc_macro;
use proc_macro::TokenStream;
use syn;
// 引入 syn crate，用于解析 Rust 代码
use syn::{parse_macro_input, DeriveInput};
// 引入 quote crate，用于生成代码片段
use quote::quote;

mod builder;

// 定义 Builder 派生宏
#[proc_macro_derive(Builder)]
pub fn derive_builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    builder::BuilderContext::from(input).render().into()
}

// 定义 log_bench 属性宏
#[proc_macro_attribute]
pub fn log_bench(_: TokenStream, item: TokenStream) -> TokenStream {
    // 解析传入的函数定义
    let input_fn = parse_macro_input!(item as syn::ItemFn);

    // 获取函数名和函数体
    let fn_name = &input_fn.sig.ident;
    let fn_block = &input_fn.block;

    // 构建新的函数定义，包含性能日志输出
    let expanded = quote! {
        fn #fn_name() {
            // 获取函数执行开始时间点
            let start = std::time::Instant::now();
            println!("进入函数: {}", stringify!(#fn_name));
            // 执行原函数体
            #fn_block
            println!("离开函数: {} (耗时 {} ms)", stringify!(#fn_name), start.elapsed().as_millis());
        }
    };

    // 将生成的代码片段转换为 TokenStream，以便返回
    TokenStream::from(expanded)
}
