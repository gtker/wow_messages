use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Reply to [`CMSG_QUERY_TIME`](crate::vanilla::CMSG_QUERY_TIME).
/// [`CMSG_QUERY_TIME`](crate::vanilla::CMSG_QUERY_TIME) and this reply does not actually appear to set the time. Instead [`SMSG_LOGIN_SETTIMESPEED`](crate::vanilla::SMSG_LOGIN_SETTIMESPEED) seems to correctly set the time. Running the client with `-console` will print the date when [`SMSG_LOGIN_SETTIMESPEED`](crate::vanilla::SMSG_LOGIN_SETTIMESPEED) is received, but not when this message is received.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_query_time_response.wowm:21`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_query_time_response.wowm#L21):
/// ```text
/// smsg SMSG_QUERY_TIME_RESPONSE = 0x01CF {
///     u32 time;
///     u32 time_until_daily_quest_reset;
/// }
/// ```
pub struct SMSG_QUERY_TIME_RESPONSE {
    /// Seconds since 1970, 1st of January (Unix Time).
    ///
    pub time: u32,
    /// Units need confirmation, but it's likely in seconds, since many other time related things are also seconds.
    ///
    pub time_until_daily_quest_reset: u32,
}

impl crate::private::Sealed for SMSG_QUERY_TIME_RESPONSE {}
impl crate::Message for SMSG_QUERY_TIME_RESPONSE {
    const OPCODE: u32 = 0x01cf;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // time: u32
        w.write_all(&self.time.to_le_bytes())?;

        // time_until_daily_quest_reset: u32
        w.write_all(&self.time_until_daily_quest_reset.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01CF, size: body_size });
        }

        // time: u32
        let time = crate::util::read_u32_le(&mut r)?;

        // time_until_daily_quest_reset: u32
        let time_until_daily_quest_reset = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            time,
            time_until_daily_quest_reset,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_QUERY_TIME_RESPONSE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_QUERY_TIME_RESPONSE {}

