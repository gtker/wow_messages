use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::wrath::AuraUpdate;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_aura_update.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_aura_update.wowm#L1):
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

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // unit: PackedGuid
        self.unit.write_packed_guid_into_vec(w);

        // aura_update: AuraUpdate
        self.aura_update.write_into_vec(w)?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(10..=34).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0496, size: body_size as u32 });
        }

        // unit: PackedGuid
        let unit = Guid::read_packed(r)?;

        // aura_update: AuraUpdate
        let aura_update = AuraUpdate::read(r)?;

        Ok(Self {
            unit,
            aura_update,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_AURA_UPDATE {}

impl SMSG_AURA_UPDATE {
    pub(crate) fn size(&self) -> usize {
        self.unit.size() // unit: Guid
        + self.aura_update.size() // aura_update: AuraUpdate
    }
}

