use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_userlist_update.wowm:11`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_userlist_update.wowm#L11):
/// ```text
/// smsg SMSG_USERLIST_UPDATE = 0x03F2 {
///     Guid player;
///     u8 player_flags;
///     u8 flags;
///     u32 amount_of_players;
///     CString name;
/// }
/// ```
pub struct SMSG_USERLIST_UPDATE {
    pub player: Guid,
    pub player_flags: u8,
    pub flags: u8,
    pub amount_of_players: u32,
    pub name: String,
}

impl crate::Message for SMSG_USERLIST_UPDATE {
    const OPCODE: u32 = 0x03f2;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
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
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(15..=270).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03F2, size: body_size as u32 });
        }

        // player: Guid
        let player = Guid::read(r)?;

        // player_flags: u8
        let player_flags = crate::util::read_u8_le(r)?;

        // flags: u8
        let flags = crate::util::read_u8_le(r)?;

        // amount_of_players: u32
        let amount_of_players = crate::util::read_u32_le(r)?;

        // name: CString
        let name = {
            let name = crate::util::read_c_string_to_vec(r)?;
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
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_USERLIST_UPDATE {}

impl SMSG_USERLIST_UPDATE {
    pub(crate) fn size(&self) -> usize {
        8 // player: Guid
        + 1 // player_flags: u8
        + 1 // flags: u8
        + 4 // amount_of_players: u32
        + self.name.len() + 1 // name: CString
    }
}

