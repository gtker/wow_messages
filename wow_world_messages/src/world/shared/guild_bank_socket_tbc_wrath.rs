use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild_bank/smsg_guild_bank_list.wowm:22`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild_bank/smsg_guild_bank_list.wowm#L22):
/// ```text
/// struct GuildBankSocket {
///     u8 socket_index;
///     u32 gem;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct GuildBankSocket {
    pub socket_index: u8,
    pub gem: u32,
}

impl GuildBankSocket {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // socket_index: u8
        w.write_all(&self.socket_index.to_le_bytes())?;

        // gem: u32
        w.write_all(&self.gem.to_le_bytes())?;

        Ok(())
    }
}

impl GuildBankSocket {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, std::io::Error> {
        // socket_index: u8
        let socket_index = crate::util::read_u8_le(&mut r)?;

        // gem: u32
        let gem = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            socket_index,
            gem,
        })
    }

}

