use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::ExperienceAwardType;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/exp/smsg_log_xpgain.wowm:21`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/exp/smsg_log_xpgain.wowm#L21):
/// ```text
/// smsg SMSG_LOG_XPGAIN = 0x01D0 {
///     Guid target;
///     u32 total_exp;
///     ExperienceAwardType exp_type;
///     if (exp_type == KILL) {
///         u32 experience_without_rested;
///         f32 exp_group_bonus;
///     }
///     Bool exp_includes_recruit_a_friend_bonus;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct SMSG_LOG_XPGAIN {
    pub target: Guid,
    pub total_exp: u32,
    pub exp_type: SMSG_LOG_XPGAIN_ExperienceAwardType,
    pub exp_includes_recruit_a_friend_bonus: bool,
}

impl crate::private::Sealed for SMSG_LOG_XPGAIN {}
impl SMSG_LOG_XPGAIN {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(14..=22).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // target: Guid
        let target = crate::util::read_guid(&mut r)?;

        // total_exp: u32
        let total_exp = crate::util::read_u32_le(&mut r)?;

        // exp_type: ExperienceAwardType
        let exp_type = crate::util::read_u8_le(&mut r)?.try_into()?;

        let exp_type_if = match exp_type {
            ExperienceAwardType::Kill => {
                // experience_without_rested: u32
                let experience_without_rested = crate::util::read_u32_le(&mut r)?;

                // exp_group_bonus: f32
                let exp_group_bonus = crate::util::read_f32_le(&mut r)?;

                SMSG_LOG_XPGAIN_ExperienceAwardType::Kill {
                    exp_group_bonus,
                    experience_without_rested,
                }
            }
            ExperienceAwardType::NonKill => SMSG_LOG_XPGAIN_ExperienceAwardType::NonKill,
        };

        // exp_includes_recruit_a_friend_bonus: Bool
        let exp_includes_recruit_a_friend_bonus = crate::util::read_bool_u8(&mut r)?;

        Ok(Self {
            target,
            total_exp,
            exp_type: exp_type_if,
            exp_includes_recruit_a_friend_bonus,
        })
    }

}

impl crate::Message for SMSG_LOG_XPGAIN {
    const OPCODE: u32 = 0x01d0;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_LOG_XPGAIN"
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // target: Guid
        w.write_all(&self.target.guid().to_le_bytes())?;

        // total_exp: u32
        w.write_all(&self.total_exp.to_le_bytes())?;

        // exp_type: ExperienceAwardType
        w.write_all(&(self.exp_type.as_int().to_le_bytes()))?;

        match &self.exp_type {
            SMSG_LOG_XPGAIN_ExperienceAwardType::Kill {
                exp_group_bonus,
                experience_without_rested,
            } => {
                // experience_without_rested: u32
                w.write_all(&experience_without_rested.to_le_bytes())?;

                // exp_group_bonus: f32
                w.write_all(&exp_group_bonus.to_le_bytes())?;

            }
            _ => {}
        }

        // exp_includes_recruit_a_friend_bonus: Bool
        w.write_all(u8::from(self.exp_includes_recruit_a_friend_bonus).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(464, "SMSG_LOG_XPGAIN", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_LOG_XPGAIN {}

impl SMSG_LOG_XPGAIN {
    pub(crate) const fn size(&self) -> usize {
        8 // target: Guid
        + 4 // total_exp: u32
        + self.exp_type.size() // exp_type: SMSG_LOG_XPGAIN_ExperienceAwardType
        + 1 // exp_includes_recruit_a_friend_bonus: Bool
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum SMSG_LOG_XPGAIN_ExperienceAwardType {
    Kill {
        exp_group_bonus: f32,
        experience_without_rested: u32,
    },
    NonKill,
}

impl Default for SMSG_LOG_XPGAIN_ExperienceAwardType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::NonKill
    }
}

impl SMSG_LOG_XPGAIN_ExperienceAwardType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Kill { .. } => 0,
            Self::NonKill => 1,
        }
    }

}

impl std::fmt::Display for SMSG_LOG_XPGAIN_ExperienceAwardType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Kill{ .. } => f.write_str("Kill"),
            Self::NonKill => f.write_str("NonKill"),
        }
    }
}

impl SMSG_LOG_XPGAIN_ExperienceAwardType {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::Kill {
                ..
            } => {
                1
                + 4 // exp_group_bonus: f32
                + 4 // experience_without_rested: u32
            }
            _ => 1,
        }
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_LOG_XPGAIN;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::wrath::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    const RAW0: [u8; 26] = [ 0x00, 0x18, 0xD0, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x32, 0x00, 0x00, 0x00, 0x00, 0x32, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x80, 0x3F, 0x00, ];

    pub(crate) fn expected0() -> SMSG_LOG_XPGAIN {
        SMSG_LOG_XPGAIN {
            target: Guid::new(0x1),
            total_exp: 0x32,
            exp_type: SMSG_LOG_XPGAIN_ExperienceAwardType::Kill {
                exp_group_bonus: 1_f32,
                experience_without_rested: 0x32,
            },
            exp_includes_recruit_a_friend_bonus: false,
        }

    }

    // Generated from `wow_message_parser/wowm/world/exp/smsg_log_xpgain.wowm` line 35.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_log_xpgain0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOG_XPGAIN(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOG_XPGAIN, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/exp/smsg_log_xpgain.wowm` line 35.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_log_xpgain0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOG_XPGAIN(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOG_XPGAIN, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/exp/smsg_log_xpgain.wowm` line 35.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_log_xpgain0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOG_XPGAIN(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOG_XPGAIN, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}
