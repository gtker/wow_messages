use crate::DateTime;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/smsg_guild_info.wowm:12`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_guild_info.wowm#L12):
/// ```text
/// smsg SMSG_GUILD_INFO = 0x0088 {
///     CString guild_name;
///     DateTime created;
///     u32 amount_of_characters_in_guild;
///     u32 amount_of_accounts_in_guild;
/// }
/// ```
pub struct SMSG_GUILD_INFO {
    pub guild_name: String,
    pub created: DateTime,
    pub amount_of_characters_in_guild: u32,
    pub amount_of_accounts_in_guild: u32,
}

impl crate::Message for SMSG_GUILD_INFO {
    const OPCODE: u32 = 0x0088;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // guild_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.guild_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `guild_name` must not be null-terminated.");
        w.write_all(self.guild_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // created: DateTime
        w.write_all(&self.created.as_int().to_le_bytes())?;

        // amount_of_characters_in_guild: u32
        w.write_all(&self.amount_of_characters_in_guild.to_le_bytes())?;

        // amount_of_accounts_in_guild: u32
        w.write_all(&self.amount_of_accounts_in_guild.to_le_bytes())?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(13..=268).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0088, size: body_size as u32 });
        }

        // guild_name: CString
        let guild_name = {
            let guild_name = crate::util::read_c_string_to_vec(r)?;
            String::from_utf8(guild_name)?
        };

        // created: DateTime
        let created: DateTime = crate::util::read_u32_le(r)?.try_into()?;
        // amount_of_characters_in_guild: u32
        let amount_of_characters_in_guild = crate::util::read_u32_le(r)?;

        // amount_of_accounts_in_guild: u32
        let amount_of_accounts_in_guild = crate::util::read_u32_le(r)?;

        Ok(Self {
            guild_name,
            created,
            amount_of_characters_in_guild,
            amount_of_accounts_in_guild,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_GUILD_INFO {}

impl SMSG_GUILD_INFO {
    pub(crate) fn size(&self) -> usize {
        self.guild_name.len() + 1 // guild_name: CString
        + 4 // created: DateTime
        + 4 // amount_of_characters_in_guild: u32
        + 4 // amount_of_accounts_in_guild: u32
    }
}

