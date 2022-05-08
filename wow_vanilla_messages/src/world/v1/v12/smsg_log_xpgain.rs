use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{ExperienceAwardType, ExperienceAwardTypeError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_LOG_XPGAIN {
    pub target_guid: Guid,
    pub total_exp: u32,
    pub exp_type: SMSG_LOG_XPGAINExperienceAwardType,
}

impl ServerMessageWrite for SMSG_LOG_XPGAIN {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_LOG_XPGAIN {
    const OPCODE: u16 = 0x01d0;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_LOG_XPGAINError;

    #[cfg(feature = "sync")]
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

    #[cfg(feature = "sync")]
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

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // target_guid: Guid
        let target_guid = Guid::tokio_read(r).await?;

        // total_exp: u32
        let total_exp = crate::util::tokio_read_u32_le(r).await?;

        // exp_type: ExperienceAwardType
        let exp_type = ExperienceAwardType::tokio_read(r).await?;

        let exp_type_if = match exp_type {
            ExperienceAwardType::KILL => SMSG_LOG_XPGAINExperienceAwardType::KILL,
            ExperienceAwardType::NON_KILL => {
                // experience_without_rested: u32
                let experience_without_rested = crate::util::tokio_read_u32_le(r).await?;

                // exp_group_bonus: f32
                let exp_group_bonus = crate::util::tokio_read_f32_le(r).await?;
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

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // target_guid: Guid
        self.target_guid.tokio_write(w).await?;

        // total_exp: u32
        w.write_all(&self.total_exp.to_le_bytes()).await?;

        // exp_type: ExperienceAwardType
        self.exp_type.tokio_write(w).await?;

        match &self.exp_type {
            SMSG_LOG_XPGAINExperienceAwardType::KILL => {}
            SMSG_LOG_XPGAINExperienceAwardType::NON_KILL {
                experience_without_rested,
                exp_group_bonus,
            } => {
                // experience_without_rested: u32
                w.write_all(&experience_without_rested.to_le_bytes()).await?;

                // exp_group_bonus: f32
                w.write_all(&exp_group_bonus.to_le_bytes()).await?;

            }
        }

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // target_guid: Guid
        let target_guid = Guid::astd_read(r).await?;

        // total_exp: u32
        let total_exp = crate::util::astd_read_u32_le(r).await?;

        // exp_type: ExperienceAwardType
        let exp_type = ExperienceAwardType::astd_read(r).await?;

        let exp_type_if = match exp_type {
            ExperienceAwardType::KILL => SMSG_LOG_XPGAINExperienceAwardType::KILL,
            ExperienceAwardType::NON_KILL => {
                // experience_without_rested: u32
                let experience_without_rested = crate::util::astd_read_u32_le(r).await?;

                // exp_group_bonus: f32
                let exp_group_bonus = crate::util::astd_read_f32_le(r).await?;
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

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // target_guid: Guid
        self.target_guid.astd_write(w).await?;

        // total_exp: u32
        w.write_all(&self.total_exp.to_le_bytes()).await?;

        // exp_type: ExperienceAwardType
        self.exp_type.astd_write(w).await?;

        match &self.exp_type {
            SMSG_LOG_XPGAINExperienceAwardType::KILL => {}
            SMSG_LOG_XPGAINExperienceAwardType::NON_KILL {
                experience_without_rested,
                exp_group_bonus,
            } => {
                // experience_without_rested: u32
                w.write_all(&experience_without_rested.to_le_bytes()).await?;

                // exp_group_bonus: f32
                w.write_all(&exp_group_bonus.to_le_bytes()).await?;

            }
        }

        Ok(())
    }

}

impl VariableSized for SMSG_LOG_XPGAIN {
    fn size(&self) -> usize {
        0
        + 8 // target_guid: Guid
        + 4 // total_exp: u32
        + self.exp_type.size() // exp_type: SMSG_LOG_XPGAINExperienceAwardType
    }
}

impl MaximumPossibleSized for SMSG_LOG_XPGAIN {
    fn maximum_possible_size() -> usize {
        0
        + 8 // target_guid: Guid
        + 4 // total_exp: u32
        + 9 // exp_type: SMSG_LOG_XPGAINExperienceAwardType
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
        exp_group_bonus: f32,
        experience_without_rested: u32,
    },
}

impl From<&ExperienceAwardType> for SMSG_LOG_XPGAINExperienceAwardType {
    fn from(e: &ExperienceAwardType) -> Self {
        match &e {
            ExperienceAwardType::KILL => Self::KILL,
            ExperienceAwardType::NON_KILL => Self::NON_KILL {
                exp_group_bonus: Default::default(),
                experience_without_rested: Default::default(),
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
    #[cfg(feature = "sync")]
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ExperienceAwardType = self.into();
        a.write(w)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ExperienceAwardType = self.into();
        a.tokio_write(w).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ExperienceAwardType = self.into();
        a.astd_write(w).await?;
        Ok(())
    }

    #[cfg(feature = "sync")]
    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ExperienceAwardType = self.into();
        a.write_u16_le(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u16_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ExperienceAwardType = self.into();
        a.tokio_write_u16_le(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u16_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ExperienceAwardType = self.into();
        a.astd_write_u16_le(w).await
    }

    #[cfg(feature = "sync")]
    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ExperienceAwardType = self.into();
        a.write_u16_be(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u16_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ExperienceAwardType = self.into();
        a.tokio_write_u16_be(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u16_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ExperienceAwardType = self.into();
        a.astd_write_u16_be(w).await
    }

    #[cfg(feature = "sync")]
    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ExperienceAwardType = self.into();
        a.write_u32_le(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u32_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ExperienceAwardType = self.into();
        a.tokio_write_u32_le(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u32_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ExperienceAwardType = self.into();
        a.astd_write_u32_le(w).await
    }

    #[cfg(feature = "sync")]
    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ExperienceAwardType = self.into();
        a.write_u32_be(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u32_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ExperienceAwardType = self.into();
        a.tokio_write_u32_be(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u32_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ExperienceAwardType = self.into();
        a.astd_write_u32_be(w).await
    }

    #[cfg(feature = "sync")]
    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ExperienceAwardType = self.into();
        a.write_u64_le(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u64_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ExperienceAwardType = self.into();
        a.tokio_write_u64_le(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u64_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ExperienceAwardType = self.into();
        a.astd_write_u64_le(w).await
    }

    #[cfg(feature = "sync")]
    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ExperienceAwardType = self.into();
        a.write_u64_be(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u64_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ExperienceAwardType = self.into();
        a.tokio_write_u64_be(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u64_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: ExperienceAwardType = self.into();
        a.astd_write_u64_be(w).await
    }

}

impl VariableSized for SMSG_LOG_XPGAINExperienceAwardType {
    fn size(&self) -> usize {
        match self {
            Self::KILL => {
                1
            }
            Self::NON_KILL {
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

impl MaximumPossibleSized for SMSG_LOG_XPGAINExperienceAwardType {
    fn maximum_possible_size() -> usize {
        65536 // maximum possible u16 size. TODO value.
    }
}

