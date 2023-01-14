use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_userlist_remove.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_userlist_remove.wowm#L1):
/// ```text
/// smsg SMSG_USERLIST_REMOVE = 0x03F0 {
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

impl crate::Message for SMSG_USERLIST_REMOVE {
    const OPCODE: u32 = 0x03f0;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

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

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(14..=269).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03F0, size: body_size as u32 });
        }

        // player: Guid
        let player = Guid::read(r)?;

        // flags: u8
        let flags = crate::util::read_u8_le(r)?;

        // amount_of_players: u32
        let amount_of_players = crate::util::read_u32_le(r)?;

        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        Ok(Self {
            player,
            flags,
            amount_of_players,
            name,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_USERLIST_REMOVE {}

impl SMSG_USERLIST_REMOVE {
    pub(crate) fn size(&self) -> usize {
        8 // player: Guid
        + 1 // flags: u8
        + 4 // amount_of_players: u32
        + self.name.len() + 1 // name: CString
    }
}

