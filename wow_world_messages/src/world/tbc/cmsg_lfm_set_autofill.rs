use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/cmsg_lfm_set_autofill.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/cmsg_lfm_set_autofill.wowm#L1):
/// ```text
/// cmsg CMSG_LFM_SET_AUTOFILL = 0x035E {
/// }
/// ```
pub struct CMSG_LFM_SET_AUTOFILL {
}

impl crate::private::Sealed for CMSG_LFM_SET_AUTOFILL {}
impl crate::Message for CMSG_LFM_SET_AUTOFILL {
    const OPCODE: u32 = 0x035e;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x035E, size: body_size });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_LFM_SET_AUTOFILL {}

