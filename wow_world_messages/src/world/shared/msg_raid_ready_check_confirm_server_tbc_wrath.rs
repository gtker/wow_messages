use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/msg_raid_ready_check_confirm.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/msg_raid_ready_check_confirm.wowm#L9):
/// ```text
/// smsg MSG_RAID_READY_CHECK_CONFIRM_Server = 0x03AE {
///     Guid player;
///     u8 state;
/// }
/// ```
pub struct MSG_RAID_READY_CHECK_CONFIRM_Server {
    pub player: Guid,
    pub state: u8,
}

impl crate::private::Sealed for MSG_RAID_READY_CHECK_CONFIRM_Server {}
impl crate::Message for MSG_RAID_READY_CHECK_CONFIRM_Server {
    const OPCODE: u32 = 0x03ae;

    fn size_without_header(&self) -> u32 {
        9
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

        // state: u8
        w.write_all(&self.state.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 9 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03AE, size: body_size });
        }

        // player: Guid
        let player = crate::util::read_guid(&mut r)?;

        // state: u8
        let state = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            player,
            state,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for MSG_RAID_READY_CHECK_CONFIRM_Server {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_RAID_READY_CHECK_CONFIRM_Server {}

