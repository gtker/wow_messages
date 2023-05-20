use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/msg_channel_update.wowm:7`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/msg_channel_update.wowm#L7):
/// ```text
/// smsg MSG_CHANNEL_UPDATE_Server = 0x013A {
///     PackedGuid caster;
///     u32 time;
/// }
/// ```
pub struct MSG_CHANNEL_UPDATE_Server {
    pub caster: Guid,
    pub time: u32,
}

impl crate::private::Sealed for MSG_CHANNEL_UPDATE_Server {}
impl crate::Message for MSG_CHANNEL_UPDATE_Server {
    const OPCODE: u32 = 0x013a;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // caster: PackedGuid
        self.caster.write_packed_guid_into_vec(&mut w)?;

        // time: u32
        w.write_all(&self.time.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(6..=13).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x013A, size: body_size });
        }

        // caster: PackedGuid
        let caster = Guid::read_packed(&mut r)?;

        // time: u32
        let time = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            caster,
            time,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for MSG_CHANNEL_UPDATE_Server {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_CHANNEL_UPDATE_Server {}

impl MSG_CHANNEL_UPDATE_Server {
    pub(crate) const fn size(&self) -> usize {
        self.caster.size() // caster: PackedGuid
        + 4 // time: u32
    }
}

