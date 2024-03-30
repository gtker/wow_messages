use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::AuctionListItem;
use std::time::Duration;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/smsg/smsg_auction_owner_list_result.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/smsg/smsg_auction_owner_list_result.wowm#L1):
/// ```text
/// smsg SMSG_AUCTION_OWNER_LIST_RESULT = 0x025D {
///     u32 count;
///     AuctionListItem[count] auctions;
///     u32 total_amount_of_auctions;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_AUCTION_OWNER_LIST_RESULT {
    pub auctions: Vec<AuctionListItem>,
    pub total_amount_of_auctions: u32,
}

impl crate::private::Sealed for SMSG_AUCTION_OWNER_LIST_RESULT {}
impl SMSG_AUCTION_OWNER_LIST_RESULT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(8..=65535).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // count: u32
        let count = crate::util::read_u32_le(&mut r)?;

        // auctions: AuctionListItem[count]
        let auctions = {
            let mut auctions = Vec::with_capacity(count as usize);

            let allocation_size = u64::from(count) * 64;
            if allocation_size > crate::errors::MAX_ALLOCATION_SIZE {
                return Err(crate::errors::ParseErrorKind::AllocationTooLargeError(allocation_size));
            }

            for _ in 0..count {
                auctions.push(AuctionListItem::read(&mut r)?);
            }
            auctions
        };

        // total_amount_of_auctions: u32
        let total_amount_of_auctions = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            auctions,
            total_amount_of_auctions,
        })
    }

}

impl crate::Message for SMSG_AUCTION_OWNER_LIST_RESULT {
    const OPCODE: u32 = 0x025d;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_AUCTION_OWNER_LIST_RESULT"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_AUCTION_OWNER_LIST_RESULT {{").unwrap();
        // Members
        writeln!(s, "    count = {};", self.auctions.len()).unwrap();
        writeln!(s, "    auctions = [").unwrap();
        for v in self.auctions.as_slice() {
            writeln!(s, "        {{").unwrap();
            // Members
            writeln!(s, "            id = {};", v.id).unwrap();
            writeln!(s, "            item = {};", v.item).unwrap();
            writeln!(s, "            item_enchantment = {};", v.item_enchantment).unwrap();
            writeln!(s, "            item_random_property_id = {};", v.item_random_property_id).unwrap();
            writeln!(s, "            item_suffix_factor = {};", v.item_suffix_factor).unwrap();
            writeln!(s, "            item_count = {};", v.item_count).unwrap();
            writeln!(s, "            item_charges = {};", v.item_charges).unwrap();
            writeln!(s, "            item_owner = {};", v.item_owner.guid()).unwrap();
            writeln!(s, "            start_bid = {};", v.start_bid).unwrap();
            writeln!(s, "            minimum_bid = {};", v.minimum_bid).unwrap();
            writeln!(s, "            buyout_amount = {};", v.buyout_amount).unwrap();
            writeln!(s, "            time_left = {};", v.time_left.as_millis()).unwrap();
            writeln!(s, "            highest_bidder = {};", v.highest_bidder.guid()).unwrap();
            writeln!(s, "            highest_bid = {};", v.highest_bid).unwrap();

            writeln!(s, "        }},").unwrap();
        }
        writeln!(s, "    ];").unwrap();
        writeln!(s, "    total_amount_of_auctions = {};", self.total_amount_of_auctions).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 605_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "count", "    ");
        if !self.auctions.is_empty() {
            writeln!(s, "    /* auctions: AuctionListItem[count] start */").unwrap();
            for (i, v) in self.auctions.iter().enumerate() {
                writeln!(s, "    /* auctions: AuctionListItem[count] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "id", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item_enchantment", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item_random_property_id", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item_suffix_factor", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item_count", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item_charges", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "item_owner", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "start_bid", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "minimum_bid", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "buyout_amount", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "time_left", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 8, "highest_bidder", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "highest_bid", "        ");
                writeln!(s, "    /* auctions: AuctionListItem[count] {i} end */").unwrap();
            }
            writeln!(s, "    /* auctions: AuctionListItem[count] end */").unwrap();
        }
        crate::util::write_bytes(&mut s, &mut bytes, 4, "total_amount_of_auctions", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // count: u32
        w.write_all(&(self.auctions.len() as u32).to_le_bytes())?;

        // auctions: AuctionListItem[count]
        for i in self.auctions.iter() {
            i.write_into_vec(&mut w)?;
        }

        // total_amount_of_auctions: u32
        w.write_all(&self.total_amount_of_auctions.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(605, "SMSG_AUCTION_OWNER_LIST_RESULT", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_AUCTION_OWNER_LIST_RESULT {}

impl SMSG_AUCTION_OWNER_LIST_RESULT {
    pub(crate) fn size(&self) -> usize {
        4 // count: u32
        + self.auctions.len() * 64 // auctions: AuctionListItem[count]
        + 4 // total_amount_of_auctions: u32
    }
}

