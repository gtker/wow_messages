use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/duel/smsg_duel_complete.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/duel/smsg_duel_complete.wowm#L3):
/// ```text
/// smsg SMSG_DUEL_COMPLETE = 0x016A {
///     Bool ended_without_interruption;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_DUEL_COMPLETE {
    pub ended_without_interruption: bool,
}

impl crate::private::Sealed for SMSG_DUEL_COMPLETE {}
impl SMSG_DUEL_COMPLETE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 1 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // ended_without_interruption: Bool
        let ended_without_interruption = crate::util::read_bool_u8(&mut r)?;

        Ok(Self {
            ended_without_interruption,
        })
    }

}

impl crate::Message for SMSG_DUEL_COMPLETE {
    const OPCODE: u32 = 0x016a;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_DUEL_COMPLETE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_DUEL_COMPLETE {{").unwrap();
        // Members
        writeln!(s, "    ended_without_interruption = {};", if self.ended_without_interruption { "TRUE" } else { "FALSE" }).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 3_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 362_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "ended_without_interruption", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // ended_without_interruption: Bool
        w.write_all(u8::from(self.ended_without_interruption).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(362, "SMSG_DUEL_COMPLETE", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_DUEL_COMPLETE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_DUEL_COMPLETE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_DUEL_COMPLETE {}

