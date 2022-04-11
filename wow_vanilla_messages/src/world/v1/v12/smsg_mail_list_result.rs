use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Mail, MailError};
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_MAIL_LIST_RESULT {
    pub amount_of_mails: u8,
    pub mails: Vec<Mail>,
}

impl WorldServerMessageWrite for SMSG_MAIL_LIST_RESULT {
    const OPCODE: u16 = 0x23b;

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
impl WorldMessageBody for SMSG_MAIL_LIST_RESULT {
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
            amount_of_mails,
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

