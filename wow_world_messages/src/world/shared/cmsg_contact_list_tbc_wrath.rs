use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/cmsg_contact_list.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/cmsg_contact_list.wowm#L1):
/// ```text
/// cmsg CMSG_CONTACT_LIST = 0x0066 {
///     u32 flags;
/// }
/// ```
pub struct CMSG_CONTACT_LIST {
    /// Sent back in [`SMSG_CONTACT_LIST`](crate::tbc::SMSG_CONTACT_LIST).
    ///
    pub flags: u32,
}

impl crate::Message for CMSG_CONTACT_LIST {
    const OPCODE: u32 = 0x0066;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // flags: u32
        w.write_all(&self.flags.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0066, size: body_size as u32 });
        }

        // flags: u32
        let flags = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            flags,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_CONTACT_LIST {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_CONTACT_LIST {}

