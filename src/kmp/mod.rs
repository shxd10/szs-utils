pub mod sections;
use sections::*;

use crate::binary::*;

// Inspired from KMPeek parsing.

pub struct Kmp {
    pub header: Header,
    pub ktpt: Section<Ktpt>,
    pub enpt: Section<Enpt>,
    pub enph: Section<PathGroup<Enpt>>,
    pub itpt: Section<Itpt>,
    pub itph: Section<PathGroup<Itpt>>,
    pub ckpt: Section<Ckpt>,
    pub ckph: Section<PathGroup<Ckpt>>,
    pub gobj: Section<Gobj>,
    pub poti: Section<Poti>,
    pub area: Section<Area>,
    pub came: Section<Came>,
    pub jgpt: Section<Jgpt>,
    pub cnpt: Section<Cnpt>,
    pub mspt: Section<Mspt>,
    pub stgi: Section<Stgi>,
}

pub struct Header {
    pub magic: String,
    pub file_length: u32,
    pub section_count: u16,
    pub header_length: u16,
    pub version: u32,
}

pub struct Section<T> {
    pub name: String,
    pub entry_count: u16,
    // The POTI section stores the total number of points of all routes here. The CAME section stores different values. For all other sections, the value is 0 (padding).
    pub additional: u16,

    pub entries: Vec<T>,
}

impl Header {
    pub fn parse(data: &[u8]) -> Result<Self, String> {
        Ok(Header {
            magic: read(data, 0)?,
            file_length: read(data, 0x04)?,
            section_count: read(data, 0x08)?,
            header_length: read(data, 0x0A)?,
            version: read(data, 0x0C)?,
        })
    }
}


impl<T: KmpEntry> Section<T> {
    pub fn parse(data: &[u8], offset: usize) -> Result<Self, String> {
        let name: String = read(data, offset)?;
        let entry_count: u16 = read(data, offset + 0x04)?;
        let additional: u16 = read(data, offset + 0x06)?;

        let mut entries = Vec::with_capacity(entry_count as usize);
        let mut entry_offset = offset + 0x08;

        for _ in 0..entry_count {
            entries.push(T::parse(data, entry_offset)?);
            entry_offset += T::SIZE;
        }

        Ok(Section { name, entry_count, additional, entries })
    }
}

//special case for Poti section because of variable length entries
impl Section<Poti> {
    pub fn parse(data: &[u8], offset: usize) -> Result<Self, String> {
        let name: String = read(data, offset)?;
        let entry_count: u16 = read(data, offset + 0x04)?;
        let additional: u16 = read(data, offset + 0x06)?;

        let mut entries = Vec::with_capacity(entry_count as usize);
        let mut entry_offset = offset + 0x08;

        for _ in 0..entry_count {
            let (poti, size) = Poti::parse(data, entry_offset)?;
            entries.push(poti);
            entry_offset += size;
        }

        Ok(Section { name, entry_count, additional, entries })
    }
}

impl Kmp {
    pub fn parse(path: &str) -> Result<Self, String> {
        let data = std::fs::read(path).map_err(|e| e.to_string())?;
        Self::parse_from_data(&data)
    }

    pub fn parse_from_data(data: &[u8]) -> Result<Self, String> {
        let header = Header::parse(data)?;
        let header_length = header.header_length as usize;
        let offsets: [u32; 15] = read(data, 0x10)?;

        // i do this because offsets aren't always in the same order

        let mut ktpt = None;
        let mut enpt = None;
        let mut enph = None;
        let mut itpt = None;
        let mut itph = None;
        let mut ckpt = None;
        let mut ckph = None;
        let mut gobj = None;
        let mut poti = None;
        let mut area = None;
        let mut came = None;
        let mut jgpt = None;
        let mut cnpt = None;
        let mut mspt = None;
        let mut stgi = None;

        for raw_offset in offsets {
            let section_offset = raw_offset as usize + header_length;
            let start = &data[section_offset..];
            let name: String = read(start, 0)?;

            // so i just check the name to determine which section it is

            match name.as_str() {
                "KTPT" => ktpt = Some(Section::<Ktpt>::parse(data, section_offset)?),
                "ENPT" => enpt = Some(Section::<Enpt>::parse(data, section_offset)?),
                "ENPH" => enph = Some(Section::<PathGroup<Enpt>>::parse(data, section_offset)?),
                "ITPT" => itpt = Some(Section::<Itpt>::parse(data, section_offset)?),
                "ITPH" => itph = Some(Section::<PathGroup<Itpt>>::parse(data, section_offset)?),
                "CKPT" => ckpt = Some(Section::<Ckpt>::parse(data, section_offset)?),
                "CKPH" => ckph = Some(Section::<PathGroup<Ckpt>>::parse(data, section_offset)?),
                "GOBJ" => gobj = Some(Section::<Gobj>::parse(data, section_offset)?),
                "POTI" => poti = Some(Section::<Poti>::parse(data, section_offset)?),
                "AREA" => area = Some(Section::<Area>::parse(data, section_offset)?),
                "CAME" => came = Some(Section::<Came>::parse(data, section_offset)?),
                "JGPT" => jgpt = Some(Section::<Jgpt>::parse(data, section_offset)?),
                "CNPT" => cnpt = Some(Section::<Cnpt>::parse(data, section_offset)?),
                "MSPT" => mspt = Some(Section::<Mspt>::parse(data, section_offset)?),
                "STGI" => stgi = Some(Section::<Stgi>::parse(data, section_offset)?),
                other => return Err(format!("unknown KMP section name: {:?}", other)),
            }
        }

        Ok(Self {
            header,
            ktpt: ktpt.ok_or("missing KTPT section")?,
            enpt: enpt.ok_or("missing ENPT section")?,
            enph: enph.ok_or("missing ENPH section")?,
            itpt: itpt.ok_or("missing ITPT section")?,
            itph: itph.ok_or("missing ITPH section")?,
            ckpt: ckpt.ok_or("missing CKPT section")?,
            ckph: ckph.ok_or("missing CKPH section")?,
            gobj: gobj.ok_or("missing GOBJ section")?,
            poti: poti.ok_or("missing POTI section")?,
            area: area.ok_or("missing AREA section")?,
            came: came.ok_or("missing CAME section")?,
            jgpt: jgpt.ok_or("missing JGPT section")?,
            cnpt: cnpt.ok_or("missing CNPT section")?,
            mspt: mspt.ok_or("missing MSPT section")?,
            stgi: stgi.ok_or("missing STGI section")?,
        })
    }
}