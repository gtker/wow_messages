use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{ExperienceAwardType, ExperienceAwardTypeError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, VariableSized};
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
        let exp_type: ExperienceAwardType = crate::util::read_u8_le(r)?.try_into()?;

        let exp_type_if = match exp_type {
            ExperienceAwardType::KILL => SMSG_LOG_XPGAINExperienceAwardType::KILL,
            ExperienceAwardType::NON_KILL => {
                // experience_without_rested: u32
                let experience_without_rested = crate::util::read_u32_le(r)?;

                // exp_group_bonus: f32
                let exp_group_bonus = crate::util::read_f32_le(r)?;
                SMSG_LOG_XPGAINExperienceAwardType::NON_KILL {
                    exp_group_bonus,
                    experience_without_rested,
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
        w.write_all(&(self.exp_type.as_int() as u8).to_le_bytes())?;

        match &self.exp_type {
            SMSG_LOG_XPGAINExperienceAwardType::KILL => {}
            SMSG_LOG_XPGAINExperienceAwardType::NON_KILL {
                exp_group_bonus,
                experience_without_rested,
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
    fn tokio_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // target_guid: Guid
            let target_guid = Guid::tokio_read(r).await?;

            // total_exp: u32
            let total_exp = crate::util::tokio_read_u32_le(r).await?;

            // exp_type: ExperienceAwardType
            let exp_type: ExperienceAwardType = crate::util::tokio_read_u8_le(r).await?.try_into()?;

            let exp_type_if = match exp_type {
                ExperienceAwardType::KILL => SMSG_LOG_XPGAINExperienceAwardType::KILL,
                ExperienceAwardType::NON_KILL => {
                    // experience_without_rested: u32
                    let experience_without_rested = crate::util::tokio_read_u32_le(r).await?;

                    // exp_group_bonus: f32
                    let exp_group_bonus = crate::util::tokio_read_f32_le(r).await?;
                    SMSG_LOG_XPGAINExperienceAwardType::NON_KILL {
                        exp_group_bonus,
                        experience_without_rested,
                    }
                }
            };

            Ok(Self {
                target_guid,
                total_exp,
                exp_type: exp_type_if,
            })
        })
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // target_guid: Guid
            self.target_guid.tokio_write(w).await?;

            // total_exp: u32
            w.write_all(&self.total_exp.to_le_bytes()).await?;

            // exp_type: ExperienceAwardType
            w.write_all(&(self.exp_type.as_int() as u8).to_le_bytes()).await?;

            match &self.exp_type {
                SMSG_LOG_XPGAINExperienceAwardType::KILL => {}
                SMSG_LOG_XPGAINExperienceAwardType::NON_KILL {
                    exp_group_bonus,
                    experience_without_rested,
                } => {
                    // experience_without_rested: u32
                    w.write_all(&experience_without_rested.to_le_bytes()).await?;

                    // exp_group_bonus: f32
                    w.write_all(&exp_group_bonus.to_le_bytes()).await?;

                }
            }

            Ok(())
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // target_guid: Guid
            let target_guid = Guid::astd_read(r).await?;

            // total_exp: u32
            let total_exp = crate::util::astd_read_u32_le(r).await?;

            // exp_type: ExperienceAwardType
            let exp_type: ExperienceAwardType = crate::util::astd_read_u8_le(r).await?.try_into()?;

            let exp_type_if = match exp_type {
                ExperienceAwardType::KILL => SMSG_LOG_XPGAINExperienceAwardType::KILL,
                ExperienceAwardType::NON_KILL => {
                    // experience_without_rested: u32
                    let experience_without_rested = crate::util::astd_read_u32_le(r).await?;

                    // exp_group_bonus: f32
                    let exp_group_bonus = crate::util::astd_read_f32_le(r).await?;
                    SMSG_LOG_XPGAINExperienceAwardType::NON_KILL {
                        exp_group_bonus,
                        experience_without_rested,
                    }
                }
            };

            Ok(Self {
                target_guid,
                total_exp,
                exp_type: exp_type_if,
            })
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // target_guid: Guid
            self.target_guid.astd_write(w).await?;

            // total_exp: u32
            w.write_all(&self.total_exp.to_le_bytes()).await?;

            // exp_type: ExperienceAwardType
            w.write_all(&(self.exp_type.as_int() as u8).to_le_bytes()).await?;

            match &self.exp_type {
                SMSG_LOG_XPGAINExperienceAwardType::KILL => {}
                SMSG_LOG_XPGAINExperienceAwardType::NON_KILL {
                    exp_group_bonus,
                    experience_without_rested,
                } => {
                    // experience_without_rested: u32
                    w.write_all(&experience_without_rested.to_le_bytes()).await?;

                    // exp_group_bonus: f32
                    w.write_all(&exp_group_bonus.to_le_bytes()).await?;

                }
            }

            Ok(())
        })
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

impl Default for SMSG_LOG_XPGAINExperienceAwardType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::KILL
    }
}

impl SMSG_LOG_XPGAINExperienceAwardType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::KILL => 0,
            Self::NON_KILL { .. } => 1,
        }
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
        9
    }
}

