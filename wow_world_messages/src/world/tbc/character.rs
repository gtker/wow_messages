use std::io::{Read, Write};

use crate::Guid;
use crate::tbc::{
    Area, CharacterGear, Class, CreatureFamily, Gender, InventoryType, Map, Race, 
    Vector3d,
};
use wow_world_base::shared::level_vanilla_tbc_wrath::Level;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_char_enum_2_4_3.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_char_enum_2_4_3.wowm#L8):
/// ```text
/// struct Character {
///     Guid guid;
///     CString name;
///     Race race;
///     Class class;
///     Gender gender;
///     u8 skin;
///     u8 face;
///     u8 hair_style;
///     u8 hair_color;
///     u8 facial_hair;
///     Level level;
///     Area area;
///     Map map;
///     Vector3d position;
///     u32 guild_id;
///     u32 flags;
///     Bool first_login;
///     u32 pet_display_id;
///     Level32 pet_level;
///     (u32)CreatureFamily pet_family;
///     CharacterGear[20] equipment;
/// }
/// ```
pub struct Character {
    pub guid: Guid,
    pub name: String,
    pub race: Race,
    pub class: Class,
    pub gender: Gender,
    pub skin: u8,
    pub face: u8,
    pub hair_style: u8,
    pub hair_color: u8,
    pub facial_hair: u8,
    pub level: Level,
    pub area: Area,
    pub map: Map,
    pub position: Vector3d,
    pub guild_id: u32,
    pub flags: u32,
    pub first_login: bool,
    pub pet_display_id: u32,
    pub pet_level: Level,
    pub pet_family: CreatureFamily,
    pub equipment: [CharacterGear; 20],
}

impl Character {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().rev().next(), Some(&0_u8), "String `name` must not be null-terminated.");
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // race: Race
        w.write_all(&(self.race.as_int().to_le_bytes()))?;

        // class: Class
        w.write_all(&(self.class.as_int().to_le_bytes()))?;

        // gender: Gender
        w.write_all(&(self.gender.as_int().to_le_bytes()))?;

        // skin: u8
        w.write_all(&self.skin.to_le_bytes())?;

        // face: u8
        w.write_all(&self.face.to_le_bytes())?;

        // hair_style: u8
        w.write_all(&self.hair_style.to_le_bytes())?;

        // hair_color: u8
        w.write_all(&self.hair_color.to_le_bytes())?;

        // facial_hair: u8
        w.write_all(&self.facial_hair.to_le_bytes())?;

        // level: Level
        w.write_all(&self.level.as_int().to_le_bytes())?;

        // area: Area
        w.write_all(&(self.area.as_int().to_le_bytes()))?;

        // map: Map
        w.write_all(&(self.map.as_int().to_le_bytes()))?;

        // position: Vector3d
        self.position.write_into_vec(&mut w)?;

        // guild_id: u32
        w.write_all(&self.guild_id.to_le_bytes())?;

        // flags: u32
        w.write_all(&self.flags.to_le_bytes())?;

        // first_login: Bool
        w.write_all(u8::from(self.first_login).to_le_bytes().as_slice())?;

        // pet_display_id: u32
        w.write_all(&self.pet_display_id.to_le_bytes())?;

        // pet_level: Level32
        w.write_all(&u32::from(self.pet_level.as_int()).to_le_bytes())?;

        // pet_family: CreatureFamily
        w.write_all(&u32::from(self.pet_family.as_int()).to_le_bytes())?;

        // equipment: CharacterGear[20]
        for i in self.equipment.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
}

impl Character {
    pub(crate) fn read<R: Read>(mut r: R) -> Result<Self, crate::errors::ParseError> {
        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // name: CString
        let name = {
            let name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(name)?
        };

        // race: Race
        let race = crate::util::read_u8_le(&mut r)?.try_into()?;

        // class: Class
        let class = crate::util::read_u8_le(&mut r)?.try_into()?;

        // gender: Gender
        let gender = crate::util::read_u8_le(&mut r)?.try_into()?;

        // skin: u8
        let skin = crate::util::read_u8_le(&mut r)?;

        // face: u8
        let face = crate::util::read_u8_le(&mut r)?;

        // hair_style: u8
        let hair_style = crate::util::read_u8_le(&mut r)?;

        // hair_color: u8
        let hair_color = crate::util::read_u8_le(&mut r)?;

        // facial_hair: u8
        let facial_hair = crate::util::read_u8_le(&mut r)?;

        // level: Level
        let level = Level::new(crate::util::read_u8_le(&mut r)?);

        // area: Area
        let area = crate::util::read_u32_le(&mut r)?.try_into()?;

        // map: Map
        let map = crate::util::read_u32_le(&mut r)?.try_into()?;

        // position: Vector3d
        let position = Vector3d::read(&mut r)?;

        // guild_id: u32
        let guild_id = crate::util::read_u32_le(&mut r)?;

        // flags: u32
        let flags = crate::util::read_u32_le(&mut r)?;

        // first_login: Bool
        let first_login = crate::util::read_u8_le(&mut r)? != 0;

        // pet_display_id: u32
        let pet_display_id = crate::util::read_u32_le(&mut r)?;

        // pet_level: Level32
        let pet_level = Level::new(crate::util::read_u32_le(&mut r)? as u8);

        // pet_family: CreatureFamily
        let pet_family = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        // equipment: CharacterGear[20]
        let equipment = {
            let mut equipment = [CharacterGear::default(); 20];
            for i in equipment.iter_mut() {
                *i = CharacterGear::read(&mut r)?;
            }
            equipment
        };

        Ok(Self {
            guid,
            name,
            race,
            class,
            gender,
            skin,
            face,
            hair_style,
            hair_color,
            facial_hair,
            level,
            area,
            map,
            position,
            guild_id,
            flags,
            first_login,
            pet_display_id,
            pet_level,
            pet_family,
            equipment,
        })
    }

}

impl Character {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + self.name.len() + 1 // name: CString
        + 1 // race: Race
        + 1 // class: Class
        + 1 // gender: Gender
        + 1 // skin: u8
        + 1 // face: u8
        + 1 // hair_style: u8
        + 1 // hair_color: u8
        + 1 // facial_hair: u8
        + 1 // level: Level
        + 4 // area: Area
        + 4 // map: Map
        + 12 // position: Vector3d
        + 4 // guild_id: u32
        + 4 // flags: u32
        + 1 // first_login: Bool
        + 4 // pet_display_id: u32
        + 4 // pet_level: Level32
        + 4 // pet_family: CreatureFamily
        + 20 * 9 // equipment: CharacterGear[20]
    }
}

