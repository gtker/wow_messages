use std::io::{Read, Write};

use crate::Guid;
use crate::tbc::Emote;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_emote.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_emote.wowm#L1):
/// ```text
/// smsg SMSG_EMOTE = 0x0103 {
///     Emote emote;
///     Guid guid;
/// }
/// ```
pub struct SMSG_EMOTE {
    pub emote: Emote,
    pub guid: Guid,
}

impl crate::private::Sealed for SMSG_EMOTE {}
impl SMSG_EMOTE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 12 {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x0103, size: body_size });
        }

        // emote: Emote
        let emote = crate::util::read_u32_le(&mut r)?.try_into()?;

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        Ok(Self {
            emote,
            guid,
        })
    }

}

impl crate::Message for SMSG_EMOTE {
    const OPCODE: u32 = 0x0103;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_EMOTE {{").unwrap();
        // Members
        writeln!(s, "    emote = {};", self.emote.as_test_case_value()).unwrap();
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 14_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 259_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "emote", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // emote: Emote
        w.write_all(&(self.emote.as_int().to_le_bytes()))?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_EMOTE {}

