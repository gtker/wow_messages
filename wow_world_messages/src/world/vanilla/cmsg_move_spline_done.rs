use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::MovementInfo;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_move_spline_done.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_move_spline_done.wowm#L3):
/// ```text
/// cmsg CMSG_MOVE_SPLINE_DONE = 0x02C9 {
///     MovementInfo info;
///     u32 movement_counter;
///     u32 unknown1;
/// }
/// ```
pub struct CMSG_MOVE_SPLINE_DONE {
    pub info: MovementInfo,
    pub movement_counter: u32,
    pub unknown1: u32,
}

impl ClientMessage for CMSG_MOVE_SPLINE_DONE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // info: MovementInfo
        self.info.write_into_vec(w)?;

        // movement_counter: u32
        w.write_all(&self.movement_counter.to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x02c9;

    fn client_size(&self) -> u16 {
        (self.size() + 6) as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // info: MovementInfo
        let info = MovementInfo::read(r)?;

        // movement_counter: u32
        let movement_counter = crate::util::read_u32_le(r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        Ok(Self {
            info,
            movement_counter,
            unknown1,
        })
    }

}

impl CMSG_MOVE_SPLINE_DONE {
    pub(crate) fn size(&self) -> usize {
        self.info.size() // info: MovementInfo
        + 4 // movement_counter: u32
        + 4 // unknown1: u32
    }
}

