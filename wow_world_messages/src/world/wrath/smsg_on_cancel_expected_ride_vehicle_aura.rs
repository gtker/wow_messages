use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/vehicle/smsg_on_cancel_expected_ride_vehicle_aura.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/vehicle/smsg_on_cancel_expected_ride_vehicle_aura.wowm#L1):
/// ```text
/// smsg SMSG_ON_CANCEL_EXPECTED_RIDE_VEHICLE_AURA = 0x049D {
/// }
/// ```
pub struct SMSG_ON_CANCEL_EXPECTED_RIDE_VEHICLE_AURA {
}

impl crate::Message for SMSG_ON_CANCEL_EXPECTED_RIDE_VEHICLE_AURA {
    const OPCODE: u32 = 0x049d;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x049D, size: body_size as u32 });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ON_CANCEL_EXPECTED_RIDE_VEHICLE_AURA {}

