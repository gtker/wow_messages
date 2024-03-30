use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::AchievementNameLinkType;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/achievement/smsg_server_first_achievement.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/achievement/smsg_server_first_achievement.wowm#L8):
/// ```text
/// smsg SMSG_SERVER_FIRST_ACHIEVEMENT = 0x0498 {
///     CString name;
///     Guid player;
///     u32 achievement;
///     AchievementNameLinkType link_type;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_SERVER_FIRST_ACHIEVEMENT {
    pub name: String,
    pub player: Guid,
    pub achievement: u32,
    pub link_type: AchievementNameLinkType,
}

impl crate::private::Sealed for SMSG_SERVER_FIRST_ACHIEVEMENT {}
impl SMSG_SERVER_FIRST_ACHIEVEMENT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(14..=269).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // name: CString
        let name = {
            let name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(name)?
        };

        // player: Guid
        let player = crate::util::read_guid(&mut r)?;

        // achievement: u32
        let achievement = crate::util::read_u32_le(&mut r)?;

        // link_type: AchievementNameLinkType
        let link_type = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            name,
            player,
            achievement,
            link_type,
        })
    }

}

impl crate::Message for SMSG_SERVER_FIRST_ACHIEVEMENT {
    const OPCODE: u32 = 0x0498;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_SERVER_FIRST_ACHIEVEMENT"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SERVER_FIRST_ACHIEVEMENT {{").unwrap();
        // Members
        writeln!(s, "    name = \"{}\";", self.name).unwrap();
        writeln!(s, "    player = {};", self.player.guid()).unwrap();
        writeln!(s, "    achievement = {};", self.achievement).unwrap();
        writeln!(s, "    link_type = {};", self.link_type.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1176_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, self.name.len() + 1, "name", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "player", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "achievement", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "link_type", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().next_back(), Some(&0_u8), "String `name` must not be null-terminated.");
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

        // achievement: u32
        w.write_all(&self.achievement.to_le_bytes())?;

        // link_type: AchievementNameLinkType
        w.write_all(&(self.link_type.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1176, "SMSG_SERVER_FIRST_ACHIEVEMENT", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SERVER_FIRST_ACHIEVEMENT {}

impl SMSG_SERVER_FIRST_ACHIEVEMENT {
    pub(crate) fn size(&self) -> usize {
        self.name.len() + 1 // name: CString
        + 8 // player: Guid
        + 4 // achievement: u32
        + 1 // link_type: AchievementNameLinkType
    }
}

