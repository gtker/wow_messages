use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/achievement/smsg_achievement_deleted.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/achievement/smsg_achievement_deleted.wowm#L1):
/// ```text
/// smsg SMSG_ACHIEVEMENT_DELETED = 0x049F {
///     u32 achievement;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_ACHIEVEMENT_DELETED {
    pub achievement: u32,
}

impl crate::private::Sealed for SMSG_ACHIEVEMENT_DELETED {}
impl SMSG_ACHIEVEMENT_DELETED {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 4 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // achievement: u32
        let achievement = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            achievement,
        })
    }

}

impl crate::Message for SMSG_ACHIEVEMENT_DELETED {
    const OPCODE: u32 = 0x049f;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_ACHIEVEMENT_DELETED"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_ACHIEVEMENT_DELETED {{").unwrap();
        // Members
        writeln!(s, "    achievement = {};", self.achievement).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 6_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1183_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "achievement", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // achievement: u32
        w.write_all(&self.achievement.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1183, "SMSG_ACHIEVEMENT_DELETED", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ACHIEVEMENT_DELETED {}

