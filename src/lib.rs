mod binary;
mod brres;
mod kmp;
mod szs;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::szs::arc::{DirectoryEntry, Entry};

    #[test]
    fn test() -> Result<(), String> {
        let file = kmp::Kmp::parse("course.kmp")?;
        for entry in file.ckpt.entries {
            if entry.cp_type == kmp::sections::CpType::KeyCheckpoint {
                println!(
                    "Left: <{}, {}>, Right: <{}, {}>",
                    entry.cp_left.x, entry.cp_left.y, entry.cp_right.x, entry.cp_right.y
                );
            }
        }

        Ok(())
    }

    #[test]
    fn brres_test() -> Result<(), String> {
        let brres = brres::parse("course_model.brres").map_err(|e| format!("{}", e))?;

        for subfile in brres.subfiles {
            println!("Brres subfile type: {}", subfile.header.section_type);
        }

        Ok(())
    }
}
