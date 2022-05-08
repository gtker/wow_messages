use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{GmTicketEscalationStatus, GmTicketEscalationStatusError};
use crate::world::v1::v12::{GmTicketStatus, GmTicketStatusError};
use crate::world::v1::v12::{GmTicketType, GmTicketTypeError};
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
pub struct SMSG_GMTICKET_GETTICKET {
    pub status: SMSG_GMTICKET_GETTICKETGmTicketStatus,
}

impl ServerMessageWrite for SMSG_GMTICKET_GETTICKET {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_GMTICKET_GETTICKET {
    const OPCODE: u16 = 0x0212;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_GMTICKET_GETTICKETError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // status: GmTicketStatus
        let status = GmTicketStatus::read(r)?;

        let status_if = match status {
            GmTicketStatus::DBERROR => SMSG_GMTICKET_GETTICKETGmTicketStatus::DBERROR,
            GmTicketStatus::HASTEXT => {
                // text: CString
                let text = crate::util::read_c_string_to_vec(r)?;
                let text = String::from_utf8(text)?;

                // ticket_type: GmTicketType
                let ticket_type = GmTicketType::read(r)?;

                // days_since_ticket_creation: f32
                let days_since_ticket_creation = crate::util::read_f32_le(r)?;
                // days_since_oldest_ticket_creation: f32
                let days_since_oldest_ticket_creation = crate::util::read_f32_le(r)?;
                // days_since_last_updated: f32
                let days_since_last_updated = crate::util::read_f32_le(r)?;
                // escalation_status: GmTicketEscalationStatus
                let escalation_status = GmTicketEscalationStatus::read(r)?;

                // read_by_gm: u8
                let read_by_gm = crate::util::read_u8_le(r)?;

                SMSG_GMTICKET_GETTICKETGmTicketStatus::HASTEXT {
                    text,
                    ticket_type,
                    days_since_ticket_creation,
                    days_since_oldest_ticket_creation,
                    days_since_last_updated,
                    escalation_status,
                    read_by_gm,
                }
            }
            GmTicketStatus::DEFAULT => SMSG_GMTICKET_GETTICKETGmTicketStatus::DEFAULT,
        };

