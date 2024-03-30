use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::QuestGiverStatus;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questgiver_status.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questgiver_status.wowm#L3):
/// ```text
/// smsg SMSG_QUESTGIVER_STATUS = 0x0183 {
///     Guid guid;
///     (u32)QuestGiverStatus status;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_QUESTGIVER_STATUS {
    pub guid: Guid,
    pub status: QuestGiverStatus,
}

impl crate::private::Sealed for SMSG_QUESTGIVER_STATUS {}
impl SMSG_QUESTGIVER_STATUS {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 12 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // status: QuestGiverStatus
        let status = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        Ok(Self {
            guid,
            status,
        })
    }

}

impl crate::Message for SMSG_QUESTGIVER_STATUS {
    const OPCODE: u32 = 0x0183;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_QUESTGIVER_STATUS"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_QUESTGIVER_STATUS {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    status = {};", self.status.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 14_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 387_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "status", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // status: QuestGiverStatus
        w.write_all(&u32::from(self.status.as_int()).to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(387, "SMSG_QUESTGIVER_STATUS", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_QUESTGIVER_STATUS {}

