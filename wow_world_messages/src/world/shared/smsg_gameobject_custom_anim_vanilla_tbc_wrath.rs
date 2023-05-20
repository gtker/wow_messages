use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gameobject/smsg_gameobject_custom_anim.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gameobject/smsg_gameobject_custom_anim.wowm#L3):
/// ```text
/// smsg SMSG_GAMEOBJECT_CUSTOM_ANIM = 0x00B3 {
///     Guid guid;
///     u32 animation_id;
/// }
/// ```
pub struct SMSG_GAMEOBJECT_CUSTOM_ANIM {
    pub guid: Guid,
    pub animation_id: u32,
}

impl crate::private::Sealed for SMSG_GAMEOBJECT_CUSTOM_ANIM {}
impl crate::Message for SMSG_GAMEOBJECT_CUSTOM_ANIM {
    const OPCODE: u32 = 0x00b3;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // animation_id: u32
        w.write_all(&self.animation_id.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00B3, size: body_size });
        }

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        // animation_id: u32
        let animation_id = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            guid,
            animation_id,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_GAMEOBJECT_CUSTOM_ANIM {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_GAMEOBJECT_CUSTOM_ANIM {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_GAMEOBJECT_CUSTOM_ANIM {}

