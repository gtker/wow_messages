use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/smsg_battlefield_mgr_entered.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/smsg_battlefield_mgr_entered.wowm#L1):
/// ```text
/// smsg SMSG_BATTLEFIELD_MGR_ENTERED = 0x04E0 {
///     u32 battle_id;
///     u8 unknown1;
///     u8 unknown2;
///     Bool clear_afk;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_BATTLEFIELD_MGR_ENTERED {
    pub battle_id: u32,
    pub unknown1: u8,
    pub unknown2: u8,
    pub clear_afk: bool,
}

impl crate::private::Sealed for SMSG_BATTLEFIELD_MGR_ENTERED {}
impl SMSG_BATTLEFIELD_MGR_ENTERED {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 7 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // battle_id: u32
        let battle_id = crate::util::read_u32_le(&mut r)?;

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(&mut r)?;

        // unknown2: u8
        let unknown2 = crate::util::read_u8_le(&mut r)?;

        // clear_afk: Bool
        let clear_afk = crate::util::read_bool_u8(&mut r)?;

        Ok(Self {
            battle_id,
            unknown1,
            unknown2,
            clear_afk,
        })
    }

}

impl crate::Message for SMSG_BATTLEFIELD_MGR_ENTERED {
    const OPCODE: u32 = 0x04e0;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_BATTLEFIELD_MGR_ENTERED"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_BATTLEFIELD_MGR_ENTERED {{").unwrap();
        // Members
        writeln!(s, "    battle_id = {};", self.battle_id).unwrap();
        writeln!(s, "    unknown1 = {};", self.unknown1).unwrap();
        writeln!(s, "    unknown2 = {};", self.unknown2).unwrap();
        writeln!(s, "    clear_afk = {};", if self.clear_afk { "TRUE" } else { "FALSE" }).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 9_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1248_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "battle_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "unknown1", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "unknown2", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "clear_afk", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        7
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // battle_id: u32
        w.write_all(&self.battle_id.to_le_bytes())?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: u8
        w.write_all(&self.unknown2.to_le_bytes())?;

        // clear_afk: Bool
        w.write_all(u8::from(self.clear_afk).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1248, "SMSG_BATTLEFIELD_MGR_ENTERED", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_BATTLEFIELD_MGR_ENTERED {}

