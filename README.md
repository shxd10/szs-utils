Lightweight dependency-less parsers for Mario Kart Wii.
| File     | Read | Write |
| -------- | ---- | ----- |
| .szs (yaz0 + arc)     | Yes  | No   |
| .kmp     | No  | No   |
| .kcl     | No  | No   |
| .brres   | No  | No   |

How to read the .szs tree:

```rust
use szs_utils::szs::*;

let parsed = szs::parse("beginner_course.szs")?;

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

you can also extract to a folder:

```rust
szs_utils::szs::extract("beginner_course.szs", "output.d")?;
```