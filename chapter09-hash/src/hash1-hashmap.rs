use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    explain("empty", &map);

    // 插入元素
    map.insert("a", 1);
    explain("added 1", &map);

    map.insert("b", 2);
    map.insert("c", 3);
    explain("added 3", &map);

    map.insert("d", 4);
    explain("added 4", &map);

    // assert_eq!(map.get("a"), Some(1).as_ref());
    assert_eq!(map.get(&"a"), Some(&1));
    assert_eq!(map.get_key_value(&"b"), Some((&"b", &2)));

    // 移除元素
    map.remove(&"a"); // 删除后就找不到了
    assert_eq!(map.contains_key(&"a"), false);
    assert_eq!(map.get(&"a"), None);
    explain("removed", &map);

    // shrink 缩小哈希表占用的内存空间
    map.shrink_to_fit();
    explain("shrink", &map);
}

fn explain<K, V>(name: &str, map: &HashMap<K, V>) {
    println!("{}:len:{},cap:{}", name, map.len(), map.capacity())
}

/*
empty:len:0,cap:0
added 1:len:1,cap:3
added 3:len:3,cap:3
added 4:len:4,cap:7
removed:len:3,cap:7
shrink:len:3,cap:3

 */
