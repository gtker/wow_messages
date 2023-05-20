use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::MovementInfo;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_move_gravity_enable_ack.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_move_gravity_enable_ack.wowm#L1):
/// ```text
/// cmsg CMSG_MOVE_GRAVITY_ENABLE_ACK = 0x04D1 {
///     PackedGuid guid;
///     u32 unknown;
///     MovementInfo info;
/// }
/// ```
pub struct CMSG_MOVE_GRAVITY_ENABLE_ACK {
    pub guid: Guid,
    pub unknown: u32,
    pub info: MovementInfo,
}

impl crate::private::Sealed for CMSG_MOVE_GRAVITY_ENABLE_ACK {}
impl crate::Message for CMSG_MOVE_GRAVITY_ENABLE_ACK {
    const OPCODE: u32 = 0x04d1;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(&mut w)?;

        // unknown: u32
        w.write_all(&self.unknown.to_le_bytes())?;

        // info: MovementInfo
        self.info.write_into_vec(&mut w)?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(36..=101).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04D1, size: body_size });
        }

        // guid: PackedGuid
        let guid = Guid::read_packed(&mut r)?;

        // unknown: u32
        let unknown = crate::util::read_u32_le(&mut r)?;

        // info: MovementInfo
        let info = MovementInfo::read(&mut r)?;

        Ok(Self {
            guid,
            unknown,
            info,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_MOVE_GRAVITY_ENABLE_ACK {}

impl CMSG_MOVE_GRAVITY_ENABLE_ACK {
    pub(crate) const fn size(&self) -> usize {
        self.guid.size() // guid: PackedGuid
        + 4 // unknown: u32
        + self.info.size() // info: MovementInfo
    }
}

