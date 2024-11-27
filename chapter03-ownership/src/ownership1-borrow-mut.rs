use std::cell::RefCell;

fn main() {
    let data = RefCell::new(1); // 对并未声明成 mut 的值或者引用，也想进行修改, 使用 RefCell 非线程安全实现
    {
        let mut v = data.borrow_mut();
        *v += 1;
    }
    println!("data {:?}", data);
}
