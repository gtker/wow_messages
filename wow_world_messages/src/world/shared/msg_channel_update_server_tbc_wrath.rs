use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/msg_channel_update.wowm:7`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/msg_channel_update.wowm#L7):
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

impl crate::Message for MSG_CHANNEL_UPDATE_Server {
    const OPCODE: u32 = 0x013a;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // caster: PackedGuid
        self.caster.write_packed_guid_into_vec(w);

        // time: u32
        w.write_all(&self.time.to_le_bytes())?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(6..=13).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x013A, size: body_size as u32 });
        }

        // caster: PackedGuid
        let caster = Guid::read_packed(r)?;

        // time: u32
        let time = crate::util::read_u32_le(r)?;

        Ok(Self {
            caster,
            time,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for MSG_CHANNEL_UPDATE_Server {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for MSG_CHANNEL_UPDATE_Server {}

impl MSG_CHANNEL_UPDATE_Server {
    pub(crate) fn size(&self) -> usize {
        self.caster.size() // caster: Guid
        + 4 // time: u32
    }
}

