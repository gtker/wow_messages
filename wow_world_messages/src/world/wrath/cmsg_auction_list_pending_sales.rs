use std::io::{Read, Write};

use crate::Guid;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/cmsg/cmsg_auction_list_pending_sales.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/cmsg/cmsg_auction_list_pending_sales.wowm#L1):
/// ```text
/// cmsg CMSG_AUCTION_LIST_PENDING_SALES = 0x048F {
///     Guid auctioneer;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_AUCTION_LIST_PENDING_SALES {
    pub auctioneer: Guid,
}

impl crate::private::Sealed for CMSG_AUCTION_LIST_PENDING_SALES {}
impl CMSG_AUCTION_LIST_PENDING_SALES {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 8 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // auctioneer: Guid
        let auctioneer = crate::util::read_guid(&mut r)?;

        Ok(Self {
            auctioneer,
        })
    }

}

impl crate::Message for CMSG_AUCTION_LIST_PENDING_SALES {
    const OPCODE: u32 = 0x048f;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_AUCTION_LIST_PENDING_SALES"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_AUCTION_LIST_PENDING_SALES {{").unwrap();
        // Members
        writeln!(s, "    auctioneer = {};", self.auctioneer.guid()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 12_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1167_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "auctioneer", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // auctioneer: Guid
        w.write_all(&self.auctioneer.guid().to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1167, "CMSG_AUCTION_LIST_PENDING_SALES", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_AUCTION_LIST_PENDING_SALES {}

