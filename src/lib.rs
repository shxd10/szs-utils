mod szs;
mod binary;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::szs::arc::{DirectoryEntry, Entry};

    #[test]
    fn it_works() -> Result<(), String> {
        let parsed = szs::parse("beginner_course.szs")?;

        print_tree(&parsed.root, 0);

        szs::extract("beginner_course.szs", "test")?;

        Ok(())
    }

    fn print_tree(dir: &DirectoryEntry, depth: usize) {
        for child in &dir.children {
            match child {
                Entry::File(file) => {
                    println!("{}{} ({} bytes)", "  ".repeat(depth), file.name, file.size);
                }
                Entry::Directory(dir) => {
                    println!("{}{}/", "  ".repeat(depth), dir.name);
                    print_tree(dir, depth + 1);
                }
            }
        }
    }
}