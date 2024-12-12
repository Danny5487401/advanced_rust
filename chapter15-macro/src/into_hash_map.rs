use proc_macro::TokenStream;
use quote::quote;
use syn::Data;

pub(crate) fn into_hash_map_impl(item: TokenStream) -> TokenStream {
    // 将输入的 token stream 解析为 `syn` 库提供的 `DeriveInput` 类型。
    let input = syn::parse_macro_input!(item as syn::DeriveInput);

    // 将结构体标识符（名称）存储到一个变量中，以便你可以将其插入到输出代码中。
    let struct_identifier = &input.ident;

    // 对应用了派生宏的目标类型进行匹配
    match &input.data {
        // 匹配目标是一个结构体，并从它的信息中解构 `fields` 字段。
        Data::Struct(syn::DataStruct { fields, .. }) => {
            // 声明一个新的 quote 块，它将保存你的哈希映射实现的代码。
            // 这个块将既创建一个新的哈希映射，也将用结构体中的所有字段填充它。
            let mut implementation = quote!{
                // 这是你希望在输出中看到的代码。在这种情况下，你希望创建一个新的哈希映射。
                let mut hash_map = std::collections::HashMap::<String, String>::new();
            };

            // 遍历目标结构体的所有字段
            for field in fields {
                // 创建一个变量来存储字段的标识符（名称），以备后用
                let identifier = field.ident.as_ref().unwrap();
                // 扩展你的 `implementation` 块，以便在输出中包含用当前字段的信息填充创建的哈希映射。
                implementation.extend(quote!{
                    // 使用 `stringify!` 宏将字段标识符转换为字符串。这将作为你新哈希映射条目的键。
                    // 对于这个键的值，我们使用 `value.#identifier` 访问结构体中的字段值，
                    // 其中 `#identifier` 在输出代码中替换为实际的字段名。
                    hash_map.insert(stringify!(#identifier).to_string(), String::from(value.#identifier));
                });
            }

            // 创建最终输出块
            quote! {
                // 实现 `From` 特性，以允许将你的目标结构体标识为 `struct_identifier` 转换为
                // 键和值均为 `String` 的 HashMap。
                // 就像先前一样，`#struct_identifier` 在输出代码中被替换为目标结构体的实际名称。
                impl From<#struct_identifier> for std::collections::HashMap<String, String> {
                    // `From` 特性要求你实现的一个方法。
                    // 输入值的类型再次为 `#struct_identifier`，在输出代码中被替换为目标结构体的名称。
                    fn from(value: #struct_identifier) -> Self {
                        // 使用 `quote!` 将你创建的 `implementation` 块包含在这个方法体中。
                        // `quote` 允许你自由嵌套其他的 `quote` 块。
                        #implementation

                        // 返回 hash_map。
                        hash_map
                    }
                }
            }
        }
        // 如果目标类型是任何其他类型，则触发 panic 错误。
        _ => unimplemented!()
        // 将 `quote` 使用的 `TokenStream` 类型转换为标准库和编译器使用的 `TokenStream` 类型。
    }.into()
}

// 方式二: 将整个代码块包裹在 #()* 中，代码将放在括号内。这种语法允许你在括号内使用任何迭代器
// #[proc_macro_derive(IntoHashMap2)]
// pub(crate) fn into_hash_map2(item: TokenStream) -> TokenStream {
//     let input = syn::parse_macro_input!(item as syn::DeriveInput);
//     let struct_identifier = &input.ident;
//
//     match &input.data {
//         Data::Struct(syn::DataStruct { fields, .. }) => {
//             // 使用 field_identifiers 迭代器
//             let field_identifiers = fields.iter().map(|item| item.ident.as_ref().unwrap()).collect::<Vec<_>>();
//
//             quote! {
//                 impl From<#struct_identifier> for std::collections::HashMap<String, String> {
//                     fn from(value: #struct_identifier) -> Self {
//                         let mut hash_map = std::collections::HashMap::<String, String>::new();
//
//                         #(
//                             hash_map.insert(stringify!(#field_identifiers).to_string(), String::from(value.#field_identifiers));
//                         )*
//
//                         hash_map
//                     }
//                 }
//             }
//         }
//         _ => unimplemented!()
//     }.into()
// }
