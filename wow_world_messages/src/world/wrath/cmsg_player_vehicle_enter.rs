use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/vehicle/cmsg_player_vehicle_enter.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/vehicle/cmsg_player_vehicle_enter.wowm#L1):
/// ```text
/// cmsg CMSG_PLAYER_VEHICLE_ENTER = 0x04A8 {
///     Guid vehicle;
/// }
/// ```
pub struct CMSG_PLAYER_VEHICLE_ENTER {
    pub vehicle: Guid,
}

impl crate::private::Sealed for CMSG_PLAYER_VEHICLE_ENTER {}
impl crate::Message for CMSG_PLAYER_VEHICLE_ENTER {
    const OPCODE: u32 = 0x04a8;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_PLAYER_VEHICLE_ENTER {{").unwrap();
        // Members
        writeln!(s, "    vehicle = {};", self.vehicle.guid()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 12_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1192_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "vehicle", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // vehicle: Guid
        w.write_all(&self.vehicle.guid().to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04A8, size: body_size });
        }

        // vehicle: Guid
        let vehicle = crate::util::read_guid(&mut r)?;

        Ok(Self {
            vehicle,
        })
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_PLAYER_VEHICLE_ENTER {}

