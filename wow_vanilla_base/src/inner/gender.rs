/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/enums/gender.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/enums/gender.wowm#L3):
/// ```text
/// enum Gender : u8 {
///     MALE = 0;
///     FEMALE = 1;
///     NONE = 2;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum Gender {
    MALE,
    FEMALE,
    NONE,
}

impl Default for Gender {
    fn default() -> Self {
        Self::MALE
    }
}

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MALE => f.write_str("MALE"),
            Self::FEMALE => f.write_str("FEMALE"),
            Self::NONE => f.write_str("NONE"),
        }
    }
}

impl Gender {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::MALE => 0x0,
            Self::FEMALE => 0x1,
            Self::NONE => 0x2,
        }
    }

}

