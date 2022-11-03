use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::shared::experience_award_type_vanilla_tbc_wrath::ExperienceAwardType;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
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

impl crate::Message for SMSG_LOG_XPGAIN {
    const OPCODE: u32 = 0x01d0;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // target: Guid
        w.write_all(&self.target.guid().to_le_bytes())?;

        // total_exp: u32
        w.write_all(&self.total_exp.to_le_bytes())?;

        // exp_type: ExperienceAwardType
        w.write_all(&(self.exp_type.as_int() as u8).to_le_bytes())?;

        match &self.exp_type {
            SMSG_LOG_XPGAIN_ExperienceAwardType::Kill => {}
            SMSG_LOG_XPGAIN_ExperienceAwardType::NonKill {
                exp_group_bonus,
                experience_without_rested,
            } => {
                // experience_without_rested: u32
                w.write_all(&experience_without_rested.to_le_bytes())?;

                // exp_group_bonus: f32
                w.write_all(&exp_group_bonus.to_le_bytes())?;

            }
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size < 13 || body_size > 21 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01D0, size: body_size as u32 });
        }

        // target: Guid
        let target = Guid::read(r)?;

        // total_exp: u32
        let total_exp = crate::util::read_u32_le(r)?;

        // exp_type: ExperienceAwardType
        let exp_type: ExperienceAwardType = crate::util::read_u8_le(r)?.try_into()?;

        let exp_type_if = match exp_type {
            ExperienceAwardType::Kill => SMSG_LOG_XPGAIN_ExperienceAwardType::Kill,
            ExperienceAwardType::NonKill => {
                // experience_without_rested: u32
                let experience_without_rested = crate::util::read_u32_le(r)?;

                // exp_group_bonus: f32
                let exp_group_bonus = crate::util::read_f32_le(r)?;
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
impl crate::world::vanilla::ServerMessage for SMSG_LOG_XPGAIN {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_LOG_XPGAIN {}

impl SMSG_LOG_XPGAIN {
    pub(crate) fn size(&self) -> usize {
        8 // target: Guid
        + 4 // total_exp: u32
        + self.exp_type.size() // exp_type: SMSG_LOG_XPGAIN_ExperienceAwardType
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
    pub(crate) fn size(&self) -> usize {
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

