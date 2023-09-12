use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Not implemented in Wrath or TBC emus. Only implemented in cmangos.
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
impl SMSG_EXPECTED_SPAM_RECORDS {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(4..=65535).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // amount_of_records: u32
        let amount_of_records = crate::util::read_u32_le(&mut r)?;

        // records: CString[amount_of_records]
        let records = {
            let mut records = Vec::with_capacity(amount_of_records as usize);
            for _ in 0..amount_of_records {
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

impl crate::Message for SMSG_EXPECTED_SPAM_RECORDS {
    const OPCODE: u32 = 0x0332;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_EXPECTED_SPAM_RECORDS"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_EXPECTED_SPAM_RECORDS {{").unwrap();
        // Members
        writeln!(s, "    amount_of_records = {};", self.records.len()).unwrap();
        write!(s, "    records = [").unwrap();
        for v in self.records.as_slice() {
            write!(s, "\"{v}\", ").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 818_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_records", "    ");
        if !self.records.is_empty() {
            writeln!(s, "    /* records: CString[amount_of_records] start */").unwrap();
            for (i, v) in self.records.iter().enumerate() {
                crate::util::write_bytes(&mut s, &mut bytes, v.len() + 1, &format!("records {i}"), "    ");
            }
            writeln!(s, "    /* records: CString[amount_of_records] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // amount_of_records: u32
        w.write_all(&(self.records.len() as u32).to_le_bytes())?;

        // records: CString[amount_of_records]
        for i in self.records.iter() {
            w.write_all(i.as_bytes())?;
            w.write_all(&[0])?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(818, "SMSG_EXPECTED_SPAM_RECORDS", body_size, a))
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

