use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_force_move_unroot.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_force_move_unroot.wowm#L8):
/// ```text
/// smsg SMSG_FORCE_MOVE_UNROOT = 0x00EA {
///     PackedGuid guid;
///     u32 counter;
/// }
/// ```
pub struct SMSG_FORCE_MOVE_UNROOT {
    pub guid: Guid,
    pub counter: u32,
}

impl crate::Message for SMSG_FORCE_MOVE_UNROOT {
    const OPCODE: u32 = 0x00ea;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(w);

        // counter: u32
        w.write_all(&self.counter.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        // counter: u32
        let counter = crate::util::read_u32_le(r)?;

        Ok(Self {
            guid,
            counter,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_FORCE_MOVE_UNROOT {}

impl SMSG_FORCE_MOVE_UNROOT {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: Guid
        + 4 // counter: u32
    }
}

