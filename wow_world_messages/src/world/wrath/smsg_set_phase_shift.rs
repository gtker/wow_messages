use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/smsg_set_phase_shift.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_set_phase_shift.wowm#L1):
/// ```text
/// smsg SMSG_SET_PHASE_SHIFT = 0x047C {
///     u32 new_phase;
/// }
/// ```
pub struct SMSG_SET_PHASE_SHIFT {
    pub new_phase: u32,
}

impl crate::private::Sealed for SMSG_SET_PHASE_SHIFT {}
impl SMSG_SET_PHASE_SHIFT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 4 {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x047C, size: body_size });
        }

        // new_phase: u32
        let new_phase = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            new_phase,
        })
    }

}

impl crate::Message for SMSG_SET_PHASE_SHIFT {
    const OPCODE: u32 = 0x047c;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SET_PHASE_SHIFT {{").unwrap();
        // Members
        writeln!(s, "    new_phase = {};", self.new_phase).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 6_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1148_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "new_phase", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // new_phase: u32
        w.write_all(&self.new_phase.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SET_PHASE_SHIFT {}

