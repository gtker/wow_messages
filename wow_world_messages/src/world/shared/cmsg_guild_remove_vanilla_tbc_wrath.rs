use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/cmsg_guild_remove.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/cmsg_guild_remove.wowm#L3):
/// ```text
/// cmsg CMSG_GUILD_REMOVE = 0x008E {
///     CString player_name;
/// }
/// ```
pub struct CMSG_GUILD_REMOVE {
    pub player_name: String,
}

impl crate::Message for CMSG_GUILD_REMOVE {
    const OPCODE: u32 = 0x008e;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // player_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.player_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `player_name` must not be null-terminated.");
        w.write_all(self.player_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(1..=256).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x008E, size: body_size as u32 });
        }

        // player_name: CString
        let player_name = {
            let player_name = crate::util::read_c_string_to_vec(r)?;
            String::from_utf8(player_name)?
        };

        Ok(Self {
            player_name,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_GUILD_REMOVE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_GUILD_REMOVE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_GUILD_REMOVE {}

impl CMSG_GUILD_REMOVE {
    pub(crate) fn size(&self) -> usize {
        self.player_name.len() + 1 // player_name: CString
    }
}

