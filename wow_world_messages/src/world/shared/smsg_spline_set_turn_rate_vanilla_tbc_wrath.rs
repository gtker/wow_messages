use std::io::{Read, Write};

use crate::Guid;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_spline_set_turn_rate.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_spline_set_turn_rate.wowm#L3):
/// ```text
/// smsg SMSG_SPLINE_SET_TURN_RATE = 0x0303 {
///     PackedGuid guid;
///     f32 speed;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct SMSG_SPLINE_SET_TURN_RATE {
    pub guid: Guid,
    pub speed: f32,
}

impl crate::private::Sealed for SMSG_SPLINE_SET_TURN_RATE {}
impl SMSG_SPLINE_SET_TURN_RATE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(5..=13).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid: PackedGuid
        let guid = crate::util::read_packed_guid(&mut r)?;

        // speed: f32
        let speed = crate::util::read_f32_le(&mut r)?;

        Ok(Self {
            guid,
            speed,
        })
    }

}

impl crate::Message for SMSG_SPLINE_SET_TURN_RATE {
    const OPCODE: u32 = 0x0303;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_SPLINE_SET_TURN_RATE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SPLINE_SET_TURN_RATE {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    speed = {};", if self.speed.to_string().contains('.') { self.speed.to_string() } else { format!("{}.0", self.speed) }).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 771_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.guid), "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "speed", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        crate::util::write_packed_guid(&self.guid, &mut w)?;

        // speed: f32
        w.write_all(&self.speed.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(771, "SMSG_SPLINE_SET_TURN_RATE", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_SPLINE_SET_TURN_RATE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_SPLINE_SET_TURN_RATE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SPLINE_SET_TURN_RATE {}

impl SMSG_SPLINE_SET_TURN_RATE {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.guid) // guid: PackedGuid
        + 4 // speed: f32
    }
}

