use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Sent as response to [`CMSG_REQUEST_ACCOUNT_DATA`](crate::vanilla::CMSG_REQUEST_ACCOUNT_DATA)
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_update_account_data.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_update_account_data.wowm#L1):
/// ```text
/// smsg SMSG_UPDATE_ACCOUNT_DATA = 0x020C {
///     u32 data_type;
///     u32 decompressed_size;
///     u8[-] compressed_data;
/// }
/// ```
pub struct SMSG_UPDATE_ACCOUNT_DATA {
    pub data_type: u32,
    pub decompressed_size: u32,
    pub compressed_data: Vec<u8>,
}

impl crate::private::Sealed for SMSG_UPDATE_ACCOUNT_DATA {}
impl SMSG_UPDATE_ACCOUNT_DATA {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(8..=65543).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // data_type: u32
        let data_type = crate::util::read_u32_le(&mut r)?;

        // decompressed_size: u32
        let decompressed_size = crate::util::read_u32_le(&mut r)?;

        // compressed_data: u8[-]
        let compressed_data = {
            let mut current_size = {
                4 // data_type: u32
                + 4 // decompressed_size: u32
            };
            current_size += 4; // compressed_data_decompressed_size: u32
            let mut compressed_data = Vec::with_capacity(body_size as usize - current_size);
            while current_size < (body_size as usize) {
                compressed_data.push(crate::util::read_u8_le(&mut r)?);
                current_size += 1;
            }
            compressed_data
        };

        Ok(Self {
            data_type,
            decompressed_size,
            compressed_data,
        })
    }

}

impl crate::Message for SMSG_UPDATE_ACCOUNT_DATA {
    const OPCODE: u32 = 0x020c;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_UPDATE_ACCOUNT_DATA"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_UPDATE_ACCOUNT_DATA {{").unwrap();
        // Members
        writeln!(s, "    data_type = {};", self.data_type).unwrap();
        writeln!(s, "    decompressed_size = {};", self.decompressed_size).unwrap();
        writeln!(s, "    compressed_data = [").unwrap();
        for v in self.compressed_data.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "    ];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 524_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "data_type", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "decompressed_size", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.compressed_data.len(), "compressed_data", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // data_type: u32
        w.write_all(&self.data_type.to_le_bytes())?;

        // decompressed_size: u32
        w.write_all(&self.decompressed_size.to_le_bytes())?;

        // compressed_data: u8[-]
        for i in self.compressed_data.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(524, "SMSG_UPDATE_ACCOUNT_DATA", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_UPDATE_ACCOUNT_DATA {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_UPDATE_ACCOUNT_DATA {}

impl SMSG_UPDATE_ACCOUNT_DATA {
    pub(crate) fn size(&self) -> usize {
        4 // data_type: u32
        + 4 // decompressed_size: u32
        + self.compressed_data.len() * core::mem::size_of::<u8>() // compressed_data: u8[-]
    }
}

