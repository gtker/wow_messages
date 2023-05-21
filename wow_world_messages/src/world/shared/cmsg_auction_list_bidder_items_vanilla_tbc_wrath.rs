use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/cmsg/cmsg_auction_list_bidder_items.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/cmsg/cmsg_auction_list_bidder_items.wowm#L3):
/// ```text
/// cmsg CMSG_AUCTION_LIST_BIDDER_ITEMS = 0x0264 {
///     Guid auctioneer;
///     u32 start_from_page;
///     u32 amount_of_outbid_items;
///     u32[amount_of_outbid_items] outbid_item_ids;
/// }
/// ```
pub struct CMSG_AUCTION_LIST_BIDDER_ITEMS {
    pub auctioneer: Guid,
    pub start_from_page: u32,
    pub outbid_item_ids: Vec<u32>,
}

impl crate::private::Sealed for CMSG_AUCTION_LIST_BIDDER_ITEMS {}
impl CMSG_AUCTION_LIST_BIDDER_ITEMS {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(16..=10240).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x0264, size: body_size });
        }

        // auctioneer: Guid
        let auctioneer = crate::util::read_guid(&mut r)?;

        // start_from_page: u32
        let start_from_page = crate::util::read_u32_le(&mut r)?;

        // amount_of_outbid_items: u32
        let amount_of_outbid_items = crate::util::read_u32_le(&mut r)?;

        // outbid_item_ids: u32[amount_of_outbid_items]
        let outbid_item_ids = {
            let mut outbid_item_ids = Vec::with_capacity(amount_of_outbid_items as usize);
            for _ in 0..amount_of_outbid_items {
                outbid_item_ids.push(crate::util::read_u32_le(&mut r)?);
            }
            outbid_item_ids
        };

        Ok(Self {
            auctioneer,
            start_from_page,
            outbid_item_ids,
        })
    }

}

impl crate::Message for CMSG_AUCTION_LIST_BIDDER_ITEMS {
    const OPCODE: u32 = 0x0264;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_AUCTION_LIST_BIDDER_ITEMS {{").unwrap();
        // Members
        writeln!(s, "    auctioneer = {};", self.auctioneer.guid()).unwrap();
        writeln!(s, "    start_from_page = {};", self.start_from_page).unwrap();
        writeln!(s, "    amount_of_outbid_items = {};", self.outbid_item_ids.len()).unwrap();
        write!(s, "    outbid_item_ids = [").unwrap();
        for v in self.outbid_item_ids.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 612_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "auctioneer", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "start_from_page", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_outbid_items", "    ");
        if !self.outbid_item_ids.is_empty() {
            writeln!(s, "    /* outbid_item_ids: u32[amount_of_outbid_items] start */").unwrap();
            for (i, v) in self.outbid_item_ids.iter().enumerate() {
                crate::util::write_bytes(&mut s, &mut bytes, 4, &format!("outbid_item_ids {i}"), "    ");
            }
            writeln!(s, "    /* outbid_item_ids: u32[amount_of_outbid_items] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // auctioneer: Guid
        w.write_all(&self.auctioneer.guid().to_le_bytes())?;

        // start_from_page: u32
        w.write_all(&self.start_from_page.to_le_bytes())?;

        // amount_of_outbid_items: u32
        w.write_all(&(self.outbid_item_ids.len() as u32).to_le_bytes())?;

        // outbid_item_ids: u32[amount_of_outbid_items]
        for i in self.outbid_item_ids.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_AUCTION_LIST_BIDDER_ITEMS {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_AUCTION_LIST_BIDDER_ITEMS {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_AUCTION_LIST_BIDDER_ITEMS {}

impl CMSG_AUCTION_LIST_BIDDER_ITEMS {
    pub(crate) fn size(&self) -> usize {
        8 // auctioneer: Guid
        + 4 // start_from_page: u32
        + 4 // amount_of_outbid_items: u32
        + self.outbid_item_ids.len() * core::mem::size_of::<u32>() // outbid_item_ids: u32[amount_of_outbid_items]
    }
}

