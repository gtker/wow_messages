use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_login_settimespeed.wowm:32`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_login_settimespeed.wowm#L32):
/// ```text
/// smsg SMSG_LOGIN_SETTIMESPEED = 0x0042 {
///     u32 datetime;
///     f32 timescale;
///     u32 unknown1;
/// }
/// ```
pub struct SMSG_LOGIN_SETTIMESPEED {
    /// Current server datetime.
    ///
    /// This is not just unix time but instead seems to be a custom bitfield.
    /// vmangos/cmangos/mangoszero uses the format `years_after_2000 << 24 | month << 20 | month_day << 14 | week_day << 11 | hours << 6 | minutes`. All values start at 0 and `week_day` starts on Sunday.
    /// Running the client with `-console` verifies that this message in this format sets the correct datetime. `SMSG_QUERY_TIME_RESPONSE` will not set this.
    ///
    pub datetime: u32,
    /// How many minutes should pass by every second.
    ///
    /// vmangos/cmangos/mangoszero set this to 0.01666667. This means that 1/60 minutes pass every second (one second passes every second). Setting this to 1.0 will make the client advance one minute every second.
    ///
    pub timescale: f32,
    /// arcemu/azerothcore/mangostwo: Set to 0.
    ///
    pub unknown1: u32,
}

impl crate::Message for SMSG_LOGIN_SETTIMESPEED {
    const OPCODE: u32 = 0x0042;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // datetime: u32
        w.write_all(&self.datetime.to_le_bytes())?;

        // timescale: f32
        w.write_all(&self.timescale.to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // datetime: u32
        let datetime = crate::util::read_u32_le(r)?;

        // timescale: f32
        let timescale = crate::util::read_f32_le(r)?;
        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        Ok(Self {
            datetime,
            timescale,
            unknown1,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_LOGIN_SETTIMESPEED {}

