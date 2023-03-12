use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/cmsg_set_ammo.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/cmsg_set_ammo.wowm#L3):
/// ```text
/// cmsg CMSG_SET_AMMO = 0x0268 {
///     u32 item;
/// }
/// ```
pub struct CMSG_SET_AMMO {
    pub item: u32,
}

impl crate::Message for CMSG_SET_AMMO {
    const OPCODE: u32 = 0x0268;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // item: u32
        w.write_all(&self.item.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0268, size: body_size as u32 });
        }

        // item: u32
        let item = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            item,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_SET_AMMO {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_SET_AMMO {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_SET_AMMO {}

