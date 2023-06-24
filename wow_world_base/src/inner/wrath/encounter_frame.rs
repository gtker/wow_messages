/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/smsg_update_instance_encounter_unit.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_update_instance_encounter_unit.wowm#L1):
/// ```text
/// enum EncounterFrame : u32 {
///     ENGAGE = 0;
///     DISENGAGE = 1;
///     UPDATE_PRIORITY = 2;
///     ADD_TIMER = 3;
///     ENABLE_OBJECTIVE = 4;
///     UPDATE_OBJECTIVE = 5;
///     DISABLE_OBJECTIVE = 6;
///     REFRESH_FRAMES = 7;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum EncounterFrame {
    Engage,
    Disengage,
    UpdatePriority,
    AddTimer,
    EnableObjective,
    UpdateObjective,
    DisableObjective,
    /// azerothcore: can be used to refresh frames after unit was destroyed from client and send back (phase changes)
    RefreshFrames,
}

impl EncounterFrame {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::Engage => 0x0,
            Self::Disengage => 0x1,
            Self::UpdatePriority => 0x2,
            Self::AddTimer => 0x3,
            Self::EnableObjective => 0x4,
            Self::UpdateObjective => 0x5,
            Self::DisableObjective => 0x6,
            Self::RefreshFrames => 0x7,
        }
    }

    pub const fn variants() -> [Self; 8] {
        [
            Self::Engage,
            Self::Disengage,
            Self::UpdatePriority,
            Self::AddTimer,
            Self::EnableObjective,
            Self::UpdateObjective,
            Self::DisableObjective,
            Self::RefreshFrames,
        ]
    }

}

#[cfg(feature = "print-testcase")]
impl EncounterFrame {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Engage => "ENGAGE",
            Self::Disengage => "DISENGAGE",
            Self::UpdatePriority => "UPDATE_PRIORITY",
            Self::AddTimer => "ADD_TIMER",
            Self::EnableObjective => "ENABLE_OBJECTIVE",
            Self::UpdateObjective => "UPDATE_OBJECTIVE",
            Self::DisableObjective => "DISABLE_OBJECTIVE",
            Self::RefreshFrames => "REFRESH_FRAMES",
        }
    }

}

const NAME: &str = "EncounterFrame";

impl Default for EncounterFrame {
    fn default() -> Self {
        Self::Engage
    }
}

impl std::fmt::Display for EncounterFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Engage => f.write_str("Engage"),
            Self::Disengage => f.write_str("Disengage"),
            Self::UpdatePriority => f.write_str("UpdatePriority"),
            Self::AddTimer => f.write_str("AddTimer"),
            Self::EnableObjective => f.write_str("EnableObjective"),
            Self::UpdateObjective => f.write_str("UpdateObjective"),
            Self::DisableObjective => f.write_str("DisableObjective"),
            Self::RefreshFrames => f.write_str("RefreshFrames"),
        }
    }
}

impl TryFrom<u32> for EncounterFrame {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Engage),
            1 => Ok(Self::Disengage),
            2 => Ok(Self::UpdatePriority),
            3 => Ok(Self::AddTimer),
            4 => Ok(Self::EnableObjective),
            5 => Ok(Self::UpdateObjective),
            6 => Ok(Self::DisableObjective),
            7 => Ok(Self::RefreshFrames),
            v => Err(crate::errors::EnumError::new(NAME, v.into()),)
        }
    }
}

impl TryFrom<u8> for EncounterFrame {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u16> for EncounterFrame {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for EncounterFrame {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for EncounterFrame {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for EncounterFrame {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for EncounterFrame {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for EncounterFrame {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for EncounterFrame {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

