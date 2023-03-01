use crate:: {
    Guid,
};
use crate::wrath::MovementInfo;
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_move_set_can_transition_between_swim_and_fly_ack.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_move_set_can_transition_between_swim_and_fly_ack.wowm#L1):
/// ```text
/// cmsg CMSG_MOVE_SET_CAN_TRANSITION_BETWEEN_SWIM_AND_FLY_ACK = 0x0340 {
///     PackedGuid guid;
///     u32 unknown1;
///     MovementInfo info;
///     u32 unknown2;
/// }
/// ```
pub struct CMSG_MOVE_SET_CAN_TRANSITION_BETWEEN_SWIM_AND_FLY_ACK {
    pub guid: Guid,
    pub unknown1: u32,
    pub info: MovementInfo,
    pub unknown2: u32,
}

impl crate::Message for CMSG_MOVE_SET_CAN_TRANSITION_BETWEEN_SWIM_AND_FLY_ACK {
    const OPCODE: u32 = 0x0340;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(&mut w)?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // info: MovementInfo
        self.info.write_into_vec(&mut w)?;

        // unknown2: u32
        w.write_all(&self.unknown2.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(40..=101).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0340, size: body_size as u32 });
        }

        // guid: PackedGuid
        let guid = Guid::read_packed(&mut r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(&mut r)?;

        // info: MovementInfo
        let info = MovementInfo::read(&mut r)?;

        // unknown2: u32
        let unknown2 = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            guid,
            unknown1,
            info,
            unknown2,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_MOVE_SET_CAN_TRANSITION_BETWEEN_SWIM_AND_FLY_ACK {}

impl CMSG_MOVE_SET_CAN_TRANSITION_BETWEEN_SWIM_AND_FLY_ACK {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: PackedGuid
        + 4 // unknown1: u32
        + self.info.size() // info: MovementInfo
        + 4 // unknown2: u32
    }
}

