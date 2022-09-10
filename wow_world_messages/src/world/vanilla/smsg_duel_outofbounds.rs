use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/duel/smsg_duel_outofbounds.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/duel/smsg_duel_outofbounds.wowm#L3):
/// ```text
/// smsg SMSG_DUEL_OUTOFBOUNDS = 0x0168 {
/// }
/// ```
pub struct SMSG_DUEL_OUTOFBOUNDS {
}

impl crate::Message for SMSG_DUEL_OUTOFBOUNDS {
    const OPCODE: u32 = 0x0168;

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
#[cfg(any(feature = "vanilla", feature = "tbc"))]
impl crate::helper::shared::vanilla_tbc::ServerMessage for SMSG_DUEL_OUTOFBOUNDS {}

