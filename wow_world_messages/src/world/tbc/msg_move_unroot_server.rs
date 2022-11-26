use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::tbc::MovementInfo;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/msg_move_unroot.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/msg_move_unroot.wowm#L1):
/// ```text
/// smsg MSG_MOVE_UNROOT_Server = 0x00ED {
///     PackedGuid player;
///     MovementInfo info;
/// }
/// ```
pub struct MSG_MOVE_UNROOT_Server {
    pub player: Guid,
    pub info: MovementInfo,
}

impl crate::Message for MSG_MOVE_UNROOT_Server {
    const OPCODE: u32 = 0x00ed;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // player: PackedGuid
        self.player.write_packed_guid_into_vec(w);

        // info: MovementInfo
        self.info.write_into_vec(w)?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(31..=91).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00ED, size: body_size as u32 });
        }

        // player: PackedGuid
        let player = Guid::read_packed(r)?;

        // info: MovementInfo
        let info = MovementInfo::read(r)?;

        Ok(Self {
            player,
            info,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for MSG_MOVE_UNROOT_Server {}

impl MSG_MOVE_UNROOT_Server {
    pub(crate) fn size(&self) -> usize {
        self.player.size() // player: Guid
        + self.info.size() // info: MovementInfo
    }
}

