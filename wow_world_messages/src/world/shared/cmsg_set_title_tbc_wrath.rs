use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_set_title.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_set_title.wowm#L1):
/// ```text
/// cmsg CMSG_SET_TITLE = 0x0374 {
///     u32 title;
/// }
/// ```
pub struct CMSG_SET_TITLE {
    pub title: u32,
}

impl crate::private::Sealed for CMSG_SET_TITLE {}
impl crate::Message for CMSG_SET_TITLE {
    const OPCODE: u32 = 0x0374;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // title: u32
        w.write_all(&self.title.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0374, size: body_size });
        }

        // title: u32
        let title = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            title,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_SET_TITLE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_SET_TITLE {}

