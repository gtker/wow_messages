use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{ExperienceAwardType, ExperienceAwardTypeError};
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/exp/smsg_log_xpgain.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/exp/smsg_log_xpgain.wowm#L8):
/// ```text
/// smsg SMSG_LOG_XPGAIN = 0x1D0 {
///     Guid target_guid;
///     u32 total_exp;
///     ExperienceAwardType exp_type;
///     if (exp_type == NON_KILL) {
///         u32 experience_without_rested;
///         f32 exp_group_bonus;
///     }
/// }
/// ```
pub struct SMSG_LOG_XPGAIN {
    pub target_guid: Guid,
    pub total_exp: u32,
    pub exp_type: SMSG_LOG_XPGAINExperienceAwardType,
}

impl WorldServerMessageWrite for SMSG_LOG_XPGAIN {
    const OPCODE: u16 = 0x1d0;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (self.size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (self.size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_LOG_XPGAIN {
    type Error = SMSG_LOG_XPGAINError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // target_guid: Guid
        let target_guid = Guid::read(r)?;

        // total_exp: u32
        let total_exp = crate::util::read_u32_le(r)?;

        // exp_type: ExperienceAwardType
        let exp_type = ExperienceAwardType::read(r)?;

        let exp_type_if = match exp_type {
            ExperienceAwardType::KILL => SMSG_LOG_XPGAINExperienceAwardType::KILL,
            ExperienceAwardType::NON_KILL => {
                // experience_without_rested: u32
                let experience_without_rested = crate::util::read_u32_le(r)?;

                // exp_group_bonus: f32
                let exp_group_bonus = crate::util::read_f32_le(r)?;
                SMSG_LOG_XPGAINExperienceAwardType::NON_KILL {
                    experience_without_rested,
                    exp_group_bonus,
                }
            }
        };

        Ok(Self {
            target_guid,
            total_exp,
            exp_type: exp_type_if,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // target_guid: Guid
        self.target_guid.write(w)?;

        // total_exp: u32
        w.write_all(&self.total_exp.to_le_bytes())?;

        // exp_type: ExperienceAwardType
        self.exp_type.write(w)?;

        match &self.exp_type {
            SMSG_LOG_XPGAINExperienceAwardType::KILL => {}
            SMSG_LOG_XPGAINExperienceAwardType::NON_KILL {
                experience_without_rested,
                exp_group_bonus,
            } => {
                // experience_without_rested: u32
                w.write_all(&experience_without_rested.to_le_bytes())?;

                // exp_group_bonus: f32
                w.write_all(&exp_group_bonus.to_le_bytes())?;

            }
        }

        Ok(())
    }
}

impl VariableSized for SMSG_LOG_XPGAIN {
    fn size(&self) -> usize {
        8 // target_guid: Guid
        + 4 // total_exp: u32
        + self.exp_type.size() // exp_type: ExperienceAwardType and subfields
    }
}

impl MaximumPossibleSized for SMSG_LOG_XPGAIN {
    fn maximum_possible_size() -> usize {
        8 // target_guid: Guid
        + 4 // total_exp: u32
        + ExperienceAwardType::maximum_possible_size() // exp_type: ExperienceAwardType
    }
}

#[derive(Debug)]
pub enum SMSG_LOG_XPGAINError {
    Io(std::io::Error),
    ExperienceAwardType(ExperienceAwardTypeError),
}

impl std::error::Error for SMSG_LOG_XPGAINError {}
impl std::fmt::Display for SMSG_LOG_XPGAINError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::ExperienceAwardType(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_LOG_XPGAINError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<ExperienceAwardTypeError> for SMSG_LOG_XPGAINError {
    fn from(e: ExperienceAwardTypeError) -> Self {
        Self::ExperienceAwardType(e)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SMSG_LOG_XPGAINExperienceAwardType {
    KILL,
    NON_KILL {
        experience_without_rested: u32,
        exp_group_bonus: f32,
    },
}

impl From<&ExperienceAwardType> for SMSG_LOG_XPGAINExperienceAwardType {
    fn from(e: &ExperienceAwardType) -> Self {
        match &e {
            ExperienceAwardType::KILL => Self::KILL,
            ExperienceAwardType::NON_KILL => Self::NON_KILL {
                experience_without_rested: Default::default(),
                exp_group_bonus: Default::default(),
            },
        }
    }
}

impl From<&SMSG_LOG_XPGAINExperienceAwardType> for ExperienceAwardType {
    fn from(v: &SMSG_LOG_XPGAINExperienceAwardType) -> Self {
        match &v {
            SMSG_LOG_XPGAINExperienceAwardType::KILL => Self::KILL,
            SMSG_LOG_XPGAINExperienceAwardType::NON_KILL { .. } => Self::NON_KILL,
        }
    }
}

impl Default for SMSG_LOG_XPGAINExperienceAwardType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::KILL
    }
}

impl SMSG_LOG_XPGAINExperienceAwardType {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ExperienceAwardType = self.into();
        a.write(w)?;
        Ok(())
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ExperienceAwardType = self.into();
        a.write_u16_le(w)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ExperienceAwardType = self.into();
        a.write_u16_be(w)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ExperienceAwardType = self.into();
        a.write_u32_le(w)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ExperienceAwardType = self.into();
        a.write_u32_be(w)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ExperienceAwardType = self.into();
        a.write_u64_le(w)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ExperienceAwardType = self.into();
        a.write_u64_be(w)
    }

}

impl VariableSized for SMSG_LOG_XPGAINExperienceAwardType {
    fn size(&self) -> usize {
        match self {
            Self::KILL =>  {
                1
            }
            Self::NON_KILL  {
                experience_without_rested,
                exp_group_bonus,
            } => {
                1
                + 4 // experience_without_rested: u32
                + 4 // exp_group_bonus: f32
            }
        }
    }
}

impl MaximumPossibleSized for SMSG_LOG_XPGAINExperienceAwardType {
    fn maximum_possible_size() -> usize {
        65536 // maximum possible u16 size. TODO value.
    }
}

