use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::MovementInfo;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_move_feather_fall_ack.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_move_feather_fall_ack.wowm#L1):
/// ```text
/// cmsg CMSG_MOVE_FEATHER_FALL_ACK = 0x02CF {
///     Guid guid;
///     u32 movement_counter;
///     MovementInfo info;
///     u32 apply;
/// }
/// ```
pub struct CMSG_MOVE_FEATHER_FALL_ACK {
    pub guid: Guid,
    pub movement_counter: u32,
    pub info: MovementInfo,
    pub apply: u32,
}

impl crate::private::Sealed for CMSG_MOVE_FEATHER_FALL_ACK {}
impl crate::Message for CMSG_MOVE_FEATHER_FALL_ACK {
    const OPCODE: u32 = 0x02cf;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // movement_counter: u32
        w.write_all(&self.movement_counter.to_le_bytes())?;

        // info: MovementInfo
        self.info.write_into_vec(&mut w)?;

        // apply: u32
        w.write_all(&self.apply.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(46..=104).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02CF, size: body_size });
        }

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        // movement_counter: u32
        let movement_counter = crate::util::read_u32_le(&mut r)?;

        // info: MovementInfo
        let info = MovementInfo::read(&mut r)?;

        // apply: u32
        let apply = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            guid,
            movement_counter,
            info,
            apply,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_MOVE_FEATHER_FALL_ACK {}

impl CMSG_MOVE_FEATHER_FALL_ACK {
    pub(crate) const fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // movement_counter: u32
        + self.info.size() // info: MovementInfo
        + 4 // apply: u32
    }
}

