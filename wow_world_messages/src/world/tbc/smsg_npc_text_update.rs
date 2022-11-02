use std::convert::{TryFrom, TryInto};
use crate::world::tbc::NpcTextUpdate;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gossip/smsg_npc_text_update.wowm:10`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gossip/smsg_npc_text_update.wowm#L10):
/// ```text
/// smsg SMSG_NPC_TEXT_UPDATE = 0x0180 {
///     u32 text_id;
///     NpcTextUpdate[8] texts;
/// }
/// ```
pub struct SMSG_NPC_TEXT_UPDATE {
    pub text_id: u32,
    pub texts: [NpcTextUpdate; 8],
}

impl crate::Message for SMSG_NPC_TEXT_UPDATE {
    const OPCODE: u32 = 0x0180;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // text_id: u32
        w.write_all(&self.text_id.to_le_bytes())?;

        // texts: NpcTextUpdate[8]
        for i in self.texts.iter() {
            i.write_into_vec(w)?;
        }

        assert_eq!(self.size() as usize, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // text_id: u32
        let text_id = crate::util::read_u32_le(r)?;

        // texts: NpcTextUpdate[8]
        let mut texts = Vec::with_capacity(8);
        for i in 0..8 {
            texts.push(NpcTextUpdate::read(r)?);
        }
        let texts = texts.try_into().unwrap();

        Ok(Self {
            text_id,
            texts,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_NPC_TEXT_UPDATE {}

impl SMSG_NPC_TEXT_UPDATE {
    pub(crate) fn size(&self) -> usize {
        4 // text_id: u32
        + self.texts.iter().fold(0, |acc, x| acc + x.size()) // texts: NpcTextUpdate[8]
    }
}

