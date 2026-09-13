Example: Reading the subfiles of a brres

```rust
let brres = brres::parse("course_model.brres").map_err(|e| format!("{}", e))?;
for subfile in brres.subfiles {
    println!("Brres subfile type: {}", subfile.header.section_type);
}
```
