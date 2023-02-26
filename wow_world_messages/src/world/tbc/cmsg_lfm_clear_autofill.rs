use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/cmsg_lfm_clear_autofill.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/cmsg_lfm_clear_autofill.wowm#L1):
/// ```text
/// cmsg CMSG_LFM_CLEAR_AUTOFILL = 0x035F {
/// }
/// ```
pub struct CMSG_LFM_CLEAR_AUTOFILL {
}

impl crate::Message for CMSG_LFM_CLEAR_AUTOFILL {
    const OPCODE: u32 = 0x035f;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x035F, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_LFM_CLEAR_AUTOFILL {}

