use crate:: {
};
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_cancel_mount_aura.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_cancel_mount_aura.wowm#L1):
/// ```text
/// cmsg CMSG_CANCEL_MOUNT_AURA = 0x0375 {
/// }
/// ```
pub struct CMSG_CANCEL_MOUNT_AURA {
}

impl crate::Message for CMSG_CANCEL_MOUNT_AURA {
    const OPCODE: u32 = 0x0375;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0375, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_CANCEL_MOUNT_AURA {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_CANCEL_MOUNT_AURA {}

