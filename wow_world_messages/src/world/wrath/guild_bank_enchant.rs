use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_guild_bank_list.wowm:22`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_guild_bank_list.wowm#L22):
/// ```text
/// struct GuildBankEnchant {
///     u8 enchant_index;
///     u32 enchant;
/// }
/// ```
pub struct GuildBankEnchant {
    pub enchant_index: u8,
    pub enchant: u32,
}

impl GuildBankEnchant {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // enchant_index: u8
        w.write_all(&self.enchant_index.to_le_bytes())?;

        // enchant: u32
        w.write_all(&self.enchant.to_le_bytes())?;

        Ok(())
    }
}

impl GuildBankEnchant {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // enchant_index: u8
        let enchant_index = crate::util::read_u8_le(r)?;

        // enchant: u32
        let enchant = crate::util::read_u32_le(r)?;

        Ok(Self {
            enchant_index,
            enchant,
        })
    }

}

