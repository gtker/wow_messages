use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
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

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // caster: Guid
        w.write_all(&self.caster.guid().to_le_bytes())?;

        // target: Guid
        w.write_all(&self.target.guid().to_le_bytes())?;

        // spells: u32[-]
        for i in self.spells.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(16..=65551).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0262, size: body_size as u32 });
        }

        // caster: Guid
        let caster = Guid::read(&mut r)?;

        // target: Guid
        let target = Guid::read(&mut r)?;

        // spells: u32[-]
        let spells = {
            let mut current_size = {
                8 // caster: Guid
                + 8 // target: Guid
            };
            let mut spells = Vec::with_capacity(body_size as usize - current_size);
            while current_size < (body_size as usize) {
                spells.push(crate::util::read_u32_le(&mut r)?);
                current_size += 1;
            }
            spells
        };

        Ok(Self {
            caster,
            target,
            spells,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_DISPEL_FAILED {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_DISPEL_FAILED {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_DISPEL_FAILED {}

impl SMSG_DISPEL_FAILED {
    pub(crate) fn size(&self) -> usize {
        8 // caster: Guid
        + 8 // target: Guid
        + self.spells.len() * core::mem::size_of::<u32>() // spells: u32[-]
    }
}

