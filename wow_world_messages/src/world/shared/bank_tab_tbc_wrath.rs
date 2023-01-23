use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/msg_guild_permissions.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/msg_guild_permissions.wowm#L9):
/// ```text
/// struct BankTab {
///     u32 flags;
///     u32 stacks_per_day;
/// }
/// ```
pub struct BankTab {
    pub flags: u32,
    pub stacks_per_day: u32,
}

impl BankTab {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // flags: u32
        w.write_all(&self.flags.to_le_bytes())?;

        // stacks_per_day: u32
        w.write_all(&self.stacks_per_day.to_le_bytes())?;

        Ok(())
    }
}

impl BankTab {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // flags: u32
        let flags = crate::util::read_u32_le(r)?;

        // stacks_per_day: u32
        let stacks_per_day = crate::util::read_u32_le(r)?;

        Ok(Self {
            flags,
            stacks_per_day,
        })
    }

}

