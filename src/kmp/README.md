Example: Reading the position of every Key Checkpoint

```rust
let file = szs_utils::kmp::Kmp::parse("course.kmp")?;
for entry in file.ckpt.entries {
    if entry.cp_type == szs_utils::kmp::sections::CpType::KeyCheckpoint {
        println!("Left: <{}, {}>, Right: <{}, {}>", entry.cp_left.x, entry.cp_left.y, entry.cp_right.x, entry.cp_right.y);
    }
}
```