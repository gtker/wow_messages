use std::io::{Read, Write};

use crate::wrath::MovementInfo;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_move_chng_transport.wowm:7`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_move_chng_transport.wowm#L7):
/// ```text
/// cmsg CMSG_MOVE_CHNG_TRANSPORT = 0x038D {
///     MovementInfo info;
/// }
/// ```
pub struct CMSG_MOVE_CHNG_TRANSPORT {
    pub info: MovementInfo,
}

impl crate::private::Sealed for CMSG_MOVE_CHNG_TRANSPORT {}
impl crate::Message for CMSG_MOVE_CHNG_TRANSPORT {
    const OPCODE: u32 = 0x038d;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // info: MovementInfo
        self.info.write_into_vec(&mut w)?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(30..=88).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x038D, size: body_size as u32 });
        }

        // info: MovementInfo
        let info = MovementInfo::read(&mut r)?;

        Ok(Self {
            info,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_MOVE_CHNG_TRANSPORT {}

impl CMSG_MOVE_CHNG_TRANSPORT {
    pub(crate) const fn size(&self) -> usize {
        self.info.size() // info: MovementInfo
    }
}

