use std::io::{Read, Write};

use crate::Guid;
use wow_world_base::shared::join_arena_type_tbc_wrath::JoinArenaType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/cmsg_battlemaster_join_arena.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/cmsg_battlemaster_join_arena.wowm#L9):
/// ```text
/// cmsg CMSG_BATTLEMASTER_JOIN_ARENA = 0x0358 {
///     Guid battlemaster;
///     JoinArenaType arena_type;
///     Bool as_group;
///     Bool rated;
/// }
/// ```
pub struct CMSG_BATTLEMASTER_JOIN_ARENA {
    pub battlemaster: Guid,
    pub arena_type: JoinArenaType,
    pub as_group: bool,
    pub rated: bool,
}

impl crate::private::Sealed for CMSG_BATTLEMASTER_JOIN_ARENA {}
impl CMSG_BATTLEMASTER_JOIN_ARENA {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 11 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // battlemaster: Guid
        let battlemaster = crate::util::read_guid(&mut r)?;

        // arena_type: JoinArenaType
        let arena_type = crate::util::read_u8_le(&mut r)?.try_into()?;

        // as_group: Bool
        let as_group = crate::util::read_u8_le(&mut r)? != 0;

        // rated: Bool
        let rated = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            battlemaster,
            arena_type,
            as_group,
            rated,
        })
    }

}

impl crate::Message for CMSG_BATTLEMASTER_JOIN_ARENA {
    const OPCODE: u32 = 0x0358;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_BATTLEMASTER_JOIN_ARENA {{").unwrap();
        // Members
        writeln!(s, "    battlemaster = {};", self.battlemaster.guid()).unwrap();
        writeln!(s, "    arena_type = {};", self.arena_type.as_test_case_value()).unwrap();
        writeln!(s, "    as_group = {};", if self.as_group { "TRUE" } else { "FALSE" }).unwrap();
        writeln!(s, "    rated = {};", if self.rated { "TRUE" } else { "FALSE" }).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 15_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 856_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "battlemaster", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "arena_type", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "as_group", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "rated", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        11
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // battlemaster: Guid
        w.write_all(&self.battlemaster.guid().to_le_bytes())?;

        // arena_type: JoinArenaType
        w.write_all(&(self.arena_type.as_int().to_le_bytes()))?;

        // as_group: Bool
        w.write_all(u8::from(self.as_group).to_le_bytes().as_slice())?;

        // rated: Bool
        w.write_all(u8::from(self.rated).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(856, "CMSG_BATTLEMASTER_JOIN_ARENA", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_BATTLEMASTER_JOIN_ARENA {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_BATTLEMASTER_JOIN_ARENA {}

