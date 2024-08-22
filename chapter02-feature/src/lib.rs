pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn print_feature(){
    #[cfg(feature="print-a")]
    println!("a");

    #[cfg(feature="print-b")]
    println!("b");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
