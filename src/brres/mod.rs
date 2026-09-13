// BRRES Headers
mod index_group;
mod mdl0;
mod subfile;
mod tex0;

use crate::binary::*;
use index_group::{FromIndexGroup, IndexHeader};
use std::fs;
use subfile::SubFile;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RawBrres<'a> {
    data: &'a [u8],
}

impl<'a> RawBrres<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    fn slice(&self, offset: usize, size: usize) -> Option<&'a [u8]> {
        self.data.get(offset..offset.checked_add(size)?)
    }
}

pub struct Brres {
    pub file_header: Option<FileHeader>,
    pub root_header: Option<RootHeader>,
    pub root_index: Option<IndexHeader>,
    pub subfiles: Vec<SubFile>,
}

impl FromIndexGroup for Brres {
    fn set_data(&mut self, brres: RawBrres, offset: u32, index: u16) -> Result<(), String> {
        self.subfiles.push(SubFile::new(brres, (offset) as usize)?);

        Ok(())
    }

    fn is_subfile() -> bool {
        false
    }
}

pub struct FileHeader {
    magic: String,
    byte_order: Endian,
    file_size: u32,
    root_offset: u16,
    num_sections: u16,
}

impl FileHeader {
    fn new(data: &[u8; 0x10]) -> Result<Self, String> {
        let magic = read(data, 0x0)?;

        if magic != "bres" {
            return Err("Invalid file header".to_string());
        }

        let slice = data
            .get(0x4..0x6)
            .ok_or(format!("FileHeader: Unable to get slice: line {}", line!()))?;
        let byte_order = if slice == [0xFF, 0xFE] {
            Endian::Little
        } else {
            Endian::Big
        };

        let file_size = read(data, 0x8)?;
        let root_offset = read(data, 0xc)?;
        let num_sections = read(data, 0xe)?;

        Ok(FileHeader {
            magic,
            byte_order,
            file_size,
            root_offset,
            num_sections,
        })
    }
}

pub struct RootHeader {
    magic: String,
    len_sections: u32,
    offset: usize,
}

impl RootHeader {
    fn new(data: &[u8; 0x8], offset: usize) -> Result<Self, String> {
        let magic = read(data, 0x0)?;

        if magic != "root" {
            return Err("Invalid root header".to_string());
        }

        let len_sections = read(data, 0x4)?;

        Ok(RootHeader {
            magic,
            len_sections,
            offset,
        })
    }
}

pub fn parse(file: &str) -> std::io::Result<Brres> {
    let data = fs::read(file)?;

    let brres_file = RawBrres::new(&data);

    let mut brres = Brres {
        file_header: None,
        root_header: None,
        root_index: None,
        subfiles: Vec::new(),
    };

    let file_header_data: &[u8; 0x10] = brres_file
        .slice(0, 0x10)
        .and_then(|d| d.try_into().ok())
        .ok_or(std::io::Error::other(
        "Did not get 0x10 bytes for the file header",
    ))?;

    let file_header = FileHeader::new(file_header_data).map_err(std::io::Error::other)?;

    let root_header_data: &[u8; 0x8] = brres_file
        .slice(file_header.root_offset.into(), 0x8)
        .and_then(|d| d.try_into().ok())
        .ok_or(std::io::Error::other(
            "Did not get 0x8 bytes for the root header",
        ))?;

    let root_header = RootHeader::new(root_header_data, file_header.root_offset.into())
        .map_err(std::io::Error::other)?;

    let mut index_header =
        IndexHeader::new(brres_file, root_header.offset + 0x8).map_err(std::io::Error::other)?;

    index_header
        .root
        .get_data(brres_file, &mut brres)
        .map_err(|e| std::io::Error::other(e))?;

    brres.file_header = Some(file_header);
    brres.root_header = Some(root_header);
    brres.root_index = Some(index_header);

    for subfile in brres.subfiles.iter_mut() {
        subfile
            .generate_subfile(brres_file)
            .map_err(|e| std::io::Error::other(e))?;
    }

    Ok(brres)
}
