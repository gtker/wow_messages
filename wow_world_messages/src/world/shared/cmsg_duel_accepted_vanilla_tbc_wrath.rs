use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/duel/cmsg_duel_accepted.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/duel/cmsg_duel_accepted.wowm#L3):
/// ```text
/// cmsg CMSG_DUEL_ACCEPTED = 0x016C {
///     Guid guid;
/// }
/// ```
pub struct CMSG_DUEL_ACCEPTED {
    pub guid: Guid,
}

impl crate::private::Sealed for CMSG_DUEL_ACCEPTED {}
impl crate::Message for CMSG_DUEL_ACCEPTED {
    const OPCODE: u32 = 0x016c;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x016C, size: body_size });
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        Ok(Self {
            guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_DUEL_ACCEPTED {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_DUEL_ACCEPTED {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_DUEL_ACCEPTED {}

