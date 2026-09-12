// https://github.com/Hibyehello/brres_parser/blob/master/src/brres/common.rs

use std::fmt;
use std::mem;
use std::panic::Location;

// custom u24 for szs/arc.rs

#[derive(Copy, Clone, Debug)]
pub struct u24([u8; 3]);

impl u24 {
    pub fn from_u32(value: u32) -> Self {
        let bytes = value.to_be_bytes();
        Self([bytes[1], bytes[2], bytes[3]])
    }

    pub fn to_u32(&self) -> u32 {
        u32::from_be_bytes([0, self.0[0], self.0[1], self.0[2]])
    }
}


pub trait FromBytes: Sized {
    #[track_caller]
    fn from_bytes(data: &[u8], offset: usize) -> Result<Self, String>;
}

pub struct VariableString {
    pub string: String,
    pub length: usize,
}

pub struct AsciiString {
    pub string: String,
    pub length: usize,
}


impl FromBytes for VariableString {
    #[track_caller]
    fn from_bytes(data: &[u8], offset: usize) -> Result<Self, String> {
        let mut slice = data.get(offset - 0x4..offset).ok_or(format!(
            "Failed to get 0x4 bytes. caused by {}",
            Location::caller()
        ))?;
        let len = u32::from_be_bytes(slice.try_into().map_err(|_| {
            format!(
                "Unable to convert Slice: caused by {}",
                Location::caller()
            )
        })?) as usize;
        slice = data.get(offset..offset + len).ok_or(format!(
            "Failed to get {:#02x} bytes. caused by {}",
            len,
            Location::caller()
        ))?;
        Ok(VariableString {
            string: String::from_utf8(slice.to_vec()).map_err(|_| {
                format!(
                    "Unable to convert slice to String: caused by {}",
                    Location::caller()
                )
            })?,
            length: len,
        })
    }
}

impl FromBytes for AsciiString {
    #[track_caller]
    fn from_bytes(data: &[u8], offset: usize) -> Result<Self, String> {
        if offset >= data.len() {
            return Err(format!(
                "offset {:#02x} out of bounds (data len {:#02x}): caused by {}",
                offset,
                data.len(),
                Location::caller()
            ));
        }

        let end = data[offset..]
            .iter()
            .position(|&b| b == 0)
            .map(|rel| offset + rel)
            .ok_or_else(|| {
                format!(
                    "no null terminator found starting at offset {:#02x}: caused by {}",
                    offset,
                    Location::caller()
                )
            })?;

        let slice = &data[offset..end];

        let string = String::from_utf8(slice.to_vec()).map_err(|_| {
            format!(
                "Unable to convert slice to String: caused by {}",
                Location::caller()
            )
        })?;

        Ok(AsciiString {
            length: slice.len(),
            string,
        })
    }
}

impl FromBytes for String {
    #[track_caller]
    fn from_bytes(data: &[u8], offset: usize) -> Result<Self, String> {
        let slice = data.get(offset..offset + 0x4).ok_or(format!(
            "Unable to get slice: caused by {}",
            Location::caller()
        ))?;
        let magic = String::from_utf8(slice.to_vec()).map_err(|_| {
            format!(
                "Unable to convert slice to String: caused by {}",
                Location::caller()
            )
        })?;

        Ok(magic)
    }
}

impl FromBytes for bool {
    #[track_caller]
    fn from_bytes(data: &[u8], offset: usize) -> Result<Self, String> {
        let slice = data.get(offset..offset + 1).ok_or_else(|| {
            format!(
                "Unable to get {:#02x} bytes from offset {:#02x}: caused by {}",
                1,
                offset,
                Location::caller()
            )
        })?;

        Ok(slice[0] != 0)
    }
}

impl<const N: usize> FromBytes for [u8; N] {
    #[track_caller]
    fn from_bytes(data: &[u8], offset: usize) -> Result<Self, String> {
        data.get(offset..offset + N)
            .and_then(|s| s.try_into().ok())
            .ok_or_else(|| format!(
                "Unable to get {} bytes at offset {:#02x}: caused by {}",
                N, offset, Location::caller()
            ))
    }
}

