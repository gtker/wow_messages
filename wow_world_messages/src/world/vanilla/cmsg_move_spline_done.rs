use std::io::{Read, Write};

use crate::vanilla::MovementInfo;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_move_spline_done.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_move_spline_done.wowm#L1):
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

impl crate::private::Sealed for CMSG_MOVE_SPLINE_DONE {}
impl crate::Message for CMSG_MOVE_SPLINE_DONE {
    const OPCODE: u32 = 0x02c9;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // info: MovementInfo
        self.info.write_into_vec(&mut w)?;

        // movement_counter: u32
        w.write_all(&self.movement_counter.to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(36..=89).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02C9, size: body_size });
        }

        // info: MovementInfo
        let info = MovementInfo::read(&mut r)?;

        // movement_counter: u32
        let movement_counter = crate::util::read_u32_le(&mut r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            info,
            movement_counter,
            unknown1,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_MOVE_SPLINE_DONE {}

impl CMSG_MOVE_SPLINE_DONE {
    pub(crate) const fn size(&self) -> usize {
        self.info.size() // info: MovementInfo
        + 4 // movement_counter: u32
        + 4 // unknown1: u32
    }
}

