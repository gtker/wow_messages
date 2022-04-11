use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/cmsg/cmsg_auction_list_items.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/cmsg/cmsg_auction_list_items.wowm#L3):
/// ```text
/// cmsg CMSG_AUCTION_LIST_ITEMS = 0x258 {
///     Guid auctioneer_guid;
///     u32 list_start_item;
///     CString searched_name;
///     u8 minimum_level;
///     u8 maximum_level;
///     u32 auction_slot_id;
///     u32 auction_main_category;
///     u32 auction_sub_category;
///     u32 auction_quality;
///     u8 usable;
/// }
/// ```
pub struct CMSG_AUCTION_LIST_ITEMS {
    pub auctioneer_guid: Guid,
    pub list_start_item: u32,
    pub searched_name: String,
    pub minimum_level: u8,
    pub maximum_level: u8,
    pub auction_slot_id: u32,
    pub auction_main_category: u32,
    pub auction_sub_category: u32,
    pub auction_quality: u32,
    pub usable: u8,
}

impl WorldClientMessageWrite for CMSG_AUCTION_LIST_ITEMS {
    const OPCODE: u32 = 0x258;

    fn write_unencrypted_client<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        crate::util::write_u16_be(w, (self.size() + 4) as u16)?;
        crate::util::write_u32_le(w, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_client<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        e.write_encrypted_client_header(w, (self.size() + 4) as u16, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for CMSG_AUCTION_LIST_ITEMS {
    type Error = CMSG_AUCTION_LIST_ITEMSError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // auctioneer_guid: Guid
        let auctioneer_guid = Guid::read(r)?;

        // list_start_item: u32
        let list_start_item = crate::util::read_u32_le(r)?;

        // searched_name: CString
        let searched_name = crate::util::read_c_string_to_vec(r)?;
        let searched_name = String::from_utf8(searched_name)?;

        // minimum_level: u8
        let minimum_level = crate::util::read_u8_le(r)?;

        // maximum_level: u8
        let maximum_level = crate::util::read_u8_le(r)?;

        // auction_slot_id: u32
        let auction_slot_id = crate::util::read_u32_le(r)?;

        // auction_main_category: u32
        let auction_main_category = crate::util::read_u32_le(r)?;

        // auction_sub_category: u32
        let auction_sub_category = crate::util::read_u32_le(r)?;

        // auction_quality: u32
        let auction_quality = crate::util::read_u32_le(r)?;

        // usable: u8
        let usable = crate::util::read_u8_le(r)?;

        Ok(Self {
            auctioneer_guid,
            list_start_item,
            searched_name,
            minimum_level,
            maximum_level,
            auction_slot_id,
            auction_main_category,
            auction_sub_category,
            auction_quality,
            usable,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // auctioneer_guid: Guid
        self.auctioneer_guid.write(w)?;

        // list_start_item: u32
        w.write_all(&self.list_start_item.to_le_bytes())?;

        // searched_name: CString
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

        // auction_quality: u32
        w.write_all(&self.auction_quality.to_le_bytes())?;

        // usable: u8
        w.write_all(&self.usable.to_le_bytes())?;

        Ok(())
    }
}

impl VariableSized for CMSG_AUCTION_LIST_ITEMS {
    fn size(&self) -> usize {
        8 // auctioneer_guid: Guid
        + 4 // list_start_item: u32
        + self.searched_name.len() + 1 // searched_name: CString and Null Terminator
        + 1 // minimum_level: u8
        + 1 // maximum_level: u8
        + 4 // auction_slot_id: u32
        + 4 // auction_main_category: u32
        + 4 // auction_sub_category: u32
        + 4 // auction_quality: u32
        + 1 // usable: u8
    }
}

impl MaximumPossibleSized for CMSG_AUCTION_LIST_ITEMS {
    fn maximum_possible_size() -> usize {
        8 // auctioneer_guid: Guid
        + 4 // list_start_item: u32
        + 256 // searched_name: CString
        + 1 // minimum_level: u8
        + 1 // maximum_level: u8
        + 4 // auction_slot_id: u32
        + 4 // auction_main_category: u32
        + 4 // auction_sub_category: u32
        + 4 // auction_quality: u32
        + 1 // usable: u8
    }
}

#[derive(Debug)]
pub enum CMSG_AUCTION_LIST_ITEMSError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for CMSG_AUCTION_LIST_ITEMSError {}
impl std::fmt::Display for CMSG_AUCTION_LIST_ITEMSError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_AUCTION_LIST_ITEMSError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CMSG_AUCTION_LIST_ITEMSError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

