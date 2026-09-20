pub mod binary;
pub mod brres;
pub mod kmp;
pub mod szs;
pub mod kcl;

const FILE_PATH: &str = "beginner_course.szs";

#[cfg(test)]
mod tests {
    use super::*;
    use szs::*;
    use kmp::*;
    use kcl::*;

    #[test]
    fn szs() -> Result<(), String> {
        let file = szs::parse(FILE_PATH)?;
        
        Ok(())
    }

    #[test]
    fn kmp() -> Result<(), String> {
        let file = Kmp::from_szs(FILE_PATH)?;
        for entry in file.stgi.entries {
            println!("Lap Count: {}", entry.lap_count)
        }
        Ok(())
    }
    
    #[test]
    fn kcl() -> Result<(), String> {
        let kcl = Kcl::from_szs(FILE_PATH)?;
        std::fs::write("course.obj", kcl.to_obj()).unwrap();
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
