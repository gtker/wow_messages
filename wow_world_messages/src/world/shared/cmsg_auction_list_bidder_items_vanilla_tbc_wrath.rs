use std::io::{Read, Write};
use crate::Guid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/cmsg/cmsg_auction_list_bidder_items.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/cmsg/cmsg_auction_list_bidder_items.wowm#L3):
/// ```text
/// cmsg CMSG_AUCTION_LIST_BIDDER_ITEMS = 0x0264 {
///     Guid auctioneer;
///     u32 start_from_page;
///     u32 amount_of_outbid_items;
///     u32[amount_of_outbid_items] outbid_item_ids;
/// }
/// ```
pub struct CMSG_AUCTION_LIST_BIDDER_ITEMS {
    pub auctioneer: Guid,
    pub start_from_page: u32,
    pub outbid_item_ids: Vec<u32>,
}

impl crate::Message for CMSG_AUCTION_LIST_BIDDER_ITEMS {
    const OPCODE: u32 = 0x0264;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // auctioneer: Guid
        w.write_all(&self.auctioneer.guid().to_le_bytes())?;

        // start_from_page: u32
        w.write_all(&self.start_from_page.to_le_bytes())?;

        // amount_of_outbid_items: u32
        w.write_all(&(self.outbid_item_ids.len() as u32).to_le_bytes())?;

        // outbid_item_ids: u32[amount_of_outbid_items]
        for i in self.outbid_item_ids.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(16..=4294967294).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0264, size: body_size as u32 });
        }

        // auctioneer: Guid
        let auctioneer = Guid::read(&mut r)?;

        // start_from_page: u32
        let start_from_page = crate::util::read_u32_le(&mut r)?;

        // amount_of_outbid_items: u32
        let amount_of_outbid_items = crate::util::read_u32_le(&mut r)?;

        // outbid_item_ids: u32[amount_of_outbid_items]
        let outbid_item_ids = {
            let mut outbid_item_ids = Vec::with_capacity(amount_of_outbid_items as usize);
            for i in 0..amount_of_outbid_items {
                outbid_item_ids.push(crate::util::read_u32_le(&mut r)?);
            }
            outbid_item_ids
        };

        Ok(Self {
            auctioneer,
            start_from_page,
            outbid_item_ids,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_AUCTION_LIST_BIDDER_ITEMS {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_AUCTION_LIST_BIDDER_ITEMS {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_AUCTION_LIST_BIDDER_ITEMS {}

impl CMSG_AUCTION_LIST_BIDDER_ITEMS {
    pub(crate) fn size(&self) -> usize {
        8 // auctioneer: Guid
        + 4 // start_from_page: u32
        + 4 // amount_of_outbid_items: u32
        + self.outbid_item_ids.len() * core::mem::size_of::<u32>() // outbid_item_ids: u32[amount_of_outbid_items]
    }
}

