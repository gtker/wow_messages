use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/cmsg_controller_eject_passenger.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/cmsg_controller_eject_passenger.wowm#L1):
/// ```text
/// cmsg CMSG_CONTROLLER_EJECT_PASSENGER = 0x04A9 {
///     Guid player;
/// }
/// ```
pub struct CMSG_CONTROLLER_EJECT_PASSENGER {
    pub player: Guid,
}

impl crate::Message for CMSG_CONTROLLER_EJECT_PASSENGER {
    const OPCODE: u32 = 0x04a9;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04A9, size: body_size as u32 });
        }

        // player: Guid
        let player = Guid::read(r)?;

        Ok(Self {
            player,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_CONTROLLER_EJECT_PASSENGER {}

