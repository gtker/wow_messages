use std::io::{Read, Write};

use crate::wrath::AuctionListItem;
use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/smsg/smsg_auction_bidder_list_result.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/smsg/smsg_auction_bidder_list_result.wowm#L9):
/// ```text
/// smsg SMSG_AUCTION_BIDDER_LIST_RESULT = 0x0265 {
///     u32 count;
///     AuctionListItem[count] auctions;
///     u32 total_amount_of_auctions;
///     Milliseconds auction_search_delay;
/// }
/// ```
pub struct SMSG_AUCTION_BIDDER_LIST_RESULT {
    pub auctions: Vec<AuctionListItem>,
    pub total_amount_of_auctions: u32,
    pub auction_search_delay: Duration,
}

#[cfg(feature = "print-testcase")]
impl SMSG_AUCTION_BIDDER_LIST_RESULT {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_AUCTION_BIDDER_LIST_RESULT {{").unwrap();
        // Members
        writeln!(s, "    count = {};", self.auctions.len()).unwrap();
        write!(s, "    auctions = [").unwrap();
        for v in self.auctions.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        id = {};", v.id).unwrap();
            writeln!(s, "        item = {};", v.item).unwrap();
            write!(s, "        enchantments = [").unwrap();
            for v in v.enchantments.as_slice() {
                writeln!(s, "{{").unwrap();
                // Members
                writeln!(s, "            enchant_id = {};", v.enchant_id).unwrap();
                writeln!(s, "            enchant_duration = {};", v.enchant_duration).unwrap();
                writeln!(s, "            enchant_charges = {};", v.enchant_charges).unwrap();

                writeln!(s, "    }},").unwrap();
            }
            writeln!(s, "];").unwrap();
            writeln!(s, "        item_random_property_id = {};", v.item_random_property_id).unwrap();
            writeln!(s, "        item_suffix_factor = {};", v.item_suffix_factor).unwrap();
            writeln!(s, "        item_count = {};", v.item_count).unwrap();
            writeln!(s, "        item_charges = {};", v.item_charges).unwrap();
            writeln!(s, "        item_flags = {};", v.item_flags).unwrap();
            writeln!(s, "        item_owner = {};", v.item_owner.guid()).unwrap();
            writeln!(s, "        start_bid = {};", v.start_bid).unwrap();
            writeln!(s, "        minimum_bid = {};", v.minimum_bid).unwrap();
            writeln!(s, "        buyout_amount = {};", v.buyout_amount).unwrap();
            writeln!(s, "        time_left = {};", v.time_left.as_millis()).unwrap();
            writeln!(s, "        highest_bidder = {};", v.highest_bidder.guid()).unwrap();
            writeln!(s, "        highest_bid = {};", v.highest_bid).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();
        writeln!(s, "    total_amount_of_auctions = {};", self.total_amount_of_auctions).unwrap();
        writeln!(s, "    auction_search_delay = {};", self.auction_search_delay.as_millis()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 613_u16.to_le_bytes();
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
                writeln!(s, "    /* enchantments: AuctionEnchantment[7] start */").unwrap();
                for (i, v) in v.enchantments.iter().enumerate() {
                    writeln!(s, "    /* enchantments: AuctionEnchantment[7] {i} start */").unwrap();
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "enchant_id", "            ");
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "enchant_duration", "            ");
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "enchant_charges", "            ");
                    writeln!(s, "    /* enchantments: AuctionEnchantment[7] {i} end */").unwrap();
                }
                writeln!(s, "    /* enchantments: AuctionEnchantment[7] end */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item_random_property_id", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item_suffix_factor", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item_count", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item_charges", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item_flags", "        ");
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
        crate::util::write_bytes(&mut s, &mut bytes, 4, "auction_search_delay", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_AUCTION_BIDDER_LIST_RESULT {}
impl crate::Message for SMSG_AUCTION_BIDDER_LIST_RESULT {
    const OPCODE: u32 = 0x0265;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_AUCTION_BIDDER_LIST_RESULT::to_test_case_string(self)
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

        // auction_search_delay: Milliseconds
        w.write_all((self.auction_search_delay.as_millis() as u32).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(12..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0265, size: body_size });
        }

        // count: u32
        let count = crate::util::read_u32_le(&mut r)?;

        // auctions: AuctionListItem[count]
        let auctions = {
            let mut auctions = Vec::with_capacity(count as usize);
            for _ in 0..count {
                auctions.push(AuctionListItem::read(&mut r)?);
            }
            auctions
        };

        // total_amount_of_auctions: u32
        let total_amount_of_auctions = crate::util::read_u32_le(&mut r)?;

        // auction_search_delay: Milliseconds
        let auction_search_delay = Duration::from_millis(crate::util::read_u32_le(&mut r)?.into());

        Ok(Self {
            auctions,
            total_amount_of_auctions,
            auction_search_delay,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_AUCTION_BIDDER_LIST_RESULT {}

impl SMSG_AUCTION_BIDDER_LIST_RESULT {
    pub(crate) fn size(&self) -> usize {
        4 // count: u32
        + self.auctions.len() * 148 // auctions: AuctionListItem[count]
        + 4 // total_amount_of_auctions: u32
        + 4 // auction_search_delay: Milliseconds
    }
}

