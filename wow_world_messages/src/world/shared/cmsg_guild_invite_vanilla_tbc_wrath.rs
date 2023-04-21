use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/cmsg_guild_invite.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/cmsg_guild_invite.wowm#L3):
/// ```text
/// cmsg CMSG_GUILD_INVITE = 0x0082 {
///     CString invited_player;
/// }
/// ```
pub struct CMSG_GUILD_INVITE {
    pub invited_player: String,
}

impl crate::private::Sealed for CMSG_GUILD_INVITE {}
impl crate::Message for CMSG_GUILD_INVITE {
    const OPCODE: u32 = 0x0082;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // invited_player: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.invited_player.as_bytes().iter().rev().next(), Some(&0_u8), "String `invited_player` must not be null-terminated.");
        w.write_all(self.invited_player.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(1..=256).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0082, size: body_size as u32 });
        }

        // invited_player: CString
        let invited_player = {
            let invited_player = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(invited_player)?
        };

        Ok(Self {
            invited_player,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_GUILD_INVITE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_GUILD_INVITE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_GUILD_INVITE {}

impl CMSG_GUILD_INVITE {
    pub(crate) fn size(&self) -> usize {
        self.invited_player.len() + 1 // invited_player: CString
    }
}

