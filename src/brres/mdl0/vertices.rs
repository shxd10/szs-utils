use crate::binary::*;
use crate::brres::{RawBrres, index_group::FromIndexGroup};

pub struct Vertices {
    pub vertices: Box<[Option<Header>]>,
    pub num_vertex_groups: u32,
}

impl Vertices {
    pub fn new(num_vertex_groups: u32) -> Self {
        Vertices {
            vertices: std::iter::repeat_with(|| None)
                .take(num_vertex_groups as usize)
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            num_vertex_groups,
        }
    }
}

pub struct Header {
    pub length: u32,
    mdl0_offset: i32,
    data_offset: i32,
    name_offset: i32,
    pub name: VariableString,
    index: u32,
    is_3d: bool,
    format: u32,
    divisor: u8,
    stride: u8,
    vertex_count: u16,
    bounding_volume_min: Vec3,
    bounding_volume_max: Vec3,
}

impl Header {
    pub fn new(brres: RawBrres, offset: usize) -> Result<Self, String> {
        let ctx = format!("MDL0::Vertices::Header[{:#02x}]:", offset);
        let data = brres
            .slice(offset, 0x38)
            .ok_or_else(|| format!("{ctx}: Unable to get {:#02x} bytes: {}", 0x38, line!()))?;

        let length = read(data, 0x0)?;
        let mdl0_offset = read(data, 0x4)?;
        let data_offset = read(data, 0x8)?;
        let name_offset = read(data, 0xC)?;
        let name = read(brres.data, offset + name_offset as usize)?;
        let index = read(data, 0x10)?;
        let is_3d = read::<u32>(data, 0x14)? == 0x1;
        let format = read(data, 0x18)?;
        let divisor = read(data, 0x1C)?;
        let stride = read(data, 0x1D)?;
        let vertex_count = read(data, 0x1E)?;
        let bounding_volume_min = read(data, 0x20)?;
        let bounding_volume_max = read(data, 0x2C)?;

        Ok(Header {
            length,
            mdl0_offset,
            data_offset,
            name_offset,
            name,
            index,
            is_3d,
            format,
            divisor,
            stride,
            vertex_count,
            bounding_volume_min,
            bounding_volume_max,
        })
    }
}

impl FromIndexGroup for Vertices {
    fn set_data(&mut self, brres: RawBrres, offset: u32, index: u16) -> Result<(), String> {
        self.vertices[index as usize - 1] = Some(Header::new(brres, offset as usize)?);
        Ok(())
    }
}
