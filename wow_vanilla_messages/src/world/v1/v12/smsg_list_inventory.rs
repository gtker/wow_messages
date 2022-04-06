use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::ListInventoryItem;
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_list_inventory.wowm:15`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_list_inventory.wowm#L15):
/// ```text
/// smsg SMSG_LIST_INVENTORY = 0x19F {
///     Guid vendor;
///     u8 amount_of_items;
///     ListInventoryItem[amount_of_items] items;
/// }
/// ```
pub struct SMSG_LIST_INVENTORY {
    pub vendor: Guid,
    pub amount_of_items: u8,
    pub items: Vec<ListInventoryItem>,
}

impl WorldServerMessageWrite for SMSG_LIST_INVENTORY {
    const OPCODE: u16 = 0x19f;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (self.size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (self.size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_LIST_INVENTORY {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
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
            amount_of_items,
            items,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // vendor: Guid
        self.vendor.write(w)?;

        // amount_of_items: u8
        w.write_all(&(self.items.len() as u8).to_le_bytes())?;

        // items: ListInventoryItem[amount_of_items]
        for i in self.items.iter() {
            i.write(w)?;
        }

        Ok(())
    }
}

impl VariableSized for SMSG_LIST_INVENTORY {
    fn size(&self) -> usize {
        8 // vendor: Guid
        + 1 // amount_of_items: u8
        + self.items.iter().fold(0, |acc, x| acc + ListInventoryItem::size()) // items: ListInventoryItem[amount_of_items]
    }
}

impl MaximumPossibleSized for SMSG_LIST_INVENTORY {
    fn maximum_possible_size() -> usize {
        8 // vendor: Guid
        + 1 // amount_of_items: u8
        + 255 * ListInventoryItem::maximum_possible_size() // items: ListInventoryItem[amount_of_items]
    }
}

