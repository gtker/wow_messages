use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/achievement/smsg_achievement_deleted.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/achievement/smsg_achievement_deleted.wowm#L1):
/// ```text
/// smsg SMSG_ACHIEVEMENT_DELETED = 0x049F {
///     u32 achievement;
/// }
/// ```
pub struct SMSG_ACHIEVEMENT_DELETED {
    pub achievement: u32,
}

#[cfg(feature = "print-testcase")]
impl SMSG_ACHIEVEMENT_DELETED {
    pub fn to_test_case_string(&self) -> Option<String> {
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
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_ACHIEVEMENT_DELETED {}
impl crate::Message for SMSG_ACHIEVEMENT_DELETED {
    const OPCODE: u32 = 0x049f;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_ACHIEVEMENT_DELETED::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // achievement: u32
        w.write_all(&self.achievement.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x049F, size: body_size });
        }

        // achievement: u32
        let achievement = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            achievement,
        })
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_ACHIEVEMENT_DELETED {}

