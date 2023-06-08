use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/cmsg/cmsg_auction_sell_item.wowm:11`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/cmsg/cmsg_auction_sell_item.wowm#L11):
/// ```text
/// cmsg CMSG_AUCTION_SELL_ITEM = 0x0256 {
///     Guid auctioneer;
///     u32 unknown1;
///     Guid item;
///     u32 unknown2;
///     u32 starting_bid;
///     u32 buyout;
///     u32 auction_duration_in_minutes;
/// }
/// ```
pub struct CMSG_AUCTION_SELL_ITEM {
    pub auctioneer: Guid,
    pub unknown1: u32,
    pub item: Guid,
    pub unknown2: u32,
    pub starting_bid: u32,
    pub buyout: u32,
    pub auction_duration_in_minutes: u32,
}

#[cfg(feature = "print-testcase")]
impl CMSG_AUCTION_SELL_ITEM {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_AUCTION_SELL_ITEM {{").unwrap();
        // Members
        writeln!(s, "    auctioneer = {};", self.auctioneer.guid()).unwrap();
        writeln!(s, "    unknown1 = {};", self.unknown1).unwrap();
        writeln!(s, "    item = {};", self.item.guid()).unwrap();
        writeln!(s, "    unknown2 = {};", self.unknown2).unwrap();
        writeln!(s, "    starting_bid = {};", self.starting_bid).unwrap();
        writeln!(s, "    buyout = {};", self.buyout).unwrap();
        writeln!(s, "    auction_duration_in_minutes = {};", self.auction_duration_in_minutes).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 40_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 598_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "auctioneer", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown1", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "item", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown2", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "starting_bid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "buyout", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "auction_duration_in_minutes", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for CMSG_AUCTION_SELL_ITEM {}
impl crate::Message for CMSG_AUCTION_SELL_ITEM {
    const OPCODE: u32 = 0x0256;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        CMSG_AUCTION_SELL_ITEM::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        36
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // auctioneer: Guid
        w.write_all(&self.auctioneer.guid().to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // item: Guid
        w.write_all(&self.item.guid().to_le_bytes())?;

        // unknown2: u32
        w.write_all(&self.unknown2.to_le_bytes())?;

        // starting_bid: u32
        w.write_all(&self.starting_bid.to_le_bytes())?;

        // buyout: u32
        w.write_all(&self.buyout.to_le_bytes())?;

        // auction_duration_in_minutes: u32
        w.write_all(&self.auction_duration_in_minutes.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 36 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0256, size: body_size });
        }

        // auctioneer: Guid
        let auctioneer = crate::util::read_guid(&mut r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(&mut r)?;

        // item: Guid
        let item = crate::util::read_guid(&mut r)?;

        // unknown2: u32
        let unknown2 = crate::util::read_u32_le(&mut r)?;

        // starting_bid: u32
        let starting_bid = crate::util::read_u32_le(&mut r)?;

        // buyout: u32
        let buyout = crate::util::read_u32_le(&mut r)?;

        // auction_duration_in_minutes: u32
        let auction_duration_in_minutes = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            auctioneer,
            unknown1,
            item,
            unknown2,
            starting_bid,
            buyout,
            auction_duration_in_minutes,
        })
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_AUCTION_SELL_ITEM {}

