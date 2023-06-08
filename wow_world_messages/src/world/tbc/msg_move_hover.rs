use std::io::{Read, Write};

use crate::Guid;
use crate::tbc::MovementInfo;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_hover.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_hover.wowm#L1):
/// ```text
/// msg MSG_MOVE_HOVER = 0x00F7 {
///     PackedGuid player;
///     MovementInfo info;
/// }
/// ```
pub struct MSG_MOVE_HOVER {
    pub player: Guid,
    pub info: MovementInfo,
}

impl crate::private::Sealed for MSG_MOVE_HOVER {}
impl crate::Message for MSG_MOVE_HOVER {
    const OPCODE: u32 = 0x00f7;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        panic!("MSG types not supported");
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // player: PackedGuid
        crate::util::write_packed_guid(&self.player, &mut w)?;

        // info: MovementInfo
        self.info.write_into_vec(&mut w)?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(31..=91).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00F7, size: body_size });
        }

        // player: PackedGuid
        let player = crate::util::read_packed_guid(&mut r)?;

        // info: MovementInfo
        let info = MovementInfo::read(&mut r)?;

        Ok(Self {
            player,
            info,
        })
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for MSG_MOVE_HOVER {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for MSG_MOVE_HOVER {}

impl MSG_MOVE_HOVER {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.player) // player: PackedGuid
        + self.info.size() // info: MovementInfo
    }
}

