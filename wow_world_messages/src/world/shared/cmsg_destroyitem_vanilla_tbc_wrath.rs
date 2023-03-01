use crate:: {
};
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_destroyitem.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_destroyitem.wowm#L3):
/// ```text
/// cmsg CMSG_DESTROYITEM = 0x0111 {
///     u8 bag;
///     u8 slot;
///     u8 amount;
///     u8 unknown1;
///     u8 unknown2;
///     u8 unknown3;
/// }
/// ```
pub struct CMSG_DESTROYITEM {
    pub bag: u8,
    pub slot: u8,
    pub amount: u8,
    pub unknown1: u8,
    pub unknown2: u8,
    pub unknown3: u8,
}

impl crate::Message for CMSG_DESTROYITEM {
    const OPCODE: u32 = 0x0111;

    fn size_without_header(&self) -> u32 {
        6
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // bag: u8
        w.write_all(&self.bag.to_le_bytes())?;

        // slot: u8
        w.write_all(&self.slot.to_le_bytes())?;

        // amount: u8
        w.write_all(&self.amount.to_le_bytes())?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: u8
        w.write_all(&self.unknown2.to_le_bytes())?;

        // unknown3: u8
        w.write_all(&self.unknown3.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 6 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0111, size: body_size as u32 });
        }

        // bag: u8
        let bag = crate::util::read_u8_le(&mut r)?;

        // slot: u8
        let slot = crate::util::read_u8_le(&mut r)?;

        // amount: u8
        let amount = crate::util::read_u8_le(&mut r)?;

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(&mut r)?;

        // unknown2: u8
        let unknown2 = crate::util::read_u8_le(&mut r)?;

        // unknown3: u8
        let unknown3 = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            bag,
            slot,
            amount,
            unknown1,
            unknown2,
            unknown3,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_DESTROYITEM {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_DESTROYITEM {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_DESTROYITEM {}

