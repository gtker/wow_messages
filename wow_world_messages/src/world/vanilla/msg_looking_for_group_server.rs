use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/msg_looking_for_group_server.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/msg_looking_for_group_server.wowm#L3):
/// ```text
/// smsg MSG_LOOKING_FOR_GROUP_Server = 0x01FF {
///     u32 unknown1;
/// }
/// ```
pub struct MSG_LOOKING_FOR_GROUP_Server {
    /// vmangos sets to 0. cmangos/mangoszero don't implement
    pub unknown1: u32,
}

impl crate::private::Sealed for MSG_LOOKING_FOR_GROUP_Server {}
impl MSG_LOOKING_FOR_GROUP_Server {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 4 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            unknown1,
        })
    }

}

impl crate::Message for MSG_LOOKING_FOR_GROUP_Server {
    const OPCODE: u32 = 0x01ff;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_LOOKING_FOR_GROUP_Server {{").unwrap();
        // Members
        writeln!(s, "    unknown1 = {};", self.unknown1).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 6_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 511_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown1", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(511, "MSG_LOOKING_FOR_GROUP_Server", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for MSG_LOOKING_FOR_GROUP_Server {}

