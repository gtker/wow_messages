use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::MiniMoveOpcode;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_multiple_moves.wowm:11`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_multiple_moves.wowm#L11):
/// ```text
/// struct MiniMoveMessage {
///     u8 size = self.size;
///     MiniMoveOpcode opcode;
///     PackedGuid guid;
///     u32 movement_counter;
/// }
/// ```
pub struct MiniMoveMessage {
    pub opcode: MiniMoveOpcode,
    pub guid: Guid,
    pub movement_counter: u32,
}

impl MiniMoveMessage {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // size: u8
        w.write_all(&((self.size() - 1) as u8).to_le_bytes())?;

        // opcode: MiniMoveOpcode
        w.write_all(&(self.opcode.as_int().to_le_bytes()))?;

        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(&mut w)?;

        // movement_counter: u32
        w.write_all(&self.movement_counter.to_le_bytes())?;

        Ok(())
    }
}

impl MiniMoveMessage {
    pub(crate) fn read<R: Read>(mut r: R) -> Result<Self, crate::errors::ParseError> {
        // size: u8
        let _size = crate::util::read_u8_le(&mut r)?;
        // size is expected to always be self.size (0)

        // opcode: MiniMoveOpcode
        let opcode: MiniMoveOpcode = crate::util::read_u16_le(&mut r)?.try_into()?;

        // guid: PackedGuid
        let guid = Guid::read_packed(&mut r)?;

        // movement_counter: u32
        let movement_counter = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            opcode,
            guid,
            movement_counter,
        })
    }

}

impl MiniMoveMessage {
    pub(crate) const fn size(&self) -> usize {
        1 // size: u8
        + 2 // opcode: MiniMoveOpcode
        + self.guid.size() // guid: PackedGuid
        + 4 // movement_counter: u32
    }
}

