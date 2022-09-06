use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/client_set/cmsg_set_target_obsolete.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/client_set/cmsg_set_target_obsolete.wowm#L3):
/// ```text
/// cmsg CMSG_SET_TARGET_OBSOLETE = 0x013E {
///     Guid guid;
/// }
/// ```
pub struct CMSG_SET_TARGET_OBSOLETE {
    pub guid: Guid,
}

impl crate::Message for CMSG_SET_TARGET_OBSOLETE {
    const OPCODE: u32 = 0x013e;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_SET_TARGET_OBSOLETE {}

