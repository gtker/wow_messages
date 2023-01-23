use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::wrath::MiniMoveOpcode;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_multiple_moves.wowm:11`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_multiple_moves.wowm#L11):
/// ```text
/// struct MiniMoveMessage {
///     u8 size;
///     MiniMoveOpcode opcode;
///     PackedGuid guid;
///     u32 movement_counter;
/// }
/// ```
pub struct MiniMoveMessage {
    pub size: u8,
    pub opcode: MiniMoveOpcode,
    pub guid: Guid,
    pub movement_counter: u32,
}

impl MiniMoveMessage {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // size: u8
        w.write_all(&self.size.to_le_bytes())?;

        // opcode: MiniMoveOpcode
        w.write_all(&(self.opcode.as_int() as u16).to_le_bytes())?;

        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(w);

        // movement_counter: u32
        w.write_all(&self.movement_counter.to_le_bytes())?;

        Ok(())
    }
}

impl MiniMoveMessage {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // size: u8
        let size = crate::util::read_u8_le(r)?;

        // opcode: MiniMoveOpcode
        let opcode: MiniMoveOpcode = crate::util::read_u16_le(r)?.try_into()?;

        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        // movement_counter: u32
        let movement_counter = crate::util::read_u32_le(r)?;

        Ok(Self {
            size,
            opcode,
            guid,
            movement_counter,
        })
    }

}

impl MiniMoveMessage {
    pub(crate) fn size(&self) -> usize {
        1 // size: u8
        + 2 // opcode: MiniMoveOpcode
        + self.guid.size() // guid: Guid
        + 4 // movement_counter: u32
    }
}

