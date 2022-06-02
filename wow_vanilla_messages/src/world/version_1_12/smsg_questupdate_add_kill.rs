use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questupdate_add_kill.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questupdate_add_kill.wowm#L3):
/// ```text
/// smsg SMSG_QUESTUPDATE_ADD_KILL = 0x0199 {
///     u32 quest_id;
///     u32 create_id;
///     u32 kill_count;
///     u32 required_kill_count;
///     Guid guid;
/// }
/// ```
pub struct SMSG_QUESTUPDATE_ADD_KILL {
    pub quest_id: u32,
    /// # Comment
    ///
    /// Unsure of name
    pub create_id: u32,
    pub kill_count: u32,
    pub required_kill_count: u32,
    pub guid: Guid,
}

impl ServerMessage for SMSG_QUESTUPDATE_ADD_KILL {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // create_id: u32
        w.write_all(&self.create_id.to_le_bytes())?;

        // kill_count: u32
        w.write_all(&self.kill_count.to_le_bytes())?;

        // required_kill_count: u32
        w.write_all(&self.required_kill_count.to_le_bytes())?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0199;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        24
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // quest_id: u32
        let quest_id = crate::util::read_u32_le(r)?;

        // create_id: u32
        let create_id = crate::util::read_u32_le(r)?;

        // kill_count: u32
        let kill_count = crate::util::read_u32_le(r)?;

        // required_kill_count: u32
        let required_kill_count = crate::util::read_u32_le(r)?;

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            quest_id,
            create_id,
            kill_count,
            required_kill_count,
            guid,
        })
    }

}

