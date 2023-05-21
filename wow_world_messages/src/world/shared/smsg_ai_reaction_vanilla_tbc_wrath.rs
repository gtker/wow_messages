use std::io::{Read, Write};

use crate::Guid;
use wow_world_base::shared::ai_reaction_vanilla_tbc_wrath::AiReaction;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_ai_reaction.wowm:21`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_ai_reaction.wowm#L21):
/// ```text
/// smsg SMSG_AI_REACTION = 0x013C {
///     Guid guid;
///     AiReaction reaction;
/// }
/// ```
pub struct SMSG_AI_REACTION {
    pub guid: Guid,
    pub reaction: AiReaction,
}

impl crate::private::Sealed for SMSG_AI_REACTION {}
impl SMSG_AI_REACTION {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 12 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // reaction: AiReaction
        let reaction = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            guid,
            reaction,
        })
    }

}

impl crate::Message for SMSG_AI_REACTION {
    const OPCODE: u32 = 0x013c;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_AI_REACTION {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    reaction = {};", self.reaction.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 14_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 316_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "reaction", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // reaction: AiReaction
        w.write_all(&(self.reaction.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(316, "SMSG_AI_REACTION", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_AI_REACTION {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_AI_REACTION {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_AI_REACTION {}

