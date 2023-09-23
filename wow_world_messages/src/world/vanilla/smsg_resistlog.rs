use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Structure as comment on `https://github1s.com/mangoszero/server/blob/HEAD/src/game/Server/Opcodes.h#L525`.
/// Not used in azerothcore/trinitycore/mangostwo/arcemu.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_resistlog.wowm:5`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_resistlog.wowm#L5):
/// ```text
/// smsg SMSG_RESISTLOG = 0x01D6 {
///     Guid guid1;
///     Guid guid2;
///     u32 unknown1;
///     f32 unknown2;
///     f32 unknown3;
///     u32 unknown4;
///     u32 unknown5;
/// }
/// ```
pub struct SMSG_RESISTLOG {
    pub guid1: Guid,
    pub guid2: Guid,
    pub unknown1: u32,
    pub unknown2: f32,
    pub unknown3: f32,
    pub unknown4: u32,
    pub unknown5: u32,
}

impl crate::private::Sealed for SMSG_RESISTLOG {}
impl SMSG_RESISTLOG {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 36 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid1: Guid
        let guid1 = crate::util::read_guid(&mut r)?;

        // guid2: Guid
        let guid2 = crate::util::read_guid(&mut r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(&mut r)?;

        // unknown2: f32
        let unknown2 = crate::util::read_f32_le(&mut r)?;

        // unknown3: f32
        let unknown3 = crate::util::read_f32_le(&mut r)?;

        // unknown4: u32
        let unknown4 = crate::util::read_u32_le(&mut r)?;

        // unknown5: u32
        let unknown5 = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            guid1,
            guid2,
            unknown1,
            unknown2,
            unknown3,
            unknown4,
            unknown5,
        })
    }

}

impl crate::Message for SMSG_RESISTLOG {
    const OPCODE: u32 = 0x01d6;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_RESISTLOG"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_RESISTLOG {{").unwrap();
        // Members
        writeln!(s, "    guid1 = {};", self.guid1.guid()).unwrap();
        writeln!(s, "    guid2 = {};", self.guid2.guid()).unwrap();
        writeln!(s, "    unknown1 = {};", self.unknown1).unwrap();
        writeln!(s, "    unknown2 = {};", if self.unknown2.to_string().contains('.') { self.unknown2.to_string() } else { format!("{}.0", self.unknown2) }).unwrap();
        writeln!(s, "    unknown3 = {};", if self.unknown3.to_string().contains('.') { self.unknown3.to_string() } else { format!("{}.0", self.unknown3) }).unwrap();
        writeln!(s, "    unknown4 = {};", self.unknown4).unwrap();
        writeln!(s, "    unknown5 = {};", self.unknown5).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 38_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 470_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid1", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid2", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown1", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown2", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown3", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown4", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown5", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        36
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid1: Guid
        w.write_all(&self.guid1.guid().to_le_bytes())?;

        // guid2: Guid
        w.write_all(&self.guid2.guid().to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: f32
        w.write_all(&self.unknown2.to_le_bytes())?;

        // unknown3: f32
        w.write_all(&self.unknown3.to_le_bytes())?;

        // unknown4: u32
        w.write_all(&self.unknown4.to_le_bytes())?;

        // unknown5: u32
        w.write_all(&self.unknown5.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(470, "SMSG_RESISTLOG", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_RESISTLOG {}

