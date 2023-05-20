use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Not implemented in Wrath or TBC emus. Only implemented in cmangos.
///
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

impl crate::private::Sealed for SMSG_EXPECTED_SPAM_RECORDS {}
impl crate::Message for SMSG_EXPECTED_SPAM_RECORDS {
    const OPCODE: u32 = 0x0332;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // amount_of_records: u32
        w.write_all(&(self.records.len() as u32).to_le_bytes())?;

        // records: CString[amount_of_records]
        for i in self.records.iter() {
            w.write_all(i.as_bytes())?;
            w.write_all(&[0])?;
        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(4..=65535).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0332, size: body_size });
        }

        // amount_of_records: u32
        let amount_of_records = crate::util::read_u32_le(&mut r)?;

        // records: CString[amount_of_records]
        let records = {
            let mut records = Vec::with_capacity(amount_of_records as usize);
            for i in 0..amount_of_records {
                let s = crate::util::read_c_string_to_vec(&mut r)?;
                records.push(String::from_utf8(s)?);
            }
            records
        };

        Ok(Self {
            records,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_EXPECTED_SPAM_RECORDS {}

impl SMSG_EXPECTED_SPAM_RECORDS {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_records: u32
        + self.records.iter().fold(0, |acc, x| acc + x.len() + 1) // records: CString[amount_of_records]
    }
}

