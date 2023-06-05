use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::{
    Emote, TextEmote,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/cmsg_text_emote.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/cmsg_text_emote.wowm#L1):
/// ```text
/// cmsg CMSG_TEXT_EMOTE = 0x0104 {
///     TextEmote text_emote;
///     Emote emote;
///     Guid guid;
/// }
/// ```
pub struct CMSG_TEXT_EMOTE {
    pub text_emote: TextEmote,
    pub emote: Emote,
    pub guid: Guid,
}

#[cfg(feature = "print-testcase")]
impl CMSG_TEXT_EMOTE {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_TEXT_EMOTE {{").unwrap();
        // Members
        writeln!(s, "    text_emote = {};", self.text_emote.as_test_case_value()).unwrap();
        writeln!(s, "    emote = {};", self.emote.as_test_case_value()).unwrap();
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 20_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 260_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "text_emote", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "emote", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1.12\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for CMSG_TEXT_EMOTE {}
impl crate::Message for CMSG_TEXT_EMOTE {
    const OPCODE: u32 = 0x0104;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        CMSG_TEXT_EMOTE::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // text_emote: TextEmote
        w.write_all(&(self.text_emote.as_int().to_le_bytes()))?;

        // emote: Emote
        w.write_all(&(self.emote.as_int().to_le_bytes()))?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0104, size: body_size });
        }

        // text_emote: TextEmote
        let text_emote = crate::util::read_u32_le(&mut r)?.try_into()?;

        // emote: Emote
        let emote = crate::util::read_u32_le(&mut r)?.try_into()?;

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        Ok(Self {
            text_emote,
            emote,
            guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_TEXT_EMOTE {}

