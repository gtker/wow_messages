use std::convert::{TryFrom, TryInto};
use crate::{ConstantSized, ReadableAndWritable, MaximumPossibleSized};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new_all.wowm:2929`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new_all.wowm#L2929):
/// ```text
/// enum BuybackSlot : u32 {
///     SLOT1 = 69;
///     SLOT2 = 70;
///     SLOT3 = 71;
///     SLOT4 = 72;
///     SLOT5 = 73;
///     SLOT6 = 74;
///     SLOT7 = 75;
///     SLOT8 = 76;
///     SLOT9 = 77;
///     SLOT10 = 78;
///     SLOT11 = 79;
///     SLOT12 = 80;
///     SLOT13 = 81;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum BuybackSlot {
    SLOT1,
    SLOT2,
    SLOT3,
    SLOT4,
    SLOT5,
    SLOT6,
    SLOT7,
    SLOT8,
    SLOT9,
    SLOT10,
    SLOT11,
    SLOT12,
    SLOT13,
}

impl ReadableAndWritable for BuybackSlot {
    type Error = BuybackSlotError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        let a = crate::util::read_u32_le(r)?;

        Ok(a.try_into()?)
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        w.write_all(&self.as_u32().to_le_bytes())?;
        Ok(())
    }

}

impl BuybackSlot {
    pub fn read_u32_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, BuybackSlotError> {
        let a = crate::util::read_u32_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u32_be(w, self.as_u32() as u32)?;
        Ok(())
    }

    pub fn read_u64_le<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, BuybackSlotError> {
        let a = crate::util::read_u64_le(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_le(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub fn read_u64_be<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, BuybackSlotError> {
        let a = crate::util::read_u64_be(r)?;
        Ok((a as u32).try_into()?)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        crate::util::write_u64_be(w, self.as_u32() as u64)?;
        Ok(())
    }

    pub const fn as_u32(&self) -> u32 {
        match self {
            Self::SLOT1 => 0x45,
            Self::SLOT2 => 0x46,
            Self::SLOT3 => 0x47,
            Self::SLOT4 => 0x48,
            Self::SLOT5 => 0x49,
            Self::SLOT6 => 0x4a,
            Self::SLOT7 => 0x4b,
            Self::SLOT8 => 0x4c,
            Self::SLOT9 => 0x4d,
            Self::SLOT10 => 0x4e,
            Self::SLOT11 => 0x4f,
            Self::SLOT12 => 0x50,
            Self::SLOT13 => 0x51,
        }
    }

    pub const fn new() -> Self {
        Self::SLOT1
    }

}

impl ConstantSized for BuybackSlot {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for BuybackSlot {
    fn maximum_possible_size() -> usize {
        4
    }
}

impl Default for BuybackSlot {
    fn default() -> Self {
        Self::SLOT1
    }
}

impl std::fmt::Display for BuybackSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SLOT1 => f.write_str("SLOT1"),
            Self::SLOT2 => f.write_str("SLOT2"),
            Self::SLOT3 => f.write_str("SLOT3"),
            Self::SLOT4 => f.write_str("SLOT4"),
            Self::SLOT5 => f.write_str("SLOT5"),
            Self::SLOT6 => f.write_str("SLOT6"),
            Self::SLOT7 => f.write_str("SLOT7"),
            Self::SLOT8 => f.write_str("SLOT8"),
            Self::SLOT9 => f.write_str("SLOT9"),
            Self::SLOT10 => f.write_str("SLOT10"),
            Self::SLOT11 => f.write_str("SLOT11"),
            Self::SLOT12 => f.write_str("SLOT12"),
            Self::SLOT13 => f.write_str("SLOT13"),
        }
    }
}

impl TryFrom<u32> for BuybackSlot {
    type Error = TryFromBuybackSlotError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            69 => Ok(Self::SLOT1),
            70 => Ok(Self::SLOT2),
            71 => Ok(Self::SLOT3),
            72 => Ok(Self::SLOT4),
            73 => Ok(Self::SLOT5),
            74 => Ok(Self::SLOT6),
            75 => Ok(Self::SLOT7),
            76 => Ok(Self::SLOT8),
            77 => Ok(Self::SLOT9),
            78 => Ok(Self::SLOT10),
            79 => Ok(Self::SLOT11),
            80 => Ok(Self::SLOT12),
            81 => Ok(Self::SLOT13),
            _ => Err(TryFromBuybackSlotError::new(value))
        }
    }
}

#[derive(Debug)]
pub struct TryFromBuybackSlotError {
    value: u32,
}

impl TryFromBuybackSlotError {
    pub const fn new(value: u32) -> Self {
        Self { value }
    }
}

#[derive(Debug)]
pub enum BuybackSlotError {
    Read(std::io::Error),
    TryFrom(TryFromBuybackSlotError),
}

impl std::error::Error for BuybackSlotError {}
impl std::fmt::Display for TryFromBuybackSlotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("invalid value for enum 'BuybackSlot': '{}'", self.value))
    }
}

impl std::fmt::Display for BuybackSlotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Read(e) => e.fmt(f),
            Self::TryFrom(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for BuybackSlotError {
    fn from(value: std::io::Error) -> Self {
        Self::Read(value)
    }
}

impl From<TryFromBuybackSlotError> for BuybackSlotError {
    fn from(value: TryFromBuybackSlotError) -> Self {
        Self::TryFrom(value)
    }
}

