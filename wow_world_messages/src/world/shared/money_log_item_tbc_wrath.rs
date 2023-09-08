use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/msg_guild_bank_log_query.wowm:22`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/msg_guild_bank_log_query.wowm#L22):
/// ```text
/// struct MoneyLogItem {
///     u8 action;
///     Guid player;
///     u32 entry;
///     u32 timestamp;
/// }
/// ```
pub struct MoneyLogItem {
    pub action: u8,
    pub player: Guid,
    pub entry: u32,
    pub timestamp: u32,
}

impl MoneyLogItem {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // action: u8
        w.write_all(&self.action.to_le_bytes())?;

        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

        // entry: u32
        w.write_all(&self.entry.to_le_bytes())?;

        // timestamp: u32
        w.write_all(&self.timestamp.to_le_bytes())?;

        Ok(())
    }
}

impl MoneyLogItem {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, std::io::Error> {
        // action: u8
        let action = crate::util::read_u8_le(&mut r)?;

        // player: Guid
        let player = crate::util::read_guid(&mut r)?;

        // entry: u32
        let entry = crate::util::read_u32_le(&mut r)?;

        // timestamp: u32
        let timestamp = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            action,
            player,
            entry,
            timestamp,
        })
    }

}

