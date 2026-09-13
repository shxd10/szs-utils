use crate::binary::*;
use crate::brres::{
    RawBrres,
    index_group::IndexHeader,
    mdl0::{normals::Normals, vertices::Vertices, *},
    tex0::TEX0,
};
use std::fmt;
use std::panic::Location;

pub(crate) struct Header {
    pub header_length: usize,
    pub section_type: SectionType,
    length: u32,
    version: u32,
    out_brres_off: u32,
    num_section_offsets: u32,
    section_offsets: Vec<u32>, // These offsets point to an Index Group
    name: VariableString,
    file_off: usize,
}

impl Header {
    pub(crate) fn new(brres: RawBrres, offset: usize) -> Result<Self, String> {
        let data = brres
            .slice(offset, 0x10)
            .ok_or("Subfile::Header: Failed to get 0x10 bytes")?;

        let section_type = SectionType::str_to_type(read(data, 0x0)?)?;
        let length = read(data, 0x4)?;
        let version = read(data, 0x8)?;
        let out_brres_off = read(data, 0xC)?;
        let num_section_offsets = section_type.get_section_count(version);
        let mut section_offsets: Vec<u32> = Vec::new();
        let section_data_size: usize = (num_section_offsets as usize * 4) + 4;

        let data = brres
            .slice(offset + 0x10, section_data_size)
            .ok_or_else(|| {
                format!(
                    "Subfile::Header: Failed to get {:#02x} bytes",
                    section_data_size
                )
            })?;

        for n in 0..num_section_offsets {
            let off = read(data, (n * 4) as usize)?;
            section_offsets.push(off);
        }

        let rel_name_off: u32 = read(data, (num_section_offsets * 4) as usize)?;
        let name_off: usize = offset + rel_name_off as usize;
        let name = read(brres.data, name_off)?;

        Ok(Header {
            header_length: (num_section_offsets as usize * 4) + 0x14,
            section_type,
            length,
            version,
            out_brres_off,
            num_section_offsets,
            section_offsets,
            name,
            file_off: offset,
        })
    }
}

pub enum SubFileData {
    Mdl0(MDL0),
    Tex0(TEX0),
    Unsupported,
}

pub struct SubFile {
    pub header: Header,
    pub file: Option<SubFileData>,
}

impl SubFile {
    pub fn new(brres: RawBrres, offset: usize) -> Result<Self, String> {
        let header = Header::new(brres, offset)?;

        Ok(SubFile { header, file: None })
    }

    fn create_mdl0(&mut self, brres: RawBrres) -> Result<(), String> {
        let mdl0 = MDL0::new(brres, self.header.file_off, self.header.header_length)?;

        self.file = Some(SubFileData::Mdl0(mdl0));

        let mut verts_index_group = IndexHeader::new(
            brres,
            self.header.file_off + self.header.section_offsets[2] as usize,
        )?;
        let mut norms_index_group = IndexHeader::new(
            brres,
            self.header.file_off + self.header.section_offsets[3] as usize,
        )?;

        let mut vertices = Vertices::new(verts_index_group.num_group);
        let mut normals = Normals::new(norms_index_group.num_group);

        verts_index_group.root.get_data(brres, &mut vertices)?;
        norms_index_group.root.get_data(brres, &mut normals)?;

        Ok(())
    }

    fn create_tex0(&mut self, brres: RawBrres) -> Result<(), String> {
        let tex0 = TEX0::new(brres, self.header.file_off, self.header.header_length)?;

        self.file = Some(SubFileData::Tex0(tex0));

        Ok(())
    }

    pub fn generate_subfile(&mut self, brres: RawBrres) -> Result<(), String> {
        match self.header.section_type {
            SectionType::MDL0 => self.create_mdl0(brres),
            SectionType::TEX0 => self.create_tex0(brres),
            _ => {
                println!(
                    "Unsupported subfile encountered: {}",
                    self.header.section_type
                );
                self.file = Some(SubFileData::Unsupported);
                Ok(())
            }
        }
    }
}

#[derive(Clone, Copy)]
pub enum SectionType {
    MDL0,
    TEX0,
    SRT0,
    CHR0,
    PAT0,
    CLR0,
    SHP0,
    SCN0,
    PLT0,
    VIS0,
}

impl SectionType {
    #[track_caller]
    pub fn str_to_type(magic: String) -> Result<SectionType, String> {
        match magic.as_str() {
            "MDL0" => Ok(SectionType::MDL0),
            "TEX0" => Ok(SectionType::TEX0),
            "SRT0" => Ok(SectionType::SRT0),
            "CHR0" => Ok(SectionType::CHR0),
            "PAT0" => Ok(SectionType::PAT0),
            "CLR0" => Ok(SectionType::CLR0),
            "SHP0" => Ok(SectionType::SHP0),
            "SCN0" => Ok(SectionType::SCN0),
            "PLT0" => Ok(SectionType::PLT0),
            "VIS0" => Ok(SectionType::VIS0),
            &_ => Err(format!(
                "Invalid Section Magic recieved: {} called from {}",
                magic,
                Location::caller()
            )),
        }
    }

    pub fn get_section_count(&self, version: u32) -> u32 {
        match self {
            SectionType::MDL0 => match version {
                8 => 11,
                11 => 14,
                _ => 0,
            },
            SectionType::TEX0 => match version {
                1 => 1,
                2 => 2,
                3 => 1,
                _ => 0,
            },
            SectionType::SRT0 => match version {
                4 => 1,
                5 => 2,
                _ => 0,
            },
            SectionType::CHR0 => match version {
                3 => 1,
                5 => 2,
                _ => 0,
            },
            SectionType::PAT0 => match version {
                4 => 6,
                _ => 0,
            },
            SectionType::CLR0 => match version {
                4 => 2,
                _ => 0,
            },
            SectionType::SHP0 => match version {
                4 => 3,
                _ => 0,
            },
            SectionType::SCN0 => match version {
                4 => 6,
                5 => 7,
                _ => 0,
            },
            SectionType::PLT0 => match version {
                _ => 0,
            },
            SectionType::VIS0 => match version {
                _ => 0,
            },
        }
    }
}

impl fmt::Display for SectionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SectionType::MDL0 => write!(f, "MDL0"),
            SectionType::TEX0 => write!(f, "TEX0"),
            SectionType::SRT0 => write!(f, "SRT0"),
            SectionType::CHR0 => write!(f, "CHR0"),
            SectionType::PAT0 => write!(f, "PAT0"),
            SectionType::CLR0 => write!(f, "CLR0"),
            SectionType::SHP0 => write!(f, "SHP0"),
            SectionType::SCN0 => write!(f, "SCN0"),
            SectionType::PLT0 => write!(f, "PLT0"),
            SectionType::VIS0 => write!(f, "VIS0"),
        }
    }
}
