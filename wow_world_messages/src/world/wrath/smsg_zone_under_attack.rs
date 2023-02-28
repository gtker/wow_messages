use crate::wrath::Area;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_zone_under_attack.wowm:13`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_zone_under_attack.wowm#L13):
/// ```text
/// smsg SMSG_ZONE_UNDER_ATTACK = 0x0254 {
///     Area zone_id;
/// }
/// ```
pub struct SMSG_ZONE_UNDER_ATTACK {
    pub zone_id: Area,
}

impl crate::Message for SMSG_ZONE_UNDER_ATTACK {
    const OPCODE: u32 = 0x0254;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // zone_id: Area
        w.write_all(&u32::from(self.zone_id.as_int()).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0254, size: body_size as u32 });
        }

        // zone_id: Area
        let zone_id: Area = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            zone_id,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ZONE_UNDER_ATTACK {}

