/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_buyback_item.wowm:19`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_buyback_item.wowm#L19):
/// ```text
/// enum BuybackSlot : u32 {
///     SLOT1 = 74;
///     SLOT2 = 75;
///     SLOT3 = 76;
///     SLOT4 = 77;
///     SLOT5 = 78;
///     SLOT6 = 79;
///     SLOT7 = 80;
///     SLOT8 = 81;
///     SLOT9 = 82;
///     SLOT10 = 83;
///     SLOT11 = 84;
///     SLOT12 = 85;
/// }
/// ```
#[derive(Debug, Default, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum BuybackSlot {
    #[default]
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
}

impl BuybackSlot {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::Slot1 => 0x4a,
            Self::Slot2 => 0x4b,
            Self::Slot3 => 0x4c,
            Self::Slot4 => 0x4d,
            Self::Slot5 => 0x4e,
            Self::Slot6 => 0x4f,
            Self::Slot7 => 0x50,
            Self::Slot8 => 0x51,
            Self::Slot9 => 0x52,
            Self::Slot10 => 0x53,
            Self::Slot11 => 0x54,
            Self::Slot12 => 0x55,
        }
    }

    pub const fn variants() -> [Self; 12] {
        [
            Self::Slot1,
            Self::Slot2,
            Self::Slot3,
            Self::Slot4,
            Self::Slot5,
            Self::Slot6,
            Self::Slot7,
            Self::Slot8,
            Self::Slot9,
            Self::Slot10,
            Self::Slot11,
            Self::Slot12,
        ]
    }

    pub const fn from_int(value: u32) -> Result<Self, crate::errors::EnumError> {
        match value {
            74 => Ok(Self::Slot1),
            75 => Ok(Self::Slot2),
            76 => Ok(Self::Slot3),
            77 => Ok(Self::Slot4),
            78 => Ok(Self::Slot5),
            79 => Ok(Self::Slot6),
            80 => Ok(Self::Slot7),
            81 => Ok(Self::Slot8),
            82 => Ok(Self::Slot9),
            83 => Ok(Self::Slot10),
            84 => Ok(Self::Slot11),
            85 => Ok(Self::Slot12),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl BuybackSlot {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Slot1 => "SLOT1",
            Self::Slot2 => "SLOT2",
            Self::Slot3 => "SLOT3",
            Self::Slot4 => "SLOT4",
            Self::Slot5 => "SLOT5",
            Self::Slot6 => "SLOT6",
            Self::Slot7 => "SLOT7",
            Self::Slot8 => "SLOT8",
            Self::Slot9 => "SLOT9",
            Self::Slot10 => "SLOT10",
            Self::Slot11 => "SLOT11",
            Self::Slot12 => "SLOT12",
        }
    }

}

const NAME: &str = "BuybackSlot";

impl std::fmt::Display for BuybackSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Slot1 => "Slot1",
            Self::Slot2 => "Slot2",
            Self::Slot3 => "Slot3",
            Self::Slot4 => "Slot4",
            Self::Slot5 => "Slot5",
            Self::Slot6 => "Slot6",
            Self::Slot7 => "Slot7",
            Self::Slot8 => "Slot8",
            Self::Slot9 => "Slot9",
            Self::Slot10 => "Slot10",
            Self::Slot11 => "Slot11",
            Self::Slot12 => "Slot12",
        })
    }
}

impl TryFrom<u32> for BuybackSlot {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u8> for BuybackSlot {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value.into())
    }
}

impl TryFrom<u16> for BuybackSlot {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        Self::from_int(value.into())
    }
}

impl TryFrom<u64> for BuybackSlot {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for BuybackSlot {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for BuybackSlot {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for BuybackSlot {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let v = u32::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i64> for BuybackSlot {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for BuybackSlot {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

