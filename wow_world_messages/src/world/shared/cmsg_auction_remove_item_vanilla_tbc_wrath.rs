use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/cmsg/cmsg_auction_remove_item.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/cmsg/cmsg_auction_remove_item.wowm#L3):
/// ```text
/// cmsg CMSG_AUCTION_REMOVE_ITEM = 0x0257 {
///     Guid auctioneer;
///     u32 auction_id;
/// }
/// ```
pub struct CMSG_AUCTION_REMOVE_ITEM {
    pub auctioneer: Guid,
    pub auction_id: u32,
}

impl crate::private::Sealed for CMSG_AUCTION_REMOVE_ITEM {}
impl CMSG_AUCTION_REMOVE_ITEM {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 12 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // auctioneer: Guid
        let auctioneer = crate::util::read_guid(&mut r)?;

        // auction_id: u32
        let auction_id = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            auctioneer,
            auction_id,
        })
    }

}

impl crate::Message for CMSG_AUCTION_REMOVE_ITEM {
    const OPCODE: u32 = 0x0257;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_AUCTION_REMOVE_ITEM {{").unwrap();
        // Members
        writeln!(s, "    auctioneer = {};", self.auctioneer.guid()).unwrap();
        writeln!(s, "    auction_id = {};", self.auction_id).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 16_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 599_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "auctioneer", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "auction_id", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // auctioneer: Guid
        w.write_all(&self.auctioneer.guid().to_le_bytes())?;

        // auction_id: u32
        w.write_all(&self.auction_id.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(599, "CMSG_AUCTION_REMOVE_ITEM", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_AUCTION_REMOVE_ITEM {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_AUCTION_REMOVE_ITEM {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_AUCTION_REMOVE_ITEM {}

