use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_userlist_add.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_userlist_add.wowm#L1):
/// ```text
/// smsg SMSG_USERLIST_ADD = 0x03EF {
///     Guid player;
///     u8 player_flags;
///     u8 flags;
///     u32 amount_of_players;
///     CString name;
/// }
/// ```
pub struct SMSG_USERLIST_ADD {
    pub player: Guid,
    pub player_flags: u8,
    pub flags: u8,
    pub amount_of_players: u32,
    pub name: String,
}

#[cfg(feature = "print-testcase")]
impl SMSG_USERLIST_ADD {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_USERLIST_ADD {{").unwrap();
        // Members
        writeln!(s, "    player = {};", self.player.guid()).unwrap();
        writeln!(s, "    player_flags = {};", self.player_flags).unwrap();
        writeln!(s, "    flags = {};", self.flags).unwrap();
        writeln!(s, "    amount_of_players = {};", self.amount_of_players).unwrap();
        writeln!(s, "    name = \"{}\";", self.name).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1007_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "player");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"2.4.3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_USERLIST_ADD {}
impl crate::Message for SMSG_USERLIST_ADD {
    const OPCODE: u32 = 0x03ef;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

        // player_flags: u8
        w.write_all(&self.player_flags.to_le_bytes())?;

        // flags: u8
        w.write_all(&self.flags.to_le_bytes())?;

        // amount_of_players: u32
        w.write_all(&self.amount_of_players.to_le_bytes())?;

        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().rev().next(), Some(&0_u8), "String `name` must not be null-terminated.");
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(15..=270).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03EF, size: body_size });
        }

        // player: Guid
        let player = crate::util::read_guid(&mut r)?;

        // player_flags: u8
        let player_flags = crate::util::read_u8_le(&mut r)?;

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
            player_flags,
            flags,
            amount_of_players,
            name,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_USERLIST_ADD {}

impl SMSG_USERLIST_ADD {
    pub(crate) fn size(&self) -> usize {
        8 // player: Guid
        + 1 // player_flags: u8
        + 1 // flags: u8
        + 4 // amount_of_players: u32
        + self.name.len() + 1 // name: CString
    }
}

