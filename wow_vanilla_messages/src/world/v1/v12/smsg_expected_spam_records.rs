use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_EXPECTED_SPAM_RECORDS {
    pub records: Vec<String>,
}

impl ServerMessage for SMSG_EXPECTED_SPAM_RECORDS {
    fn as_bytes(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // amount_of_records: u32
        w.write_all(&(self.records.len() as u32).to_le_bytes())?;

        // records: CString[amount_of_records]
        for i in self.records.iter() {
            w.write_all(&i.as_bytes())?;
            w.write_all(&[0])?;
        }

        Ok(())
    }
    const OPCODE: u16 = 0x0332;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    type Error = crate::errors::ParseError;

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
            records,
        })
    }

}

impl SMSG_EXPECTED_SPAM_RECORDS {
    pub fn size(&self) -> usize {
        0
        + 4 // amount_of_records: u32
        + self.records.iter().fold(0, |acc, x| acc + x.len() + 1) // records: CString[amount_of_records]
    }
}

