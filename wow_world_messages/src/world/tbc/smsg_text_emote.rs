use std::io::{Read, Write};

use crate::Guid;
use crate::tbc::TextEmote;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_text_emote.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_text_emote.wowm#L1):
/// ```text
/// smsg SMSG_TEXT_EMOTE = 0x0105 {
///     Guid guid;
///     TextEmote text_emote;
///     u32 emote;
///     SizedCString name;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_TEXT_EMOTE {
    pub guid: Guid,
    pub text_emote: TextEmote,
    pub emote: u32,
    pub name: String,
}

impl crate::private::Sealed for SMSG_TEXT_EMOTE {}
impl SMSG_TEXT_EMOTE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(21..=8020).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // text_emote: TextEmote
        let text_emote = crate::util::read_u32_le(&mut r)?.try_into()?;

        // emote: u32
        let emote = crate::util::read_u32_le(&mut r)?;

        // name: SizedCString
        let name = {
            let name = crate::util::read_u32_le(&mut r)?;
            let name = crate::util::read_sized_c_string_to_vec(&mut r, name)?;
            String::from_utf8(name)?
        };

        Ok(Self {
            guid,
            text_emote,
            emote,
            name,
        })
    }

}

impl crate::Message for SMSG_TEXT_EMOTE {
    const OPCODE: u32 = 0x0105;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_TEXT_EMOTE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_TEXT_EMOTE {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    text_emote = {};", self.text_emote.as_test_case_value()).unwrap();
        writeln!(s, "    emote = {};", self.emote).unwrap();
        writeln!(s, "    name = \"{}\";", self.name).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 261_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "text_emote", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "emote", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.name.len() + 5, "name", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // text_emote: TextEmote
        w.write_all(&(self.text_emote.as_int().to_le_bytes()))?;

        // emote: u32
        w.write_all(&self.emote.to_le_bytes())?;

        // name: SizedCString
        w.write_all(&((self.name.len() + 1) as u32).to_le_bytes())?;
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(261, "SMSG_TEXT_EMOTE", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_TEXT_EMOTE {}

impl SMSG_TEXT_EMOTE {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // text_emote: TextEmote
        + 4 // emote: u32
        + self.name.len() + 5 // name: SizedCString
    }
}

