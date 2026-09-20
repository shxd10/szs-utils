#[derive(Clone, Copy, Debug)]
pub struct Flag {
    pub base_type: BaseType,
    pub variant: u8,
    pub blight: u8,
    pub wheel_depth: WheelDepth,
    pub collision_effect: CollisionEffect,
}

#[derive(Clone, Copy, Debug)]
pub enum BaseType {
    Road,
    SlipperyRoad1,
    WeakOffroad,
    Offroad,
    HeavyOffroad,
    SlipperyRoad2,
    Boost,
    BoostRamp,
    JumpPad,
    ItemRoad,
    SolidFall,
    MovingWater,
    Wall,
    InvisibleWall,
    ItemWall,
    Wall2,
    FallBoundary,
    CannonTrigger,
    ForceRecalculation,
    HalfPipeRamp,
    PlayerOnlyWall,
    MovingRoad,
    StickyRoad,
    Road2,
    SoundTrigger,
    WeakWall,
    EffectTrigger,
    ItemStateModifier,
    HalfPipeInvisibleWall,
    RotatingRoad,
    SpecialWall,
    InvisibleWall2,
    Unknown(u8),
}

#[derive(Clone, Copy, Debug)]
pub enum WheelDepth {
    None = 0,
    Shallow = 1,
    Medium = 2,
    Deep = 3,
}

#[derive(Clone, Copy, Debug)]
pub struct CollisionEffect {
    pub trickable: bool,
    pub reject_road: bool,
    pub soft_wall: bool,
}

impl Flag {
    pub fn from_u16(value: u16) -> Self {
        let base_type = BaseType::from_u8((value & 0x1F) as u8);
        let variant = ((value >> 5) & 0x7) as u8;
        let blight = ((value >> 8) & 0x7) as u8;
        let wheel_depth = WheelDepth::from_u8(((value >> 11) & 0x3) as u8);
        let collision_effect = CollisionEffect::from_u8(((value >> 13) & 0x7) as u8);

        Flag { base_type, variant, blight, wheel_depth, collision_effect }
    }

    pub fn to_u16(&self) -> u16 {
        let mut value: u16 = 0;
        value |= self.base_type.to_u8() as u16;
        value |= (self.variant as u16) << 5;
        value |= (self.blight as u16) << 8;
        value |= (self.wheel_depth.to_u8() as u16) << 11;
        value |= (self.collision_effect.to_u8() as u16) << 13;
        value
    }
}
impl BaseType {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0x00 => BaseType::Road,
            0x01 => BaseType::SlipperyRoad1,
            0x02 => BaseType::WeakOffroad,
            0x03 => BaseType::Offroad,
            0x04 => BaseType::HeavyOffroad,
            0x05 => BaseType::SlipperyRoad2,
            0x06 => BaseType::Boost,
            0x07 => BaseType::BoostRamp,
            0x08 => BaseType::JumpPad,
            0x09 => BaseType::ItemRoad,
            0x0A => BaseType::SolidFall,
            0x0B => BaseType::MovingWater,
            0x0C => BaseType::Wall,
            0x0D => BaseType::InvisibleWall,
            0x0E => BaseType::ItemWall,
            0x0F => BaseType::Wall2,
            0x10 => BaseType::FallBoundary,
            0x11 => BaseType::CannonTrigger,
            0x12 => BaseType::ForceRecalculation,
            0x13 => BaseType::HalfPipeRamp,
            0x14 => BaseType::PlayerOnlyWall,
            0x15 => BaseType::MovingRoad,
            0x16 => BaseType::StickyRoad,
            0x17 => BaseType::Road2,
            0x18 => BaseType::SoundTrigger,
            0x19 => BaseType::WeakWall,
            0x1A => BaseType::EffectTrigger,
            0x1B => BaseType::ItemStateModifier,
            0x1C => BaseType::HalfPipeInvisibleWall,
            0x1D => BaseType::RotatingRoad,
            0x1E => BaseType::SpecialWall,
            0x1F => BaseType::InvisibleWall2,
            other => BaseType::Unknown(other),
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            BaseType::Road => 0x00,
            BaseType::SlipperyRoad1 => 0x01,
            BaseType::WeakOffroad => 0x02,
            BaseType::Offroad => 0x03,
            BaseType::HeavyOffroad => 0x04,
            BaseType::SlipperyRoad2 => 0x05,
            BaseType::Boost => 0x06,
            BaseType::BoostRamp => 0x07,
            BaseType::JumpPad => 0x08,
            BaseType::ItemRoad => 0x09,
            BaseType::SolidFall => 0x0A,
            BaseType::MovingWater => 0x0B,
            BaseType::Wall => 0x0C,
            BaseType::InvisibleWall => 0x0D,
            BaseType::ItemWall => 0x0E,
            BaseType::Wall2 => 0x0F,
            BaseType::FallBoundary => 0x10,
            BaseType::CannonTrigger => 0x11,
            BaseType::ForceRecalculation => 0x12,
            BaseType::HalfPipeRamp => 0x13,
            BaseType::PlayerOnlyWall => 0x14,
            BaseType::MovingRoad => 0x15,
            BaseType::StickyRoad => 0x16,
            BaseType::Road2 => 0x17,
            BaseType::SoundTrigger => 0x18,
            BaseType::WeakWall => 0x19,
            BaseType::EffectTrigger => 0x1A,
            BaseType::ItemStateModifier => 0x1B,
            BaseType::HalfPipeInvisibleWall => 0x1C,
            BaseType::RotatingRoad => 0x1D,
            BaseType::SpecialWall => 0x1E,
            BaseType::InvisibleWall2 => 0x1F,
            BaseType::Unknown(value) => *value,
        }
    }
}

impl WheelDepth {
    pub fn from_u8(value: u8) -> Self {
        match value {
            0x0 => WheelDepth::None,
            0x1 => WheelDepth::Shallow,
            0x2 => WheelDepth::Medium,
            0x3 => WheelDepth::Deep,
            _ => unreachable!("WheelDepth value must be 0-3, got {}", value),
        }
    }

    pub fn to_u8(self) -> u8 {
        match self {
            WheelDepth::None => 0x0,
            WheelDepth::Shallow => 0x1,
            WheelDepth::Medium => 0x2,
            WheelDepth::Deep => 0x3,
        }
    }
}

impl CollisionEffect {
    pub fn from_u8(value: u8) -> Self {
        CollisionEffect {
            trickable: value & 1 != 0,
            reject_road: (value >> 1) & 1 != 0,
            soft_wall: (value >> 2) & 1 != 0,
        }
    }

    pub fn to_u8(self) -> u8 {
        (self.trickable as u8) | (self.reject_road as u8) << 1 | (self.soft_wall as u8) << 2
    }
}
