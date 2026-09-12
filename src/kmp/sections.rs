#![allow(dead_code)]

use std::marker::PhantomData;

use crate::binary::*;

// creating a trait for sections
pub trait KmpEntry: Sized {
    const SIZE: usize;
    fn parse(data: &[u8], offset: usize) -> Result<Self, String>;
}

/// The KTPT (kart point) section describes kart points; the starting position for racers.
pub struct Ktpt {
    pub position: Vec3,
    pub rotation: Vec3,
    pub player_index: i16,
    pub _padding: u16,
}

/// The ENPT (enemy point) section describes enemy points; the routes of CPU racers. The CPU racers attempt to follow the path described by each group of points (as determined by ENPH). More than 0xFF (255) entries will force a console freeze while loading the track.
pub struct Enpt {
    pub position: Vec3,
    pub leniency: f32,
    pub setting_1: u16,
    pub setting_2: u8,
    pub setting_3: u8,
}

/// The PathGroup section describes the structure of ENPH, ITPH, and CKPH groups:
/// * The ENPH (enemy path) section describes enemy point grouping; how the routes of CPU racers link together.
/// * The ITPH (item path) section describes item point grouping; how the item routes link together. When all previous or next group indices are set to 0xFF, the game assumes the order of points as they appear in the ITPT section.
/// * The CKPH (checkpoint path) section describes checkpoint grouping; how the routes of checkpoints link together.
pub struct PathGroup<T> {
    pub start: u8,
    pub group_length: u8,
    pub prev_group: [u8; 6],
    pub next_group: [u8; 6],
    pub group_link: u16,
    _p: PhantomData<T>,
}

/// The ITPT (item point) section describes item points; the Red Shell and Bullet Bill routes. The items attempt to follow the path described by each group of points (as determined by ITPH). More than 0xFF (255) entries will force a console freeze while loading the track.
pub struct Itpt {
    pub position: Vec3,
    pub bullet_control: f32,
    pub setting_1: u16,
    pub setting_2: u16,
}

/// The CKPT (checkpoint) section describes checkpoints; the routes players must follow to count laps. The racers must follow the path described by each group of points (as determined by CKPH). More than 0xFF (255) entries are possible if the last group begins at index ≤254. This is not recommended because Lakitu will always appear on-screen.
pub struct Ckpt {
    pub cp_left: Vec2,
    pub cp_right: Vec2,
    pub respawn_idx: u8,
    pub cp_type: CpType,
    pub prev_cp: u8,
    pub next_cp: u8,
}

#[derive(PartialEq)]
pub enum CpType {
    LapCounter,
    KeyCheckpoint,
    Checkpoint,
}

impl CpType {
    pub fn from_i8(value: i8) -> Self {
        match value {
            0 => Self::LapCounter,
            -1 => Self::Checkpoint,
            _ => Self::KeyCheckpoint,
        }
    }

    pub fn to_string(&self) -> &str {
        match self {
            Self::LapCounter => "Lap Counter",
            Self::KeyCheckpoint => "Key Checkpoint",
            Self::Checkpoint => "Checkpoint",
        }
    }
}

/// The GOBJ (geo object) section describes objects; things such as item boxes, pipes and also controlled objects such as sound triggers.
pub struct Gobj {
    pub id: u16,
    /// * this is part of the extended presence flags, but the value must be 0 if the object does not use this extension
    pub padding: u16,
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
    pub route: u16,
    pub settings: [u16; 8],
    pub presence_flags: u16,
}

/// Each POTI entry can contain a number of POTI entries/points.
pub struct PotiPoint {
    pub position: Vec3,
    pub setting_1: u16,
    pub setting_2: u16,
}

/// The POTI (point information) section describes routes; these are routes for many things including cameras and objects.
pub struct Poti {
    pub num_points: u16,
    pub setting_1: u8,
    pub setting_2: u8,
    pub points: Vec<PotiPoint>,
}

/// The AREA (area) section describes areas; used to determine which camera to use, for example. The size is 5000 for both the positive and negative sides of the X and Z-axes, and 10000 for only the positive side of the Y-axis.
pub struct Area {
    pub shape: Shape,
    pub kind: u8,
    pub came_index: u8,
    pub priority: u8,
    pub position: Vec3,
    pub rotation: Vec3,
    pub scale: Vec3,
    pub setting_1: u16,
    pub setting_2: u16,
    pub route: u8,
    pub enpt_id: u8,
    pub _padding: u16,
}

pub enum Shape {
    Box,
    Cylinder,
    Unknown(u8),
}

impl Shape {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Box,
            1 => Self::Cylinder,
            u => Self::Unknown(u),
        }
    }
}

