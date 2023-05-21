use std::io::{Read, Write};

use crate::Guid;
use crate::tbc::{
    AuctionSort, ItemQuality,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/cmsg/cmsg_auction_list_items.wowm:23`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/cmsg/cmsg_auction_list_items.wowm#L23):
/// ```text
/// cmsg CMSG_AUCTION_LIST_ITEMS = 0x0258 {
///     Guid auctioneer;
///     u32 list_start_item;
///     CString searched_name;
///     u8 minimum_level;
///     u8 maximum_level;
///     u32 auction_slot_id;
///     u32 auction_main_category;
///     u32 auction_sub_category;
///     (u32)ItemQuality auction_quality;
///     u8 usable;
///     u8 is_full;
///     u8 amount_of_sorted_auctions;
///     AuctionSort[amount_of_sorted_auctions] sorted_auctions;
/// }
/// ```
pub struct CMSG_AUCTION_LIST_ITEMS {
    pub auctioneer: Guid,
    pub list_start_item: u32,
    pub searched_name: String,
    pub minimum_level: u8,
    pub maximum_level: u8,
    pub auction_slot_id: u32,
    pub auction_main_category: u32,
    pub auction_sub_category: u32,
    pub auction_quality: ItemQuality,
    pub usable: u8,
    pub is_full: u8,
    pub sorted_auctions: Vec<AuctionSort>,
}

impl crate::private::Sealed for CMSG_AUCTION_LIST_ITEMS {}
impl CMSG_AUCTION_LIST_ITEMS {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(34..=801).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0258, size: body_size });
        }

        // auctioneer: Guid
        let auctioneer = crate::util::read_guid(&mut r)?;

        // list_start_item: u32
        let list_start_item = crate::util::read_u32_le(&mut r)?;

        // searched_name: CString
        let searched_name = {
            let searched_name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(searched_name)?
        };

        // minimum_level: u8
        let minimum_level = crate::util::read_u8_le(&mut r)?;

        // maximum_level: u8
        let maximum_level = crate::util::read_u8_le(&mut r)?;

        // auction_slot_id: u32
        let auction_slot_id = crate::util::read_u32_le(&mut r)?;

        // auction_main_category: u32
        let auction_main_category = crate::util::read_u32_le(&mut r)?;

        // auction_sub_category: u32
        let auction_sub_category = crate::util::read_u32_le(&mut r)?;

        // auction_quality: ItemQuality
        let auction_quality = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        // usable: u8
        let usable = crate::util::read_u8_le(&mut r)?;

        // is_full: u8
        let is_full = crate::util::read_u8_le(&mut r)?;

        // amount_of_sorted_auctions: u8
        let amount_of_sorted_auctions = crate::util::read_u8_le(&mut r)?;

        // sorted_auctions: AuctionSort[amount_of_sorted_auctions]
        let sorted_auctions = {
            let mut sorted_auctions = Vec::with_capacity(amount_of_sorted_auctions as usize);
            for _ in 0..amount_of_sorted_auctions {
                sorted_auctions.push(AuctionSort::read(&mut r)?);
            }
            sorted_auctions
        };

        Ok(Self {
            auctioneer,
            list_start_item,
            searched_name,
            minimum_level,
            maximum_level,
            auction_slot_id,
            auction_main_category,
            auction_sub_category,
            auction_quality,
            usable,
            is_full,
            sorted_auctions,
        })
    }

}

