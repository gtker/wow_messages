use crate::Guid;
use crate::shared::loot_item_vanilla_tbc_wrath::LootItem;
use wow_world_base::shared::loot_method_vanilla_tbc_wrath::LootMethod;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/smsg_loot_response.wowm:50`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/smsg_loot_response.wowm#L50):
/// ```text
/// smsg SMSG_LOOT_RESPONSE = 0x0160 {
///     Guid guid;
///     LootMethod loot_method;
///     u32 gold;
///     u8 amount_of_items;
///     LootItem[amount_of_items] items;
/// }
/// ```
pub struct SMSG_LOOT_RESPONSE {
    pub guid: Guid,
    pub loot_method: LootMethod,
    pub gold: u32,
    pub items: Vec<LootItem>,
}

impl crate::Message for SMSG_LOOT_RESPONSE {
    const OPCODE: u32 = 0x0160;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // loot_method: LootMethod
        w.write_all(&(self.loot_method.as_int() as u8).to_le_bytes())?;

        // gold: u32
        w.write_all(&self.gold.to_le_bytes())?;

        // amount_of_items: u8
        w.write_all(&(self.items.len() as u8).to_le_bytes())?;

        // items: LootItem[amount_of_items]
        for i in self.items.iter() {
            i.write_into_vec(w)?;
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(14..=1550).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0160, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        // loot_method: LootMethod
        let loot_method: LootMethod = crate::util::read_u8_le(r)?.try_into()?;

        // gold: u32
        let gold = crate::util::read_u32_le(r)?;

        // amount_of_items: u8
        let amount_of_items = crate::util::read_u8_le(r)?;

        // items: LootItem[amount_of_items]
        let items = {
            let mut items = Vec::with_capacity(amount_of_items as usize);
            for i in 0..amount_of_items {
                items.push(LootItem::read(r)?);
            }
            items
        };
        Ok(Self {
            guid,
            loot_method,
            gold,
            items,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_LOOT_RESPONSE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_LOOT_RESPONSE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_LOOT_RESPONSE {}

impl SMSG_LOOT_RESPONSE {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + 1 // loot_method: LootMethod
        + 4 // gold: u32
        + 1 // amount_of_items: u8
        + self.items.len() * 6 // items: LootItem[amount_of_items]
    }
}

