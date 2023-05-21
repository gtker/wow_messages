use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::AuctionHouse;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/msg/msg_auction_hello_server.wowm:15`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/msg/msg_auction_hello_server.wowm#L15):
/// ```text
/// smsg MSG_AUCTION_HELLO_Server = 0x0255 {
///     Guid auctioneer;
///     AuctionHouse auction_house;
///     Bool auction_house_enabled;
/// }
/// ```
pub struct MSG_AUCTION_HELLO_Server {
    pub auctioneer: Guid,
    pub auction_house: AuctionHouse,
    pub auction_house_enabled: bool,
}

impl crate::private::Sealed for MSG_AUCTION_HELLO_Server {}
impl MSG_AUCTION_HELLO_Server {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 13 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0255, size: body_size });
        }

        // auctioneer: Guid
        let auctioneer = crate::util::read_guid(&mut r)?;

        // auction_house: AuctionHouse
        let auction_house = crate::util::read_u32_le(&mut r)?.try_into()?;

        // auction_house_enabled: Bool
        let auction_house_enabled = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            auctioneer,
            auction_house,
            auction_house_enabled,
        })
    }

}

impl crate::Message for MSG_AUCTION_HELLO_Server {
    const OPCODE: u32 = 0x0255;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_AUCTION_HELLO_Server {{").unwrap();
        // Members
        writeln!(s, "    auctioneer = {};", self.auctioneer.guid()).unwrap();
        writeln!(s, "    auction_house = {};", self.auction_house.as_test_case_value()).unwrap();
        writeln!(s, "    auction_house_enabled = {};", if self.auction_house_enabled { "TRUE" } else { "FALSE" }).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 15_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 597_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "auctioneer", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "auction_house", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "auction_house_enabled", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.3 3.3.4 3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        13
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // auctioneer: Guid
        w.write_all(&self.auctioneer.guid().to_le_bytes())?;

        // auction_house: AuctionHouse
        w.write_all(&(self.auction_house.as_int().to_le_bytes()))?;

        // auction_house_enabled: Bool
        w.write_all(u8::from(self.auction_house_enabled).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_AUCTION_HELLO_Server {}

