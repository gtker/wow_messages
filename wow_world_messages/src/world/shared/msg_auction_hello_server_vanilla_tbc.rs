use std::io::{Read, Write};

use crate::Guid;
use wow_world_base::shared::auction_house_vanilla_tbc_wrath::AuctionHouse;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/msg/msg_auction_hello_server.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/msg/msg_auction_hello_server.wowm#L8):
/// ```text
/// smsg MSG_AUCTION_HELLO_Server = 0x0255 {
///     Guid auctioneer;
///     AuctionHouse auction_house;
/// }
/// ```
pub struct MSG_AUCTION_HELLO_Server {
    pub auctioneer: Guid,
    pub auction_house: AuctionHouse,
}

#[cfg(feature = "print-testcase")]
impl MSG_AUCTION_HELLO_Server {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_AUCTION_HELLO_Server {{").unwrap();
        // Members
        writeln!(s, "    auctioneer = {};", self.auctioneer.guid()).unwrap();
        writeln!(s, "    auction_house = {};", self.auction_house.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 16_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 597_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "auctioneer");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1.12 2 3.0 3.1 3.2 3.3.0 3.3.1 3.3.2\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for MSG_AUCTION_HELLO_Server {}
impl crate::Message for MSG_AUCTION_HELLO_Server {
    const OPCODE: u32 = 0x0255;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // auctioneer: Guid
        w.write_all(&self.auctioneer.guid().to_le_bytes())?;

        // auction_house: AuctionHouse
        w.write_all(&(self.auction_house.as_int().to_le_bytes()))?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0255, size: body_size });
        }

        // auctioneer: Guid
        let auctioneer = crate::util::read_guid(&mut r)?;

        // auction_house: AuctionHouse
        let auction_house = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            auctioneer,
            auction_house,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for MSG_AUCTION_HELLO_Server {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for MSG_AUCTION_HELLO_Server {}

