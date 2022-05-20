use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{LogoutResult, LogoutResultError};
use crate::world::v1::v12::{LogoutSpeed, LogoutSpeedError};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_LOGOUT_RESPONSE {
    pub reason: LogoutResult,
    pub speed: LogoutSpeed,
}

impl ServerMessageWrite for SMSG_LOGOUT_RESPONSE {}

impl SMSG_LOGOUT_RESPONSE {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 5], std::io::Error> {
        let mut array_w = [0u8; 5];
        let mut w = array_w.as_mut_slice();
        // reason: LogoutResult
        w.write_all(&(self.reason.as_int() as u32).to_le_bytes())?;

        // speed: LogoutSpeed
        w.write_all(&(self.speed.as_int() as u8).to_le_bytes())?;

        Ok(array_w)
    }
}

impl MessageBody for SMSG_LOGOUT_RESPONSE {
    const OPCODE: u16 = 0x004c;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = SMSG_LOGOUT_RESPONSEError;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // reason: LogoutResult
        let reason: LogoutResult = crate::util::read_u32_le(r)?.try_into()?;

        // speed: LogoutSpeed
        let speed: LogoutSpeed = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            reason,
            speed,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let inner = self.as_bytes()?;
        w.write_all(&inner)
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
            // reason: LogoutResult
            let reason: LogoutResult = crate::util::tokio_read_u32_le(r).await?.try_into()?;

            // speed: LogoutSpeed
            let speed: LogoutSpeed = crate::util::tokio_read_u8_le(r).await?.try_into()?;

            Ok(Self {
                reason,
                speed,
            })
        })
    }

    #[cfg(feature = "tokio")]
    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let inner = self.as_bytes()?;
            w.write_all(&inner).await
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
            // reason: LogoutResult
            let reason: LogoutResult = crate::util::astd_read_u32_le(r).await?.try_into()?;

            // speed: LogoutSpeed
            let speed: LogoutSpeed = crate::util::astd_read_u8_le(r).await?.try_into()?;

            Ok(Self {
                reason,
                speed,
            })
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let inner = self.as_bytes()?;
            w.write_all(&inner).await
        })
    }

}

impl SMSG_LOGOUT_RESPONSE {
    pub(crate) fn size() -> usize {
        0
        + 4 // reason: LogoutResult
        + 1 // speed: LogoutSpeed
    }
}

#[derive(Debug)]
pub enum SMSG_LOGOUT_RESPONSEError {
    Io(std::io::Error),
    LogoutResult(LogoutResultError),
    LogoutSpeed(LogoutSpeedError),
}

impl std::error::Error for SMSG_LOGOUT_RESPONSEError {}
impl std::fmt::Display for SMSG_LOGOUT_RESPONSEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::LogoutResult(i) => i.fmt(f),
            Self::LogoutSpeed(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_LOGOUT_RESPONSEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<LogoutResultError> for SMSG_LOGOUT_RESPONSEError {
    fn from(e: LogoutResultError) -> Self {
        Self::LogoutResult(e)
    }
}

impl From<LogoutSpeedError> for SMSG_LOGOUT_RESPONSEError {
    fn from(e: LogoutSpeedError) -> Self {
        Self::LogoutSpeed(e)
    }
}

#[cfg(test)]
mod test {
    use super::SMSG_LOGOUT_RESPONSE;
    use crate::world::v1::v12::LogoutResult;
    use crate::world::v1::v12::LogoutSpeed;
    use super::*;
    use super::super::*;
    use crate::world::v1::v12::opcodes::ServerOpcodeMessage;
    use crate::{MessageBody, ClientMessageWrite, ServerMessageWrite, OpcodeMessage};

    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_LOGOUT_RESPONSE0() {
        let raw: Vec<u8> = vec![ 0x00, 0x07, 0x4C, 0x00, 0x00, 0x00, 0x00, 0x00,
             0x01, ];

        let expected = SMSG_LOGOUT_RESPONSE {
            reason: LogoutResult::SUCCESS,
            speed: LogoutSpeed::INSTANT,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&raw)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOGOUT_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOGOUT_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.reason, expected.reason);
        assert_eq!(t.speed, expected.speed);

        assert_eq!(5 + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_LOGOUT_RESPONSE0() {
        let raw: Vec<u8> = vec![ 0x00, 0x07, 0x4C, 0x00, 0x00, 0x00, 0x00, 0x00,
             0x01, ];

        let expected = SMSG_LOGOUT_RESPONSE {
            reason: LogoutResult::SUCCESS,
            speed: LogoutSpeed::INSTANT,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOGOUT_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOGOUT_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.reason, expected.reason);
        assert_eq!(t.speed, expected.speed);

        assert_eq!(5 + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_LOGOUT_RESPONSE0() {
        let raw: Vec<u8> = vec![ 0x00, 0x07, 0x4C, 0x00, 0x00, 0x00, 0x00, 0x00,
             0x01, ];

        let expected = SMSG_LOGOUT_RESPONSE {
            reason: LogoutResult::SUCCESS,
            speed: LogoutSpeed::INSTANT,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOGOUT_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOGOUT_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.reason, expected.reason);
        assert_eq!(t.speed, expected.speed);

        assert_eq!(5 + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, raw);
    }

}
