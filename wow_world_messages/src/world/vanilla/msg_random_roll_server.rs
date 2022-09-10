use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/msg_random_roll_server.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/msg_random_roll_server.wowm#L3):
/// ```text
/// smsg MSG_RANDOM_ROLL_Server = 0x01FB {
///     u32 minimum;
///     u32 maximum;
///     u32 actual_roll;
///     Guid guid;
/// }
/// ```
pub struct MSG_RANDOM_ROLL_Server {
    pub minimum: u32,
    pub maximum: u32,
    pub actual_roll: u32,
    pub guid: Guid,
}

impl crate::Message for MSG_RANDOM_ROLL_Server {
    const OPCODE: u32 = 0x01fb;

    fn size_without_header(&self) -> u32 {
        20
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // minimum: u32
        w.write_all(&self.minimum.to_le_bytes())?;

        // maximum: u32
        w.write_all(&self.maximum.to_le_bytes())?;

        // actual_roll: u32
        w.write_all(&self.actual_roll.to_le_bytes())?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 20 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // minimum: u32
        let minimum = crate::util::read_u32_le(r)?;

        // maximum: u32
        let maximum = crate::util::read_u32_le(r)?;

        // actual_roll: u32
        let actual_roll = crate::util::read_u32_le(r)?;

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            minimum,
            maximum,
            actual_roll,
            guid,
        })
    }

}
#[cfg(any(feature = "vanilla", feature = "tbc"))]
impl crate::helper::shared::vanilla_tbc::ServerMessage for MSG_RANDOM_ROLL_Server {}

