use darling::FromMeta;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, LitStr};

pub fn constant_string_impl(item: TokenStream) -> TokenStream {
    // 将输入解析为字符串字面量
    let constant_value = parse_macro_input!(item as LitStr);

    // 从传递的字符串值创建一个新的 `Ident`（标识符）。
    // 这将成为常量变量的名称。
    let constant_value_name = Ident::from_string(&constant_value.value()).unwrap();

    // 生成声明常量变量的代码。
    quote!(pub const #constant_value_name: &str = #constant_value;).into()
}