/// The CAME (camera) section describes cameras; used to determine cameras for starting routes, Time Trial pans, etc.
pub struct Came {
    pub kind: u8,
    pub next_index: u8,
    pub shake: u8,
    pub route: u8,
    pub point_velocity: u16,
    pub zoom_velocity: u16,
    pub view_velocity: u16,
    pub start: u8,
    pub movie: u8,
    pub position: Vec3,
    pub rotation: Vec3,
    pub zoom_start: f32,
    pub zoom_end: f32,
    pub view_start: Vec3,
    pub view_end: Vec3,
    pub time: f32,
}

/// The JGPT (jugem point) section describes "Jugem" points; the respawn positions. The index is relevant for the link of the CKPT section.
pub struct Jgpt {
    pub position: Vec3,
    pub rotation: Vec3,
    pub respawn_id: u16,
    pub extra_data: i16,
}

/// The CNPT (cannon point) section describes cannon points; the cannon target positions.
pub struct Cnpt {
    pub destination: Vec3,
    pub angle: Vec3,
    pub id: u16,
    pub shoot_effect: i16,
}

/// The MSPT (mission success point) section describes end positions. After battles and tournaments have ended, the players are placed on this point(s).
pub struct Mspt {
    pub position: Vec3,
    pub rotation: Vec3,
    pub entry_id: u16,
    pub _unknown: u16,
}

/// The STGI (stage info) section describes stage information; information about a track.
pub struct Stgi {
    pub lap_count: u8,
    pub pole_pos: u8,
    pub driver_distance: u8,
    pub lens_flare_flashing: u8,
    pub flare_color: [u8; 4],
    pub flare_transparency: u8,
    // last byte is first byte of speed modifier f32
    pub padding_1: u16,
    // second byte of speed modifier f32
    pub padding_2: u8,
}

// reading the sections into the trait

impl<T> KmpEntry for PathGroup<T> {
    const SIZE: usize = 0x10;

    fn parse(data: &[u8], offset: usize) -> Result<Self, String> {
        Ok(PathGroup {
            start: read(data, offset)?,
            group_length: read(data, offset + 0x01)?,
            prev_group: read(data, offset + 0x02)?,
            next_group: read(data, offset + 0x08)?,
            group_link: read(data, offset + 0x0E)?,
            _p: PhantomData,
        })
    }
}

impl KmpEntry for Ktpt {
    const SIZE: usize = 0x1C;

    fn parse(data: &[u8], offset: usize) -> Result<Self, String> {
        Ok(Ktpt {
            position: read(data, offset)?,
            rotation: read(data, offset + 0x0C)?,
            player_index: read(data, offset + 0x18)?,
            _padding: read(data, offset + 0x1A)?,
        })
    }
}

impl KmpEntry for Enpt {
    const SIZE: usize = 0x14;

    fn parse(data: &[u8], offset: usize) -> Result<Self, String> {
        Ok(Enpt {
            position: read(data, offset)?,
            leniency: read(data, offset + 0x0C)?,
            setting_1: read(data, offset + 0x10)?,
            setting_2: read(data, offset + 0x12)?,
            setting_3: read(data, offset + 0x13)?,
        })
    }
}

impl KmpEntry for Itpt {
    const SIZE: usize = 0x14;

    fn parse(data: &[u8], offset: usize) -> Result<Self, String> {
        Ok(Itpt {
            position: read(data, offset)?,
            bullet_control: read(data, offset + 0x0C)?,
            setting_1: read(data, offset + 0x10)?,
            setting_2: read(data, offset + 0x12)?,
        })
    }
}

impl KmpEntry for Ckpt {
    const SIZE: usize = 0x14;

    fn parse(data: &[u8], offset: usize) -> Result<Self, String> {
        Ok(Ckpt {
            cp_left: read(data, offset)?,
            cp_right: read(data, offset + 0x08)?,
            respawn_idx: read(data, offset + 0x10)?,
            cp_type: CpType::from_i8(read::<i8>(data, offset + 0x11)?),
            prev_cp: read(data, offset + 0x12)?,
            next_cp: read(data, offset + 0x13)?,
        })
    }
}

impl KmpEntry for Gobj {
    const SIZE: usize = 0x3C;

    fn parse(data: &[u8], offset: usize) -> Result<Self, String> {
        Ok(Gobj {
            id: read(data, offset)?,
            padding: read(data, offset + 0x02)?,
            position: read(data, offset + 0x04)?,
            rotation: read(data, offset + 0x0C)?,
            scale: read(data, offset + 0x14)?,
            route: read(data, offset + 0x1C)?,
            settings: [
                read(data, offset + 0x20)?,
                read(data, offset + 0x22)?,
                read(data, offset + 0x24)?,
                read(data, offset + 0x26)?,
                read(data, offset + 0x28)?,
                read(data, offset + 0x2A)?,
                read(data, offset + 0x2C)?,
                read(data, offset + 0x2E)?,
            ],
            presence_flags: read(data, offset + 0x30)?,
        })
    }
}

