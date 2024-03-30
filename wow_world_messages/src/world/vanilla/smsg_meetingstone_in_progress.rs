use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/meetingstone/smsg_meetingstone_in_progress.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/meetingstone/smsg_meetingstone_in_progress.wowm#L3):
/// ```text
/// smsg SMSG_MEETINGSTONE_IN_PROGRESS = 0x0298 {
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_MEETINGSTONE_IN_PROGRESS {
}

impl crate::private::Sealed for SMSG_MEETINGSTONE_IN_PROGRESS {}
impl SMSG_MEETINGSTONE_IN_PROGRESS {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 0 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        Ok(Self {
        })
    }

}

impl crate::Message for SMSG_MEETINGSTONE_IN_PROGRESS {
    const OPCODE: u32 = 0x0298;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_MEETINGSTONE_IN_PROGRESS"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_MEETINGSTONE_IN_PROGRESS {{").unwrap();
        // Members

        writeln!(s, "}} [").unwrap();

        let [a, b] = 2_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 664_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();



        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1".to_string())).unwrap();
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
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(664, "SMSG_MEETINGSTONE_IN_PROGRESS", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_MEETINGSTONE_IN_PROGRESS {}

