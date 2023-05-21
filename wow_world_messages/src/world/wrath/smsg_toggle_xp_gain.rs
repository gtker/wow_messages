use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Only exists as comment in azerothcore/trinitycore.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/exp/smsg_toggle_xp_gain.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/exp/smsg_toggle_xp_gain.wowm#L1):
/// ```text
/// smsg SMSG_TOGGLE_XP_GAIN = 0x04ED {
/// }
/// ```
pub struct SMSG_TOGGLE_XP_GAIN {
}

impl crate::private::Sealed for SMSG_TOGGLE_XP_GAIN {}
impl SMSG_TOGGLE_XP_GAIN {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 0 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        Ok(Self {
        })
    }

}

impl crate::Message for SMSG_TOGGLE_XP_GAIN {
    const OPCODE: u32 = 0x04ed;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_TOGGLE_XP_GAIN {{").unwrap();
        // Members

        writeln!(s, "}} [").unwrap();

        let [a, b] = 2_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1261_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();



        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1261, "SMSG_TOGGLE_XP_GAIN", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_TOGGLE_XP_GAIN {}

