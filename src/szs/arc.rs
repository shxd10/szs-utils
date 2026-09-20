#![allow(dead_code)]

use crate::binary::*;

pub struct Arc {
    pub header: Header,
    pub tree: DirectoryEntry,
}

pub struct Header {
    pub magic: u32,
    pub first_node_offset: i32,
    pub nodes_size: i32,
    pub data_offset: i32,
    pub _reserved: [i32; 4],
}

#[derive(Copy, Clone)]
pub struct Node {
    pub node_type: NodeType,
    pub filename_offset: u24,
    // if it's a file, this is the offset of begin of data
    // if it's a directory, this is the index of the parent directory
    pub data_offset_or_parent_idx: u32,
    // if it's a file, this is the size of the data
    // if it's a directory, this is the index of the first node that is not part of this directory (skip node)
    pub size_or_skip_idx: u32,
}

#[derive(Copy, Clone)]
pub enum NodeType {
    File = 0x00,
    Directory = 0x01,
    Unknown,
}

pub enum Entry {
    File(FileEntry),
    Directory(DirectoryEntry),
}

impl Entry {
    pub fn name(&self) -> &str {
        match self {
            Entry::File(f) => &f.name,
            Entry::Directory(d) => &d.name,
        }
    }
}

pub struct FileEntry {
    pub name: String,
    pub data: Vec<u8>,
    pub size: u32,
}

pub struct DirectoryEntry {
    pub name: String,
    pub children: Vec<Entry>,
}

pub struct ParsedNode {
    pub node_type: NodeType,
    pub name: String,
    pub data_offset_or_parent_idx: u32,
    pub size_or_skip_idx: u32,
}

impl NodeType {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0x00 => NodeType::File,
            0x01 => NodeType::Directory,
            _ => NodeType::Unknown,
        }
    }
}

impl Arc {
    pub fn new(data: &[u8]) -> Result<Self, String> {
        let header = Header::new(data, 0x0)?;

        let node_size = 0xC;

        let root = Node::new(data, header.first_node_offset as usize)?;
        let node_count = root.size_or_skip_idx as usize;
        
        let mut nodes = Vec::with_capacity(node_count);
        let mut offset = header.first_node_offset as usize;
        
        for _ in 0..node_count {
            nodes.push(Node::new(data, offset)?);
            offset += node_size;
        }

        let string_pool = &data[offset..];

        let parsed_nodes: Vec<ParsedNode> = nodes.into_iter().map(|node| -> Result<ParsedNode, String> {
            let name = read::<AsciiString>(string_pool, node.filename_offset.to_u32() as usize)?;
            Ok(ParsedNode {
                node_type: node.node_type,
                name: name.string,
                data_offset_or_parent_idx: node.data_offset_or_parent_idx,
                size_or_skip_idx: node.size_or_skip_idx,
            })
        }).collect::<Result<Vec<ParsedNode>, String>>()?;

        let tree = Self::build_tree(parsed_nodes, &data)?;
        Ok(Arc { header, tree })
    }
    
    pub fn extract_dir(dir: &DirectoryEntry, out: &std::path::Path) -> Result<(), String> {
        if !out.is_dir() {
            std::fs::create_dir_all(out)
                .map_err(|e| e.to_string())?;
        }
        
        for child in &dir.children {
            match child {
                Entry::File(file) => {
                    let file_path = out.join(&file.name);
    
                    std::fs::write(file_path, &file.data)
                        .map_err(|e| e.to_string())?;
                }
    
                Entry::Directory(dir) => {
                    let dir_path = out.join(&dir.name);
    
                    std::fs::create_dir_all(&dir_path)
                        .map_err(|e| e.to_string())?;
    
                    Self::extract_dir(dir, &dir_path)?;
                }
            }
        }
    
        Ok(())
    }

    pub fn build_tree(nodes: Vec<ParsedNode>, data: &[u8]) -> Result<DirectoryEntry, String> {
        let (root, _) = Self::build_dir(&nodes, data, 0)?;
        Ok(root)
    }

    pub fn build_dir(nodes: &[ParsedNode], data: &[u8], index: usize) -> Result<(DirectoryEntry, usize), String> {
        let dir = nodes.get(index)
            .ok_or_else(|| format!("node index {} out of bounds (len {})", index, nodes.len()))?;
    
        let end = dir.size_or_skip_idx as usize;
        if end > nodes.len() {
            return Err(format!("skip index {} exceeds node count {}", end, nodes.len()));
        }
        if end <= index {
            return Err(format!("skip index {} does not advance past {}", end, index));
        }
    
        let mut children = Vec::new();
        let mut i = index + 1;
    
        while i < end {
            match nodes[i].node_type {
                NodeType::File => {
                    let data_offset = nodes[i].data_offset_or_parent_idx as usize;
                    let size = nodes[i].size_or_skip_idx as usize;
    
                    let data_slice = data.get(data_offset..data_offset + size)
                        .ok_or_else(|| format!(
                            "file data range {}..{} out of bounds (data len {})",
                            data_offset, data_offset + size, data.len()
                        ))?
                        .to_vec();
    
                    children.push(Entry::File(FileEntry {
                        name: nodes[i].name.clone(),
                        data: data_slice,
                        size: size as u32,
                    }));
                    i += 1;
                }
    
                NodeType::Directory => {
                    let (child_dir, next) = Self::build_dir(nodes, data, i)?;
                    children.push(Entry::Directory(child_dir));
                    i = next;
                }
    
                NodeType::Unknown => {
                    i += 1;
                }
            }
        }
    
        Ok((
            DirectoryEntry {
                name: dir.name.clone(),
                children,
            },
            end,
        ))
    }
}

impl Header {
    pub fn new(data: &[u8], offset: usize) -> Result<Self, String> {
        
        let magic: u32 = read(data, offset)?;
        let first_node_offset: i32 = read(data, offset + 0x4)?;
        let nodes_size: i32 = read(data, offset + 0x8)?;
        let data_offset: i32 = read(data, offset + 0xC)?;
        let _reserved: [i32; 4] = [
            read(data, offset + 0x10)?,
            read(data, offset + 0x14)?,
            read(data, offset + 0x18)?,
            read(data, offset + 0x1C)?,
        ];

        Ok(Header { magic, first_node_offset, nodes_size, data_offset, _reserved })
    }
}

impl Node {
    pub fn new(data: &[u8], offset: usize) -> Result<Self, String> {
        let raw_type: u8 = read(data, offset)?;
        let node_type: NodeType = NodeType::from_u8(raw_type);
        
        let raw_u32_with_type: u32 = read(data, offset)?;
        let filename_offset = u24::from_u32(raw_u32_with_type);
        let data_offset_or_parent_idx: u32 = read(data, offset + 0x4)?;
        let size_or_skip_idx: u32 = read(data, offset + 0x8)?;

        Ok(Node { node_type, filename_offset, data_offset_or_parent_idx, size_or_skip_idx })
    }
}

impl DirectoryEntry {
    pub fn find(&self, name: &str) -> Option<&FileEntry> {
        for child in &self.children {
            match child {
                Entry::File(file) if file.name == name => return Some(file),
                Entry::Directory(subdir) => {
                    if let Some(found) = subdir.find(name) {
                        return Some(found);
                    }
                }
                _ => {}
            }
        }
        None
    }
}