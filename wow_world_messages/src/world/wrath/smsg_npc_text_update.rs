use std::io::{Read, Write};

use crate::wrath::NpcTextUpdate;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
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

impl crate::private::Sealed for SMSG_NPC_TEXT_UPDATE {}
impl crate::Message for SMSG_NPC_TEXT_UPDATE {
    const OPCODE: u32 = 0x0180;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // text_id: u32
        w.write_all(&self.text_id.to_le_bytes())?;

        // texts: NpcTextUpdate[8]
        for i in self.texts.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(252..=4332).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0180, size: body_size as u32 });
        }

        // text_id: u32
        let text_id = crate::util::read_u32_le(&mut r)?;

        // texts: NpcTextUpdate[8]
        let texts = {
            let mut texts = [(); 8].map(|_| NpcTextUpdate::default());
            for i in texts.iter_mut() {
                *i = NpcTextUpdate::read(&mut r)?;
            }
            texts
        };

        Ok(Self {
            text_id,
            texts,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_NPC_TEXT_UPDATE {}

impl SMSG_NPC_TEXT_UPDATE {
    pub(crate) fn size(&self) -> usize {
        4 // text_id: u32
        + self.texts.iter().fold(0, |acc, x| acc + x.size()) // texts: NpcTextUpdate[8]
    }
}

