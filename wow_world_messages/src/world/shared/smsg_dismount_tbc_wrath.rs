use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_dismount.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_dismount.wowm#L1):
/// ```text
/// smsg SMSG_DISMOUNT = 0x03AC {
///     PackedGuid player;
/// }
/// ```
pub struct SMSG_DISMOUNT {
    pub player: Guid,
}

impl crate::Message for SMSG_DISMOUNT {
    const OPCODE: u32 = 0x03ac;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // player: PackedGuid
        self.player.write_packed_guid_into_vec(&mut w)?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(2..=9).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03AC, size: body_size as u32 });
        }

        // player: PackedGuid
        let player = Guid::read_packed(&mut r)?;

        Ok(Self {
            player,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_DISMOUNT {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_DISMOUNT {}

impl SMSG_DISMOUNT {
    pub(crate) const fn size(&self) -> usize {
        self.player.size() // player: PackedGuid
    }
}

