use std::io::{Read, Write};

use crate::vanilla::{
    Area, MeetingStoneStatus,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/meetingstone/smsg_meetingstone_setqueue.wowm:14`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/meetingstone/smsg_meetingstone_setqueue.wowm#L14):
/// ```text
/// smsg SMSG_MEETINGSTONE_SETQUEUE = 0x0295 {
///     Area area;
///     MeetingStoneStatus status;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_MEETINGSTONE_SETQUEUE {
    pub area: Area,
    pub status: MeetingStoneStatus,
}

impl crate::private::Sealed for SMSG_MEETINGSTONE_SETQUEUE {}
impl SMSG_MEETINGSTONE_SETQUEUE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 5 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // area: Area
        let area = crate::util::read_u32_le(&mut r)?.try_into()?;

        // status: MeetingStoneStatus
        let status = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            area,
            status,
        })
    }

}

impl crate::Message for SMSG_MEETINGSTONE_SETQUEUE {
    const OPCODE: u32 = 0x0295;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_MEETINGSTONE_SETQUEUE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_MEETINGSTONE_SETQUEUE {{").unwrap();
        // Members
        writeln!(s, "    area = {};", self.area.as_test_case_value()).unwrap();
        writeln!(s, "    status = {};", self.status.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 7_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 661_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "area", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "status", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        5
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // area: Area
        w.write_all(&(self.area.as_int().to_le_bytes()))?;

        // status: MeetingStoneStatus
        w.write_all(&(self.status.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(661, "SMSG_MEETINGSTONE_SETQUEUE", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_MEETINGSTONE_SETQUEUE {}

