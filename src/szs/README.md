Yaz0 decompressor and ARC (U8) parser.

How to read the file tree:

```rust
let parsed = szs_utils::szs::parse("beginner_course.szs")?;

walk(&parsed.arc.tree, 0);

fn walk(dir: &DirectoryEntry, depth: usize) {
    for child in &dir.children {
        match child {
            Entry::File(file) => {
                // your code for files
                println!("{}{} ({} bytes)", "  ".repeat(depth), file.name, file.size);
            }
            Entry::Directory(dir) => {
                // your code for dirs
                println!("{}{}/", "  ".repeat(depth), dir.name);
                walk(dir, depth + 1);
            }
        }
    }
}
```

to find a specific file:

```rust
let szs = szs_utils::szs::parse("beginner_course.szs")?;
if let Some(file) = szs.root.find("course.kmp") {
    let kmp = szs_utils::kmp::Kmp::parse_from_data(&file.data)?;
    println!("{}", kmp.header.magic);
}
```

you can also extract to a folder:

```rust
szs_utils::szs::extract("beginner_course.szs", "output.d")?;
```