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
    Slot1,
    Slot2,
    Slot3,
    Slot4,
    Slot5,
    Slot6,
    Slot7,
    Slot8,
    Slot9,
    Slot10,
    Slot11,
    Slot12,
    Slot13,
}

impl BuybackSlot {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::Slot1 => 0x45,
            Self::Slot2 => 0x46,
            Self::Slot3 => 0x47,
            Self::Slot4 => 0x48,
            Self::Slot5 => 0x49,
            Self::Slot6 => 0x4a,
            Self::Slot7 => 0x4b,
            Self::Slot8 => 0x4c,
            Self::Slot9 => 0x4d,
            Self::Slot10 => 0x4e,
            Self::Slot11 => 0x4f,
            Self::Slot12 => 0x50,
            Self::Slot13 => 0x51,
        }
    }

}

impl Default for BuybackSlot {
    fn default() -> Self {
        Self::Slot1
    }
}

impl std::fmt::Display for BuybackSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Slot1 => f.write_str("Slot1"),
            Self::Slot2 => f.write_str("Slot2"),
            Self::Slot3 => f.write_str("Slot3"),
            Self::Slot4 => f.write_str("Slot4"),
            Self::Slot5 => f.write_str("Slot5"),
            Self::Slot6 => f.write_str("Slot6"),
            Self::Slot7 => f.write_str("Slot7"),
            Self::Slot8 => f.write_str("Slot8"),
            Self::Slot9 => f.write_str("Slot9"),
            Self::Slot10 => f.write_str("Slot10"),
            Self::Slot11 => f.write_str("Slot11"),
            Self::Slot12 => f.write_str("Slot12"),
            Self::Slot13 => f.write_str("Slot13"),
        }
    }
}

impl TryFrom<u32> for BuybackSlot {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            69 => Ok(Self::Slot1),
            70 => Ok(Self::Slot2),
            71 => Ok(Self::Slot3),
            72 => Ok(Self::Slot4),
            73 => Ok(Self::Slot5),
            74 => Ok(Self::Slot6),
            75 => Ok(Self::Slot7),
            76 => Ok(Self::Slot8),
            77 => Ok(Self::Slot9),
            78 => Ok(Self::Slot10),
            79 => Ok(Self::Slot11),
            80 => Ok(Self::Slot12),
            81 => Ok(Self::Slot13),
            v => Err(crate::errors::EnumError::new("BuybackSlot", v),)
        }
    }
}

