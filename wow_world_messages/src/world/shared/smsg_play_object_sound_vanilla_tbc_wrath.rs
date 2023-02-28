use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// vmangos: Nostalrius: ignored by client if unit is not loaded
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gameobject/smsg_player_object_sound.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gameobject/smsg_player_object_sound.wowm#L3):
/// ```text
/// smsg SMSG_PLAY_OBJECT_SOUND = 0x0278 {
///     u32 sound_id;
///     Guid guid;
/// }
/// ```
pub struct SMSG_PLAY_OBJECT_SOUND {
    pub sound_id: u32,
    pub guid: Guid,
}

impl crate::Message for SMSG_PLAY_OBJECT_SOUND {
    const OPCODE: u32 = 0x0278;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // sound_id: u32
        w.write_all(&self.sound_id.to_le_bytes())?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0278, size: body_size as u32 });
        }

        // sound_id: u32
        let sound_id = crate::util::read_u32_le(r)?;

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            sound_id,
            guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_PLAY_OBJECT_SOUND {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_PLAY_OBJECT_SOUND {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_PLAY_OBJECT_SOUND {}

