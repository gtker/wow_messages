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
impl crate::Message for SMSG_AI_REACTION {
    const OPCODE: u32 = 0x013c;

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
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x013C, size: body_size });
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // reaction: AiReaction
        let reaction: AiReaction = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            guid,
            reaction,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_AI_REACTION {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_AI_REACTION {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_AI_REACTION {}

