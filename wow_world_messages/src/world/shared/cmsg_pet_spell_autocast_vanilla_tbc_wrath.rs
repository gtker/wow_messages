use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/cmsg_pet_spell_autocast.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/cmsg_pet_spell_autocast.wowm#L3):
/// ```text
/// cmsg CMSG_PET_SPELL_AUTOCAST = 0x02F3 {
///     Guid guid;
///     u32 id;
///     Bool autocast_enabled;
/// }
/// ```
pub struct CMSG_PET_SPELL_AUTOCAST {
    pub guid: Guid,
    pub id: u32,
    pub autocast_enabled: bool,
}

impl crate::private::Sealed for CMSG_PET_SPELL_AUTOCAST {}
impl crate::Message for CMSG_PET_SPELL_AUTOCAST {
    const OPCODE: u32 = 0x02f3;

    fn size_without_header(&self) -> u32 {
        13
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // autocast_enabled: Bool
        w.write_all(u8::from(self.autocast_enabled).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 13 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02F3, size: body_size });
        }

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        // id: u32
        let id = crate::util::read_u32_le(&mut r)?;

        // autocast_enabled: Bool
        let autocast_enabled = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            guid,
            id,
            autocast_enabled,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_PET_SPELL_AUTOCAST {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_PET_SPELL_AUTOCAST {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_PET_SPELL_AUTOCAST {}

