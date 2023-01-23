use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::wrath::Vector3d;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_set_projectile_position.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_set_projectile_position.wowm#L1):
/// ```text
/// smsg SMSG_SET_PROJECTILE_POSITION = 0x04BF {
///     Guid caster;
///     u8 amount_of_casts;
///     Vector3d position;
/// }
/// ```
pub struct SMSG_SET_PROJECTILE_POSITION {
    pub caster: Guid,
    pub amount_of_casts: u8,
    pub position: Vector3d,
}

impl crate::Message for SMSG_SET_PROJECTILE_POSITION {
    const OPCODE: u32 = 0x04bf;

    fn size_without_header(&self) -> u32 {
        21
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // caster: Guid
        w.write_all(&self.caster.guid().to_le_bytes())?;

        // amount_of_casts: u8
        w.write_all(&self.amount_of_casts.to_le_bytes())?;

        // position: Vector3d
        self.position.write_into_vec(w)?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 21 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04BF, size: body_size as u32 });
        }

        // caster: Guid
        let caster = Guid::read(r)?;

        // amount_of_casts: u8
        let amount_of_casts = crate::util::read_u8_le(r)?;

        // position: Vector3d
        let position = Vector3d::read(r)?;

        Ok(Self {
            caster,
            amount_of_casts,
            position,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_SET_PROJECTILE_POSITION {}

