use crate::binary::*;
use crate::brres::RawBrres;

pub struct IndexHeader {
    pub len_group: u32,
    pub num_group: u32,
    pub root: IndexEntry,
    pub offset: usize,
}

impl IndexHeader {
    #[track_caller]
    pub fn new(brres: RawBrres, offset: usize) -> Result<Self, String> {
        let data = brres.slice(offset, 0x8).ok_or(format!(
            "IndexHeader: Failed to get 0x8 bytes: line {}",
            line!()
        ))?;

        let len_group = read(data, 0x0)?;

        let num_group = read(data, 0x4)?;

        Ok(IndexHeader {
            len_group,
            num_group,
            root: IndexEntry::new(brres, offset + 0x8, true, 0)?,
            offset,
        })
    }
}

pub trait FromIndexGroup {
    fn set_data(&mut self, brres: RawBrres, offset: u32, index: u16) -> Result<(), String>;

    fn is_subfile() -> bool {
        true
    }
}

pub struct IndexEntry {
    id: u16,
    flag: u16, // Always 0?
    left_idx: u16,
    left: Option<Box<IndexEntry>>,
    right_idx: u16,
    right: Option<Box<IndexEntry>>,
    name: String,
    len_name: u32, // 4 bytes before Offset of Name
    data_ptr: u32,
    root: bool, // Root node of the index groups
    current_idx: u16,
    group_offset: u32,
    sub_index: Option<Box<IndexHeader>>,
}

impl IndexEntry {
    #[track_caller]
    pub fn new(
        brres: RawBrres,
        root_offset: usize,
        root: bool,
        current_idx: u16,
    ) -> Result<Self, String> {
        let group_offset = root_offset - 0x8;
        let offset = if root {
            root_offset
        } else {
            root_offset + (0x10 * current_idx as usize)
        };
        let data = brres.slice(offset, 0x10).ok_or(format!(
            "IndexEntry: Failed to get 0x10 bytes: line {}",
            line!()
        ))?;

        let id = read(data, 0x0)?;
        let flag = read(data, 0x2)?;
        let left_idx = read(data, 0x4)?;
        let right_idx = read(data, 0x6)?;

        let is_child = |child_idx: u16| -> bool {
            if child_idx == 0 {
                return false;
            }
            let child_off = root_offset + (0x10 * child_idx as usize);
            if let Some(slice) = brres.slice(child_off, 2)
                && let Ok(bytes) = slice.try_into()
            {
                u16::from_be_bytes(bytes) < id
            } else {
                false
            }
        };

        let left = if is_child(left_idx) {
            Some(Box::new(IndexEntry::new(
                brres,
                root_offset,
                false,
                left_idx,
            )?))
        } else {
            None
        };

        let right = if is_child(right_idx) {
            Some(Box::new(IndexEntry::new(
                brres,
                root_offset,
                false,
                right_idx,
            )?))
        } else {
            None
        };

        let rel_name_off: u32 = read(data, 0x8)?;

        let (name, len_name) = if root {
            ("ROOT".to_string(), 4)
        } else {
            let name_off = group_offset + rel_name_off as usize;
            let name: VariableString = read(brres.data, name_off)?;

            (name.string, name.length as u32)
        };

        let data_ptr = read(data, 0xC)?;

        let sub_index = None;

        Ok(IndexEntry {
            id,
            flag,
            left_idx,
            left,
            right_idx,
            right,
            name,
            len_name,
            data_ptr,
            root,
            current_idx,
            group_offset: group_offset as u32,
            sub_index,
        })
    }

    pub fn get_data<T: FromIndexGroup>(
        &mut self,
        brres: RawBrres,
        file: &mut T,
    ) -> Result<(), String> {
        if self.root == false && self.sub_index.is_none() {
            if T::is_subfile() {
                file.set_data(brres, self.data_ptr + self.group_offset, self.current_idx)?;
            } else {
                let magic_raw = brres
                    .slice((self.data_ptr + self.group_offset) as usize, 0x4)
                    .ok_or("Failed to get 0x4 bytes for IndexEntry")?;

                let magic_alpha = magic_raw.iter().all(|b| b.is_ascii_alphanumeric());

                if magic_alpha {
                    file.set_data(brres, self.data_ptr + self.group_offset, self.current_idx)?;
                } else {
                    self.sub_index = Some(Box::new(IndexHeader::new(
                        brres,
                        (self.data_ptr + self.group_offset) as usize,
                    )?));
                }
            }
        }

        if let Some(sub_index) = &mut self.sub_index {
            sub_index.root.get_data(brres, file)?;
        }

        if let Some(left) = &mut self.left {
            left.get_data(brres, file)?;
        }

        if let Some(right) = &mut self.right {
            right.get_data(brres, file)?;
        }

        Ok(())
    }

    pub fn print_entry_names(&self) {
        println!("Node Name: {}", self.name,);

        if let Some(sub) = &self.sub_index {
            println!("--> Entering Sub-Folder: {}", self.name);
            sub.root.print_entry_names();
        }

        if let Some(left) = &self.left {
            left.print_entry_names();
        }
        if let Some(right) = &self.right {
            right.print_entry_names();
        }
    }
}
