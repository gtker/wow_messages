/// A direct item slot inside a Wrath container object.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/container_slot.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/container_slot.wowm#L2):
/// ```text
/// enum ContainerSlot : u8 {
///     SLOT_0 = 0;
///     SLOT_1 = 1;
///     SLOT_2 = 2;
///     SLOT_3 = 3;
///     SLOT_4 = 4;
///     SLOT_5 = 5;
///     SLOT_6 = 6;
///     SLOT_7 = 7;
///     SLOT_8 = 8;
///     SLOT_9 = 9;
///     SLOT_10 = 10;
///     SLOT_11 = 11;
///     SLOT_12 = 12;
///     SLOT_13 = 13;
///     SLOT_14 = 14;
///     SLOT_15 = 15;
///     SLOT_16 = 16;
///     SLOT_17 = 17;
///     SLOT_18 = 18;
///     SLOT_19 = 19;
///     SLOT_20 = 20;
///     SLOT_21 = 21;
///     SLOT_22 = 22;
///     SLOT_23 = 23;
///     SLOT_24 = 24;
///     SLOT_25 = 25;
///     SLOT_26 = 26;
///     SLOT_27 = 27;
///     SLOT_28 = 28;
///     SLOT_29 = 29;
///     SLOT_30 = 30;
///     SLOT_31 = 31;
///     SLOT_32 = 32;
///     SLOT_33 = 33;
///     SLOT_34 = 34;
///     SLOT_35 = 35;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ContainerSlot {
    Slot0,
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
    Slot14,
    Slot15,
    Slot16,
    Slot17,
    Slot18,
    Slot19,
    Slot20,
    Slot21,
    Slot22,
    Slot23,
    Slot24,
    Slot25,
    Slot26,
    Slot27,
    Slot28,
    Slot29,
    Slot30,
    Slot31,
    Slot32,
    Slot33,
    Slot34,
    Slot35,
}
impl ContainerSlot {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Slot0 => 0x0,
            Self::Slot1 => 0x1,
            Self::Slot2 => 0x2,
            Self::Slot3 => 0x3,
            Self::Slot4 => 0x4,
            Self::Slot5 => 0x5,
            Self::Slot6 => 0x6,
            Self::Slot7 => 0x7,
            Self::Slot8 => 0x8,
            Self::Slot9 => 0x9,
            Self::Slot10 => 0xa,
            Self::Slot11 => 0xb,
            Self::Slot12 => 0xc,
            Self::Slot13 => 0xd,
            Self::Slot14 => 0xe,
            Self::Slot15 => 0xf,
            Self::Slot16 => 0x10,
            Self::Slot17 => 0x11,
            Self::Slot18 => 0x12,
            Self::Slot19 => 0x13,
            Self::Slot20 => 0x14,
            Self::Slot21 => 0x15,
            Self::Slot22 => 0x16,
            Self::Slot23 => 0x17,
            Self::Slot24 => 0x18,
            Self::Slot25 => 0x19,
            Self::Slot26 => 0x1a,
            Self::Slot27 => 0x1b,
            Self::Slot28 => 0x1c,
            Self::Slot29 => 0x1d,
            Self::Slot30 => 0x1e,
            Self::Slot31 => 0x1f,
            Self::Slot32 => 0x20,
            Self::Slot33 => 0x21,
            Self::Slot34 => 0x22,
            Self::Slot35 => 0x23,
        }
    }

    pub const fn variants() -> [Self; 36] {
        [
            Self::Slot0,
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
            Self::Slot13,
            Self::Slot14,
            Self::Slot15,
            Self::Slot16,
            Self::Slot17,
            Self::Slot18,
            Self::Slot19,
            Self::Slot20,
            Self::Slot21,
            Self::Slot22,
            Self::Slot23,
            Self::Slot24,
            Self::Slot25,
            Self::Slot26,
            Self::Slot27,
            Self::Slot28,
            Self::Slot29,
            Self::Slot30,
            Self::Slot31,
            Self::Slot32,
            Self::Slot33,
            Self::Slot34,
            Self::Slot35,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::Slot0),
            1 => Ok(Self::Slot1),
            2 => Ok(Self::Slot2),
            3 => Ok(Self::Slot3),
            4 => Ok(Self::Slot4),
            5 => Ok(Self::Slot5),
            6 => Ok(Self::Slot6),
            7 => Ok(Self::Slot7),
            8 => Ok(Self::Slot8),
            9 => Ok(Self::Slot9),
            10 => Ok(Self::Slot10),
            11 => Ok(Self::Slot11),
            12 => Ok(Self::Slot12),
            13 => Ok(Self::Slot13),
            14 => Ok(Self::Slot14),
            15 => Ok(Self::Slot15),
            16 => Ok(Self::Slot16),
            17 => Ok(Self::Slot17),
            18 => Ok(Self::Slot18),
            19 => Ok(Self::Slot19),
            20 => Ok(Self::Slot20),
            21 => Ok(Self::Slot21),
            22 => Ok(Self::Slot22),
            23 => Ok(Self::Slot23),
            24 => Ok(Self::Slot24),
            25 => Ok(Self::Slot25),
            26 => Ok(Self::Slot26),
            27 => Ok(Self::Slot27),
            28 => Ok(Self::Slot28),
            29 => Ok(Self::Slot29),
            30 => Ok(Self::Slot30),
            31 => Ok(Self::Slot31),
            32 => Ok(Self::Slot32),
            33 => Ok(Self::Slot33),
            34 => Ok(Self::Slot34),
            35 => Ok(Self::Slot35),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl ContainerSlot {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Slot0 => "SLOT_0",
            Self::Slot1 => "SLOT_1",
            Self::Slot2 => "SLOT_2",
            Self::Slot3 => "SLOT_3",
            Self::Slot4 => "SLOT_4",
            Self::Slot5 => "SLOT_5",
            Self::Slot6 => "SLOT_6",
            Self::Slot7 => "SLOT_7",
            Self::Slot8 => "SLOT_8",
            Self::Slot9 => "SLOT_9",
            Self::Slot10 => "SLOT_10",
            Self::Slot11 => "SLOT_11",
            Self::Slot12 => "SLOT_12",
            Self::Slot13 => "SLOT_13",
            Self::Slot14 => "SLOT_14",
            Self::Slot15 => "SLOT_15",
            Self::Slot16 => "SLOT_16",
            Self::Slot17 => "SLOT_17",
            Self::Slot18 => "SLOT_18",
            Self::Slot19 => "SLOT_19",
            Self::Slot20 => "SLOT_20",
            Self::Slot21 => "SLOT_21",
            Self::Slot22 => "SLOT_22",
            Self::Slot23 => "SLOT_23",
            Self::Slot24 => "SLOT_24",
            Self::Slot25 => "SLOT_25",
            Self::Slot26 => "SLOT_26",
            Self::Slot27 => "SLOT_27",
            Self::Slot28 => "SLOT_28",
            Self::Slot29 => "SLOT_29",
            Self::Slot30 => "SLOT_30",
            Self::Slot31 => "SLOT_31",
            Self::Slot32 => "SLOT_32",
            Self::Slot33 => "SLOT_33",
            Self::Slot34 => "SLOT_34",
            Self::Slot35 => "SLOT_35",
        }
    }

}

