use crate::wrath::ReferAFriendError;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_refer_a_friend_failure.wowm:30`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_refer_a_friend_failure.wowm#L30):
/// ```text
/// smsg SMSG_REFER_A_FRIEND_FAILURE = 0x0421 {
///     (u32)ReferAFriendError error;
///     if (error == NOT_IN_GROUP) {
///         CString target_name;
///     }
/// }
/// ```
pub struct SMSG_REFER_A_FRIEND_FAILURE {
    pub error: SMSG_REFER_A_FRIEND_FAILURE_ReferAFriendError,
}

impl crate::Message for SMSG_REFER_A_FRIEND_FAILURE {
    const OPCODE: u32 = 0x0421;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // error: ReferAFriendError
        w.write_all(&u32::from(self.error.as_int()).to_le_bytes())?;

        match &self.error {
            SMSG_REFER_A_FRIEND_FAILURE_ReferAFriendError::None => {}
            SMSG_REFER_A_FRIEND_FAILURE_ReferAFriendError::NotReferredBy => {}
            SMSG_REFER_A_FRIEND_FAILURE_ReferAFriendError::TargetTooHigh => {}
            SMSG_REFER_A_FRIEND_FAILURE_ReferAFriendError::InsufficientGrantableLevels => {}
            SMSG_REFER_A_FRIEND_FAILURE_ReferAFriendError::TooFar => {}
            SMSG_REFER_A_FRIEND_FAILURE_ReferAFriendError::DifferentFaction => {}
            SMSG_REFER_A_FRIEND_FAILURE_ReferAFriendError::NotNow => {}
            SMSG_REFER_A_FRIEND_FAILURE_ReferAFriendError::GrantLevelMax => {}
            SMSG_REFER_A_FRIEND_FAILURE_ReferAFriendError::NoTarget => {}
            SMSG_REFER_A_FRIEND_FAILURE_ReferAFriendError::NotInGroup {
                target_name,
            } => {
                // target_name: CString
                // TODO: Guard against strings that are already null-terminated
                assert_ne!(target_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `target_name` must not be null-terminated.");
                w.write_all(target_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            SMSG_REFER_A_FRIEND_FAILURE_ReferAFriendError::SummonLevelMax => {}
            SMSG_REFER_A_FRIEND_FAILURE_ReferAFriendError::SummonCooldown => {}
            SMSG_REFER_A_FRIEND_FAILURE_ReferAFriendError::InsufficientExpansionLevel => {}
            SMSG_REFER_A_FRIEND_FAILURE_ReferAFriendError::SummonOffline => {}
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(4..=260).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0421, size: body_size as u32 });
        }

        // error: ReferAFriendError
        let error: ReferAFriendError = (crate::util::read_u32_le(r)? as u8).try_into()?;

        let error_if = match error {
            ReferAFriendError::None => SMSG_REFER_A_FRIEND_FAILURE_ReferAFriendError::None,
            ReferAFriendError::NotReferredBy => SMSG_REFER_A_FRIEND_FAILURE_ReferAFriendError::NotReferredBy,
            ReferAFriendError::TargetTooHigh => SMSG_REFER_A_FRIEND_FAILURE_ReferAFriendError::TargetTooHigh,
            ReferAFriendError::InsufficientGrantableLevels => SMSG_REFER_A_FRIEND_FAILURE_ReferAFriendError::InsufficientGrantableLevels,
            ReferAFriendError::TooFar => SMSG_REFER_A_FRIEND_FAILURE_ReferAFriendError::TooFar,
            ReferAFriendError::DifferentFaction => SMSG_REFER_A_FRIEND_FAILURE_ReferAFriendError::DifferentFaction,
            ReferAFriendError::NotNow => SMSG_REFER_A_FRIEND_FAILURE_ReferAFriendError::NotNow,
            ReferAFriendError::GrantLevelMax => SMSG_REFER_A_FRIEND_FAILURE_ReferAFriendError::GrantLevelMax,
            ReferAFriendError::NoTarget => SMSG_REFER_A_FRIEND_FAILURE_ReferAFriendError::NoTarget,
            ReferAFriendError::NotInGroup => {
                // target_name: CString
                let target_name = {
                    let target_name = crate::util::read_c_string_to_vec(r)?;
                    String::from_utf8(target_name)?
                };

                SMSG_REFER_A_FRIEND_FAILURE_ReferAFriendError::NotInGroup {
                    target_name,
                }
            }
            ReferAFriendError::SummonLevelMax => SMSG_REFER_A_FRIEND_FAILURE_ReferAFriendError::SummonLevelMax,
            ReferAFriendError::SummonCooldown => SMSG_REFER_A_FRIEND_FAILURE_ReferAFriendError::SummonCooldown,
            ReferAFriendError::InsufficientExpansionLevel => SMSG_REFER_A_FRIEND_FAILURE_ReferAFriendError::InsufficientExpansionLevel,
            ReferAFriendError::SummonOffline => SMSG_REFER_A_FRIEND_FAILURE_ReferAFriendError::SummonOffline,
        };

        Ok(Self {
            error: error_if,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_REFER_A_FRIEND_FAILURE {}

impl SMSG_REFER_A_FRIEND_FAILURE {
    pub(crate) fn size(&self) -> usize {
        self.error.size() // error: SMSG_REFER_A_FRIEND_FAILURE_ReferAFriendError
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_REFER_A_FRIEND_FAILURE_ReferAFriendError {
    None,
    NotReferredBy,
    TargetTooHigh,
    InsufficientGrantableLevels,
    TooFar,
    DifferentFaction,
    NotNow,
    GrantLevelMax,
    NoTarget,
    NotInGroup {
        target_name: String,
    },
    SummonLevelMax,
    SummonCooldown,
    InsufficientExpansionLevel,
    SummonOffline,
}

impl Default for SMSG_REFER_A_FRIEND_FAILURE_ReferAFriendError {
    fn default() -> Self {
        // First enumerator without any fields
        Self::None
    }
}

impl SMSG_REFER_A_FRIEND_FAILURE_ReferAFriendError {
    pub(crate) const fn as_int(&self) -> u32 {
        match self {
            Self::None => 0,
            Self::NotReferredBy => 1,
            Self::TargetTooHigh => 2,
            Self::InsufficientGrantableLevels => 3,
            Self::TooFar => 4,
            Self::DifferentFaction => 5,
            Self::NotNow => 6,
            Self::GrantLevelMax => 7,
            Self::NoTarget => 8,
            Self::NotInGroup { .. } => 9,
            Self::SummonLevelMax => 10,
            Self::SummonCooldown => 11,
            Self::InsufficientExpansionLevel => 12,
            Self::SummonOffline => 13,
        }
    }

}

impl SMSG_REFER_A_FRIEND_FAILURE_ReferAFriendError {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::None => {
                4
            }
            Self::NotReferredBy => {
                4
            }
            Self::TargetTooHigh => {
                4
            }
            Self::InsufficientGrantableLevels => {
                4
            }
            Self::TooFar => {
                4
            }
            Self::DifferentFaction => {
                4
            }
            Self::NotNow => {
                4
            }
            Self::GrantLevelMax => {
                4
            }
            Self::NoTarget => {
                4
            }
            Self::NotInGroup {
                target_name,
            } => {
                4
                + target_name.len() + 1 // target_name: CString
            }
            Self::SummonLevelMax => {
                4
            }
            Self::SummonCooldown => {
                4
            }
            Self::InsufficientExpansionLevel => {
                4
            }
            Self::SummonOffline => {
                4
            }
        }
    }
}

