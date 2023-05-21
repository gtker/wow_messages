use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/msg_random_roll_server.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/msg_random_roll_server.wowm#L3):
/// ```text
/// smsg MSG_RANDOM_ROLL_Server = 0x01FB {
///     u32 minimum;
///     u32 maximum;
///     u32 actual_roll;
///     Guid guid;
/// }
/// ```
pub struct MSG_RANDOM_ROLL_Server {
    pub minimum: u32,
    pub maximum: u32,
    pub actual_roll: u32,
    pub guid: Guid,
}

impl crate::private::Sealed for MSG_RANDOM_ROLL_Server {}
impl MSG_RANDOM_ROLL_Server {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 20 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01FB, size: body_size });
        }

        // minimum: u32
        let minimum = crate::util::read_u32_le(&mut r)?;

        // maximum: u32
        let maximum = crate::util::read_u32_le(&mut r)?;

        // actual_roll: u32
        let actual_roll = crate::util::read_u32_le(&mut r)?;

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        Ok(Self {
            minimum,
            maximum,
            actual_roll,
            guid,
        })
    }

}

impl crate::Message for MSG_RANDOM_ROLL_Server {
    const OPCODE: u32 = 0x01fb;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_RANDOM_ROLL_Server {{").unwrap();
        // Members
        writeln!(s, "    minimum = {};", self.minimum).unwrap();
        writeln!(s, "    maximum = {};", self.maximum).unwrap();
        writeln!(s, "    actual_roll = {};", self.actual_roll).unwrap();
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 22_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 507_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "minimum", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "maximum", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "actual_roll", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        20
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // minimum: u32
        w.write_all(&self.minimum.to_le_bytes())?;

        // maximum: u32
        w.write_all(&self.maximum.to_le_bytes())?;

        // actual_roll: u32
        w.write_all(&self.actual_roll.to_le_bytes())?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for MSG_RANDOM_ROLL_Server {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for MSG_RANDOM_ROLL_Server {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_RANDOM_ROLL_Server {}

