use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{LogoutResult, LogoutResultError};
use crate::world::v1::v12::{LogoutSpeed, LogoutSpeedError};
use crate::{WorldServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct SMSG_LOGOUT_RESPONSE {
    pub reason: LogoutResult,
    pub speed: LogoutSpeed,
}

impl WorldServerMessageWrite for SMSG_LOGOUT_RESPONSE {
    const OPCODE: u16 = 0x4c;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (Self::size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (Self::size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl MessageBody for SMSG_LOGOUT_RESPONSE {
    type Error = SMSG_LOGOUT_RESPONSEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // reason: LogoutResult
        let reason = LogoutResult::read(r)?;

        // speed: LogoutSpeed
        let speed = LogoutSpeed::read(r)?;

        Ok(Self {
            reason,
            speed,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // reason: LogoutResult
        self.reason.write(w)?;

        // speed: LogoutSpeed
        self.speed.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_LOGOUT_RESPONSE {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_LOGOUT_RESPONSE {
    fn maximum_possible_size() -> usize {
        LogoutResult::size() // reason: LogoutResult
        + LogoutSpeed::size() // speed: LogoutSpeed
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
    use crate::ReadableAndWritable;
    use std::io::Cursor;
    use super::SMSG_LOGOUT_RESPONSE;
    use crate::ConstantSized;
    use crate::world::v1::v12::LogoutResult;
    use crate::world::v1::v12::LogoutSpeed;
    use super::*;
    use super::super::*;
    use crate::world::v1::v12::opcodes::WorldServerOpcodeMessage;
    use crate::{MessageBody, WorldClientMessageWrite, WorldServerMessageWrite, OpcodeMessage};

    #[test]
    fn SMSG_LOGOUT_RESPONSE0() {
        let raw: Vec<u8> = vec![ 0x00, 0x07, 0x4C, 0x00, 0x00, 0x00, 0x00, 0x00,
             0x01, ];

        let expected = SMSG_LOGOUT_RESPONSE {
            reason: LogoutResult::SUCCESS,
            speed: LogoutSpeed::INSTANT,
        };

        let header_size = 2 + 2;
        let t = WorldServerOpcodeMessage::read_unencrypted(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            WorldServerOpcodeMessage::SMSG_LOGOUT_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOGOUT_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.reason, expected.reason);
        assert_eq!(t.speed, expected.speed);

        assert_eq!(SMSG_LOGOUT_RESPONSE::size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write_unencrypted_server(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

}
