use crate:: {
};
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Only exists as comment in azerothcore/trinitycore.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/smsg_gmresponse_db_error.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/smsg_gmresponse_db_error.wowm#L1):
/// ```text
/// smsg SMSG_GMRESPONSE_DB_ERROR = 0x04EE {
/// }
/// ```
pub struct SMSG_GMRESPONSE_DB_ERROR {
}

impl crate::Message for SMSG_GMRESPONSE_DB_ERROR {
    const OPCODE: u32 = 0x04ee;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04EE, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_GMRESPONSE_DB_ERROR {}

