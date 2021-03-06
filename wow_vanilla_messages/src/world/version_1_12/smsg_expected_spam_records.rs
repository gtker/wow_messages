use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_expected_spam_records.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_expected_spam_records.wowm#L3):
/// ```text
/// smsg SMSG_EXPECTED_SPAM_RECORDS = 0x0332 {
///     u32 amount_of_records;
///     CString[amount_of_records] records;
/// }
/// ```
pub struct SMSG_EXPECTED_SPAM_RECORDS {
    pub records: Vec<String>,
}

impl ServerMessage for SMSG_EXPECTED_SPAM_RECORDS {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // amount_of_records: u32
        w.write_all(&(self.records.len() as u32).to_le_bytes())?;

        // records: CString[amount_of_records]
        for i in self.records.iter() {
            w.write_all(i.as_bytes())?;
            w.write_all(&[0])?;
        }

        Ok(())
    }
    const OPCODE: u16 = 0x0332;

    fn server_size(&self) -> u16 {
        (self.size() + 4) as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
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
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_records: u32
        + self.records.iter().fold(0, |acc, x| acc + x.len() + 1) // records: CString[amount_of_records]
    }
}

