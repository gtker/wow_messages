use std::io::{Read, Write};

use wow_world_base::shared::arena_type_tbc_wrath::ArenaType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/arena/smsg_arena_error.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/arena/smsg_arena_error.wowm#L1):
/// ```text
/// smsg SMSG_ARENA_ERROR = 0x0376 {
///     u32 unknown;
///     ArenaType arena_type;
/// }
/// ```
pub struct SMSG_ARENA_ERROR {
    pub unknown: u32,
    pub arena_type: ArenaType,
}

impl crate::private::Sealed for SMSG_ARENA_ERROR {}
impl SMSG_ARENA_ERROR {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 5 {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x0376, size: body_size });
        }

        // unknown: u32
        let unknown = crate::util::read_u32_le(&mut r)?;

        // arena_type: ArenaType
        let arena_type = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            unknown,
            arena_type,
        })
    }

}

impl crate::Message for SMSG_ARENA_ERROR {
    const OPCODE: u32 = 0x0376;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_ARENA_ERROR {{").unwrap();
        // Members
        writeln!(s, "    unknown = {};", self.unknown).unwrap();
        writeln!(s, "    arena_type = {};", self.arena_type.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 7_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 886_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "arena_type", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        5
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // unknown: u32
        w.write_all(&self.unknown.to_le_bytes())?;

        // arena_type: ArenaType
        w.write_all(&(self.arena_type.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_ARENA_ERROR {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ARENA_ERROR {}

