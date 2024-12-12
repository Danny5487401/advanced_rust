macro_rules! my_vec {
    // 匹配 my_vec![]
    () => {
        std::vec::Vec::new()
    };
    // 匹配 my_vec![1,2,3]
    ($($el:expr), +) => {
        // 这段代码需要用{}包裹起来，因为宏需要展开，这样能保证作用域正常，不影响外部。这也是rust的宏是 Hygienic Macros 的体现。
        // 而 C/C++ 的宏不强制要求，但是如果遇到代码片段，在 C/C++ 中也应该使用{}包裹起来。
        {
            // 这里是多次分配的方式
            let mut v = std::vec::Vec::new();
            $(v.push($el);)* // 由于匹配的时候匹配到一个 $(...)* （我们可以不管分隔符），在执行的代码块中，我们也要相应地使用 $(...)* 展开。所以这句 $(v.push($el);)* 相当于匹配出多少个 $el就展开多少句 push 语句。
            v
        }
    };
    // 匹配 my_vec![1; 3]
    ($el:expr; $n:expr) => {
        std::vec::from_elem($el, $n)
    };
}

fn main() {
    println!("{:?}", my_vec![1, 2, 3]);
}

/*
✗ cargo expand --package chapter15-macro --example macro1


#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use chapter15_macro::my_vec;
fn main() {
    {
        ::std::io::_print(
            format_args!(
                "{0:?}\n",
                {
                    let mut v = std::vec::Vec::new();
                    v.push(1);
                    v.push(2);
                    v.push(3);

 */