        Ok(Self {
            status: status_if,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // status: GmTicketStatus
        self.status.write(w)?;

        match &self.status {
            SMSG_GMTICKET_GETTICKETGmTicketStatus::DBERROR => {}
            SMSG_GMTICKET_GETTICKETGmTicketStatus::HASTEXT {
                text,
                ticket_type,
                days_since_ticket_creation,
                days_since_oldest_ticket_creation,
                days_since_last_updated,
                escalation_status,
                read_by_gm,
            } => {
                // text: CString
                w.write_all(text.as_bytes())?;
                // Null terminator
                w.write_all(&[0])?;

                // ticket_type: GmTicketType
                ticket_type.write(w)?;

                // days_since_ticket_creation: f32
                w.write_all(&days_since_ticket_creation.to_le_bytes())?;

                // days_since_oldest_ticket_creation: f32
                w.write_all(&days_since_oldest_ticket_creation.to_le_bytes())?;

                // days_since_last_updated: f32
                w.write_all(&days_since_last_updated.to_le_bytes())?;

                // escalation_status: GmTicketEscalationStatus
                escalation_status.write(w)?;

                // read_by_gm: u8
                w.write_all(&read_by_gm.to_le_bytes())?;

            }
            SMSG_GMTICKET_GETTICKETGmTicketStatus::DEFAULT => {}
        }

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // status: GmTicketStatus
        let status = GmTicketStatus::tokio_read(r).await?;

        let status_if = match status {
            GmTicketStatus::DBERROR => SMSG_GMTICKET_GETTICKETGmTicketStatus::DBERROR,
            GmTicketStatus::HASTEXT => {
                // text: CString
                let text = crate::util::tokio_read_c_string_to_vec(r).await?;
                let text = String::from_utf8(text)?;

                // ticket_type: GmTicketType
                let ticket_type = GmTicketType::tokio_read(r).await?;

                // days_since_ticket_creation: f32
                let days_since_ticket_creation = crate::util::tokio_read_f32_le(r).await?;
                // days_since_oldest_ticket_creation: f32
                let days_since_oldest_ticket_creation = crate::util::tokio_read_f32_le(r).await?;
                // days_since_last_updated: f32
                let days_since_last_updated = crate::util::tokio_read_f32_le(r).await?;
                // escalation_status: GmTicketEscalationStatus
                let escalation_status = GmTicketEscalationStatus::tokio_read(r).await?;

                // read_by_gm: u8
                let read_by_gm = crate::util::tokio_read_u8_le(r).await?;

                SMSG_GMTICKET_GETTICKETGmTicketStatus::HASTEXT {
                    text,
                    ticket_type,
                    days_since_ticket_creation,
                    days_since_oldest_ticket_creation,
                    days_since_last_updated,
                    escalation_status,
                    read_by_gm,
                }
            }
            GmTicketStatus::DEFAULT => SMSG_GMTICKET_GETTICKETGmTicketStatus::DEFAULT,
        };

        Ok(Self {
            status: status_if,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // status: GmTicketStatus
        self.status.tokio_write(w).await?;

        match &self.status {
            SMSG_GMTICKET_GETTICKETGmTicketStatus::DBERROR => {}
            SMSG_GMTICKET_GETTICKETGmTicketStatus::HASTEXT {
                text,
                ticket_type,
                days_since_ticket_creation,
                days_since_oldest_ticket_creation,
                days_since_last_updated,
                escalation_status,
                read_by_gm,
            } => {
                // text: CString
                w.write_all(text.as_bytes()).await?;
                // Null terminator
                w.write_all(&[0]).await?;

                // ticket_type: GmTicketType
                ticket_type.tokio_write(w).await?;

                // days_since_ticket_creation: f32
                w.write_all(&days_since_ticket_creation.to_le_bytes()).await?;

                // days_since_oldest_ticket_creation: f32
                w.write_all(&days_since_oldest_ticket_creation.to_le_bytes()).await?;

                // days_since_last_updated: f32
                w.write_all(&days_since_last_updated.to_le_bytes()).await?;

                // escalation_status: GmTicketEscalationStatus
                escalation_status.tokio_write(w).await?;

                // read_by_gm: u8
                w.write_all(&read_by_gm.to_le_bytes()).await?;

            }
            SMSG_GMTICKET_GETTICKETGmTicketStatus::DEFAULT => {}
        }

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // status: GmTicketStatus
        let status = GmTicketStatus::astd_read(r).await?;

        let status_if = match status {
            GmTicketStatus::DBERROR => SMSG_GMTICKET_GETTICKETGmTicketStatus::DBERROR,
            GmTicketStatus::HASTEXT => {
                // text: CString
                let text = crate::util::astd_read_c_string_to_vec(r).await?;
                let text = String::from_utf8(text)?;

                // ticket_type: GmTicketType
                let ticket_type = GmTicketType::astd_read(r).await?;

                // days_since_ticket_creation: f32
                let days_since_ticket_creation = crate::util::astd_read_f32_le(r).await?;
                // days_since_oldest_ticket_creation: f32
                let days_since_oldest_ticket_creation = crate::util::astd_read_f32_le(r).await?;
                // days_since_last_updated: f32
                let days_since_last_updated = crate::util::astd_read_f32_le(r).await?;
                // escalation_status: GmTicketEscalationStatus
                let escalation_status = GmTicketEscalationStatus::astd_read(r).await?;

                // read_by_gm: u8
                let read_by_gm = crate::util::astd_read_u8_le(r).await?;

                SMSG_GMTICKET_GETTICKETGmTicketStatus::HASTEXT {
                    text,
                    ticket_type,
                    days_since_ticket_creation,
                    days_since_oldest_ticket_creation,
                    days_since_last_updated,
                    escalation_status,
                    read_by_gm,
                }
            }
            GmTicketStatus::DEFAULT => SMSG_GMTICKET_GETTICKETGmTicketStatus::DEFAULT,
        };

        Ok(Self {
            status: status_if,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // status: GmTicketStatus
        self.status.astd_write(w).await?;

        match &self.status {
            SMSG_GMTICKET_GETTICKETGmTicketStatus::DBERROR => {}
            SMSG_GMTICKET_GETTICKETGmTicketStatus::HASTEXT {
                text,
                ticket_type,
                days_since_ticket_creation,
                days_since_oldest_ticket_creation,
                days_since_last_updated,
                escalation_status,
                read_by_gm,
            } => {
                // text: CString
                w.write_all(text.as_bytes()).await?;
                // Null terminator
                w.write_all(&[0]).await?;

                // ticket_type: GmTicketType
                ticket_type.astd_write(w).await?;

                // days_since_ticket_creation: f32
                w.write_all(&days_since_ticket_creation.to_le_bytes()).await?;

                // days_since_oldest_ticket_creation: f32
                w.write_all(&days_since_oldest_ticket_creation.to_le_bytes()).await?;

                // days_since_last_updated: f32
                w.write_all(&days_since_last_updated.to_le_bytes()).await?;

                // escalation_status: GmTicketEscalationStatus
                escalation_status.astd_write(w).await?;

                // read_by_gm: u8
                w.write_all(&read_by_gm.to_le_bytes()).await?;

            }
            SMSG_GMTICKET_GETTICKETGmTicketStatus::DEFAULT => {}
        }

        Ok(())
    }

}

impl VariableSized for SMSG_GMTICKET_GETTICKET {
    fn size(&self) -> usize {
        0
        + self.status.size() // status: SMSG_GMTICKET_GETTICKETGmTicketStatus
    }
}

impl MaximumPossibleSized for SMSG_GMTICKET_GETTICKET {
    fn maximum_possible_size() -> usize {
        0
        + 275 // status: SMSG_GMTICKET_GETTICKETGmTicketStatus
    }
}

#[derive(Debug)]
pub enum SMSG_GMTICKET_GETTICKETError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
    GmTicketEscalationStatus(GmTicketEscalationStatusError),
    GmTicketStatus(GmTicketStatusError),
    GmTicketType(GmTicketTypeError),
}

impl std::error::Error for SMSG_GMTICKET_GETTICKETError {}
impl std::fmt::Display for SMSG_GMTICKET_GETTICKETError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::GmTicketEscalationStatus(i) => i.fmt(f),
            Self::GmTicketStatus(i) => i.fmt(f),
            Self::GmTicketType(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_GMTICKET_GETTICKETError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_GMTICKET_GETTICKETError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<GmTicketEscalationStatusError> for SMSG_GMTICKET_GETTICKETError {
    fn from(e: GmTicketEscalationStatusError) -> Self {
        Self::GmTicketEscalationStatus(e)
    }
}

impl From<GmTicketStatusError> for SMSG_GMTICKET_GETTICKETError {
    fn from(e: GmTicketStatusError) -> Self {
        Self::GmTicketStatus(e)
    }
}

impl From<GmTicketTypeError> for SMSG_GMTICKET_GETTICKETError {
    fn from(e: GmTicketTypeError) -> Self {
        Self::GmTicketType(e)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum SMSG_GMTICKET_GETTICKETGmTicketStatus {
    DBERROR,
    HASTEXT {
        text: String,
        ticket_type: GmTicketType,
        days_since_ticket_creation: f32,
        days_since_oldest_ticket_creation: f32,
        days_since_last_updated: f32,
        escalation_status: GmTicketEscalationStatus,
        read_by_gm: u8,
    },
    DEFAULT,
}

impl From<&GmTicketStatus> for SMSG_GMTICKET_GETTICKETGmTicketStatus {
    fn from(e: &GmTicketStatus) -> Self {
        match &e {
            GmTicketStatus::DBERROR => Self::DBERROR,
            GmTicketStatus::HASTEXT => Self::HASTEXT {
                text: Default::default(),
                ticket_type: Default::default(),
                days_since_ticket_creation: Default::default(),
                days_since_oldest_ticket_creation: Default::default(),
                days_since_last_updated: Default::default(),
                escalation_status: Default::default(),
                read_by_gm: Default::default(),
            },
            GmTicketStatus::DEFAULT => Self::DEFAULT,
        }
    }
}

impl From<&SMSG_GMTICKET_GETTICKETGmTicketStatus> for GmTicketStatus {
    fn from(v: &SMSG_GMTICKET_GETTICKETGmTicketStatus) -> Self {
        match &v {
            SMSG_GMTICKET_GETTICKETGmTicketStatus::DBERROR => Self::DBERROR,
            SMSG_GMTICKET_GETTICKETGmTicketStatus::HASTEXT { .. } => Self::HASTEXT,
            SMSG_GMTICKET_GETTICKETGmTicketStatus::DEFAULT => Self::DEFAULT,
        }
    }
}

impl Default for SMSG_GMTICKET_GETTICKETGmTicketStatus {
    fn default() -> Self {
        // First enumerator without any fields
        Self::DBERROR
    }
}

impl SMSG_GMTICKET_GETTICKETGmTicketStatus {
    #[cfg(feature = "sync")]
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: GmTicketStatus = self.into();
        a.write(w)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: GmTicketStatus = self.into();
        a.tokio_write(w).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: GmTicketStatus = self.into();
        a.astd_write(w).await?;
        Ok(())
    }

    #[cfg(feature = "sync")]
    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: GmTicketStatus = self.into();
        a.write_u32_be(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u32_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: GmTicketStatus = self.into();
        a.tokio_write_u32_be(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u32_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: GmTicketStatus = self.into();
        a.astd_write_u32_be(w).await
    }

    #[cfg(feature = "sync")]
    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: GmTicketStatus = self.into();
        a.write_u64_le(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u64_le<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: GmTicketStatus = self.into();
        a.tokio_write_u64_le(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u64_le<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: GmTicketStatus = self.into();
        a.astd_write_u64_le(w).await
    }

    #[cfg(feature = "sync")]
    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: GmTicketStatus = self.into();
        a.write_u64_be(w)
    }

    #[cfg(feature = "async_tokio")]
    pub async fn tokio_write_u64_be<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: GmTicketStatus = self.into();
        a.tokio_write_u64_be(w).await
    }

    #[cfg(feature = "async_std")]
    pub async fn astd_write_u64_be<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: GmTicketStatus = self.into();
        a.astd_write_u64_be(w).await
    }

}

impl VariableSized for SMSG_GMTICKET_GETTICKETGmTicketStatus {
    fn size(&self) -> usize {
        match self {
            Self::DBERROR => {
                4
            }
            Self::HASTEXT {
                days_since_last_updated,
                days_since_oldest_ticket_creation,
                days_since_ticket_creation,
                escalation_status,
                read_by_gm,
                text,
                ticket_type,
            } => {
                4
                + 4 // days_since_last_updated: f32
                + 4 // days_since_oldest_ticket_creation: f32
                + 4 // days_since_ticket_creation: f32
                + 1 // escalation_status: GmTicketEscalationStatus
                + 1 // read_by_gm: u8
                + text.len() + 1 // text: CString
                + 1 // ticket_type: GmTicketType
            }
            Self::DEFAULT => {
                4
            }
        }
    }
}

impl MaximumPossibleSized for SMSG_GMTICKET_GETTICKETGmTicketStatus {
    fn maximum_possible_size() -> usize {
        65536 // maximum possible u16 size. TODO value.
    }
}

