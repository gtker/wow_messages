use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_item_refund_info_response.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_item_refund_info_response.wowm#L1):
/// ```text
/// struct ItemRefundExtra {
///     u32 item;
///     u32 amount;
/// }
/// ```
pub struct ItemRefundExtra {
    pub item: u32,
    pub amount: u32,
}

impl ItemRefundExtra {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        // amount: u32
        w.write_all(&self.amount.to_le_bytes())?;

        Ok(())
    }
}

impl ItemRefundExtra {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // item: u32
        let item = crate::util::read_u32_le(r)?;

        // amount: u32
        let amount = crate::util::read_u32_le(r)?;

        Ok(Self {
            item,
            amount,
        })
    }

}

