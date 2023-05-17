use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/vehicle/cmsg_dismiss_controlled_vehicle.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/vehicle/cmsg_dismiss_controlled_vehicle.wowm#L1):
/// ```text
/// cmsg CMSG_DISMISS_CONTROLLED_VEHICLE = 0x046D {
/// }
/// ```
pub struct CMSG_DISMISS_CONTROLLED_VEHICLE {
}

impl crate::private::Sealed for CMSG_DISMISS_CONTROLLED_VEHICLE {}
impl crate::Message for CMSG_DISMISS_CONTROLLED_VEHICLE {
    const OPCODE: u32 = 0x046d;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x046D, size: body_size });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_DISMISS_CONTROLLED_VEHICLE {}

