use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/vehicle/cmsg_request_vehicle_next_seat.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/vehicle/cmsg_request_vehicle_next_seat.wowm#L1):
/// ```text
/// cmsg CMSG_REQUEST_VEHICLE_NEXT_SEAT = 0x0478 {
/// }
/// ```
pub struct CMSG_REQUEST_VEHICLE_NEXT_SEAT {
}

impl crate::Message for CMSG_REQUEST_VEHICLE_NEXT_SEAT {
    const OPCODE: u32 = 0x0478;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0478, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_REQUEST_VEHICLE_NEXT_SEAT {}

