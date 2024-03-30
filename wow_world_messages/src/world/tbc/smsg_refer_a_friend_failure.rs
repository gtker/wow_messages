use std::io::{Read, Write};

use crate::tbc::ReferAFriendError;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_refer_a_friend_failure.wowm:21`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_refer_a_friend_failure.wowm#L21):
/// ```text
/// smsg SMSG_REFER_A_FRIEND_FAILURE = 0x0420 {
///     (u32)ReferAFriendError error;
///     if (error == NOT_IN_GROUP) {
///         CString target_name;
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_REFER_A_FRIEND_FAILURE {
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

impl crate::private::Sealed for SMSG_REFER_A_FRIEND_FAILURE {}
impl SMSG_REFER_A_FRIEND_FAILURE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(4..=260).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // error: ReferAFriendError
        let error = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        let error_if = match error {
            ReferAFriendError::None => SMSG_REFER_A_FRIEND_FAILURE::None,
            ReferAFriendError::NotReferredBy => SMSG_REFER_A_FRIEND_FAILURE::NotReferredBy,
            ReferAFriendError::TargetTooHigh => SMSG_REFER_A_FRIEND_FAILURE::TargetTooHigh,
            ReferAFriendError::InsufficientGrantableLevels => SMSG_REFER_A_FRIEND_FAILURE::InsufficientGrantableLevels,
            ReferAFriendError::TooFar => SMSG_REFER_A_FRIEND_FAILURE::TooFar,
            ReferAFriendError::DifferentFaction => SMSG_REFER_A_FRIEND_FAILURE::DifferentFaction,
            ReferAFriendError::NotNow => SMSG_REFER_A_FRIEND_FAILURE::NotNow,
            ReferAFriendError::GrantLevelMax => SMSG_REFER_A_FRIEND_FAILURE::GrantLevelMax,
            ReferAFriendError::NoTarget => SMSG_REFER_A_FRIEND_FAILURE::NoTarget,
            ReferAFriendError::NotInGroup => {
                // target_name: CString
                let target_name = {
                    let target_name = crate::util::read_c_string_to_vec(&mut r)?;
                    String::from_utf8(target_name)?
                };

                SMSG_REFER_A_FRIEND_FAILURE::NotInGroup {
                    target_name,
                }
            }
            ReferAFriendError::SummonLevelMax => SMSG_REFER_A_FRIEND_FAILURE::SummonLevelMax,
            ReferAFriendError::SummonCooldown => SMSG_REFER_A_FRIEND_FAILURE::SummonCooldown,
            ReferAFriendError::InsufficientExpansionLevel => SMSG_REFER_A_FRIEND_FAILURE::InsufficientExpansionLevel,
            ReferAFriendError::SummonOffline => SMSG_REFER_A_FRIEND_FAILURE::SummonOffline,
        };

        Ok(error_if)
    }

}

impl crate::Message for SMSG_REFER_A_FRIEND_FAILURE {
    const OPCODE: u32 = 0x0420;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_REFER_A_FRIEND_FAILURE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_REFER_A_FRIEND_FAILURE {{").unwrap();
        // Members
        writeln!(s, "    error = {};", ReferAFriendError::try_from(self.as_int()as u8).unwrap().as_test_case_value()).unwrap();
        match &self {
            crate::tbc::SMSG_REFER_A_FRIEND_FAILURE::NotInGroup {
                target_name,
            } => {
                writeln!(s, "    target_name = \"{}\";", target_name).unwrap();
            }
            _ => {}
        }


        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1056_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "error", "    ");
        match &self {
            crate::tbc::SMSG_REFER_A_FRIEND_FAILURE::NotInGroup {
                target_name,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, target_name.len() + 1, "target_name", "    ");
            }
            _ => {}
        }



        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // error: ReferAFriendError
        w.write_all(&(self.as_int().to_le_bytes()))?;

        match &self {
            SMSG_REFER_A_FRIEND_FAILURE::NotInGroup {
                target_name,
            } => {
                // target_name: CString
                // TODO: Guard against strings that are already null-terminated
                assert_ne!(target_name.as_bytes().iter().next_back(), Some(&0_u8), "String `target_name` must not be null-terminated.");
                w.write_all(target_name.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

            }
            _ => {}
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1056, "SMSG_REFER_A_FRIEND_FAILURE", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_REFER_A_FRIEND_FAILURE {}

impl SMSG_REFER_A_FRIEND_FAILURE {
    pub(crate) fn size(&self) -> usize {
        (match self {
            Self::NotInGroup {
                target_name,
            } => {
                4
                + target_name.len() + 1 // target_name: CString
            }
            _ => 4,
        }) // error: SMSG_REFER_A_FRIEND_FAILURE
    }
}

impl Default for SMSG_REFER_A_FRIEND_FAILURE {
    fn default() -> Self {
        // First enumerator without any fields
        Self::None
    }
}

impl SMSG_REFER_A_FRIEND_FAILURE {
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

impl std::fmt::Display for SMSG_REFER_A_FRIEND_FAILURE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("None"),
            Self::NotReferredBy => f.write_str("NotReferredBy"),
            Self::TargetTooHigh => f.write_str("TargetTooHigh"),
            Self::InsufficientGrantableLevels => f.write_str("InsufficientGrantableLevels"),
            Self::TooFar => f.write_str("TooFar"),
            Self::DifferentFaction => f.write_str("DifferentFaction"),
            Self::NotNow => f.write_str("NotNow"),
            Self::GrantLevelMax => f.write_str("GrantLevelMax"),
            Self::NoTarget => f.write_str("NoTarget"),
            Self::NotInGroup{ .. } => f.write_str("NotInGroup"),
            Self::SummonLevelMax => f.write_str("SummonLevelMax"),
            Self::SummonCooldown => f.write_str("SummonCooldown"),
            Self::InsufficientExpansionLevel => f.write_str("InsufficientExpansionLevel"),
            Self::SummonOffline => f.write_str("SummonOffline"),
        }
    }
}

