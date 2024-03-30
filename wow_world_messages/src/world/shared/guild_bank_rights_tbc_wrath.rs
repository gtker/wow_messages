use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/cmsg_guild_rank.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/cmsg_guild_rank.wowm#L9):
/// ```text
/// struct GuildBankRights {
///     u32 rights;
///     u32 slots_per_day;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct GuildBankRights {
    pub rights: u32,
    pub slots_per_day: u32,
}

impl GuildBankRights {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // rights: u32
        w.write_all(&self.rights.to_le_bytes())?;

        // slots_per_day: u32
        w.write_all(&self.slots_per_day.to_le_bytes())?;

        Ok(())
    }
}

impl GuildBankRights {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, std::io::Error> {
        // rights: u32
        let rights = crate::util::read_u32_le(&mut r)?;

        // slots_per_day: u32
        let slots_per_day = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            rights,
            slots_per_day,
        })
    }

}

