fn id<T>(x: T) -> T {
    x
}

fn main() {
    let my_int = id(10);
    let my_string = id("Danny");
    println!("{}:{}", my_int, my_string)
}
