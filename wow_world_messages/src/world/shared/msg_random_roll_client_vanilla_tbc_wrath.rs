use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/msg_random_roll_client.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/msg_random_roll_client.wowm#L3):
/// ```text
/// cmsg MSG_RANDOM_ROLL_Client = 0x01FB {
///     u32 minimum;
///     u32 maximum;
/// }
/// ```
pub struct MSG_RANDOM_ROLL_Client {
    pub minimum: u32,
    pub maximum: u32,
}

impl crate::Message for MSG_RANDOM_ROLL_Client {
    const OPCODE: u32 = 0x01fb;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // minimum: u32
        w.write_all(&self.minimum.to_le_bytes())?;

        // maximum: u32
        w.write_all(&self.maximum.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01FB, size: body_size as u32 });
        }

        // minimum: u32
        let minimum = crate::util::read_u32_le(r)?;

        // maximum: u32
        let maximum = crate::util::read_u32_le(r)?;

        Ok(Self {
            minimum,
            maximum,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for MSG_RANDOM_ROLL_Client {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for MSG_RANDOM_ROLL_Client {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for MSG_RANDOM_ROLL_Client {}

