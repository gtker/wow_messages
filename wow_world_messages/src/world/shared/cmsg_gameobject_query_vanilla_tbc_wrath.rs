use std::io::{Read, Write};

use crate::Guid;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/cmsg_gameobject_query.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/cmsg_gameobject_query.wowm#L3):
/// ```text
/// cmsg CMSG_GAMEOBJECT_QUERY = 0x005E {
///     u32 entry_id;
///     Guid guid;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_GAMEOBJECT_QUERY {
    pub entry_id: u32,
    pub guid: Guid,
}

impl crate::private::Sealed for CMSG_GAMEOBJECT_QUERY {}
impl CMSG_GAMEOBJECT_QUERY {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 12 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // entry_id: u32
        let entry_id = crate::util::read_u32_le(&mut r)?;

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        Ok(Self {
            entry_id,
            guid,
        })
    }

}

impl crate::Message for CMSG_GAMEOBJECT_QUERY {
    const OPCODE: u32 = 0x005e;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_GAMEOBJECT_QUERY"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_GAMEOBJECT_QUERY {{").unwrap();
        // Members
        writeln!(s, "    entry_id = {};", self.entry_id).unwrap();
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 16_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 94_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "entry_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // entry_id: u32
        w.write_all(&self.entry_id.to_le_bytes())?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(94, "CMSG_GAMEOBJECT_QUERY", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_GAMEOBJECT_QUERY {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_GAMEOBJECT_QUERY {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_GAMEOBJECT_QUERY {}

