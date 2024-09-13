use core::time;
use std::thread;

use chapter15_macro::log_bench;

#[log_bench]
fn test_my_macro() {
    for num in 0..3 {
        thread::sleep(time::Duration::from_secs(1));
        println!("{}", num);
    }
}

fn main() {
    test_my_macro();
}
