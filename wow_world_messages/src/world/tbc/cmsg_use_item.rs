use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::tbc::SpellCastTargets;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_use_item.wowm:10`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_use_item.wowm#L10):
/// ```text
/// cmsg CMSG_USE_ITEM = 0x00AB {
///     u8 bag_index;
///     u8 bag_slot;
///     u8 spell_index;
///     u8 cast_count;
///     Guid item;
///     SpellCastTargets targets;
/// }
/// ```
pub struct CMSG_USE_ITEM {
    pub bag_index: u8,
    pub bag_slot: u8,
    pub spell_index: u8,
    /// mangosone: next cast if exists (single or not)
    ///
    pub cast_count: u8,
    pub item: Guid,
    pub targets: SpellCastTargets,
}

impl crate::Message for CMSG_USE_ITEM {
    const OPCODE: u32 = 0x00ab;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // bag_index: u8
        w.write_all(&self.bag_index.to_le_bytes())?;

        // bag_slot: u8
        w.write_all(&self.bag_slot.to_le_bytes())?;

        // spell_index: u8
        w.write_all(&self.spell_index.to_le_bytes())?;

        // cast_count: u8
        w.write_all(&self.cast_count.to_le_bytes())?;

        // item: Guid
        w.write_all(&self.item.guid().to_le_bytes())?;

        // targets: SpellCastTargets
        self.targets.write_into_vec(w)?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(14..=330).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00AB, size: body_size as u32 });
        }

        // bag_index: u8
        let bag_index = crate::util::read_u8_le(r)?;

        // bag_slot: u8
        let bag_slot = crate::util::read_u8_le(r)?;

        // spell_index: u8
        let spell_index = crate::util::read_u8_le(r)?;

        // cast_count: u8
        let cast_count = crate::util::read_u8_le(r)?;

        // item: Guid
        let item = Guid::read(r)?;

        // targets: SpellCastTargets
        let targets = SpellCastTargets::read(r)?;

        Ok(Self {
            bag_index,
            bag_slot,
            spell_index,
            cast_count,
            item,
            targets,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_USE_ITEM {}

impl CMSG_USE_ITEM {
    pub(crate) fn size(&self) -> usize {
        1 // bag_index: u8
        + 1 // bag_slot: u8
        + 1 // spell_index: u8
        + 1 // cast_count: u8
        + 8 // item: Guid
        + self.targets.size() // targets: SpellCastTargets
    }
}

