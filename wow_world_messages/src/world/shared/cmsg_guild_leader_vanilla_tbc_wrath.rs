use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/cmsg_guild_leader.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/cmsg_guild_leader.wowm#L3):
/// ```text
/// cmsg CMSG_GUILD_LEADER = 0x0090 {
///     CString new_guild_leader_name;
/// }
/// ```
pub struct CMSG_GUILD_LEADER {
    pub new_guild_leader_name: String,
}

impl crate::Message for CMSG_GUILD_LEADER {
    const OPCODE: u32 = 0x0090;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // new_guild_leader_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.new_guild_leader_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `new_guild_leader_name` must not be null-terminated.");
        w.write_all(self.new_guild_leader_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size < 1 || body_size > 256 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0090, size: body_size as u32 });
        }

        // new_guild_leader_name: CString
        let new_guild_leader_name = crate::util::read_c_string_to_vec(r)?;
        let new_guild_leader_name = String::from_utf8(new_guild_leader_name)?;

        Ok(Self {
            new_guild_leader_name,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_GUILD_LEADER {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_GUILD_LEADER {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_GUILD_LEADER {}

impl CMSG_GUILD_LEADER {
    pub(crate) fn size(&self) -> usize {
        self.new_guild_leader_name.len() + 1 // new_guild_leader_name: CString
    }
}

