use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_clear_cooldown.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_clear_cooldown.wowm#L3):
/// ```text
/// smsg SMSG_CLEAR_COOLDOWN = 0x01DE {
///     u32 id;
///     Guid target;
/// }
/// ```
pub struct SMSG_CLEAR_COOLDOWN {
    pub id: u32,
    pub target: Guid,
}

impl crate::private::Sealed for SMSG_CLEAR_COOLDOWN {}
impl SMSG_CLEAR_COOLDOWN {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 12 {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x01DE, size: body_size });
        }

        // id: u32
        let id = crate::util::read_u32_le(&mut r)?;

        // target: Guid
        let target = crate::util::read_guid(&mut r)?;

        Ok(Self {
            id,
            target,
        })
    }

}

impl crate::Message for SMSG_CLEAR_COOLDOWN {
    const OPCODE: u32 = 0x01de;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_CLEAR_COOLDOWN {{").unwrap();
        // Members
        writeln!(s, "    id = {};", self.id).unwrap();
        writeln!(s, "    target = {};", self.target.guid()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 14_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 478_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "target", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // target: Guid
        w.write_all(&self.target.guid().to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_CLEAR_COOLDOWN {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_CLEAR_COOLDOWN {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CLEAR_COOLDOWN {}

