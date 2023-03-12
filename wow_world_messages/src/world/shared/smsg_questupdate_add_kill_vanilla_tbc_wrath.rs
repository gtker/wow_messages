use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questupdate_add_kill.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questupdate_add_kill.wowm#L3):
/// ```text
/// smsg SMSG_QUESTUPDATE_ADD_KILL = 0x0199 {
///     u32 quest_id;
///     u32 creature_id;
///     u32 kill_count;
///     u32 required_kill_count;
///     Guid guid;
/// }
/// ```
pub struct SMSG_QUESTUPDATE_ADD_KILL {
    pub quest_id: u32,
    /// Unsure of name
    ///
    pub creature_id: u32,
    pub kill_count: u32,
    pub required_kill_count: u32,
    pub guid: Guid,
}

impl crate::Message for SMSG_QUESTUPDATE_ADD_KILL {
    const OPCODE: u32 = 0x0199;

    fn size_without_header(&self) -> u32 {
        24
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // creature_id: u32
        w.write_all(&self.creature_id.to_le_bytes())?;

        // kill_count: u32
        w.write_all(&self.kill_count.to_le_bytes())?;

        // required_kill_count: u32
        w.write_all(&self.required_kill_count.to_le_bytes())?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 24 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0199, size: body_size as u32 });
        }

        // quest_id: u32
        let quest_id = crate::util::read_u32_le(&mut r)?;

        // creature_id: u32
        let creature_id = crate::util::read_u32_le(&mut r)?;

        // kill_count: u32
        let kill_count = crate::util::read_u32_le(&mut r)?;

        // required_kill_count: u32
        let required_kill_count = crate::util::read_u32_le(&mut r)?;

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        Ok(Self {
            quest_id,
            creature_id,
            kill_count,
            required_kill_count,
            guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_QUESTUPDATE_ADD_KILL {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_QUESTUPDATE_ADD_KILL {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_QUESTUPDATE_ADD_KILL {}

