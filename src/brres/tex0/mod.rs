use crate::binary::*;
use crate::brres::RawBrres;

pub struct TEX0 {
    pub header: Header,
    pub data: Vec<u8>,
}

impl TEX0 {
    pub fn new(brres: RawBrres, offset: usize, header_length: usize) -> Result<Self, String> {
        let header = Header::new(brres, offset + header_length)?;
        Ok(TEX0 {
            header,
            data: Vec::new(),
        })
    }
}

pub struct Header {
    pub flag: u32,
    pub width: u16,
    pub height: u16,
    pub format: u32,
    pub mipmap_num: u32,
    pub min_mipmap: f32,
    pub max_mipmap: f32,
}

impl Header {
    fn new(brres: RawBrres, offset: usize) -> Result<Self, String> {
        let data = brres
            .slice(offset, 0x1C)
            .ok_or_else(|| format!("TEX0::Header: Unable to get TEX0 header"))?;

        let flag = read(data, 0x0)?;
        let width = read(data, 0x4)?;
        let height = read(data, 0x6)?;
        let format = read(data, 0x8)?;
        let mipmap_num = read(data, 0xC)?;
        let min_mipmap = read(data, 0x10)?;
        let max_mipmap = read(data, 0x14)?;

        Ok(Header {
            flag,
            width,
            height,
            format,
            mipmap_num,
            min_mipmap,
            max_mipmap,
        })
    }
}
