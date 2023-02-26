use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/cmsg_instance_lock_response.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/cmsg_instance_lock_response.wowm#L1):
/// ```text
/// cmsg CMSG_INSTANCE_LOCK_RESPONSE = 0x013F {
///     Bool accept;
/// }
/// ```
pub struct CMSG_INSTANCE_LOCK_RESPONSE {
    pub accept: bool,
}

impl crate::Message for CMSG_INSTANCE_LOCK_RESPONSE {
    const OPCODE: u32 = 0x013f;

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // accept: Bool
        w.write_all(u8::from(self.accept).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x013F, size: body_size as u32 });
        }

        // accept: Bool
        let accept = crate::util::read_u8_le(r)? != 0;

        Ok(Self {
            accept,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_INSTANCE_LOCK_RESPONSE {}

