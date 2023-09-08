use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/auction_common.wowm:20`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/auction_common.wowm#L20):
/// ```text
/// struct AuctionEnchantment {
///     u32 enchant_id;
///     u32 enchant_duration;
///     u32 enchant_charges;
/// }
/// ```
pub struct AuctionEnchantment {
    pub enchant_id: u32,
    pub enchant_duration: u32,
    pub enchant_charges: u32,
}

impl AuctionEnchantment {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // enchant_id: u32
        w.write_all(&self.enchant_id.to_le_bytes())?;

        // enchant_duration: u32
        w.write_all(&self.enchant_duration.to_le_bytes())?;

        // enchant_charges: u32
        w.write_all(&self.enchant_charges.to_le_bytes())?;

        Ok(())
    }
}

impl AuctionEnchantment {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, std::io::Error> {
        // enchant_id: u32
        let enchant_id = crate::util::read_u32_le(&mut r)?;

        // enchant_duration: u32
        let enchant_duration = crate::util::read_u32_le(&mut r)?;

        // enchant_charges: u32
        let enchant_charges = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            enchant_id,
            enchant_duration,
            enchant_charges,
        })
    }

}

