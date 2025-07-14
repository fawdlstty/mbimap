# mbimap

Similar to [bimap](https://github.com/billyrieger/bimap-rs), but it provides a pooled dataset, facilitating the storage of multiple mappings.

## Example

```rust
let mut map = MbiMap::new();
map.insert(1, "a");
map.insert_by_left(2, vec!["a", "b", "c"]);
map.insert_by_right(vec![3, 4], "b");
println!("{:?}", map.get_by_left(&2)); // [a, b, c]
println!("{:?}", map.get_by_right(&"b")); // [2, 3, 4]
map.remove_all_by_left(&1);
map.remove_all_by_right(&"c");
println!("{map:?}"); // 2->[a, b],  3->[b],  4->[b]
```
