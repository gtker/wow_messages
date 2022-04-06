use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_questupdate_add_kill.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_questupdate_add_kill.wowm#L3):
/// ```text
/// smsg SMSG_QUESTUPDATE_ADD_KILL = 0x199 {
///     u32 quest_id;
///     u32 create_id;
///     u32 kill_count;
///     u32 required_kill_count;
///     u64 guid;
/// }
/// ```
pub struct SMSG_QUESTUPDATE_ADD_KILL {
    pub quest_id: u32,
    pub create_id: u32,
    pub kill_count: u32,
    pub required_kill_count: u32,
    pub guid: u64,
}

impl WorldServerMessageWrite for SMSG_QUESTUPDATE_ADD_KILL {
    const OPCODE: u16 = 0x199;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (Self::size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (Self::size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_QUESTUPDATE_ADD_KILL {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // quest_id: u32
        let quest_id = crate::util::read_u32_le(r)?;

        // create_id: u32
        let create_id = crate::util::read_u32_le(r)?;

        // kill_count: u32
        let kill_count = crate::util::read_u32_le(r)?;

        // required_kill_count: u32
        let required_kill_count = crate::util::read_u32_le(r)?;

        // guid: u64
        let guid = crate::util::read_u64_le(r)?;

        Ok(Self {
            quest_id,
            create_id,
            kill_count,
            required_kill_count,
            guid,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // create_id: u32
        w.write_all(&self.create_id.to_le_bytes())?;

        // kill_count: u32
        w.write_all(&self.kill_count.to_le_bytes())?;

        // required_kill_count: u32
        w.write_all(&self.required_kill_count.to_le_bytes())?;

        // guid: u64
        w.write_all(&self.guid.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for SMSG_QUESTUPDATE_ADD_KILL {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_QUESTUPDATE_ADD_KILL {
    fn maximum_possible_size() -> usize {
        4 // quest_id: u32
        + 4 // create_id: u32
        + 4 // kill_count: u32
        + 4 // required_kill_count: u32
        + 8 // guid: u64
    }
}

