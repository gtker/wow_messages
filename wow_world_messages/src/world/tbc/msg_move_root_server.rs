use std::convert::{TryFrom, TryInto};
use crate::world::tbc::MovementInfo;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
/// There does not appear to be a CMSG version of this MSG.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/msg_move_root.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/msg_move_root.wowm#L1):
/// ```text
/// smsg MSG_MOVE_ROOT_Server = 0x00EC {
///     MovementInfo info;
/// }
/// ```
pub struct MSG_MOVE_ROOT_Server {
    pub info: MovementInfo,
}

impl crate::Message for MSG_MOVE_ROOT_Server {
    const OPCODE: u32 = 0x00ec;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // info: MovementInfo
        self.info.write_into_vec(w)?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(29..=82).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00EC, size: body_size as u32 });
        }

        // info: MovementInfo
        let info = MovementInfo::read(r)?;

        Ok(Self {
            info,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for MSG_MOVE_ROOT_Server {}

impl MSG_MOVE_ROOT_Server {
    pub(crate) fn size(&self) -> usize {
        self.info.size() // info: MovementInfo
    }
}

