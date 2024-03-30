use std::io::{Read, Write};

use crate::Guid;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_flight_spline_sync.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_flight_spline_sync.wowm#L8):
/// ```text
/// smsg SMSG_FLIGHT_SPLINE_SYNC = 0x0388 {
///     f32 elapsed_value;
///     PackedGuid guid;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct SMSG_FLIGHT_SPLINE_SYNC {
    pub elapsed_value: f32,
    pub guid: Guid,
}

impl crate::private::Sealed for SMSG_FLIGHT_SPLINE_SYNC {}
impl SMSG_FLIGHT_SPLINE_SYNC {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(5..=13).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // elapsed_value: f32
        let elapsed_value = crate::util::read_f32_le(&mut r)?;

        // guid: PackedGuid
        let guid = crate::util::read_packed_guid(&mut r)?;

        Ok(Self {
            elapsed_value,
            guid,
        })
    }

}

impl crate::Message for SMSG_FLIGHT_SPLINE_SYNC {
    const OPCODE: u32 = 0x0388;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_FLIGHT_SPLINE_SYNC"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_FLIGHT_SPLINE_SYNC {{").unwrap();
        // Members
        writeln!(s, "    elapsed_value = {};", if self.elapsed_value.to_string().contains('.') { self.elapsed_value.to_string() } else { format!("{}.0", self.elapsed_value) }).unwrap();
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 904_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "elapsed_value", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.guid), "guid", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // elapsed_value: f32
        w.write_all(&self.elapsed_value.to_le_bytes())?;

        // guid: PackedGuid
        crate::util::write_packed_guid(&self.guid, &mut w)?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(904, "SMSG_FLIGHT_SPLINE_SYNC", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_FLIGHT_SPLINE_SYNC {}

impl SMSG_FLIGHT_SPLINE_SYNC {
    pub(crate) const fn size(&self) -> usize {
        4 // elapsed_value: f32
        + crate::util::packed_guid_size(&self.guid) // guid: PackedGuid
    }
}

