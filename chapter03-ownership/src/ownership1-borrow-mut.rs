use std::cell::RefCell;

fn main() {
    let data = RefCell::new(1);
    {
        let mut v = data.borrow_mut();
        *v+=1;

    }
    println!("data {:?}",data);
}
