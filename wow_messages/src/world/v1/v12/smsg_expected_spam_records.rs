use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/remaining.wowm:4444`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/remaining.wowm#L4444):
/// ```text
/// smsg SMSG_EXPECTED_SPAM_RECORDS = 0x332 {
///     u32 amount_of_records;
///     CString[amount_of_records] records;
/// }
/// ```
pub struct SMSG_EXPECTED_SPAM_RECORDS {
    pub amount_of_records: u32,
    pub records: Vec<String>,
}

impl WorldServerMessageWrite for SMSG_EXPECTED_SPAM_RECORDS {
    const OPCODE: u16 = 0x332;

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
impl WorldMessageBody for SMSG_EXPECTED_SPAM_RECORDS {
    type Error = SMSG_EXPECTED_SPAM_RECORDSError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // amount_of_records: u32
        let amount_of_records = crate::util::read_u32_le(r)?;

        // records: CString[amount_of_records]
        let mut records = Vec::with_capacity(amount_of_records as usize);
        for i in 0..amount_of_records {
            let s = crate::util::read_c_string_to_vec(r)?;
            records.push(String::from_utf8(s)?);
        }

        Ok(Self {
            amount_of_records,
            records,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // amount_of_records: u32
        w.write_all(&(self.records.len() as u32).to_le_bytes())?;

        // records: CString[amount_of_records]
        for i in self.records.iter() {
            w.write_all(&i.as_bytes())?;
            w.write_all(&[0])?;
        }

        Ok(())
    }
}

impl VariableSized for SMSG_EXPECTED_SPAM_RECORDS {
    fn size(&self) -> usize {
        4 // amount_of_records: u32
        + self.records.iter().fold(0, |acc, x| acc + x.len() + 1) // records: CString[amount_of_records]
    }
}

impl MaximumPossibleSized for SMSG_EXPECTED_SPAM_RECORDS {
    fn maximum_possible_size() -> usize {
        4 // amount_of_records: u32
        + 4294967295 * 256 // records: CString[amount_of_records]
    }
}

#[derive(Debug)]
pub enum SMSG_EXPECTED_SPAM_RECORDSError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for SMSG_EXPECTED_SPAM_RECORDSError {}
impl std::fmt::Display for SMSG_EXPECTED_SPAM_RECORDSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_EXPECTED_SPAM_RECORDSError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_EXPECTED_SPAM_RECORDSError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

