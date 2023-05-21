use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_spline_move_set_run_mode.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_spline_move_set_run_mode.wowm#L3):
/// ```text
/// smsg SMSG_SPLINE_MOVE_SET_RUN_MODE = 0x030D {
///     PackedGuid guid;
/// }
/// ```
pub struct SMSG_SPLINE_MOVE_SET_RUN_MODE {
    pub guid: Guid,
}

impl crate::private::Sealed for SMSG_SPLINE_MOVE_SET_RUN_MODE {}
impl SMSG_SPLINE_MOVE_SET_RUN_MODE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(2..=9).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid: PackedGuid
        let guid = crate::util::read_packed_guid(&mut r)?;

        Ok(Self {
            guid,
        })
    }

}

impl crate::Message for SMSG_SPLINE_MOVE_SET_RUN_MODE {
    const OPCODE: u32 = 0x030d;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SPLINE_MOVE_SET_RUN_MODE {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 781_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.guid), "guid", "    ");


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

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(781, "SMSG_SPLINE_MOVE_SET_RUN_MODE", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_SPLINE_MOVE_SET_RUN_MODE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_SPLINE_MOVE_SET_RUN_MODE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SPLINE_MOVE_SET_RUN_MODE {}

impl SMSG_SPLINE_MOVE_SET_RUN_MODE {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.guid) // guid: PackedGuid
    }
}

