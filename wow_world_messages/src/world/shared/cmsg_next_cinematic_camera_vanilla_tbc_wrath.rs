use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Sent by client when cinematic beings.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/cinematic/cmsg_next_cinematic_camera.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/cinematic/cmsg_next_cinematic_camera.wowm#L3):
/// ```text
/// cmsg CMSG_NEXT_CINEMATIC_CAMERA = 0x00FB {
/// }
/// ```
pub struct CMSG_NEXT_CINEMATIC_CAMERA {
}

impl crate::Message for CMSG_NEXT_CINEMATIC_CAMERA {
    const OPCODE: u32 = 0x00fb;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00FB, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_NEXT_CINEMATIC_CAMERA {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_NEXT_CINEMATIC_CAMERA {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_NEXT_CINEMATIC_CAMERA {}

