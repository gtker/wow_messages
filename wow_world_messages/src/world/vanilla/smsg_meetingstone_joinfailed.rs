use std::io::{Read, Write};

use crate::vanilla::MeetingStoneFailure;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/meetingstone/smsg_meetingstone_joinfailed.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/meetingstone/smsg_meetingstone_joinfailed.wowm#L9):
/// ```text
/// smsg SMSG_MEETINGSTONE_JOINFAILED = 0x02BB {
///     MeetingStoneFailure reason;
/// }
/// ```
pub struct SMSG_MEETINGSTONE_JOINFAILED {
    pub reason: MeetingStoneFailure,
}

impl crate::private::Sealed for SMSG_MEETINGSTONE_JOINFAILED {}
impl SMSG_MEETINGSTONE_JOINFAILED {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 1 {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x02BB, size: body_size });
        }

        // reason: MeetingStoneFailure
        let reason = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            reason,
        })
    }

}

impl crate::Message for SMSG_MEETINGSTONE_JOINFAILED {
    const OPCODE: u32 = 0x02bb;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_MEETINGSTONE_JOINFAILED {{").unwrap();
        // Members
        writeln!(s, "    reason = {};", self.reason.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 3_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 699_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "reason", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // reason: MeetingStoneFailure
        w.write_all(&(self.reason.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_MEETINGSTONE_JOINFAILED {}

