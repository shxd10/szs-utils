use crate::binary::*;
use crate::brres::{RawBrres, index_group::FromIndexGroup};

pub struct Normals {
    pub normals: Box<[Option<Header>]>,
    pub num_vertex_groups: u32,
}

impl Normals {
    pub fn new(num_vertex_groups: u32) -> Self {
        Normals {
            normals: std::iter::repeat_with(|| None)
                .take(num_vertex_groups as usize)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            num_vertex_groups,
        }
    }
}

pub struct Header {
    length: u32,
    mdl0_offset: i32,
    data_offset: i32,
    name_offset: i32,
    pub name: VariableString,
    index: u32,
    component_count: u32,
    format: u32,
    divisor: u8,
    stride: u8,
    normal_count: u16,
}

impl Header {
    pub fn new(brres: RawBrres, offset: usize) -> Result<Self, String> {
        let ctx = format!("MDL0::Normals::Header[{offset:#02x}]");
        let data = brres
            .slice(offset, 0x20)
            .ok_or_else(|| format!("{ctx}: Unable to get {:#02x} bytes: {}", 0x38, line!()))?;

        let length = read(data, 0x0)?;
        let mdl0_offset = read(data, 0x4)?;
        let data_offset = read(data, 0x8)?;
        let name_offset = read(data, 0xC)?;
        let name = read(brres.data, offset + name_offset as usize)?;
        let index = read(data, 0x10)?;
        let component_count = read(data, 0x14)?;
        let format = read(data, 0x18)?;
        let divisor = read(data, 0x1C)?;
        let stride = read(data, 0x1D)?;
        let normal_count = read(data, 0x1E)?;

        Ok(Header {
            length,
            mdl0_offset,
            data_offset,
            name_offset,
            name,
            index,
            component_count,
            format,
            divisor,
            stride,
            normal_count,
        })
    }
}

impl FromIndexGroup for Normals {
    fn set_data(&mut self, brres: RawBrres, offset: u32, index: u16) -> Result<(), String> {
        self.normals[index as usize - 1] = Some(Header::new(brres, offset as usize)?);
        Ok(())
    }
}
