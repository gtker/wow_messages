use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/msg_minimap_ping_server.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/msg_minimap_ping_server.wowm#L3):
/// ```text
/// smsg MSG_MINIMAP_PING_Server = 0x01D5 {
///     Guid guid;
///     f32 position_x;
///     f32 position_y;
/// }
/// ```
pub struct MSG_MINIMAP_PING_Server {
    pub guid: Guid,
    pub position_x: f32,
    pub position_y: f32,
}

impl crate::private::Sealed for MSG_MINIMAP_PING_Server {}
impl MSG_MINIMAP_PING_Server {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 16 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // position_x: f32
        let position_x = crate::util::read_f32_le(&mut r)?;

        // position_y: f32
        let position_y = crate::util::read_f32_le(&mut r)?;

        Ok(Self {
            guid,
            position_x,
            position_y,
        })
    }

}

impl crate::Message for MSG_MINIMAP_PING_Server {
    const OPCODE: u32 = 0x01d5;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_MINIMAP_PING_Server {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    {}", if self.position_x.to_string().contains('.') { self.position_x.to_string() } else { format!("{}.0", self.position_x) }).unwrap();
        writeln!(s, "    {}", if self.position_y.to_string().contains('.') { self.position_y.to_string() } else { format!("{}.0", self.position_y) }).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 18_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 469_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "position_x", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "position_y", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // position_x: f32
        w.write_all(&self.position_x.to_le_bytes())?;

        // position_y: f32
        w.write_all(&self.position_y.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(469, "MSG_MINIMAP_PING_Server", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for MSG_MINIMAP_PING_Server {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for MSG_MINIMAP_PING_Server {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_MINIMAP_PING_Server {}

