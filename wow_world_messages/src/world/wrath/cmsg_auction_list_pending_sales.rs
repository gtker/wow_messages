use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/cmsg/cmsg_auction_list_pending_sales.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/cmsg/cmsg_auction_list_pending_sales.wowm#L1):
/// ```text
/// cmsg CMSG_AUCTION_LIST_PENDING_SALES = 0x048F {
///     Guid auctioneer;
/// }
/// ```
pub struct CMSG_AUCTION_LIST_PENDING_SALES {
    pub auctioneer: Guid,
}

#[cfg(feature = "print-testcase")]
impl CMSG_AUCTION_LIST_PENDING_SALES {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_AUCTION_LIST_PENDING_SALES {{").unwrap();
        // Members
        writeln!(s, "    auctioneer = {};", self.auctioneer.guid()).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 14_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1167_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
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
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for CMSG_AUCTION_LIST_PENDING_SALES {}
impl crate::Message for CMSG_AUCTION_LIST_PENDING_SALES {
    const OPCODE: u32 = 0x048f;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // auctioneer: Guid
        w.write_all(&self.auctioneer.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x048F, size: body_size });
        }

        // auctioneer: Guid
        let auctioneer = crate::util::read_guid(&mut r)?;

        Ok(Self {
            auctioneer,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_AUCTION_LIST_PENDING_SALES {}

