use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::SpellCastTargets;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/cmsg_pet_cast_spell.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/cmsg_pet_cast_spell.wowm#L3):
/// ```text
/// cmsg CMSG_PET_CAST_SPELL = 0x01F0 {
///     Guid guid;
///     u32 id;
///     SpellCastTargets targets;
/// }
/// ```
pub struct CMSG_PET_CAST_SPELL {
    pub guid: Guid,
    pub id: u32,
    pub targets: SpellCastTargets,
}

impl crate::Message for CMSG_PET_CAST_SPELL {
    const OPCODE: u32 = 0x01f0;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // targets: SpellCastTargets
        self.targets.write_into_vec(w)?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // targets: SpellCastTargets
        let targets = SpellCastTargets::read(r)?;

        Ok(Self {
            guid,
            id,
            targets,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_PET_CAST_SPELL {}

impl CMSG_PET_CAST_SPELL {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // id: u32
        + self.targets.size() // targets: SpellCastTargets
    }
}

