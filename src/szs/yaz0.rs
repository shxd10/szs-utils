use crate::binary::*;

pub struct Yaz0 {
    pub header: Header,
    pub decompressed_data: Vec<u8>,
}

pub struct Header {
    pub magic: [u8; 4],
    pub decompressed_size: u32,
    pub _reserved: [u32; 2],
}

impl Yaz0 {
    pub fn decompress(data: &[u8]) -> Result<Self, String> {
        let header = Header::new(data, 0x0)?;

        // i tried to keep the code as similar as the C example as possible
        
        let header_size = 0x10;

        let mut offset: usize = header_size;

        let dest_end = header.decompressed_size as usize;
        let mut dest = Vec::with_capacity(dest_end);

        let mut ops_left: u8 = 0;
        let mut group_header: u8 = 0;

        while dest.len() < dest_end {
            if ops_left == 0 {
                // start a new data group and read the group header byte.
                group_header = read(data, offset)?;
                offset += 1;
                ops_left = 8;
            }
            
            if (group_header & 0x80) != 0 {
                // bit in group header byte is set -> copy 1 byte direct
                dest.push(read(data, offset)?);
                offset += 1;
            } else {
                // bit in group header byte is not set -> run length encoding
                // read the first 2 bytes of the chunk
                let b1: u8 = read(data, offset)?;
                offset += 1;
                let b2: u8 = read(data, offset)?;
                offset += 1;
                
               	// calculate the source position
                let distance = (((b1 & 0xF) as usize) << 8) | (b2 as usize);
                
                if distance + 1 > dest.len() {
                    return Err("Invalid distance".to_string());
                }
                let mut copy_src = dest.len() - (distance + 1);
                
                // calculate the number of bytes to copy.
                let mut n: usize = (b1 >> 4) as usize;

                if n == 0 {
                    let third_byte: u8 = read::<u8>(data, offset)?;
                    offset += 1;
                    n = (third_byte as usize) + 0x12;
                } else {
                    n += 2; // add 2 to length
                }
        
                for _ in 0..n {
                    dest.push(dest[copy_src]);
                    copy_src += 1;
                }
            }
    
            // shift group header byte
            group_header <<= 1;
            ops_left -= 1;
        }
        
        Ok(Yaz0 { header, decompressed_data: dest })
    }
}

impl Header {
    pub fn new(data: &[u8], offset: usize) -> Result<Self, String> {
        let magic: [u8; 4] = read(data, offset)?;
        if magic != "Yaz0".as_bytes() {
            return Err("Invalid magic".to_string());
        }
        
        let decompressed_size: u32 = read(data, offset + 0x4)?;
        let _reserved: [u32; 2] = [
            read(data, offset + 0x8)?, 
            read(data, offset + 0xC)?
        ];

        Ok(Header { magic, decompressed_size, _reserved })
    }
}
