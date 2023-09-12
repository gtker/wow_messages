use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/warden/smsg_warden_data.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/warden/smsg_warden_data.wowm#L1):
/// ```text
/// smsg SMSG_WARDEN_DATA = 0x02E6 {
///     u8[-] encrypted_data;
/// }
/// ```
pub struct SMSG_WARDEN_DATA {
    pub encrypted_data: Vec<u8>,
}

impl crate::private::Sealed for SMSG_WARDEN_DATA {}
impl SMSG_WARDEN_DATA {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size > 65535 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // encrypted_data: u8[-]
        let encrypted_data = {
            let mut current_size = {
                0
            };
            let mut encrypted_data = Vec::with_capacity(body_size as usize - current_size);
            while current_size < (body_size as usize) {
                encrypted_data.push(crate::util::read_u8_le(&mut r)?);
                current_size += 1;
            }
            encrypted_data
        };

        Ok(Self {
            encrypted_data,
        })
    }

}

impl crate::Message for SMSG_WARDEN_DATA {
    const OPCODE: u32 = 0x02e6;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_WARDEN_DATA"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_WARDEN_DATA {{").unwrap();
        // Members
        write!(s, "    encrypted_data = [").unwrap();
        for v in self.encrypted_data.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 742_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, self.encrypted_data.len(), "encrypted_data", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // encrypted_data: u8[-]
        for i in self.encrypted_data.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(742, "SMSG_WARDEN_DATA", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_WARDEN_DATA {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_WARDEN_DATA {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_WARDEN_DATA {}

impl SMSG_WARDEN_DATA {
    pub(crate) fn size(&self) -> usize {
        self.encrypted_data.len() * core::mem::size_of::<u8>() // encrypted_data: u8[-]
    }
}

