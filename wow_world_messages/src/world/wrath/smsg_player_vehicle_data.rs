use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/vehicle/smsg_player_vehicle_data.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/vehicle/smsg_player_vehicle_data.wowm#L1):
/// ```text
/// smsg SMSG_PLAYER_VEHICLE_DATA = 0x04A7 {
///     PackedGuid target;
///     u32 vehicle_id;
/// }
/// ```
pub struct SMSG_PLAYER_VEHICLE_DATA {
    pub target: Guid,
    pub vehicle_id: u32,
}

impl crate::private::Sealed for SMSG_PLAYER_VEHICLE_DATA {}
impl SMSG_PLAYER_VEHICLE_DATA {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(5..=13).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // target: PackedGuid
        let target = crate::util::read_packed_guid(&mut r)?;

        // vehicle_id: u32
        let vehicle_id = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            target,
            vehicle_id,
        })
    }

}

impl crate::Message for SMSG_PLAYER_VEHICLE_DATA {
    const OPCODE: u32 = 0x04a7;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_PLAYER_VEHICLE_DATA"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_PLAYER_VEHICLE_DATA {{").unwrap();
        // Members
        writeln!(s, "    target = {};", self.target.guid()).unwrap();
        writeln!(s, "    vehicle_id = {};", self.vehicle_id).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1191_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.target), "target", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "vehicle_id", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // target: PackedGuid
        crate::util::write_packed_guid(&self.target, &mut w)?;

        // vehicle_id: u32
        w.write_all(&self.vehicle_id.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1191, "SMSG_PLAYER_VEHICLE_DATA", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_PLAYER_VEHICLE_DATA {}

impl SMSG_PLAYER_VEHICLE_DATA {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.target) // target: PackedGuid
        + 4 // vehicle_id: u32
    }
}

