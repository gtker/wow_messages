use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct MSG_QUERY_NEXT_MAIL_TIME_Server {
    pub unread_mails: f32,
}

impl ServerMessage for MSG_QUERY_NEXT_MAIL_TIME_Server {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // unread_mails: f32
        w.write_all(&self.unread_mails.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0284;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        4
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // unread_mails: f32
        let unread_mails = crate::util::read_f32_le(r)?;
        Ok(Self {
            unread_mails,
        })
    }

}

