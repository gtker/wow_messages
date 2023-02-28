use crate::wrath::MoneyLogItem;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/msg_guild_bank_log_query.wowm:31`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/msg_guild_bank_log_query.wowm#L31):
/// ```text
/// smsg MSG_GUILD_BANK_LOG_QUERY_Server = 0x03EE {
///     u32 unix_time;
///     u8 slot;
///     u8 amount_of_money_logs;
///     MoneyLogItem[amount_of_money_logs] money_logs;
/// }
/// ```
pub struct MSG_GUILD_BANK_LOG_QUERY_Server {
    pub unix_time: u32,
    pub slot: u8,
    pub money_logs: Vec<MoneyLogItem>,
}

impl crate::Message for MSG_GUILD_BANK_LOG_QUERY_Server {
    const OPCODE: u32 = 0x03ee;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // unix_time: u32
        w.write_all(&self.unix_time.to_le_bytes())?;

        // slot: u8
        w.write_all(&self.slot.to_le_bytes())?;

        // amount_of_money_logs: u8
        w.write_all(&(self.money_logs.len() as u8).to_le_bytes())?;

        // money_logs: MoneyLogItem[amount_of_money_logs]
        for i in self.money_logs.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(6..=4358).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03EE, size: body_size as u32 });
        }

        // unix_time: u32
        let unix_time = crate::util::read_u32_le(&mut r)?;

        // slot: u8
        let slot = crate::util::read_u8_le(&mut r)?;

        // amount_of_money_logs: u8
        let amount_of_money_logs = crate::util::read_u8_le(&mut r)?;

        // money_logs: MoneyLogItem[amount_of_money_logs]
        let money_logs = {
            let mut money_logs = Vec::with_capacity(amount_of_money_logs as usize);
            for i in 0..amount_of_money_logs {
                money_logs.push(MoneyLogItem::read(&mut r)?);
            }
            money_logs
        };

        Ok(Self {
            unix_time,
            slot,
            money_logs,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_GUILD_BANK_LOG_QUERY_Server {}

impl MSG_GUILD_BANK_LOG_QUERY_Server {
    pub(crate) fn size(&self) -> usize {
        4 // unix_time: u32
        + 1 // slot: u8
        + 1 // amount_of_money_logs: u8
        + self.money_logs.len() * 17 // money_logs: MoneyLogItem[amount_of_money_logs]
    }
}

