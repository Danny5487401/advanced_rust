pub fn print_feature() {
    #[cfg(feature = "print-a")]
    println!("a");

    #[cfg(feature = "print-b")]
    println!("b");
}
