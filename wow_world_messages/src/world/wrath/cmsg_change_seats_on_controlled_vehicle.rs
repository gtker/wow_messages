use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::wrath::MovementInfo;
use std::io::{Write, Read};

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

impl crate::Message for CMSG_CHANGE_SEATS_ON_CONTROLLED_VEHICLE {
    const OPCODE: u32 = 0x049b;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // vehicle: PackedGuid
        self.vehicle.write_packed_guid_into_vec(w);

        // info: MovementInfo
        self.info.write_into_vec(w)?;

        // accessory: PackedGuid
        self.accessory.write_packed_guid_into_vec(w);

        // seat: u8
        w.write_all(&self.seat.to_le_bytes())?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(35..=103).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x049B, size: body_size as u32 });
        }

        // vehicle: PackedGuid
        let vehicle = Guid::read_packed(r)?;

        // info: MovementInfo
        let info = MovementInfo::read(r)?;

        // accessory: PackedGuid
        let accessory = Guid::read_packed(r)?;

        // seat: u8
        let seat = crate::util::read_u8_le(r)?;

        Ok(Self {
            vehicle,
            info,
            accessory,
            seat,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_CHANGE_SEATS_ON_CONTROLLED_VEHICLE {}

impl CMSG_CHANGE_SEATS_ON_CONTROLLED_VEHICLE {
    pub(crate) fn size(&self) -> usize {
        self.vehicle.size() // vehicle: Guid
        + self.info.size() // info: MovementInfo
        + self.accessory.size() // accessory: Guid
        + 1 // seat: u8
    }
}

