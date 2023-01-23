use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_add_rune_power.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_add_rune_power.wowm#L1):
/// ```text
/// smsg SMSG_ADD_RUNE_POWER = 0x0488 {
///     u32 rune;
/// }
/// ```
pub struct SMSG_ADD_RUNE_POWER {
    /// Emus bitshifts 1 by the rune index instead of directly sending the index.
    /// mangostwo: mask (0x00-0x3F probably)
    ///
    pub rune: u32,
}

impl crate::Message for SMSG_ADD_RUNE_POWER {
    const OPCODE: u32 = 0x0488;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // rune: u32
        w.write_all(&self.rune.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0488, size: body_size as u32 });
        }

        // rune: u32
        let rune = crate::util::read_u32_le(r)?;

        Ok(Self {
            rune,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_ADD_RUNE_POWER {}

