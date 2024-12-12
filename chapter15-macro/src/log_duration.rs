use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

pub(crate) fn log_duration_impl(_args: TokenStream, input: TokenStream) -> TokenStream {
    // 将输入解析为 `ItemFn`，这是 `syn` 提供的一种表示函数的类型。
    let input = parse_macro_input!(input as ItemFn);

    let ItemFn {
        // 函数签名
        sig,
        // 该函数的可见性说明符
        vis,
        // 函数体
        block,
        // 应用于此函数的其他属性
        attrs,
    } = input;

    // 提取函数体中的语句
    let statements = block.stmts;

    // 存储用于日志记录的函数标识符
    let function_identifier = sig.ident.clone();

    // 使用解析的输入重新构建函数作为输出
    quote!(
        // 重新应用此函数上的所有其他属性。
        // 编译器不会在此列表中包含我们当前正在处理的宏。
        #(#attrs)*
        // 重新构建函数声明
        #vis #sig {
            // 在函数开始时，创建一个 `Instant` 实例
            let __start = std::time::Instant::now();

            // 创建一个新的块，其主体是函数的主体。
            // 将此块的返回值存储为一个变量，以便我们之后可以从父函数中返回它。
            let __result = {
                #(#statements)*
            };

            // 记录此函数的持续时间信息
            println!("{} 耗时 {}μs", stringify!(#function_identifier), __start.elapsed().as_micros());

            // 返回结果（如果有的话）
            return __result;
        }
    )
        .into()
}
