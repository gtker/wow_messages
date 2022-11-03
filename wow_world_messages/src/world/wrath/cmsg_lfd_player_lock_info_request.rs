use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/cmsg_lfg_player_lock_info_request.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/cmsg_lfg_player_lock_info_request.wowm#L3):
/// ```text
/// cmsg CMSG_LFD_PLAYER_LOCK_INFO_REQUEST = 0x036E {
/// }
/// ```
pub struct CMSG_LFD_PLAYER_LOCK_INFO_REQUEST {
}

impl crate::Message for CMSG_LFD_PLAYER_LOCK_INFO_REQUEST {
    const OPCODE: u32 = 0x036e;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x036E, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_LFD_PLAYER_LOCK_INFO_REQUEST {}

