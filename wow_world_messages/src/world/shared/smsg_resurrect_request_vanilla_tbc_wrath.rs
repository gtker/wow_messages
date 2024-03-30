use std::io::{Read, Write};

use crate::Guid;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/resurrect/smsg_resurrect_request.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/resurrect/smsg_resurrect_request.wowm#L1):
/// ```text
/// smsg SMSG_RESURRECT_REQUEST = 0x015B {
///     Guid guid;
///     SizedCString name;
///     Bool player;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_RESURRECT_REQUEST {
    pub guid: Guid,
    pub name: String,
    pub player: bool,
}

impl crate::private::Sealed for SMSG_RESURRECT_REQUEST {}
impl SMSG_RESURRECT_REQUEST {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(14..=8013).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // name: SizedCString
        let name = {
            let name = crate::util::read_u32_le(&mut r)?;
            let name = crate::util::read_sized_c_string_to_vec(&mut r, name)?;
            String::from_utf8(name)?
        };

        // player: Bool
        let player = crate::util::read_bool_u8(&mut r)?;

        Ok(Self {
            guid,
            name,
            player,
        })
    }

}

impl crate::Message for SMSG_RESURRECT_REQUEST {
    const OPCODE: u32 = 0x015b;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_RESURRECT_REQUEST"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_RESURRECT_REQUEST {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    name = \"{}\";", self.name).unwrap();
        writeln!(s, "    player = {};", if self.player { "TRUE" } else { "FALSE" }).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 347_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.name.len() + 5, "name", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "player", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // name: SizedCString
        w.write_all(&((self.name.len() + 1) as u32).to_le_bytes())?;
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // player: Bool
        w.write_all(u8::from(self.player).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(347, "SMSG_RESURRECT_REQUEST", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_RESURRECT_REQUEST {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_RESURRECT_REQUEST {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_RESURRECT_REQUEST {}

impl SMSG_RESURRECT_REQUEST {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + self.name.len() + 5 // name: SizedCString
        + 1 // player: Bool
    }
}

