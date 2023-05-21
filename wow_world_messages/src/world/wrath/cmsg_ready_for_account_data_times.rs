use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Respond with [`SMSG_ACCOUNT_DATA_TIMES`](crate::wrath::SMSG_ACCOUNT_DATA_TIMES)
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/cmsg_ready_for_account_data_times.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/cmsg_ready_for_account_data_times.wowm#L3):
/// ```text
/// cmsg CMSG_READY_FOR_ACCOUNT_DATA_TIMES = 0x04FF {
/// }
/// ```
pub struct CMSG_READY_FOR_ACCOUNT_DATA_TIMES {
}

impl crate::private::Sealed for CMSG_READY_FOR_ACCOUNT_DATA_TIMES {}
impl CMSG_READY_FOR_ACCOUNT_DATA_TIMES {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 0 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        Ok(Self {
        })
    }

}

impl crate::Message for CMSG_READY_FOR_ACCOUNT_DATA_TIMES {
    const OPCODE: u32 = 0x04ff;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_READY_FOR_ACCOUNT_DATA_TIMES {{").unwrap();
        // Members

        writeln!(s, "}} [").unwrap();

        let [a, b] = 4_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1279_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();



        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1279, "CMSG_READY_FOR_ACCOUNT_DATA_TIMES", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_READY_FOR_ACCOUNT_DATA_TIMES {}

