/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/meetingstone/smsg_meetingstone_joinfailed.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/meetingstone/smsg_meetingstone_joinfailed.wowm#L3):
/// ```text
/// enum MeetingStoneFailure : u8 {
///     MEETINGSTONE_FAIL_PARTYLEADER = 1;
///     MEETINGSTONE_FAIL_FULL_GROUP = 2;
///     MEETINGSTONE_FAIL_RAID_GROUP = 3;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum MeetingStoneFailure {
    MeetingstoneFailPartyleader,
    MeetingstoneFailFullGroup,
    MeetingstoneFailRaidGroup,
}

impl MeetingStoneFailure {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::MeetingstoneFailPartyleader => 0x1,
            Self::MeetingstoneFailFullGroup => 0x2,
            Self::MeetingstoneFailRaidGroup => 0x3,
        }
    }

    pub const fn variants() -> [Self; 3] {
        [
            Self::MeetingstoneFailPartyleader,
            Self::MeetingstoneFailFullGroup,
            Self::MeetingstoneFailRaidGroup,
        ]
    }

    pub const fn from_int(value: u8) -> Result<Self, crate::errors::EnumError> {
        match value {
            1 => Ok(Self::MeetingstoneFailPartyleader),
            2 => Ok(Self::MeetingstoneFailFullGroup),
            3 => Ok(Self::MeetingstoneFailRaidGroup),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl MeetingStoneFailure {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::MeetingstoneFailPartyleader => "MEETINGSTONE_FAIL_PARTYLEADER",
            Self::MeetingstoneFailFullGroup => "MEETINGSTONE_FAIL_FULL_GROUP",
            Self::MeetingstoneFailRaidGroup => "MEETINGSTONE_FAIL_RAID_GROUP",
        }
    }

}

const NAME: &str = "MeetingStoneFailure";

impl Default for MeetingStoneFailure {
    fn default() -> Self {
        Self::MeetingstoneFailPartyleader
    }
}

impl std::fmt::Display for MeetingStoneFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MeetingstoneFailPartyleader => f.write_str("MeetingstoneFailPartyleader"),
            Self::MeetingstoneFailFullGroup => f.write_str("MeetingstoneFailFullGroup"),
            Self::MeetingstoneFailRaidGroup => f.write_str("MeetingstoneFailRaidGroup"),
        }
    }
}

impl TryFrom<u8> for MeetingStoneFailure {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u16> for MeetingStoneFailure {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for MeetingStoneFailure {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for MeetingStoneFailure {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for MeetingStoneFailure {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        Self::from_int(v)
    }
}

impl TryFrom<i16> for MeetingStoneFailure {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for MeetingStoneFailure {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for MeetingStoneFailure {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for MeetingStoneFailure {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

