use std::convert::{TryFrom, TryInto};
use crate::world::wrath::PendingAuctionSale;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/smsg/smsg_auction_list_pending_sales.wowm:19`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/smsg/smsg_auction_list_pending_sales.wowm#L19):
/// ```text
/// smsg SMSG_AUCTION_LIST_PENDING_SALES = 0x0490 {
///     u32 amount_of_pending_sales;
///     PendingAuctionSale[amount_of_pending_sales] pending_sales;
/// }
/// ```
pub struct SMSG_AUCTION_LIST_PENDING_SALES {
    pub pending_sales: Vec<PendingAuctionSale>,
}

impl crate::Message for SMSG_AUCTION_LIST_PENDING_SALES {
    const OPCODE: u32 = 0x0490;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // amount_of_pending_sales: u32
        w.write_all(&(self.pending_sales.len() as u32).to_le_bytes())?;

        // pending_sales: PendingAuctionSale[amount_of_pending_sales]
        for i in self.pending_sales.iter() {
            i.write_into_vec(w)?;
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(4..=4294967294).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0490, size: body_size as u32 });
        }

        // amount_of_pending_sales: u32
        let amount_of_pending_sales = crate::util::read_u32_le(r)?;

        // pending_sales: PendingAuctionSale[amount_of_pending_sales]
        let mut pending_sales = Vec::with_capacity(amount_of_pending_sales as usize);
        for i in 0..amount_of_pending_sales {
            pending_sales.push(PendingAuctionSale::read(r)?);
        }

        Ok(Self {
            pending_sales,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_AUCTION_LIST_PENDING_SALES {}

impl SMSG_AUCTION_LIST_PENDING_SALES {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_pending_sales: u32
        + self.pending_sales.iter().fold(0, |acc, x| acc + x.size()) // pending_sales: PendingAuctionSale[amount_of_pending_sales]
    }
}

