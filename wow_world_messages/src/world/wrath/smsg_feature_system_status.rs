use std::io::{Read, Write};

use crate::wrath::ComplaintStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_feature_system_status.wowm:16`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_feature_system_status.wowm#L16):
/// ```text
/// smsg SMSG_FEATURE_SYSTEM_STATUS = 0x03C9 {
///     ComplaintStatus complaint_status;
///     Bool voice_chat_enabled;
/// }
/// ```
pub struct SMSG_FEATURE_SYSTEM_STATUS {
    pub complaint_status: ComplaintStatus,
    pub voice_chat_enabled: bool,
}

impl crate::private::Sealed for SMSG_FEATURE_SYSTEM_STATUS {}
impl crate::Message for SMSG_FEATURE_SYSTEM_STATUS {
    const OPCODE: u32 = 0x03c9;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_FEATURE_SYSTEM_STATUS {{").unwrap();
        // Members
        writeln!(s, "    complaint_status = {};", self.complaint_status.as_test_case_value()).unwrap();
        writeln!(s, "    voice_chat_enabled = {};", if self.voice_chat_enabled { "TRUE" } else { "FALSE" }).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 4_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 969_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "complaint_status", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "voice_chat_enabled", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        2
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // complaint_status: ComplaintStatus
        w.write_all(&(self.complaint_status.as_int().to_le_bytes()))?;

        // voice_chat_enabled: Bool
        w.write_all(u8::from(self.voice_chat_enabled).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 2 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03C9, size: body_size });
        }

        // complaint_status: ComplaintStatus
        let complaint_status = crate::util::read_u8_le(&mut r)?.try_into()?;

        // voice_chat_enabled: Bool
        let voice_chat_enabled = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            complaint_status,
            voice_chat_enabled,
        })
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_FEATURE_SYSTEM_STATUS {}

