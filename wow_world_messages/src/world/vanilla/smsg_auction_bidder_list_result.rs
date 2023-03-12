use std::io::{Read, Write};

use crate::vanilla::AuctionListItem;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/smsg/smsg_auction_bidder_list_result.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/smsg/smsg_auction_bidder_list_result.wowm#L1):
/// ```text
/// smsg SMSG_AUCTION_BIDDER_LIST_RESULT = 0x0265 {
///     u32 count;
///     AuctionListItem[count] auctions;
///     u32 total_amount_of_auctions;
/// }
/// ```
pub struct SMSG_AUCTION_BIDDER_LIST_RESULT {
    pub auctions: Vec<AuctionListItem>,
    pub total_amount_of_auctions: u32,
}

impl crate::Message for SMSG_AUCTION_BIDDER_LIST_RESULT {
    const OPCODE: u32 = 0x0265;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // count: u32
        w.write_all(&(self.auctions.len() as u32).to_le_bytes())?;

        // auctions: AuctionListItem[count]
        for i in self.auctions.iter() {
            i.write_into_vec(&mut w)?;
        }

        // total_amount_of_auctions: u32
        w.write_all(&self.total_amount_of_auctions.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(8..=4294967294).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0265, size: body_size as u32 });
        }

        // count: u32
        let count = crate::util::read_u32_le(&mut r)?;

        // auctions: AuctionListItem[count]
        let auctions = {
            let mut auctions = Vec::with_capacity(count as usize);
            for i in 0..count {
                auctions.push(AuctionListItem::read(&mut r)?);
            }
            auctions
        };

        // total_amount_of_auctions: u32
        let total_amount_of_auctions = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            auctions,
            total_amount_of_auctions,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_AUCTION_BIDDER_LIST_RESULT {}

impl SMSG_AUCTION_BIDDER_LIST_RESULT {
    pub(crate) fn size(&self) -> usize {
        4 // count: u32
        + self.auctions.len() * 64 // auctions: AuctionListItem[count]
        + 4 // total_amount_of_auctions: u32
    }
}

