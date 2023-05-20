use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/msg_guild_bank_log_query.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/msg_guild_bank_log_query.wowm#L1):
/// ```text
/// cmsg MSG_GUILD_BANK_LOG_QUERY_Client = 0x03ED {
///     u8 slot;
/// }
/// ```
pub struct MSG_GUILD_BANK_LOG_QUERY_Client {
    pub slot: u8,
}

impl crate::private::Sealed for MSG_GUILD_BANK_LOG_QUERY_Client {}
impl crate::Message for MSG_GUILD_BANK_LOG_QUERY_Client {
    const OPCODE: u32 = 0x03ed;

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // slot: u8
        w.write_all(&self.slot.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03ED, size: body_size });
        }

        // slot: u8
        let slot = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            slot,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for MSG_GUILD_BANK_LOG_QUERY_Client {}

