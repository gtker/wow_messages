use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::EnchantMask;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_inspect_talent.wowm:23`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_inspect_talent.wowm#L23):
/// ```text
/// struct InspectTalentGear {
///     Item item;
///     EnchantMask enchant_mask;
///     u16 unknown1;
///     PackedGuid creator;
///     u32 unknown2;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct InspectTalentGear {
    pub item: u32,
    pub enchant_mask: EnchantMask,
    pub unknown1: u16,
    pub creator: Guid,
    pub unknown2: u32,
}

impl InspectTalentGear {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // item: Item
        w.write_all(&self.item.to_le_bytes())?;

        // enchant_mask: EnchantMask
        self.enchant_mask.write_into_vec(&mut w)?;

        // unknown1: u16
        w.write_all(&self.unknown1.to_le_bytes())?;

        // creator: PackedGuid
        crate::util::write_packed_guid(&self.creator, &mut w)?;

        // unknown2: u32
        w.write_all(&self.unknown2.to_le_bytes())?;

        Ok(())
    }
}

impl InspectTalentGear {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, std::io::Error> {
        // item: Item
        let item = crate::util::read_u32_le(&mut r)?;

        // enchant_mask: EnchantMask
        let enchant_mask = EnchantMask::read(&mut r)?;

        // unknown1: u16
        let unknown1 = crate::util::read_u16_le(&mut r)?;

        // creator: PackedGuid
        let creator = crate::util::read_packed_guid(&mut r)?;

        // unknown2: u32
        let unknown2 = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            item,
            enchant_mask,
            unknown1,
            creator,
            unknown2,
        })
    }

}

impl InspectTalentGear {
    pub(crate) const fn size(&self) -> usize {
        4 // item: Item
        + self.enchant_mask.size() // enchant_mask: EnchantMask
        + 2 // unknown1: u16
        + crate::util::packed_guid_size(&self.creator) // creator: PackedGuid
        + 4 // unknown2: u32
    }
}

