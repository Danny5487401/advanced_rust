mod constant_string;
mod into_hash_map;
mod log_duration;

extern crate proc_macro2;

use constant_string::constant_string_impl;
use into_hash_map::into_hash_map_impl;
use log_duration::log_duration_impl;
use proc_macro::TokenStream;

// 告诉编译器这个函数是一个派生宏，而派生的标识符是 `IntoHashMap`。
#[proc_macro_derive(IntoHashMap)]
// 声明一个函数，该函数接收一个输入 `TokenStream` 并输出 `TokenStream`。
pub fn into_hash_map(item: TokenStream) -> TokenStream {
    into_hash_map_impl(item)
}

// 应用于任何函数（或方法），并在每次调用函数时记录函数的总运行时间
#[proc_macro_attribute]
pub fn log_duration(args: TokenStream, item: TokenStream) -> TokenStream {
    //第一个是传递给属性宏的参数，第二个是属性宏的目标。
    log_duration_impl(args, item)
}

// 它将一个字符串字面量（类型为 &str）作为输入，并为其创建一个全局公共常量（变量名称与值相同）。
#[proc_macro]
pub fn constant_string(item: TokenStream) -> TokenStream {
    constant_string_impl(item)
}
