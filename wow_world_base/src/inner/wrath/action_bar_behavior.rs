/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_action_buttons.wowm:21`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_action_buttons.wowm#L21):
/// ```text
/// enum ActionBarBehavior : u8 {
///     INITIAL = 0;
///     SET = 1;
///     CLEAR = 2;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ActionBarBehavior {
    Initial,
    Set,
    Clear,
}

impl ActionBarBehavior {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::Initial => 0x0,
            Self::Set => 0x1,
            Self::Clear => 0x2,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl ActionBarBehavior {
    pub fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Initial => "INITIAL",
            Self::Set => "SET",
            Self::Clear => "CLEAR",
        }
    }

}

impl Default for ActionBarBehavior {
    fn default() -> Self {
        Self::Initial
    }
}

impl std::fmt::Display for ActionBarBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Initial => f.write_str("Initial"),
            Self::Set => f.write_str("Set"),
            Self::Clear => f.write_str("Clear"),
        }
    }
}

impl TryFrom<u8> for ActionBarBehavior {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Initial),
            1 => Ok(Self::Set),
            2 => Ok(Self::Clear),
            v => Err(crate::errors::EnumError::new("ActionBarBehavior", v as u64),)
        }
    }
}

