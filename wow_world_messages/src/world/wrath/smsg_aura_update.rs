use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::AuraUpdate;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_aura_update.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_aura_update.wowm#L1):
/// ```text
/// smsg SMSG_AURA_UPDATE = 0x0496 {
///     PackedGuid unit;
///     AuraUpdate aura_update;
/// }
/// ```
pub struct SMSG_AURA_UPDATE {
    pub unit: Guid,
    pub aura_update: AuraUpdate,
}

impl crate::Message for SMSG_AURA_UPDATE {
    const OPCODE: u32 = 0x0496;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // unit: PackedGuid
        self.unit.write_packed_guid_into_vec(&mut w)?;

        // aura_update: AuraUpdate
        self.aura_update.write_into_vec(&mut w)?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(10..=34).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0496, size: body_size as u32 });
        }

        // unit: PackedGuid
        let unit = Guid::read_packed(&mut r)?;

        // aura_update: AuraUpdate
        let aura_update = AuraUpdate::read(&mut r)?;

        Ok(Self {
            unit,
            aura_update,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_AURA_UPDATE {}

impl SMSG_AURA_UPDATE {
    pub(crate) fn size(&self) -> usize {
        self.unit.size() // unit: PackedGuid
        + self.aura_update.size() // aura_update: AuraUpdate
    }
}

