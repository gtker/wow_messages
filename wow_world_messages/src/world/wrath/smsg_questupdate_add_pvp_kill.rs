use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questupdate_add_pvp_kill.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questupdate_add_pvp_kill.wowm#L1):
/// ```text
/// smsg SMSG_QUESTUPDATE_ADD_PVP_KILL = 0x046F {
///     u32 quest_id;
///     u32 count;
///     u32 players_slain;
/// }
/// ```
pub struct SMSG_QUESTUPDATE_ADD_PVP_KILL {
    pub quest_id: u32,
    pub count: u32,
    pub players_slain: u32,
}

impl crate::private::Sealed for SMSG_QUESTUPDATE_ADD_PVP_KILL {}
impl crate::Message for SMSG_QUESTUPDATE_ADD_PVP_KILL {
    const OPCODE: u32 = 0x046f;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // count: u32
        w.write_all(&self.count.to_le_bytes())?;

        // players_slain: u32
        w.write_all(&self.players_slain.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x046F, size: body_size });
        }

        // quest_id: u32
        let quest_id = crate::util::read_u32_le(&mut r)?;

        // count: u32
        let count = crate::util::read_u32_le(&mut r)?;

        // players_slain: u32
        let players_slain = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            quest_id,
            count,
            players_slain,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_QUESTUPDATE_ADD_PVP_KILL {}