impl<const N: usize> FromBytes for [u32; N] {
    #[track_caller]
    fn from_bytes(data: &[u8], offset: usize) -> Result<Self, String> {
        let mut result = [0u32; N];
        for i in 0..N {
            result[i] = read(data, offset + i * 4)?;
        }
        Ok(result)
    }
}

macro_rules! impl_one_byte_FromBytes {
    ($($t:ty), *) => {
        $(
            impl FromBytes for $t {
                #[track_caller]
                fn from_bytes(data: &[u8], offset: usize) -> Result<Self, String> {
                    let slice = data.get(offset..offset + 1).ok_or_else(|| {
                        format!(
                            "Unable to get {:#02x} bytes from offset {:#02x}: caused by {}",
                            1,
                            offset,
                            Location::caller()
                        )
                    })?;

                    Ok(slice[0] as $t)
                }
            }
        )*
    };
}

macro_rules! impl_primitive_FromBytes {
    ($($t:ty), *) => {
        $(
            impl FromBytes for $t {
                #[track_caller]
                fn from_bytes(data: &[u8], offset: usize) -> Result<Self, String> {
                    let size = std::mem::size_of::<$t>();

                    let slice = data.get(offset..offset + size).ok_or_else(|| {
                        format!(
                            "Unable to get {:#02x} bytes from offset {:#02x}: caused by {}",
                            size, offset, Location::caller()
                        )
                    })?;

                    Ok(<$t>::from_be_bytes(slice.try_into().map_err(|_| {
                        format!(
                            "Internal error converting bytes to array at offset {:#02x}: caused by {}",
                            offset, Location::caller()
                        )
                    })?))

                }
            }
        )*
    };
}

impl_one_byte_FromBytes!(u8, i8);
impl_primitive_FromBytes!(u16, i16, u32, i32, u64, i64, f32, f64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vec3 { x, y, z }
    }

    pub fn raw_new(data: &[u8; 12]) -> Result<Self, String> {
        let x_slice = data.get(0x0..0x4).ok_or("Failed to get X slice")?;
        let y_slice = data.get(0x4..0x8).ok_or("Failed to get Y slice")?;
        let z_slice = data.get(0x8..0xC).ok_or("Failed to get Z slice")?;

        let x = f32::from_be_bytes(
            x_slice
                .try_into()
                .map_err(|_| "Vec3: Failed to convert X bytes")?,
        );
        let y = f32::from_be_bytes(
            y_slice
                .try_into()
                .map_err(|_| "Vec3: Failed to convert Y bytes")?,
        );
        let z = f32::from_be_bytes(
            z_slice
                .try_into()
                .map_err(|_| "Vec3: Failed to convert Z bytes")?,
        );

        Ok(Vec3 { x, y, z })
    }
}

impl FromBytes for Vec3 {
    fn from_bytes(data: &[u8], offset: usize) -> Result<Self, String> {
        Ok(Vec3 {
            x: read(data, offset)?,
            y: read(data, offset + 4)?,
            z: read(data, offset + 8)?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self {
        Vec2 { x, y }
    }

    pub fn raw_new(data: &[u8; 8]) -> Result<Self, String> {
        let x_slice = data.get(0x0..0x4).ok_or("Failed to get X slice")?;
        let y_slice = data.get(0x4..0x8).ok_or("Failed to get Y slice")?;

        let x = f32::from_be_bytes(
            x_slice
                .try_into()
                .map_err(|_| "Vec3: Failed to convert X bytes")?,
        );
        let y = f32::from_be_bytes(
            y_slice
                .try_into()
                .map_err(|_| "Vec3: Failed to convert Y bytes")?,
        );

        Ok(Vec2 { x, y })
    }
}

impl FromBytes for Vec2 {
    #[track_caller]
    fn from_bytes(data: &[u8], offset: usize) -> Result<Self, String> {
        Ok(Vec2 {
            x: read(data, offset)?,
            y: read(data, offset + 4)?,
        })
    }
}

//TODO: Actually use this to determine from_xx_bytes calls
pub enum Endian {
    Big,
    Little,
}

impl fmt::Display for Endian {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Endian::Big => write!(f, "Big"),
            Endian::Little => write!(f, "Little"),
        }
    }
}

#[track_caller]
pub fn read<T: FromBytes>(data: &[u8], offset: usize) -> Result<T, String> {
    T::from_bytes(data, offset)
}