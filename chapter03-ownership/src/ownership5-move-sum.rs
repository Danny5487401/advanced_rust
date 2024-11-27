fn main() {
    let data = vec![1, 2, 3, 4];
    println!("addr of value: {}, addr of ref: {:p}", data[0], &data[0]); //addr of value: 1, addr of ref: 0x600003be8030
    println!("sum of data1: {}", sum(data));
}

fn sum(data: Vec<u32>) -> u32 {
    // 值的地址会改变么？引用的地址会改变么？
    println!("addr of value: {}, addr of ref: {:p}", data[0], &data[0]); //addr of value: 1, addr of ref: 0x600003be8030
    data.iter().sum()
}
