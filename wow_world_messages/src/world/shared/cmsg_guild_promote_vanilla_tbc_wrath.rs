use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/cmsg_guild_promote.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/cmsg_guild_promote.wowm#L3):
/// ```text
/// cmsg CMSG_GUILD_PROMOTE = 0x008B {
///     CString player_name;
/// }
/// ```
pub struct CMSG_GUILD_PROMOTE {
    pub player_name: String,
}

impl crate::Message for CMSG_GUILD_PROMOTE {
    const OPCODE: u32 = 0x008b;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // player_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.player_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `player_name` must not be null-terminated.");
        w.write_all(self.player_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        assert_eq!(self.size() as usize, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // player_name: CString
        let player_name = crate::util::read_c_string_to_vec(r)?;
        let player_name = String::from_utf8(player_name)?;

        Ok(Self {
            player_name,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_GUILD_PROMOTE {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_GUILD_PROMOTE {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_GUILD_PROMOTE {}

impl CMSG_GUILD_PROMOTE {
    pub(crate) fn size(&self) -> usize {
        self.player_name.len() + 1 // player_name: CString
    }
}