impl crate::Message for CMSG_AUCTION_LIST_ITEMS {
    const OPCODE: u32 = 0x0258;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_AUCTION_LIST_ITEMS {{").unwrap();
        // Members
        writeln!(s, "    auctioneer = {};", self.auctioneer.guid()).unwrap();
        writeln!(s, "    list_start_item = {};", self.list_start_item).unwrap();
        writeln!(s, "    searched_name = \"{}\";", self.searched_name).unwrap();
        writeln!(s, "    minimum_level = {};", self.minimum_level).unwrap();
        writeln!(s, "    maximum_level = {};", self.maximum_level).unwrap();
        writeln!(s, "    auction_slot_id = {};", self.auction_slot_id).unwrap();
        writeln!(s, "    auction_main_category = {};", self.auction_main_category).unwrap();
        writeln!(s, "    auction_sub_category = {};", self.auction_sub_category).unwrap();
        writeln!(s, "    auction_quality = {};", self.auction_quality.as_test_case_value()).unwrap();
        writeln!(s, "    usable = {};", self.usable).unwrap();
        writeln!(s, "    is_full = {};", self.is_full).unwrap();
        writeln!(s, "    amount_of_sorted_auctions = {};", self.sorted_auctions.len()).unwrap();
        write!(s, "    sorted_auctions = [").unwrap();
        for v in self.sorted_auctions.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        column = {};", v.column).unwrap();
            writeln!(s, "        reversed = {};", v.reversed).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 600_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "auctioneer", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "list_start_item", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.searched_name.len() + 1, "searched_name", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "minimum_level", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "maximum_level", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "auction_slot_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "auction_main_category", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "auction_sub_category", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "auction_quality", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "usable", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "is_full", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_sorted_auctions", "    ");
        if !self.sorted_auctions.is_empty() {
            writeln!(s, "    /* sorted_auctions: AuctionSort[amount_of_sorted_auctions] start */").unwrap();
            for (i, v) in self.sorted_auctions.iter().enumerate() {
                writeln!(s, "    /* sorted_auctions: AuctionSort[amount_of_sorted_auctions] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 1, "column", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "reversed", "        ");
                writeln!(s, "    /* sorted_auctions: AuctionSort[amount_of_sorted_auctions] {i} end */").unwrap();
            }
            writeln!(s, "    /* sorted_auctions: AuctionSort[amount_of_sorted_auctions] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // auctioneer: Guid
        w.write_all(&self.auctioneer.guid().to_le_bytes())?;

        // list_start_item: u32
        w.write_all(&self.list_start_item.to_le_bytes())?;

        // searched_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.searched_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `searched_name` must not be null-terminated.");
        w.write_all(self.searched_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // minimum_level: u8
        w.write_all(&self.minimum_level.to_le_bytes())?;

        // maximum_level: u8
        w.write_all(&self.maximum_level.to_le_bytes())?;

        // auction_slot_id: u32
        w.write_all(&self.auction_slot_id.to_le_bytes())?;

        // auction_main_category: u32
        w.write_all(&self.auction_main_category.to_le_bytes())?;

        // auction_sub_category: u32
        w.write_all(&self.auction_sub_category.to_le_bytes())?;

        // auction_quality: ItemQuality
        w.write_all(&u32::from(self.auction_quality.as_int()).to_le_bytes())?;

        // usable: u8
        w.write_all(&self.usable.to_le_bytes())?;

        // is_full: u8
        w.write_all(&self.is_full.to_le_bytes())?;

        // amount_of_sorted_auctions: u8
        w.write_all(&(self.sorted_auctions.len() as u8).to_le_bytes())?;

        // sorted_auctions: AuctionSort[amount_of_sorted_auctions]
        for i in self.sorted_auctions.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_AUCTION_LIST_ITEMS {}

impl CMSG_AUCTION_LIST_ITEMS {
    pub(crate) fn size(&self) -> usize {
        8 // auctioneer: Guid
        + 4 // list_start_item: u32
        + self.searched_name.len() + 1 // searched_name: CString
        + 1 // minimum_level: u8
        + 1 // maximum_level: u8
        + 4 // auction_slot_id: u32
        + 4 // auction_main_category: u32
        + 4 // auction_sub_category: u32
        + 4 // auction_quality: ItemQuality
        + 1 // usable: u8
        + 1 // is_full: u8
        + 1 // amount_of_sorted_auctions: u8
        + self.sorted_auctions.len() * 2 // sorted_auctions: AuctionSort[amount_of_sorted_auctions]
    }
}

