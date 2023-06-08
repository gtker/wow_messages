use std::io::{Read, Write};

use crate::Guid;
use crate::tbc::{
    Gender, Race,
};

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
    pub display_ids: [u32; 11],
}

impl crate::private::Sealed for SMSG_MIRRORIMAGE_DATA {}
impl crate::Message for SMSG_MIRRORIMAGE_DATA {
    const OPCODE: u32 = 0x0401;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_MIRRORIMAGE_DATA {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    display_id = {};", self.display_id).unwrap();
        writeln!(s, "    race = {};", self.race.as_test_case_value()).unwrap();
        writeln!(s, "    gender = {};", self.gender.as_test_case_value()).unwrap();
        writeln!(s, "    skin_color = {};", self.skin_color).unwrap();
        writeln!(s, "    face = {};", self.face).unwrap();
        writeln!(s, "    hair_style = {};", self.hair_style).unwrap();
        writeln!(s, "    hair_color = {};", self.hair_color).unwrap();
        writeln!(s, "    facial_hair = {};", self.facial_hair).unwrap();
        writeln!(s, "    guild_id = {};", self.guild_id).unwrap();
        write!(s, "    display_ids = [").unwrap();
        for v in self.display_ids.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 69_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1025_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "display_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "race", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "gender", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "skin_color", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "face", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "hair_style", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "hair_color", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "facial_hair", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "guild_id", "    ");
        writeln!(s, "    /* display_ids: u32[11] start */").unwrap();
        for (i, v) in self.display_ids.iter().enumerate() {
            crate::util::write_bytes(&mut s, &mut bytes, 4, &format!("display_ids {i}"), "    ");
        }
        writeln!(s, "    /* display_ids: u32[11] end */").unwrap();


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        67
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
        if body_size != 67 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0401, size: body_size });
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // display_id: u32
        let display_id = crate::util::read_u32_le(&mut r)?;

        // race: Race
        let race = crate::util::read_u8_le(&mut r)?.try_into()?;

        // gender: Gender
        let gender = crate::util::read_u8_le(&mut r)?.try_into()?;

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
impl crate::tbc::ServerMessage for SMSG_MIRRORIMAGE_DATA {}

