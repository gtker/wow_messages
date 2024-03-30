use std::io::{Read, Write};

/// Only exists as comment in azerothcore/trinitycore.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/smsg_battlefield_mgr_state_change.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/smsg_battlefield_mgr_state_change.wowm#L2):
/// ```text
/// smsg SMSG_BATTLEFIELD_MGR_STATE_CHANGE = 0x04E8 {
///     u32 unknown1;
///     u32 unknown2;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_BATTLEFIELD_MGR_STATE_CHANGE {
    pub unknown1: u32,
    pub unknown2: u32,
}

impl crate::private::Sealed for SMSG_BATTLEFIELD_MGR_STATE_CHANGE {}
impl SMSG_BATTLEFIELD_MGR_STATE_CHANGE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 8 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(&mut r)?;

        // unknown2: u32
        let unknown2 = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            unknown1,
            unknown2,
        })
    }

}

impl crate::Message for SMSG_BATTLEFIELD_MGR_STATE_CHANGE {
    const OPCODE: u32 = 0x04e8;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_BATTLEFIELD_MGR_STATE_CHANGE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_BATTLEFIELD_MGR_STATE_CHANGE {{").unwrap();
        // Members
        writeln!(s, "    unknown1 = {};", self.unknown1).unwrap();
        writeln!(s, "    unknown2 = {};", self.unknown2).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 10_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1256_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown1", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown2", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: u32
        w.write_all(&self.unknown2.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1256, "SMSG_BATTLEFIELD_MGR_STATE_CHANGE", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_BATTLEFIELD_MGR_STATE_CHANGE {}

