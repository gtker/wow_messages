use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_userlist_remove.wowm:10`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_userlist_remove.wowm#L10):
/// ```text
/// smsg SMSG_USERLIST_REMOVE = 0x03F1 {
///     Guid player;
///     u8 flags;
///     u32 amount_of_players;
///     CString name;
/// }
/// ```
pub struct SMSG_USERLIST_REMOVE {
    pub player: Guid,
    pub flags: u8,
    pub amount_of_players: u32,
    pub name: String,
}

impl crate::private::Sealed for SMSG_USERLIST_REMOVE {}
impl SMSG_USERLIST_REMOVE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(14..=269).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // player: Guid
        let player = crate::util::read_guid(&mut r)?;

        // flags: u8
        let flags = crate::util::read_u8_le(&mut r)?;

        // amount_of_players: u32
        let amount_of_players = crate::util::read_u32_le(&mut r)?;

        // name: CString
        let name = {
            let name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(name)?
        };

        Ok(Self {
            player,
            flags,
            amount_of_players,
            name,
        })
    }

}

impl crate::Message for SMSG_USERLIST_REMOVE {
    const OPCODE: u32 = 0x03f1;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_USERLIST_REMOVE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_USERLIST_REMOVE {{").unwrap();
        // Members
        writeln!(s, "    player = {};", self.player.guid()).unwrap();
        writeln!(s, "    flags = {};", self.flags).unwrap();
        writeln!(s, "    amount_of_players = {};", self.amount_of_players).unwrap();
        writeln!(s, "    name = \"{}\";", self.name).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1009_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "player", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "flags", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_players", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.name.len() + 1, "name", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

        // flags: u8
        w.write_all(&self.flags.to_le_bytes())?;

        // amount_of_players: u32
        w.write_all(&self.amount_of_players.to_le_bytes())?;

        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().next_back(), Some(&0_u8), "String `name` must not be null-terminated.");
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1009, "SMSG_USERLIST_REMOVE", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_USERLIST_REMOVE {}

impl SMSG_USERLIST_REMOVE {
    pub(crate) fn size(&self) -> usize {
        8 // player: Guid
        + 1 // flags: u8
        + 4 // amount_of_players: u32
        + self.name.len() + 1 // name: CString
    }
}

