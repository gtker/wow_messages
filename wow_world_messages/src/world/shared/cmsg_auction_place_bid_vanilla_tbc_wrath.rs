use std::io::{Read, Write};

use crate::Guid;
use crate::shared::gold_vanilla_tbc_wrath::Gold;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/cmsg/cmsg_auction_place_bid.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/cmsg/cmsg_auction_place_bid.wowm#L3):
/// ```text
/// cmsg CMSG_AUCTION_PLACE_BID = 0x025A {
///     Guid auctioneer;
///     u32 auction_id;
///     Gold price;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_AUCTION_PLACE_BID {
    pub auctioneer: Guid,
    pub auction_id: u32,
    pub price: Gold,
}

impl crate::private::Sealed for CMSG_AUCTION_PLACE_BID {}
impl CMSG_AUCTION_PLACE_BID {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 16 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // auctioneer: Guid
        let auctioneer = crate::util::read_guid(&mut r)?;

        // auction_id: u32
        let auction_id = crate::util::read_u32_le(&mut r)?;

        // price: Gold
        let price = Gold::new(crate::util::read_u32_le(&mut r)?);

        Ok(Self {
            auctioneer,
            auction_id,
            price,
        })
    }

}

impl crate::Message for CMSG_AUCTION_PLACE_BID {
    const OPCODE: u32 = 0x025a;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_AUCTION_PLACE_BID"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_AUCTION_PLACE_BID {{").unwrap();
        // Members
        writeln!(s, "    auctioneer = {};", self.auctioneer.guid()).unwrap();
        writeln!(s, "    auction_id = {};", self.auction_id).unwrap();
        writeln!(s, "    price = {};", self.price.as_int()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 20_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 602_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "auctioneer", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "auction_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "price", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // auctioneer: Guid
        w.write_all(&self.auctioneer.guid().to_le_bytes())?;

        // auction_id: u32
        w.write_all(&self.auction_id.to_le_bytes())?;

        // price: Gold
        w.write_all((self.price.as_int()).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(602, "CMSG_AUCTION_PLACE_BID", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_AUCTION_PLACE_BID {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_AUCTION_PLACE_BID {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_AUCTION_PLACE_BID {}

