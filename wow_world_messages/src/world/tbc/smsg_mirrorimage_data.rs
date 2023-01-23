use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::tbc::Gender;
use crate::world::tbc::Race;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_mirrorimage_data.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_mirrorimage_data.wowm#L1):
/// ```text
/// smsg SMSG_MIRRORIMAGE_DATA = 0x0401 {
///     Guid guid;
///     u32 display_id;
///     Race race;
///     Gender gender;
///     u8 skin_color;
///     u8 face;
///     u8 hair_style;
///     u8 hair_color;
///     u8 facial_hair;
///     u32 guild_id;
///     u32[11] display_ids;
/// }
/// ```
pub struct SMSG_MIRRORIMAGE_DATA {
    pub guid: Guid,
    pub display_id: u32,
    pub race: Race,
    pub gender: Gender,
    pub skin_color: u8,
    pub face: u8,
    pub hair_style: u8,
    pub hair_color: u8,
    pub facial_hair: u8,
    pub guild_id: u32,
    /// This array contains the: HEAD, SHOULDERS, BODY, CHEST, WAIST, LEGS, FEET, WRISTS, HANDS, BACK, and TABARD.
    ///
    pub display_ids: [u32; 11],
}

impl crate::Message for SMSG_MIRRORIMAGE_DATA {
    const OPCODE: u32 = 0x0401;

    fn size_without_header(&self) -> u32 {
        67
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // display_id: u32
        w.write_all(&self.display_id.to_le_bytes())?;

        // race: Race
        w.write_all(&(self.race.as_int() as u8).to_le_bytes())?;

        // gender: Gender
        w.write_all(&(self.gender.as_int() as u8).to_le_bytes())?;

        // skin_color: u8
        w.write_all(&self.skin_color.to_le_bytes())?;

        // face: u8
        w.write_all(&self.face.to_le_bytes())?;

        // hair_style: u8
        w.write_all(&self.hair_style.to_le_bytes())?;

        // hair_color: u8
        w.write_all(&self.hair_color.to_le_bytes())?;

        // facial_hair: u8
        w.write_all(&self.facial_hair.to_le_bytes())?;

        // guild_id: u32
        w.write_all(&self.guild_id.to_le_bytes())?;

        // display_ids: u32[11]
        for i in self.display_ids.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 67 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0401, size: body_size as u32 });
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        // display_id: u32
        let display_id = crate::util::read_u32_le(r)?;

        // race: Race
        let race: Race = crate::util::read_u8_le(r)?.try_into()?;

        // gender: Gender
        let gender: Gender = crate::util::read_u8_le(r)?.try_into()?;

        // skin_color: u8
        let skin_color = crate::util::read_u8_le(r)?;

        // face: u8
        let face = crate::util::read_u8_le(r)?;

        // hair_style: u8
        let hair_style = crate::util::read_u8_le(r)?;

        // hair_color: u8
        let hair_color = crate::util::read_u8_le(r)?;

        // facial_hair: u8
        let facial_hair = crate::util::read_u8_le(r)?;

        // guild_id: u32
        let guild_id = crate::util::read_u32_le(r)?;

        // display_ids: u32[11]
        let mut display_ids = [u32::default(); 11];
        for i in display_ids.iter_mut() {
            *i = crate::util::read_u32_le(r)?;
        }

        Ok(Self {
            guid,
            display_id,
            race,
            gender,
            skin_color,
            face,
            hair_style,
            hair_color,
            facial_hair,
            guild_id,
            display_ids,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_MIRRORIMAGE_DATA {}

