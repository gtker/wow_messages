use std::io::{Read, Write};

use crate::tbc::LfgType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/msg_looking_for_group.wowm:13`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/msg_looking_for_group.wowm#L13):
/// ```text
/// cmsg MSG_LOOKING_FOR_GROUP_Client = 0x01FF {
///     (u32)LfgType lfg_type;
///     u32 entry;
///     u32 unknown;
/// }
/// ```
pub struct MSG_LOOKING_FOR_GROUP_Client {
    pub lfg_type: LfgType,
    /// entry from LfgDunggeons.dbc
    pub entry: u32,
    pub unknown: u32,
}

#[cfg(feature = "print-testcase")]
impl MSG_LOOKING_FOR_GROUP_Client {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_LOOKING_FOR_GROUP_Client {{").unwrap();
        // Members
        writeln!(s, "    lfg_type = {};", self.lfg_type.as_test_case_value()).unwrap();
        writeln!(s, "    entry = {};", self.entry).unwrap();
        writeln!(s, "    unknown = {};", self.unknown).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 16_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 511_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "lfg_type", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "entry", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for MSG_LOOKING_FOR_GROUP_Client {}
impl crate::Message for MSG_LOOKING_FOR_GROUP_Client {
    const OPCODE: u32 = 0x01ff;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        MSG_LOOKING_FOR_GROUP_Client::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // lfg_type: LfgType
        w.write_all(&u32::from(self.lfg_type.as_int()).to_le_bytes())?;

        // entry: u32
        w.write_all(&self.entry.to_le_bytes())?;

        // unknown: u32
        w.write_all(&self.unknown.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01FF, size: body_size });
        }

        // lfg_type: LfgType
        let lfg_type = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        // entry: u32
        let entry = crate::util::read_u32_le(&mut r)?;

        // unknown: u32
        let unknown = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            lfg_type,
            entry,
            unknown,
        })
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for MSG_LOOKING_FOR_GROUP_Client {}

