pub mod normals;
pub mod vertices;
use crate::binary::*;
use crate::brres::RawBrres;

// TODO: Implement each section
pub struct MDL0 {
    pub header: Header,
}

impl MDL0 {
    pub fn new(brres: RawBrres, offset: usize, header_length: usize) -> Result<Self, String> {
        let header = Header::new(brres, offset + header_length)?;

        Ok(MDL0 { header })
    }
}

struct Header {
    length: u32,
    file_header_offset: i32,
    scale_mode: u32,
    tex_mode: u32,
    vertex_count: u32,
    face_count: u32,
    matrices_count: u32,
    normalize_matrices: bool,
    need_tex_matrices: bool,
    bounding_volume: bool,
    matrices_offset: usize,
    bounding_volume_min: Vec3,
    bounding_volume_max: Vec3,
}

impl Header {
    fn new(brres: RawBrres, offset: usize) -> Result<Self, String> {
        let length = read(brres.data, offset)?;

        let data = brres.slice(offset, length as usize).ok_or_else(|| {
            format!(
                "MDL0::Header: Unable to get {:#02x} bytes: {}",
                length,
                line!()
            )
        })?;

        let file_header_offset = read(data, 0x04)?;
        let scale_mode = read(data, 0x08)?;
        let tex_mode = read(data, 0x0C)?;
        let vertex_count = read(data, 0x10)?;
        let face_count = read(data, 0x14)?;
        let _unused: u32 = read(data, 0x18)?;
        let matrices_count = read(data, 0x1C)?;
        let normalize_matrices = read(data, 0x20)?;
        let need_tex_matrices = read(data, 0x21)?;
        let bounding_volume = read(data, 0x22)?;
        let _padding: u32 = read(data, 0x23)?;
        let matrices_offset = read::<u32>(data, 0x24)? as usize;
        let bounding_volume_min = read(data, 0x28)?;
        let bounding_volume_max = read(data, 0x34)?;

        Ok(Header {
            length,
            file_header_offset,
            scale_mode,
            tex_mode,
            vertex_count,
            face_count,
            matrices_count,
            normalize_matrices,
            need_tex_matrices,
            bounding_volume,
            matrices_offset,
            bounding_volume_min,
            bounding_volume_max,
        })
    }
}
