use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/cmsg/cmsg_auction_list_items.wowm:16`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/cmsg/cmsg_auction_list_items.wowm#L16):
/// ```text
/// struct AuctionSort {
///     u8 column;
///     u8 reversed;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct AuctionSort {
    pub column: u8,
    pub reversed: u8,
}

impl AuctionSort {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // column: u8
        w.write_all(&self.column.to_le_bytes())?;

        // reversed: u8
        w.write_all(&self.reversed.to_le_bytes())?;

        Ok(())
    }
}

impl AuctionSort {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, std::io::Error> {
        // column: u8
        let column = crate::util::read_u8_le(&mut r)?;

        // reversed: u8
        let reversed = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            column,
            reversed,
        })
    }

}

