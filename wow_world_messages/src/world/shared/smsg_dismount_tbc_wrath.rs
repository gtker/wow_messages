use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
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

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // player: PackedGuid
        self.player.write_packed_guid_into_vec(w);

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(2..=9).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03AC, size: body_size as u32 });
        }

        // player: PackedGuid
        let player = Guid::read_packed(r)?;

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
    pub(crate) fn size(&self) -> usize {
        self.player.size() // player: Guid
    }
}

