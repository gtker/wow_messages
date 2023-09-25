/// Used in `CharSections.dbc`.
///
/// This type is not sent over the network, but is used in the game in another way.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/external/selection_type.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/external/selection_type.wowm#L2):
/// ```text
/// enum SelectionType : u8 {
///     BASE_SKIN = 0x00;
///     FACE = 0x01;
///     FACIAL_HAIR = 0x02;
///     HAIR = 0x03;
///     UNDERWEAR = 0x04;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum SelectionType {
    BaseSkin,
    Face,
    FacialHair,
    Hair,
    Underwear,
}

impl SelectionType {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::BaseSkin => 0x0,
            Self::Face => 0x1,
            Self::FacialHair => 0x2,
            Self::Hair => 0x3,
            Self::Underwear => 0x4,
        }
    }

    pub const fn variants() -> [Self; 5] {
        [
            Self::BaseSkin,
            Self::Face,
            Self::FacialHair,
            Self::Hair,
            Self::Underwear,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::BaseSkin),
            1 => Ok(Self::Face),
            2 => Ok(Self::FacialHair),
            3 => Ok(Self::Hair),
            4 => Ok(Self::Underwear),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl SelectionType {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::BaseSkin => "BASE_SKIN",
            Self::Face => "FACE",
            Self::FacialHair => "FACIAL_HAIR",
            Self::Hair => "HAIR",
            Self::Underwear => "UNDERWEAR",
        }
    }

}

const NAME: &str = "SelectionType";

impl Default for SelectionType {
    fn default() -> Self {
        Self::BaseSkin
    }
}

impl std::fmt::Display for SelectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BaseSkin => f.write_str("BaseSkin"),
            Self::Face => f.write_str("Face"),
            Self::FacialHair => f.write_str("FacialHair"),
            Self::Hair => f.write_str("Hair"),
            Self::Underwear => f.write_str("Underwear"),
        }
    }
}

impl TryFrom<u8> for SelectionType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for SelectionType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for SelectionType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for SelectionType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for SelectionType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for SelectionType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for SelectionType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for SelectionType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for SelectionType {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

