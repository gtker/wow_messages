use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_ai_reaction.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_ai_reaction.wowm#L3):
/// ```text
/// enum AiReaction : u32 {
///     ALERT = 0;
///     FRIENDLY = 1;
///     HOSTILE = 2;
///     AFRAID = 3;
///     DESTROY = 4;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub enum AiReaction {
    /// # Comment
    ///
    /// pre-aggro (used in client packet handler)
    ALERT,
    /// # Comment
    ///
    /// (NOT used in client packet handler)
    FRIENDLY,
    /// # Comment
    ///
    /// sent on every attack, triggers aggro sound (used in client packet handler)
    HOSTILE,
    /// # Comment
    ///
    /// seen for polymorph (when AI not in control of self?) (NOT used in client packet handler)
    AFRAID,
    /// # Comment
    ///
    /// used on object destroy (NOT used in client packet handler)
    DESTROY,
}

impl AiReaction {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::ALERT => 0x0,
            Self::FRIENDLY => 0x1,
            Self::HOSTILE => 0x2,
            Self::AFRAID => 0x3,
            Self::DESTROY => 0x4,
        }
    }

}

impl Default for AiReaction {
    fn default() -> Self {
        Self::ALERT
    }
}

impl std::fmt::Display for AiReaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ALERT => f.write_str("ALERT"),
            Self::FRIENDLY => f.write_str("FRIENDLY"),
            Self::HOSTILE => f.write_str("HOSTILE"),
            Self::AFRAID => f.write_str("AFRAID"),
            Self::DESTROY => f.write_str("DESTROY"),
        }
    }
}

impl TryFrom<u32> for AiReaction {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::ALERT),
            1 => Ok(Self::FRIENDLY),
            2 => Ok(Self::HOSTILE),
            3 => Ok(Self::AFRAID),
            4 => Ok(Self::DESTROY),
            v => Err(crate::errors::EnumError::new("AiReaction", v as u32),)
        }
    }
}

