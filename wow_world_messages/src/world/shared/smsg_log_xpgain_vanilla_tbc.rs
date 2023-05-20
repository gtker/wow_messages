use std::io::{Read, Write};

use crate::Guid;
use wow_world_base::shared::experience_award_type_vanilla_tbc_wrath::ExperienceAwardType;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/exp/smsg_log_xpgain.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/exp/smsg_log_xpgain.wowm#L8):
/// ```text
/// smsg SMSG_LOG_XPGAIN = 0x01D0 {
///     Guid target;
///     u32 total_exp;
///     ExperienceAwardType exp_type;
///     if (exp_type == NON_KILL) {
///         u32 experience_without_rested;
///         f32 exp_group_bonus;
///     }
/// }
/// ```
pub struct SMSG_LOG_XPGAIN {
    pub target: Guid,
    pub total_exp: u32,
    pub exp_type: SMSG_LOG_XPGAIN_ExperienceAwardType,
}

impl crate::private::Sealed for SMSG_LOG_XPGAIN {}
impl crate::Message for SMSG_LOG_XPGAIN {
    const OPCODE: u32 = 0x01d0;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // target: Guid
        w.write_all(&self.target.guid().to_le_bytes())?;

        // total_exp: u32
        w.write_all(&self.total_exp.to_le_bytes())?;

        // exp_type: ExperienceAwardType
        w.write_all(&(self.exp_type.as_int().to_le_bytes()))?;

        match &self.exp_type {
            SMSG_LOG_XPGAIN_ExperienceAwardType::NonKill {
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

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(13..=21).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01D0, size: body_size });
        }

        // target: Guid
        let target = Guid::read(&mut r)?;

        // total_exp: u32
        let total_exp = crate::util::read_u32_le(&mut r)?;

        // exp_type: ExperienceAwardType
        let exp_type: ExperienceAwardType = crate::util::read_u8_le(&mut r)?.try_into()?;

        let exp_type_if = match exp_type {
            ExperienceAwardType::Kill => SMSG_LOG_XPGAIN_ExperienceAwardType::Kill,
            ExperienceAwardType::NonKill => {
                // experience_without_rested: u32
                let experience_without_rested = crate::util::read_u32_le(&mut r)?;

                // exp_group_bonus: f32
                let exp_group_bonus = crate::util::read_f32_le(&mut r)?;

                SMSG_LOG_XPGAIN_ExperienceAwardType::NonKill {
                    exp_group_bonus,
                    experience_without_rested,
                }
            }
        };

        Ok(Self {
            target,
            total_exp,
            exp_type: exp_type_if,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_LOG_XPGAIN {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_LOG_XPGAIN {}

impl SMSG_LOG_XPGAIN {
    pub(crate) const fn size(&self) -> usize {
        8 // target: Guid
        + 4 // total_exp: u32
        + self.exp_type.size() // exp_type: SMSG_LOG_XPGAIN_ExperienceAwardType
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum SMSG_LOG_XPGAIN_ExperienceAwardType {
    Kill,
    NonKill {
        exp_group_bonus: f32,
        experience_without_rested: u32,
    },
}

impl Default for SMSG_LOG_XPGAIN_ExperienceAwardType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Kill
    }
}

impl SMSG_LOG_XPGAIN_ExperienceAwardType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Kill => 0,
            Self::NonKill { .. } => 1,
        }
    }

}

impl SMSG_LOG_XPGAIN_ExperienceAwardType {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::Kill => {
                1
            }
            Self::NonKill {
                exp_group_bonus,
                experience_without_rested,
            } => {
                1
                + 4 // exp_group_bonus: f32
                + 4 // experience_without_rested: u32
            }
        }
    }
}

