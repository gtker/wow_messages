use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Respond with [`SMSG_ACCOUNT_DATA_TIMES`](crate::world::wrath::SMSG_ACCOUNT_DATA_TIMES)
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/cmsg_ready_for_account_data_times.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/cmsg_ready_for_account_data_times.wowm#L3):
/// ```text
/// cmsg CMSG_READY_FOR_ACCOUNT_DATA_TIMES = 0x04FF {
/// }
/// ```
pub struct CMSG_READY_FOR_ACCOUNT_DATA_TIMES {
}

impl crate::Message for CMSG_READY_FOR_ACCOUNT_DATA_TIMES {
    const OPCODE: u32 = 0x04ff;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_READY_FOR_ACCOUNT_DATA_TIMES {}

