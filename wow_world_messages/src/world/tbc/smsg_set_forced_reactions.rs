use crate::tbc::ForcedReaction;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/faction/smsg_set_forced_reactions.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/faction/smsg_set_forced_reactions.wowm#L8):
/// ```text
/// smsg SMSG_SET_FORCED_REACTIONS = 0x02A5 {
///     u32 amount_of_reactions;
///     ForcedReaction[amount_of_reactions] reactions;
/// }
/// ```
pub struct SMSG_SET_FORCED_REACTIONS {
    pub reactions: Vec<ForcedReaction>,
}

impl crate::Message for SMSG_SET_FORCED_REACTIONS {
    const OPCODE: u32 = 0x02a5;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // amount_of_reactions: u32
        w.write_all(&(self.reactions.len() as u32).to_le_bytes())?;

        // reactions: ForcedReaction[amount_of_reactions]
        for i in self.reactions.iter() {
            i.write_into_vec(w)?;
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(4..=4294967294).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02A5, size: body_size as u32 });
        }

        // amount_of_reactions: u32
        let amount_of_reactions = crate::util::read_u32_le(r)?;

        // reactions: ForcedReaction[amount_of_reactions]
        let reactions = {
            let mut reactions = Vec::with_capacity(amount_of_reactions as usize);
            for i in 0..amount_of_reactions {
                reactions.push(ForcedReaction::read(r)?);
            }
            reactions
        };
        Ok(Self {
            reactions,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_SET_FORCED_REACTIONS {}

impl SMSG_SET_FORCED_REACTIONS {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_reactions: u32
        + self.reactions.len() * 6 // reactions: ForcedReaction[amount_of_reactions]
    }
}

