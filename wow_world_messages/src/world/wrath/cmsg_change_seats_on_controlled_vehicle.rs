use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::MovementInfo;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/vehicle/cmsg_change_seats_on_controlled_vehicle.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/vehicle/cmsg_change_seats_on_controlled_vehicle.wowm#L1):
/// ```text
/// cmsg CMSG_CHANGE_SEATS_ON_CONTROLLED_VEHICLE = 0x049B {
///     PackedGuid vehicle;
///     MovementInfo info;
///     PackedGuid accessory;
///     u8 seat;
/// }
/// ```
pub struct CMSG_CHANGE_SEATS_ON_CONTROLLED_VEHICLE {
    pub vehicle: Guid,
    pub info: MovementInfo,
    pub accessory: Guid,
    pub seat: u8,
}

impl crate::private::Sealed for CMSG_CHANGE_SEATS_ON_CONTROLLED_VEHICLE {}
impl crate::Message for CMSG_CHANGE_SEATS_ON_CONTROLLED_VEHICLE {
    const OPCODE: u32 = 0x049b;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // vehicle: PackedGuid
        crate::util::write_packed_guid(&self.vehicle, &mut w)?;

        // info: MovementInfo
        self.info.write_into_vec(&mut w)?;

        // accessory: PackedGuid
        crate::util::write_packed_guid(&self.accessory, &mut w)?;

        // seat: u8
        w.write_all(&self.seat.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(35..=107).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x049B, size: body_size });
        }

        // vehicle: PackedGuid
        let vehicle = crate::util::read_packed_guid(&mut r)?;

        // info: MovementInfo
        let info = MovementInfo::read(&mut r)?;

        // accessory: PackedGuid
        let accessory = crate::util::read_packed_guid(&mut r)?;

        // seat: u8
        let seat = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            vehicle,
            info,
            accessory,
            seat,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_CHANGE_SEATS_ON_CONTROLLED_VEHICLE {}

impl CMSG_CHANGE_SEATS_ON_CONTROLLED_VEHICLE {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.vehicle) // vehicle: PackedGuid
        + self.info.size() // info: MovementInfo
        + crate::util::packed_guid_size(&self.accessory) // accessory: PackedGuid
        + 1 // seat: u8
    }
}

