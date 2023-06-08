use std::io::{Read, Write};

use crate::Guid;
use wow_world_base::shared::auction_house_vanilla_tbc_wrath::AuctionHouse;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/smsg/smsg_auction_bidder_notification.wowm:27`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/smsg/smsg_auction_bidder_notification.wowm#L27):
/// ```text
/// smsg SMSG_AUCTION_BIDDER_NOTIFICATION = 0x025E {
///     AuctionHouse auction_house;
///     u32 auction_id;
///     Guid bidder;
///     u32 won;
///     u32 out_bid;
///     u32 item_template;
///     u32 item_random_property_id;
/// }
/// ```
pub struct SMSG_AUCTION_BIDDER_NOTIFICATION {
    pub auction_house: AuctionHouse,
    pub auction_id: u32,
    pub bidder: Guid,
    /// vmangos/cmangos: if 0, client shows ERR_AUCTION_WON_S, else ERR_AUCTION_OUTBID_S
    pub won: u32,
    pub out_bid: u32,
    pub item_template: u32,
    pub item_random_property_id: u32,
}

#[cfg(feature = "print-testcase")]
impl SMSG_AUCTION_BIDDER_NOTIFICATION {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_AUCTION_BIDDER_NOTIFICATION {{").unwrap();
        // Members
        writeln!(s, "    auction_house = {};", self.auction_house.as_test_case_value()).unwrap();
        writeln!(s, "    auction_id = {};", self.auction_id).unwrap();
        writeln!(s, "    bidder = {};", self.bidder.guid()).unwrap();
        writeln!(s, "    won = {};", self.won).unwrap();
        writeln!(s, "    out_bid = {};", self.out_bid).unwrap();
        writeln!(s, "    item_template = {};", self.item_template).unwrap();
        writeln!(s, "    item_random_property_id = {};", self.item_random_property_id).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 34_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 606_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "auction_house", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "auction_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "bidder", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "won", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "out_bid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "item_template", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "item_random_property_id", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12 2".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_AUCTION_BIDDER_NOTIFICATION {}
impl crate::Message for SMSG_AUCTION_BIDDER_NOTIFICATION {
    const OPCODE: u32 = 0x025e;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_AUCTION_BIDDER_NOTIFICATION::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // auction_house: AuctionHouse
        w.write_all(&(self.auction_house.as_int().to_le_bytes()))?;

        // auction_id: u32
        w.write_all(&self.auction_id.to_le_bytes())?;

        // bidder: Guid
        w.write_all(&self.bidder.guid().to_le_bytes())?;

        // won: u32
        w.write_all(&self.won.to_le_bytes())?;

        // out_bid: u32
        w.write_all(&self.out_bid.to_le_bytes())?;

        // item_template: u32
        w.write_all(&self.item_template.to_le_bytes())?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 32 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x025E, size: body_size });
        }

        // auction_house: AuctionHouse
        let auction_house = crate::util::read_u32_le(&mut r)?.try_into()?;

        // auction_id: u32
        let auction_id = crate::util::read_u32_le(&mut r)?;

        // bidder: Guid
        let bidder = crate::util::read_guid(&mut r)?;

        // won: u32
        let won = crate::util::read_u32_le(&mut r)?;

        // out_bid: u32
        let out_bid = crate::util::read_u32_le(&mut r)?;

        // item_template: u32
        let item_template = crate::util::read_u32_le(&mut r)?;

        // item_random_property_id: u32
        let item_random_property_id = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            auction_house,
            auction_id,
            bidder,
            won,
            out_bid,
            item_template,
            item_random_property_id,
        })
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_AUCTION_BIDDER_NOTIFICATION {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_AUCTION_BIDDER_NOTIFICATION {}

