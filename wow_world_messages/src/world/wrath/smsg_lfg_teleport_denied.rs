use std::io::{Read, Write};

use crate::wrath::LfgTeleportError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_lfg_teleport_denied.wowm:14`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_lfg_teleport_denied.wowm#L14):
/// ```text
/// smsg SMSG_LFG_TELEPORT_DENIED = 0x0200 {
///     LfgTeleportError error;
/// }
/// ```
pub struct SMSG_LFG_TELEPORT_DENIED {
    pub error: LfgTeleportError,
}

impl crate::private::Sealed for SMSG_LFG_TELEPORT_DENIED {}
impl SMSG_LFG_TELEPORT_DENIED {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 4 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // error: LfgTeleportError
        let error = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            error,
        })
    }

}

impl crate::Message for SMSG_LFG_TELEPORT_DENIED {
    const OPCODE: u32 = 0x0200;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_LFG_TELEPORT_DENIED"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_LFG_TELEPORT_DENIED {{").unwrap();
        // Members
        writeln!(s, "    error = {};", self.error.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 6_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 512_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "error", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // error: LfgTeleportError
        w.write_all(&(self.error.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(512, "SMSG_LFG_TELEPORT_DENIED", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_LFG_TELEPORT_DENIED {}