const NAME: &str = "ContainerSlot";

impl Default for ContainerSlot {
    fn default() -> Self {
        Self::Slot0
    }
}

impl std::fmt::Display for ContainerSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Slot0 => f.write_str("Slot0"),
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
            Self::Slot14 => f.write_str("Slot14"),
            Self::Slot15 => f.write_str("Slot15"),
            Self::Slot16 => f.write_str("Slot16"),
            Self::Slot17 => f.write_str("Slot17"),
            Self::Slot18 => f.write_str("Slot18"),
            Self::Slot19 => f.write_str("Slot19"),
            Self::Slot20 => f.write_str("Slot20"),
            Self::Slot21 => f.write_str("Slot21"),
            Self::Slot22 => f.write_str("Slot22"),
            Self::Slot23 => f.write_str("Slot23"),
            Self::Slot24 => f.write_str("Slot24"),
            Self::Slot25 => f.write_str("Slot25"),
            Self::Slot26 => f.write_str("Slot26"),
            Self::Slot27 => f.write_str("Slot27"),
            Self::Slot28 => f.write_str("Slot28"),
            Self::Slot29 => f.write_str("Slot29"),
            Self::Slot30 => f.write_str("Slot30"),
            Self::Slot31 => f.write_str("Slot31"),
            Self::Slot32 => f.write_str("Slot32"),
            Self::Slot33 => f.write_str("Slot33"),
            Self::Slot34 => f.write_str("Slot34"),
            Self::Slot35 => f.write_str("Slot35"),
        }
    }
}

impl TryFrom<u8> for ContainerSlot {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for ContainerSlot {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for ContainerSlot {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for ContainerSlot {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for ContainerSlot {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i16> for ContainerSlot {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for ContainerSlot {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for ContainerSlot {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for ContainerSlot {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}
