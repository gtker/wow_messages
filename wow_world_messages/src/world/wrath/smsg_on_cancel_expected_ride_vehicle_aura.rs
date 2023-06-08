use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/vehicle/smsg_on_cancel_expected_ride_vehicle_aura.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/vehicle/smsg_on_cancel_expected_ride_vehicle_aura.wowm#L1):
/// ```text
/// smsg SMSG_ON_CANCEL_EXPECTED_RIDE_VEHICLE_AURA = 0x049D {
/// }
/// ```
pub struct SMSG_ON_CANCEL_EXPECTED_RIDE_VEHICLE_AURA {
}

#[cfg(feature = "print-testcase")]
impl SMSG_ON_CANCEL_EXPECTED_RIDE_VEHICLE_AURA {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_ON_CANCEL_EXPECTED_RIDE_VEHICLE_AURA {{").unwrap();
        // Members

        writeln!(s, "}} [").unwrap();

        let [a, b] = 2_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1181_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();



        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_ON_CANCEL_EXPECTED_RIDE_VEHICLE_AURA {}
impl crate::Message for SMSG_ON_CANCEL_EXPECTED_RIDE_VEHICLE_AURA {
    const OPCODE: u32 = 0x049d;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_ON_CANCEL_EXPECTED_RIDE_VEHICLE_AURA::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x049D, size: body_size });
        }

        Ok(Self {
        })
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ON_CANCEL_EXPECTED_RIDE_VEHICLE_AURA {}

