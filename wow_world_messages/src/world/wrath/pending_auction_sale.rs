use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_auction_list_pending_sales.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_auction_list_pending_sales.wowm#L1):
/// ```text
/// struct PendingAuctionSale {
///     CString string1;
///     CString string2;
///     u32 unknown1;
///     u32 unknown2;
///     f32 time_left;
/// }
/// ```
pub struct PendingAuctionSale {
    /// mangostwo: string '%d:%d:%d:%d:%d' -> itemId, ItemRandomPropertyId, 2, auctionId, unk1 (stack size?, unused)
    ///
    pub string1: String,
    /// mangostwo: string '%16I64X:%d:%d:%d:%d' -> bidderGuid, bid, buyout, deposit, auctionCut
    ///
    pub string2: String,
    /// mangostwo sets to 97250.
    ///
    pub unknown1: u32,
    /// mangostwo sets to 68.
    ///
    pub unknown2: u32,
    pub time_left: f32,
}

impl PendingAuctionSale {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // string1: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.string1.as_bytes().iter().rev().next(), Some(&0_u8), "String `string1` must not be null-terminated.");
        w.write_all(self.string1.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // string2: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.string2.as_bytes().iter().rev().next(), Some(&0_u8), "String `string2` must not be null-terminated.");
        w.write_all(self.string2.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: u32
        w.write_all(&self.unknown2.to_le_bytes())?;

        // time_left: f32
        w.write_all(&self.time_left.to_le_bytes())?;

        Ok(())
    }
}

impl PendingAuctionSale {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // string1: CString
        let string1 = crate::util::read_c_string_to_vec(r)?;
        let string1 = String::from_utf8(string1)?;

        // string2: CString
        let string2 = crate::util::read_c_string_to_vec(r)?;
        let string2 = String::from_utf8(string2)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        // unknown2: u32
        let unknown2 = crate::util::read_u32_le(r)?;

        // time_left: f32
        let time_left = crate::util::read_f32_le(r)?;
        Ok(Self {
            string1,
            string2,
            unknown1,
            unknown2,
            time_left,
        })
    }

}

impl PendingAuctionSale {
    pub(crate) fn size(&self) -> usize {
        self.string1.len() + 1 // string1: CString
        + self.string2.len() + 1 // string2: CString
        + 4 // unknown1: u32
        + 4 // unknown2: u32
        + 4 // time_left: f32
    }
}

