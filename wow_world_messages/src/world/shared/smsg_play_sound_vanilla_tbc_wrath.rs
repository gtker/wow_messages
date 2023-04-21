use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/world/smsg_play_sound.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/world/smsg_play_sound.wowm#L3):
/// ```text
/// smsg SMSG_PLAY_SOUND = 0x02D2 {
///     u32 sound_id;
/// }
/// ```
pub struct SMSG_PLAY_SOUND {
    pub sound_id: u32,
}

impl crate::private::Sealed for SMSG_PLAY_SOUND {}
impl crate::Message for SMSG_PLAY_SOUND {
    const OPCODE: u32 = 0x02d2;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // sound_id: u32
        w.write_all(&self.sound_id.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02D2, size: body_size as u32 });
        }

        // sound_id: u32
        let sound_id = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            sound_id,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_PLAY_SOUND {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_PLAY_SOUND {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_PLAY_SOUND {}

