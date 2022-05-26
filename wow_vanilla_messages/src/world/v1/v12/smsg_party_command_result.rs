use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::PartyOperation;
use crate::world::v1::v12::PartyResult;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_PARTY_COMMAND_RESULT {
    pub operation: PartyOperation,
    pub member: String,
    pub result: PartyResult,
}

impl SMSG_PARTY_COMMAND_RESULT {
    pub(crate) fn as_bytes(&self) -> Result<Vec<u8>, std::io::Error> {
        let mut w = Vec::with_capacity(self.size());
        // operation: PartyOperation
        w.write_all(&(self.operation.as_int() as u32).to_le_bytes())?;

        // member: CString
        w.write_all(self.member.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // result: PartyResult
        w.write_all(&(self.result.as_int() as u32).to_le_bytes())?;

        Ok(w)
    }
}

impl ServerMessage for SMSG_PARTY_COMMAND_RESULT {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // operation: PartyOperation
        w.write_all(&(self.operation.as_int() as u32).to_le_bytes())?;

        // member: CString
        w.write_all(self.member.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // result: PartyResult
        w.write_all(&(self.result.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x007f;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = SMSG_PARTY_COMMAND_RESULTError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // operation: PartyOperation
        let operation: PartyOperation = (crate::util::read_u32_le(r)? as u8).try_into()?;

        // member: CString
        let member = crate::util::read_c_string_to_vec(r)?;
        let member = String::from_utf8(member)?;

        // result: PartyResult
        let result: PartyResult = (crate::util::read_u32_le(r)? as u8).try_into()?;

        Ok(Self {
            operation,
            member,
            result,
        })
    }

    #[cfg(feature = "tokio")]
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
            // operation: PartyOperation
            let operation: PartyOperation = (crate::util::tokio_read_u32_le(r).await? as u8).try_into()?;

            // member: CString
            let member = crate::util::tokio_read_c_string_to_vec(r).await?;
            let member = String::from_utf8(member)?;

            // result: PartyResult
            let result: PartyResult = (crate::util::tokio_read_u32_le(r).await? as u8).try_into()?;

            Ok(Self {
                operation,
                member,
                result,
            })
        })
    }

    #[cfg(feature = "async-std")]
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
            // operation: PartyOperation
            let operation: PartyOperation = (crate::util::astd_read_u32_le(r).await? as u8).try_into()?;

            // member: CString
            let member = crate::util::astd_read_c_string_to_vec(r).await?;
            let member = String::from_utf8(member)?;

            // result: PartyResult
            let result: PartyResult = (crate::util::astd_read_u32_le(r).await? as u8).try_into()?;

            Ok(Self {
                operation,
                member,
                result,
            })
        })
    }

}

impl SMSG_PARTY_COMMAND_RESULT {
    pub fn size(&self) -> usize {
        0
        + 4 // operation: PartyOperation
        + self.member.len() + 1 // member: CString
        + 4 // result: PartyResult
    }
}

#[derive(Debug)]
pub enum SMSG_PARTY_COMMAND_RESULTError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
    Enum(crate::errors::EnumError),
}

impl std::error::Error for SMSG_PARTY_COMMAND_RESULTError {}
impl std::fmt::Display for SMSG_PARTY_COMMAND_RESULTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::Enum(e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_PARTY_COMMAND_RESULTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<crate::errors::EnumError> for SMSG_PARTY_COMMAND_RESULTError {
    fn from(e: crate::errors::EnumError) -> Self {
        Self::Enum(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_PARTY_COMMAND_RESULTError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

