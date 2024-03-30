use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::AuctionHouse;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/smsg/smsg_auction_bidder_notification.wowm:39`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/smsg/smsg_auction_bidder_notification.wowm#L39):
/// ```text
/// smsg SMSG_AUCTION_BIDDER_NOTIFICATION = 0x025E {
///     AuctionHouse auction_house;
///     u32 auction_id;
///     Guid bidder;
///     u32 bid_sum;
///     u32 new_highest_bid;
///     u32 out_bid_amount;
///     u32 item_template;
///     u32 item_random_property_id;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_AUCTION_BIDDER_NOTIFICATION {
    pub auction_house: AuctionHouse,
    pub auction_id: u32,
    pub bidder: Guid,
    pub bid_sum: u32,
    pub new_highest_bid: u32,
    pub out_bid_amount: u32,
    pub item_template: u32,
    pub item_random_property_id: u32,
}

impl crate::private::Sealed for SMSG_AUCTION_BIDDER_NOTIFICATION {}
impl SMSG_AUCTION_BIDDER_NOTIFICATION {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 36 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // auction_house: AuctionHouse
        let auction_house = crate::util::read_u32_le(&mut r)?.try_into()?;

        // auction_id: u32
        let auction_id = crate::util::read_u32_le(&mut r)?;

        // bidder: Guid
        let bidder = crate::util::read_guid(&mut r)?;

        // bid_sum: u32
        let bid_sum = crate::util::read_u32_le(&mut r)?;

        // new_highest_bid: u32
        let new_highest_bid = crate::util::read_u32_le(&mut r)?;

        // out_bid_amount: u32
        let out_bid_amount = crate::util::read_u32_le(&mut r)?;

        // item_template: u32
        let item_template = crate::util::read_u32_le(&mut r)?;

        // item_random_property_id: u32
        let item_random_property_id = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            auction_house,
            auction_id,
            bidder,
            bid_sum,
            new_highest_bid,
            out_bid_amount,
            item_template,
            item_random_property_id,
        })
    }

}

impl crate::Message for SMSG_AUCTION_BIDDER_NOTIFICATION {
    const OPCODE: u32 = 0x025e;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_AUCTION_BIDDER_NOTIFICATION"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_AUCTION_BIDDER_NOTIFICATION {{").unwrap();
        // Members
        writeln!(s, "    auction_house = {};", self.auction_house.as_test_case_value()).unwrap();
        writeln!(s, "    auction_id = {};", self.auction_id).unwrap();
        writeln!(s, "    bidder = {};", self.bidder.guid()).unwrap();
        writeln!(s, "    bid_sum = {};", self.bid_sum).unwrap();
        writeln!(s, "    new_highest_bid = {};", self.new_highest_bid).unwrap();
        writeln!(s, "    out_bid_amount = {};", self.out_bid_amount).unwrap();
        writeln!(s, "    item_template = {};", self.item_template).unwrap();
        writeln!(s, "    item_random_property_id = {};", self.item_random_property_id).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 38_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 606_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "auction_house", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "auction_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "bidder", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "bid_sum", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "new_highest_bid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "out_bid_amount", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "item_template", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "item_random_property_id", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        36
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // auction_house: AuctionHouse
        w.write_all(&(self.auction_house.as_int().to_le_bytes()))?;

        // auction_id: u32
        w.write_all(&self.auction_id.to_le_bytes())?;

        // bidder: Guid
        w.write_all(&self.bidder.guid().to_le_bytes())?;

        // bid_sum: u32
        w.write_all(&self.bid_sum.to_le_bytes())?;

        // new_highest_bid: u32
        w.write_all(&self.new_highest_bid.to_le_bytes())?;

        // out_bid_amount: u32
        w.write_all(&self.out_bid_amount.to_le_bytes())?;

        // item_template: u32
        w.write_all(&self.item_template.to_le_bytes())?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(606, "SMSG_AUCTION_BIDDER_NOTIFICATION", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_AUCTION_BIDDER_NOTIFICATION {}

