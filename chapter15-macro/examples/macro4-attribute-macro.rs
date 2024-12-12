use chapter15_macro::log_duration;

#[log_duration]
#[must_use]
fn function_to_benchmark() -> u16 {
    let mut counter = 0;
    for _ in 0..u16::MAX {
        counter += 1;
    }

    counter
}

fn main() {
    println!("{}", function_to_benchmark());
}
