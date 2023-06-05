use std::io::{Read, Write};

use crate::vanilla::AuctionListItem;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/smsg/smsg_auction_bidder_list_result.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/smsg/smsg_auction_bidder_list_result.wowm#L1):
/// ```text
/// smsg SMSG_AUCTION_BIDDER_LIST_RESULT = 0x0265 {
///     u32 count;
///     AuctionListItem[count] auctions;
///     u32 total_amount_of_auctions;
/// }
/// ```
pub struct SMSG_AUCTION_BIDDER_LIST_RESULT {
    pub auctions: Vec<AuctionListItem>,
    pub total_amount_of_auctions: u32,
}

#[cfg(feature = "print-testcase")]
impl SMSG_AUCTION_BIDDER_LIST_RESULT {
    pub fn to_test_case_string(&self) -> String {
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
            writeln!(s, "    id = {};", v.id).unwrap();
            writeln!(s, "    item = {};", v.item).unwrap();
            writeln!(s, "    item_enchantment = {};", v.item_enchantment).unwrap();
            writeln!(s, "    item_random_property_id = {};", v.item_random_property_id).unwrap();
            writeln!(s, "    item_suffix_factor = {};", v.item_suffix_factor).unwrap();
            writeln!(s, "    item_count = {};", v.item_count).unwrap();
            writeln!(s, "    item_charges = {};", v.item_charges).unwrap();
            writeln!(s, "    item_owner = {};", v.item_owner.guid()).unwrap();
            writeln!(s, "    start_bid = {};", v.start_bid).unwrap();
            writeln!(s, "    minimum_bid = {};", v.minimum_bid).unwrap();
            writeln!(s, "    buyout_amount = {};", v.buyout_amount).unwrap();
            writeln!(s, "    time_left = {};", v.time_left.as_millis()).unwrap();
            writeln!(s, "    highest_bidder = {};", v.highest_bidder.guid()).unwrap();
            writeln!(s, "    highest_bid = {};", v.highest_bid).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();
        writeln!(s, "    total_amount_of_auctions = {};", self.total_amount_of_auctions).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 613_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "count");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_AUCTION_BIDDER_LIST_RESULT {}
impl crate::Message for SMSG_AUCTION_BIDDER_LIST_RESULT {
    const OPCODE: u32 = 0x0265;

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
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(8..=65535).contains(&body_size) {
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

        Ok(Self {
            auctions,
            total_amount_of_auctions,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_AUCTION_BIDDER_LIST_RESULT {}

impl SMSG_AUCTION_BIDDER_LIST_RESULT {
    pub(crate) fn size(&self) -> usize {
        4 // count: u32
        + self.auctions.len() * 64 // auctions: AuctionListItem[count]
        + 4 // total_amount_of_auctions: u32
    }
}

