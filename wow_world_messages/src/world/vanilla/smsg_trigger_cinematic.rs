use std::io::{Read, Write};

use crate::vanilla::CinematicSequenceId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/cinematic/smsg_trigger_cinematic.wowm:15`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/cinematic/smsg_trigger_cinematic.wowm#L15):
/// ```text
/// smsg SMSG_TRIGGER_CINEMATIC = 0x00FA {
///     CinematicSequenceId cinematic_sequence_id;
/// }
/// ```
pub struct SMSG_TRIGGER_CINEMATIC {
    pub cinematic_sequence_id: CinematicSequenceId,
}

impl crate::private::Sealed for SMSG_TRIGGER_CINEMATIC {}
impl SMSG_TRIGGER_CINEMATIC {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 4 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // cinematic_sequence_id: CinematicSequenceId
        let cinematic_sequence_id = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            cinematic_sequence_id,
        })
    }

}

impl crate::Message for SMSG_TRIGGER_CINEMATIC {
    const OPCODE: u32 = 0x00fa;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_TRIGGER_CINEMATIC {{").unwrap();
        // Members
        writeln!(s, "    cinematic_sequence_id = {};", self.cinematic_sequence_id.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 6_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 250_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "cinematic_sequence_id", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // cinematic_sequence_id: CinematicSequenceId
        w.write_all(&(self.cinematic_sequence_id.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(250, "SMSG_TRIGGER_CINEMATIC", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_TRIGGER_CINEMATIC {}

