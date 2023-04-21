use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_move_unset_can_fly.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_move_unset_can_fly.wowm#L1):
/// ```text
/// smsg SMSG_MOVE_UNSET_CAN_FLY = 0x0344 {
///     PackedGuid player;
///     u32 counter;
/// }
/// ```
pub struct SMSG_MOVE_UNSET_CAN_FLY {
    pub player: Guid,
    pub counter: u32,
}

impl crate::private::Sealed for SMSG_MOVE_UNSET_CAN_FLY {}
impl crate::Message for SMSG_MOVE_UNSET_CAN_FLY {
    const OPCODE: u32 = 0x0344;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // player: PackedGuid
        self.player.write_packed_guid_into_vec(&mut w)?;

        // counter: u32
        w.write_all(&self.counter.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(6..=13).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0344, size: body_size as u32 });
        }

        // player: PackedGuid
        let player = Guid::read_packed(&mut r)?;

        // counter: u32
        let counter = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            player,
            counter,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_MOVE_UNSET_CAN_FLY {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_MOVE_UNSET_CAN_FLY {}

impl SMSG_MOVE_UNSET_CAN_FLY {
    pub(crate) const fn size(&self) -> usize {
        self.player.size() // player: PackedGuid
        + 4 // counter: u32
    }
}

