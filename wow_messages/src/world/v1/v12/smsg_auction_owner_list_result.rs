use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::AuctionListItem;
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new5.wowm:994`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new5.wowm#L994):
/// ```text
/// smsg SMSG_AUCTION_OWNER_LIST_RESULT = 0x25D {
///     u32 count;
///     AuctionListItem[count] auctions;
///     u32 total_amount_of_auctions;
/// }
/// ```
pub struct SMSG_AUCTION_OWNER_LIST_RESULT {
    pub count: u32,
    pub auctions: Vec<AuctionListItem>,
    pub total_amount_of_auctions: u32,
}

impl WorldServerMessageWrite for SMSG_AUCTION_OWNER_LIST_RESULT {
    const OPCODE: u16 = 0x25d;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (self.size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (self.size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_AUCTION_OWNER_LIST_RESULT {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // count: u32
        let count = crate::util::read_u32_le(r)?;

        // auctions: AuctionListItem[count]
        let mut auctions = Vec::with_capacity(count as usize);
        for i in 0..count {
            auctions.push(AuctionListItem::read(r)?);
        }

        // total_amount_of_auctions: u32
        let total_amount_of_auctions = crate::util::read_u32_le(r)?;

        Ok(Self {
            count,
            auctions,
            total_amount_of_auctions,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // count: u32
        w.write_all(&(self.auctions.len() as u32).to_le_bytes())?;

        // auctions: AuctionListItem[count]
        for i in self.auctions.iter() {
            i.write(w)?;
        }

        // total_amount_of_auctions: u32
        w.write_all(&self.total_amount_of_auctions.to_le_bytes())?;

        Ok(())
    }
}

impl VariableSized for SMSG_AUCTION_OWNER_LIST_RESULT {
    fn size(&self) -> usize {
        4 // count: u32
        + self.auctions.iter().fold(0, |acc, x| acc + AuctionListItem::size()) // auctions: AuctionListItem[count]
        + 4 // total_amount_of_auctions: u32
    }
}

impl MaximumPossibleSized for SMSG_AUCTION_OWNER_LIST_RESULT {
    fn maximum_possible_size() -> usize {
        4 // count: u32
        + 4294967295 * AuctionListItem::maximum_possible_size() // auctions: AuctionListItem[count]
        + 4 // total_amount_of_auctions: u32
    }
}

