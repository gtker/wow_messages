use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Immediately removes an object from the presence of the player.
///
/// Used by vmangos for logout.
/// azerothcore: If the following bool is true, the client will call `void CGUnit_C::OnDeath()` for this object. `OnDeath()` does for eg trigger death animation and interrupts certain spells/missiles/auras/sounds...
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gameobject/smsg_destroy_object.wowm:21`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gameobject/smsg_destroy_object.wowm#L21):
/// ```text
/// smsg SMSG_DESTROY_OBJECT = 0x00AA {
///     Guid guid;
///     Bool target_died;
/// }
/// ```
pub struct SMSG_DESTROY_OBJECT {
    pub guid: Guid,
    pub target_died: bool,
}

impl crate::private::Sealed for SMSG_DESTROY_OBJECT {}
impl crate::Message for SMSG_DESTROY_OBJECT {
    const OPCODE: u32 = 0x00aa;

    fn size_without_header(&self) -> u32 {
        9
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // target_died: Bool
        w.write_all(u8::from(self.target_died).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 9 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00AA, size: body_size });
        }

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        // target_died: Bool
        let target_died = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            guid,
            target_died,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_DESTROY_OBJECT {}

