use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_move_unset_flight.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_move_unset_flight.wowm#L1):
/// ```text
/// smsg SMSG_MOVE_UNSET_FLIGHT = 0x033F {
///     Guid guid;
///     u32 counter;
/// }
/// ```
pub struct SMSG_MOVE_UNSET_FLIGHT {
    pub guid: Guid,
    pub counter: u32,
}

impl crate::private::Sealed for SMSG_MOVE_UNSET_FLIGHT {}
impl crate::Message for SMSG_MOVE_UNSET_FLIGHT {
    const OPCODE: u32 = 0x033f;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // counter: u32
        w.write_all(&self.counter.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x033F, size: body_size });
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // counter: u32
        let counter = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            guid,
            counter,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_MOVE_UNSET_FLIGHT {}

