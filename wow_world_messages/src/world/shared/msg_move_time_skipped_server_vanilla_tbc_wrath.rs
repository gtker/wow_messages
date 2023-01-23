use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/msg_move_time_skipped.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/msg_move_time_skipped.wowm#L1):
/// ```text
/// smsg MSG_MOVE_TIME_SKIPPED_Server = 0x0319 {
///     PackedGuid player;
///     u32 time_skipped;
/// }
/// ```
pub struct MSG_MOVE_TIME_SKIPPED_Server {
    pub player: Guid,
    pub time_skipped: u32,
}

impl crate::Message for MSG_MOVE_TIME_SKIPPED_Server {
    const OPCODE: u32 = 0x0319;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // player: PackedGuid
        self.player.write_packed_guid_into_vec(w);

        // time_skipped: u32
        w.write_all(&self.time_skipped.to_le_bytes())?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(6..=13).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0319, size: body_size as u32 });
        }

        // player: PackedGuid
        let player = Guid::read_packed(r)?;

        // time_skipped: u32
        let time_skipped = crate::util::read_u32_le(r)?;

        Ok(Self {
            player,
            time_skipped,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for MSG_MOVE_TIME_SKIPPED_Server {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for MSG_MOVE_TIME_SKIPPED_Server {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for MSG_MOVE_TIME_SKIPPED_Server {}

impl MSG_MOVE_TIME_SKIPPED_Server {
    pub(crate) fn size(&self) -> usize {
        self.player.size() // player: Guid
        + 4 // time_skipped: u32
    }
}

