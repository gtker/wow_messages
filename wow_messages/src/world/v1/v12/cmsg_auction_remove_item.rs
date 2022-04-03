use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/new3.wowm:53`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/new3.wowm#L53):
/// ```text
/// cmsg CMSG_AUCTION_REMOVE_ITEM = 0x257 {
///     u64 auctioneer_guid;
///     u32 auction_id;
/// }
/// ```
pub struct CMSG_AUCTION_REMOVE_ITEM {
    pub auctioneer_guid: u64,
    pub auction_id: u32,
}

impl WorldClientMessageWrite for CMSG_AUCTION_REMOVE_ITEM {
    const OPCODE: u32 = 0x257;

    fn write_unencrypted_client<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        crate::util::write_u16_be(w, (Self::size() + 4) as u16)?;
        crate::util::write_u32_le(w, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_client<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        e.write_encrypted_client_header(w, (Self::size() + 4) as u16, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for CMSG_AUCTION_REMOVE_ITEM {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // auctioneer_guid: u64
        let auctioneer_guid = crate::util::read_u64_le(r)?;

        // auction_id: u32
        let auction_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            auctioneer_guid,
            auction_id,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // auctioneer_guid: u64
        w.write_all(&self.auctioneer_guid.to_le_bytes())?;

        // auction_id: u32
        w.write_all(&self.auction_id.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_AUCTION_REMOVE_ITEM {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_AUCTION_REMOVE_ITEM {
    fn maximum_possible_size() -> usize {
        8 // auctioneer_guid: u64
        + 4 // auction_id: u32
    }
}

