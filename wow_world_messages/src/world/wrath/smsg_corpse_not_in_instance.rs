use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/smsg_corpse_not_in_instance.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_corpse_not_in_instance.wowm#L1):
/// ```text
/// smsg SMSG_CORPSE_NOT_IN_INSTANCE = 0x0506 {
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_CORPSE_NOT_IN_INSTANCE {
}

impl crate::private::Sealed for SMSG_CORPSE_NOT_IN_INSTANCE {}
impl SMSG_CORPSE_NOT_IN_INSTANCE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 0 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        Ok(Self {
        })
    }

}

impl crate::Message for SMSG_CORPSE_NOT_IN_INSTANCE {
    const OPCODE: u32 = 0x0506;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_CORPSE_NOT_IN_INSTANCE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_CORPSE_NOT_IN_INSTANCE {{").unwrap();
        // Members

        writeln!(s, "}} [").unwrap();

        let [a, b] = 2_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1286_u16.to_le_bytes();
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
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1286, "SMSG_CORPSE_NOT_IN_INSTANCE", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CORPSE_NOT_IN_INSTANCE {}

