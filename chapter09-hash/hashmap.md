<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
**Table of Contents**  *generated with [DocToc](https://github.com/thlorenz/doctoc)*

- [hash](#hash)
  - [hashMap](#hashmap)
    - [结构](#%E7%BB%93%E6%9E%84)
    - [扩容和缩容](#%E6%89%A9%E5%AE%B9%E5%92%8C%E7%BC%A9%E5%AE%B9)
  - [HashSet](#hashset)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

# hash

## hashMap

### 结构

```rust
pub struct HashMap<K, V, S = DefaultHashBuilder, A: Allocator = Global> {
    pub(crate) hash_builder: S,
    pub(crate) table: RawTable<(K, V), A>,
}
```

```rust
pub struct RawTable<T, A: Allocator = Global> {
    table: RawTableInner,
    alloc: A,
    // Tell dropck that we own instances of T.
    marker: PhantomData<T>,
}

/// Non-generic part of `RawTable` which allows functions to be instantiated only once regardless
/// of how many different key-value types are used.
struct RawTableInner {
    // 哈希表中哈希桶的数量减一
    bucket_mask: usize,

    // [Padding], T1, T2, ..., Tlast, C1, C2, ...
    //                                ^ points here
    // 指向堆内存哈希表末端的 ctrl 区
    ctrl: NonNull<u8>,

    // Number of elements that can be inserted before we need to grow the table
    growth_left: usize,

    // Number of elements in the table, only really used by len()
    items: usize,
}
```

### 扩容和缩容

当 HashMap::new() 时，它并没有分配空间，容量为零，随着哈希表不断插入数据，它会以 2 的幂减一的方式增长，最小是3。

以 2 的 n 次幂的方式进行扩容，0，3 (22 - 1)，7 (23 - 1)，14 (24 - 24 * 12.5%)，28 (25 - 25 * 12.5%)

```rust
pub struct HashMap<K, V, S = RandomState> {
    base: base::HashMap<K, V, S>,
}

impl<K, V, S> HashMap<K, V, S>
where
    K: Eq + Hash,
    S: BuildHasher,
{
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.base.insert(k, v)
    }
}
```

```rust
impl<K, V, S, A> HashMap<K, V, S, A>
where
    K: Eq + Hash,
    S: BuildHasher,
    A: Allocator,
{
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        // 哈希，得到一个哈希值 hash
        let hash = make_hash::<K, S>(&self.hash_builder, &k);
        let hasher = make_hasher::<_, V, S>(&self.hash_builder);
        match self
            .table
            .find_or_find_insert_slot(hash, equivalent_key(&k), hasher)
        {
            Ok(bucket) => Some(mem::replace(unsafe { &mut bucket.as_mut().1 }, v)),
            Err(slot) => {
                unsafe {
                    self.table.insert_in_slot(hash, slot, (k, v));
                }
                None
            }
        }
    }
}
```

当删除表中的数据时，原有的表大小不变，只有显式地调用 shrink_to_fit，才会让哈希表变小。

## HashSet

```rust
pub struct HashSet<T, S = DefaultHashBuilder, A: Allocator = Global> {
    pub(crate) map: HashMap<T, (), S, A>,
}

```