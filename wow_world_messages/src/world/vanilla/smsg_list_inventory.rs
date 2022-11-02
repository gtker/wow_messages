use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::ListInventoryItem;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// if `amount_of_items` is 0 it is supposedly followed by a single u8 with 0 to say that vendor has no items.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_list_inventory.wowm:30`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_list_inventory.wowm#L30):
/// ```text
/// smsg SMSG_LIST_INVENTORY = 0x019F {
///     Guid vendor;
///     u8 amount_of_items;
///     ListInventoryItem[amount_of_items] items;
/// }
/// ```
pub struct SMSG_LIST_INVENTORY {
    pub vendor: Guid,
    pub items: Vec<ListInventoryItem>,
}

impl crate::Message for SMSG_LIST_INVENTORY {
    const OPCODE: u32 = 0x019f;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // vendor: Guid
        w.write_all(&self.vendor.guid().to_le_bytes())?;

        // amount_of_items: u8
        w.write_all(&(self.items.len() as u8).to_le_bytes())?;

        // items: ListInventoryItem[amount_of_items]
        for i in self.items.iter() {
            i.write_into_vec(w)?;
        }

        assert_eq!(self.size() as usize, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // vendor: Guid
        let vendor = Guid::read(r)?;

        // amount_of_items: u8
        let amount_of_items = crate::util::read_u8_le(r)?;

        // items: ListInventoryItem[amount_of_items]
        let mut items = Vec::with_capacity(amount_of_items as usize);
        for i in 0..amount_of_items {
            items.push(ListInventoryItem::read(r)?);
        }

        Ok(Self {
            vendor,
            items,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_LIST_INVENTORY {}

impl SMSG_LIST_INVENTORY {
    pub(crate) fn size(&self) -> usize {
        8 // vendor: Guid
        + 1 // amount_of_items: u8
        + self.items.len() * 28 // items: ListInventoryItem[amount_of_items]
    }
}

