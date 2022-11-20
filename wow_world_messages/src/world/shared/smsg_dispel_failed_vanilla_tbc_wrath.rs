use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_dispel_failed.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_dispel_failed.wowm#L3):
/// ```text
/// smsg SMSG_DISPEL_FAILED = 0x0262 {
///     Guid caster;
///     Guid target;
///     u32[-] spells;
/// }
/// ```
pub struct SMSG_DISPEL_FAILED {
    pub caster: Guid,
    pub target: Guid,
    pub spells: Vec<u32>,
}

impl crate::Message for SMSG_DISPEL_FAILED {
    const OPCODE: u32 = 0x0262;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // caster: Guid
        w.write_all(&self.caster.guid().to_le_bytes())?;

        // target: Guid
        w.write_all(&self.target.guid().to_le_bytes())?;

        // spells: u32[-]
        for i in self.spells.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(16..=65551).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0262, size: body_size as u32 });
        }

        // caster: Guid
        let caster = Guid::read(r)?;

        // target: Guid
        let target = Guid::read(r)?;

        // spells: u32[-]
        let mut current_size = {
            8 // caster: Guid
            + 8 // target: Guid
        };
        let mut spells = Vec::with_capacity(body_size as usize - current_size);
        while current_size < (body_size as usize) {
            spells.push(crate::util::read_u32_le(r)?);
            current_size += 1;
        }

        Ok(Self {
            caster,
            target,
            spells,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_DISPEL_FAILED {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_DISPEL_FAILED {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_DISPEL_FAILED {}

impl SMSG_DISPEL_FAILED {
    pub(crate) fn size(&self) -> usize {
        8 // caster: Guid
        + 8 // target: Guid
        + self.spells.len() * core::mem::size_of::<u32>() // spells: u32[-]
    }
}

