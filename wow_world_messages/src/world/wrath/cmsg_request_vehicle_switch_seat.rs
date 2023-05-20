use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/vehicle/cmsg_request_vehicle_switch_seat.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/vehicle/cmsg_request_vehicle_switch_seat.wowm#L1):
/// ```text
/// cmsg CMSG_REQUEST_VEHICLE_SWITCH_SEAT = 0x0479 {
///     Guid vehicle;
///     u8 seat;
/// }
/// ```
pub struct CMSG_REQUEST_VEHICLE_SWITCH_SEAT {
    pub vehicle: Guid,
    pub seat: u8,
}

impl crate::private::Sealed for CMSG_REQUEST_VEHICLE_SWITCH_SEAT {}
impl crate::Message for CMSG_REQUEST_VEHICLE_SWITCH_SEAT {
    const OPCODE: u32 = 0x0479;

    fn size_without_header(&self) -> u32 {
        9
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // vehicle: Guid
        w.write_all(&self.vehicle.guid().to_le_bytes())?;

        // seat: u8
        w.write_all(&self.seat.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 9 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0479, size: body_size });
        }

        // vehicle: Guid
        let vehicle = Guid::read(&mut r)?;

        // seat: u8
        let seat = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            vehicle,
            seat,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_REQUEST_VEHICLE_SWITCH_SEAT {}

