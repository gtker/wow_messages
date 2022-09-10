use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_zone_under_attack.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_zone_under_attack.wowm#L3):
/// ```text
/// smsg SMSG_ZONE_UNDER_ATTACK = 0x0254 {
///     u32 zone_id;
/// }
/// ```
pub struct SMSG_ZONE_UNDER_ATTACK {
    pub zone_id: u32,
}

impl crate::Message for SMSG_ZONE_UNDER_ATTACK {
    const OPCODE: u32 = 0x0254;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // zone_id: u32
        w.write_all(&self.zone_id.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // zone_id: u32
        let zone_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            zone_id,
        })
    }

}
#[cfg(any(feature = "vanilla", feature = "tbc"))]
impl crate::helper::shared::vanilla_tbc::ServerMessage for SMSG_ZONE_UNDER_ATTACK {}

