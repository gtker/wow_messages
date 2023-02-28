use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/cmsg_dismiss_critter.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/cmsg_dismiss_critter.wowm#L1):
/// ```text
/// cmsg CMSG_DISMISS_CRITTER = 0x048D {
///     Guid critter;
/// }
/// ```
pub struct CMSG_DISMISS_CRITTER {
    pub critter: Guid,
}

impl crate::Message for CMSG_DISMISS_CRITTER {
    const OPCODE: u32 = 0x048d;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // critter: Guid
        w.write_all(&self.critter.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x048D, size: body_size as u32 });
        }

        // critter: Guid
        let critter = Guid::read(&mut r)?;

        Ok(Self {
            critter,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_DISMISS_CRITTER {}

