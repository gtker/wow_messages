use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::{
    Class, Gender, Race,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_mirrorimage_data.wowm:18`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_mirrorimage_data.wowm#L18):
/// ```text
/// smsg SMSG_MIRRORIMAGE_DATA = 0x0402 {
///     Guid guid;
///     u32 display_id;
///     Race race;
///     Gender gender;
///     Class class;
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
    pub class: Class,
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

impl crate::private::Sealed for SMSG_MIRRORIMAGE_DATA {}
impl crate::Message for SMSG_MIRRORIMAGE_DATA {
    const OPCODE: u32 = 0x0402;

    fn size_without_header(&self) -> u32 {
        68
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // display_id: u32
        w.write_all(&self.display_id.to_le_bytes())?;

        // race: Race
        w.write_all(&(self.race.as_int().to_le_bytes()))?;

        // gender: Gender
        w.write_all(&(self.gender.as_int().to_le_bytes()))?;

        // class: Class
        w.write_all(&(self.class.as_int().to_le_bytes()))?;

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
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 68 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0402, size: body_size });
        }

        // guid: Guid
        let guid = Guid::read(&mut r)?;

        // display_id: u32
        let display_id = crate::util::read_u32_le(&mut r)?;

        // race: Race
        let race: Race = crate::util::read_u8_le(&mut r)?.try_into()?;

        // gender: Gender
        let gender: Gender = crate::util::read_u8_le(&mut r)?.try_into()?;

        // class: Class
        let class: Class = crate::util::read_u8_le(&mut r)?.try_into()?;

        // skin_color: u8
        let skin_color = crate::util::read_u8_le(&mut r)?;

        // face: u8
        let face = crate::util::read_u8_le(&mut r)?;

        // hair_style: u8
        let hair_style = crate::util::read_u8_le(&mut r)?;

        // hair_color: u8
        let hair_color = crate::util::read_u8_le(&mut r)?;

        // facial_hair: u8
        let facial_hair = crate::util::read_u8_le(&mut r)?;

        // guild_id: u32
        let guild_id = crate::util::read_u32_le(&mut r)?;

        // display_ids: u32[11]
        let display_ids = {
            let mut display_ids = [u32::default(); 11];
            for i in display_ids.iter_mut() {
                *i = crate::util::read_u32_le(&mut r)?;
            }
            display_ids
        };

        Ok(Self {
            guid,
            display_id,
            race,
            gender,
            class,
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
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_MIRRORIMAGE_DATA {}

