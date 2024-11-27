fn main() {
    let data = vec![1, 2, 3, 4];
    let data1 = &data;
    // 值的地址是什么？引用的地址又是什么？
    println!(
        "addr of value: {:p}({:p}), addr of data {:p}, data1: {:p}",
        &data, data1, &&data, &data1
    ); // addr of value: 0x16da164f8(0x16da164f8), addr of data 0x16da16598, data1: 0x16da16510
    println!("sum of data1: {}", sum(data1));

    // 堆上数据的地址是什么？
    println!(
        "addr of items: [{:p}, {:p}, {:p}, {:p}]",
        &data[0], &data[1], &data[2], &data[3]
    ); // addr of items: [0x600002bf8040, 0x600002bf8044, 0x600002bf8048, 0x600002bf804c]
}

fn sum(data: &[u32]) -> u32 {
    // 值的地址会改变么？引用的地址会改变么？
    println!("addr of value: {:p}, addr of ref: {:p}", data, &data); //addr of value: 0x600002bf8040, addr of ref: 0x16da16358
    data.iter().sum()
}
