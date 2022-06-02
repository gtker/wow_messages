use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_buyback_item.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_buyback_item.wowm#L3):
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

impl BuybackSlot {
    pub(crate) const fn as_int(&self) -> u32 {
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
    type Error = crate::errors::EnumError;
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
            v => Err(crate::errors::EnumError::new("BuybackSlot", v as u32),)
        }
    }
}

