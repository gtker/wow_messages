use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::MovementInfo;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_move_fall_reset.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_move_fall_reset.wowm#L3):
/// ```text
/// cmsg CMSG_MOVE_FALL_RESET = 0x02CA {
///     MovementInfo info;
/// }
/// ```
pub struct CMSG_MOVE_FALL_RESET {
    pub info: MovementInfo,
}

impl crate::Message for CMSG_MOVE_FALL_RESET {
    const OPCODE: u32 = 0x02ca;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // info: MovementInfo
        self.info.write_into_vec(w)?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // info: MovementInfo
        let info = MovementInfo::read(r)?;

        Ok(Self {
            info,
        })
    }

}
impl ClientMessage for CMSG_MOVE_FALL_RESET {}

impl CMSG_MOVE_FALL_RESET {
    pub(crate) fn size(&self) -> usize {
        self.info.size() // info: MovementInfo
    }
}

