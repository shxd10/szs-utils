mod szs;
mod binary;
mod kmp;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::szs::arc::{DirectoryEntry, Entry};

    #[test]
    fn test() -> Result<(), String> {
        let file = kmp::Kmp::parse("course.kmp")?;
        for entry in file.ckpt.entries {
            if entry.cp_type == kmp::sections::CpType::KeyCheckpoint {
                println!("Left: <{}, {}>, Right: <{}, {}>", entry.cp_left.x, entry.cp_left.y, entry.cp_right.x, entry.cp_right.y);
            }
        }

        Ok(())
    }
}