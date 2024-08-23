fn main() {
    let s1 = String::from("Danny");
    let s2 = String::from("Joy");
    let result = max(&s1, &s2);
    println!("bigger {}", result);

    let result = get_max(&s1);
    println!("bigger one: {}", result);
}

fn get_max(s1: &str) -> &str {
    max(s1, "Chandler")
}

fn max<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1 > s2 {
        s1
    } else {
        s2
    }
}
