use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Mail, MailError};
use crate::{ServerMessageWrite, MessageBody};
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
pub struct SMSG_MAIL_LIST_RESULT {
    pub mails: Vec<Mail>,
}

impl ServerMessageWrite for SMSG_MAIL_LIST_RESULT {
    const OPCODE: u16 = 0x23b;

    fn size_without_size_field(&self) -> u16 {
        self.size() as u16
    }

}

impl MessageBody for SMSG_MAIL_LIST_RESULT {
    type Error = SMSG_MAIL_LIST_RESULTError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // amount_of_mails: u8
        let amount_of_mails = crate::util::read_u8_le(r)?;

        // mails: Mail[amount_of_mails]
        let mut mails = Vec::with_capacity(amount_of_mails as usize);
        for i in 0..amount_of_mails {
            mails.push(Mail::read(r)?);
        }

        Ok(Self {
            mails,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // amount_of_mails: u8
        w.write_all(&(self.mails.len() as u8).to_le_bytes())?;

        // mails: Mail[amount_of_mails]
        for i in self.mails.iter() {
            i.write(w)?;
        }

        Ok(())
    }
}

impl VariableSized for SMSG_MAIL_LIST_RESULT {
    fn size(&self) -> usize {
        1 // amount_of_mails: u8
        + self.mails.iter().fold(0, |acc, x| acc + x.size()) // mails: Mail[amount_of_mails]
    }
}

impl MaximumPossibleSized for SMSG_MAIL_LIST_RESULT {
    fn maximum_possible_size() -> usize {
        1 // amount_of_mails: u8
        + 255 * Mail::maximum_possible_size() // mails: Mail[amount_of_mails]
    }
}

#[derive(Debug)]
pub enum SMSG_MAIL_LIST_RESULTError {
    Io(std::io::Error),
    Mail(MailError),
}

impl std::error::Error for SMSG_MAIL_LIST_RESULTError {}
impl std::fmt::Display for SMSG_MAIL_LIST_RESULTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Mail(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_MAIL_LIST_RESULTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<MailError> for SMSG_MAIL_LIST_RESULTError {
    fn from(e: MailError) -> Self {
        Self::Mail(e)
    }
}

