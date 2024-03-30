use std::io::{Read, Write};

/// This MSG does not appear to have an SMSG version.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/msg_raid_ready_check_finished.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/msg_raid_ready_check_finished.wowm#L2):
/// ```text
/// cmsg MSG_RAID_READY_CHECK_FINISHED_Client = 0x03C5 {
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct MSG_RAID_READY_CHECK_FINISHED_Client {
}

impl crate::private::Sealed for MSG_RAID_READY_CHECK_FINISHED_Client {}
impl MSG_RAID_READY_CHECK_FINISHED_Client {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 0 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        Ok(Self {
        })
    }

}

impl crate::Message for MSG_RAID_READY_CHECK_FINISHED_Client {
    const OPCODE: u32 = 0x03c5;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "MSG_RAID_READY_CHECK_FINISHED_Client"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_RAID_READY_CHECK_FINISHED_Client {{").unwrap();
        // Members

        writeln!(s, "}} [").unwrap();

        let [a, b] = 4_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 965_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();



        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
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
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(965, "MSG_RAID_READY_CHECK_FINISHED_Client", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for MSG_RAID_READY_CHECK_FINISHED_Client {}