impl Poti {
    pub fn parse(data: &[u8], offset: usize) -> Result<(Self, usize), String> {
        let num_points: u16 = read(data, offset)?;
        let setting_1: u8 = read(data, offset + 0x02)?;
        let setting_2: u8 = read(data, offset + 0x03)?;

        let mut points = Vec::with_capacity(num_points as usize);
        let mut point_offset = offset + 0x04;

        for _ in 0..num_points {
            let position: Vec3 = read(data, point_offset)?;
            let point_setting_1: u16 = read(data, point_offset + 0xC)?;
            let point_setting_2: u16 = read(data, point_offset + 0xE)?;

            points.push(PotiPoint {
                position,
                setting_1: point_setting_1,
                setting_2: point_setting_2,
            });

            point_offset += 0x10;
        }

        let total_size = 0x04 + (num_points as usize) * 0x10;

        Ok((
            Poti { num_points, setting_1, setting_2, points },
            total_size,
        ))
    }
}

impl KmpEntry for Area {
    const SIZE: usize = 0x30;

    fn parse(data: &[u8], offset: usize) -> Result<Self, String> {
        Ok(Area {
            shape: Shape::from_u8(read::<u8>(data, offset)?),
            kind: read(data, offset + 0x01)?,
            came_index: read(data, offset + 0x02)?,
            priority: read(data, offset + 0x03)?,
            position: read(data, offset + 0x04)?,
            rotation: read(data, offset + 0x0C)?,
            scale: read(data, offset + 0x14)?,
            setting_1: read(data, offset + 0x1C)?,
            setting_2: read(data, offset + 0x1E)?,
            route: read(data, offset + 0x20)?,
            enpt_id: read(data, offset + 0x21)?,
            _padding: read(data, offset + 0x22)?,
        })
    }
}

impl KmpEntry for Came {
    const SIZE: usize = 0x48;

    fn parse(data: &[u8], offset: usize) -> Result<Self, String> {
        Ok(Came {
            kind: read(data, offset)?,
            next_index: read(data, offset + 0x01)?,
            shake: read(data, offset + 0x02)?,
            route: read(data, offset + 0x03)?,
            point_velocity: read(data, offset + 0x04)?,
            zoom_velocity: read(data, offset + 0x06)?,
            view_velocity: read(data, offset + 0x08)?,
            start: read(data, offset + 0x0A)?,
            movie: read(data, offset + 0x0B)?,
            position: read(data, offset + 0x0C)?,
            rotation: read(data, offset + 0x18)?,
            zoom_start: read(data, offset + 0x24)?,
            zoom_end: read(data, offset + 0x28)?,
            view_start: read(data, offset + 0x2C)?,
            view_end: read(data, offset + 0x38)?,
            time: read(data, offset + 0x44)?,
        })
    }
}

impl KmpEntry for Jgpt {
    const SIZE: usize = 0x1C;

    fn parse(data: &[u8], offset: usize) -> Result<Self, String> {
        Ok(Jgpt {
            position: read(data, offset)?,
            rotation: read(data, offset + 0x0C)?,
            respawn_id: read(data, offset + 0x18)?,
            extra_data: read(data, offset + 0x1A)?,
        })
    }
}

impl KmpEntry for Cnpt {
    const SIZE: usize = 0x1C;

    fn parse(data: &[u8], offset: usize) -> Result<Self, String> {
        Ok(Cnpt {
            destination: read(data, offset)?,
            angle: read(data, offset + 0x0C)?,
            id: read(data, offset + 0x18)?,
            shoot_effect: read(data, offset + 0x1A)?,
        })
    }
}

impl KmpEntry for Mspt {
    const SIZE: usize = 0x1C;

    fn parse(data: &[u8], offset: usize) -> Result<Self, String> {
        Ok(Mspt {
            position: read(data, offset)?,
            rotation: read(data, offset + 0x0C)?,
            entry_id: read(data, offset + 0x18)?,
            _unknown: read(data, offset + 0x1A)?,
        })
    }
}

impl KmpEntry for Stgi {
    const SIZE: usize = 0x0C;

    fn parse(data: &[u8], offset: usize) -> Result<Self, String> {
        Ok(Stgi {
            lap_count: read(data, offset)?,
            pole_pos: read(data, offset + 0x01)?,
            driver_distance: read(data, offset + 0x02)?,
            lens_flare_flashing: read(data, offset + 0x03)?,
            flare_color: read(data, offset + 0x04)?,
            flare_transparency: read(data, offset + 0x08)?,
            padding_1: read(data, offset + 0x09)?,
            padding_2: read(data, offset + 0x0B)?,
        })
    }
}