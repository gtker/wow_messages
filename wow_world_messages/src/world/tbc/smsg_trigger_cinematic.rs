use std::io::{Read, Write};

use crate::tbc::CinematicSequenceId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/cinematic/smsg_trigger_cinematic.wowm:40`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/cinematic/smsg_trigger_cinematic.wowm#L40):
/// ```text
/// smsg SMSG_TRIGGER_CINEMATIC = 0x00FA {
///     CinematicSequenceId cinematic_sequence_id;
/// }
/// ```
pub struct SMSG_TRIGGER_CINEMATIC {
    pub cinematic_sequence_id: CinematicSequenceId,
}

#[cfg(feature = "print-testcase")]
impl SMSG_TRIGGER_CINEMATIC {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_TRIGGER_CINEMATIC {{").unwrap();
        // Members
        writeln!(s, "    cinematic_sequence_id = {};", self.cinematic_sequence_id.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 8_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 250_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "cinematic_sequence_id");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"2.4.3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_TRIGGER_CINEMATIC {}
impl crate::Message for SMSG_TRIGGER_CINEMATIC {
    const OPCODE: u32 = 0x00fa;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // cinematic_sequence_id: CinematicSequenceId
        w.write_all(&(self.cinematic_sequence_id.as_int().to_le_bytes()))?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00FA, size: body_size });
        }

        // cinematic_sequence_id: CinematicSequenceId
        let cinematic_sequence_id = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            cinematic_sequence_id,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_TRIGGER_CINEMATIC {}

