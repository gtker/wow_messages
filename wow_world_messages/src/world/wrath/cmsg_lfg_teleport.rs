use std::convert::{TryFrom, TryInto};
use crate::wrath::LfgTeleportLocation;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/cmsg_lfg_teleport.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/cmsg_lfg_teleport.wowm#L8):
/// ```text
/// cmsg CMSG_LFG_TELEPORT = 0x0370 {
///     LfgTeleportLocation location;
/// }
/// ```
pub struct CMSG_LFG_TELEPORT {
    pub location: LfgTeleportLocation,
}

impl crate::Message for CMSG_LFG_TELEPORT {
    const OPCODE: u32 = 0x0370;

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // location: LfgTeleportLocation
        w.write_all(&(self.location.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0370, size: body_size as u32 });
        }

        // location: LfgTeleportLocation
        let location: LfgTeleportLocation = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            location,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_LFG_TELEPORT {}

