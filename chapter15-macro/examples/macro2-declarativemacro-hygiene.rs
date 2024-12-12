macro_rules! make_local {
    () => {
        let _local = 0;
    };
}
fn main() {
    let _local = 42;
    make_local!();
    assert_eq!(_local, 42);
}

/*
✗ cargo expand --package chapter15-macro --example macro2


#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
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
 */
