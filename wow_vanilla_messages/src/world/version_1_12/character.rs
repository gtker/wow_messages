use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::version_1_12::Area;
use crate::world::version_1_12::CharacterFlags;
use crate::world::version_1_12::CharacterGear;
use crate::world::version_1_12::Class;
use crate::world::version_1_12::Gender;
use crate::world::version_1_12::map::{Map, map_try_from, map_as_int};
use crate::world::version_1_12::Race;
use crate::world::version_1_12::Vector3d;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_char_enum.wowm:17`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_char_enum.wowm#L17):
/// ```text
/// struct Character {
///     Guid guid;
///     CString name;
///     Race race;
///     Class class;
///     Gender gender;
///     u8 skin;
///     u8 face;
///     u8 hairstyle;
///     u8 haircolor;
///     u8 facialhair;
///     u8 level;
///     Area area;
///     Map map;
///     Vector3d position;
///     u32 guild_id;
///     CharacterFlags flags;
///     u8 first_login;
///     u32 pet_display_id;
///     u32 pet_level;
///     u32 pet_familiy;
///     CharacterGear[19] equipment;
///     u32 first_bag_display_id = 0;
///     u8 first_bag_inventory_id = 0;
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
    pub hairstyle: u8,
    pub haircolor: u8,
    pub facialhair: u8,
    pub level: u8,
    pub area: Area,
    pub map: Map,
    pub position: Vector3d,
    pub guild_id: u32,
    pub flags: CharacterFlags,
    pub first_login: u8,
    pub pet_display_id: u32,
    pub pet_level: u32,
    pub pet_familiy: u32,
    pub equipment: [CharacterGear; 19],
}

impl Character {
    /// The field `first_bag_display_id` is constantly specified to be:
    ///
    /// | Format | Value |
    /// | ------ | ----- |
    /// | Decimal | `0` |
    /// | Hex | `0x00` |
    /// | Original | `0` |
    ///
    /// **This field is not in the Rust struct, but is written as this constant value.**
    pub const FIRST_BAG_DISPLAY_ID_VALUE: u32 = 0x00;

    /// The field `first_bag_inventory_id` is constantly specified to be:
    ///
    /// | Format | Value |
    /// | ------ | ----- |
    /// | Decimal | `0` |
    /// | Hex | `0x00` |
    /// | Original | `0` |
    ///
    /// **This field is not in the Rust struct, but is written as this constant value.**
    pub const FIRST_BAG_INVENTORY_ID_VALUE: u8 = 0x00;

}

impl Character {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // race: Race
        w.write_all(&(self.race.as_int() as u8).to_le_bytes())?;

        // class: Class
        w.write_all(&(self.class.as_int() as u8).to_le_bytes())?;

        // gender: Gender
        w.write_all(&(self.gender.as_int() as u8).to_le_bytes())?;

        // skin: u8
        w.write_all(&self.skin.to_le_bytes())?;

        // face: u8
        w.write_all(&self.face.to_le_bytes())?;

        // hairstyle: u8
        w.write_all(&self.hairstyle.to_le_bytes())?;

        // haircolor: u8
        w.write_all(&self.haircolor.to_le_bytes())?;

        // facialhair: u8
        w.write_all(&self.facialhair.to_le_bytes())?;

        // level: u8
        w.write_all(&self.level.to_le_bytes())?;

        // area: Area
        w.write_all(&(self.area.as_int() as u32).to_le_bytes())?;

        // map: Map
        w.write_all(&(map_as_int(&self.map) as u32).to_le_bytes())?;

        // position: Vector3d
        self.position.write_into_vec(w)?;

        // guild_id: u32
        w.write_all(&self.guild_id.to_le_bytes())?;

        // flags: CharacterFlags
        w.write_all(&(self.flags.as_int() as u32).to_le_bytes())?;

        // first_login: u8
        w.write_all(&self.first_login.to_le_bytes())?;

        // pet_display_id: u32
        w.write_all(&self.pet_display_id.to_le_bytes())?;

        // pet_level: u32
        w.write_all(&self.pet_level.to_le_bytes())?;

        // pet_familiy: u32
        w.write_all(&self.pet_familiy.to_le_bytes())?;

        // equipment: CharacterGear[19]
        for i in self.equipment.iter() {
            i.write_into_vec(w)?;
        }

        // first_bag_display_id: u32
        w.write_all(&Self::FIRST_BAG_DISPLAY_ID_VALUE.to_le_bytes())?;

        // first_bag_inventory_id: u8
        w.write_all(&Self::FIRST_BAG_INVENTORY_ID_VALUE.to_le_bytes())?;

        Ok(())
    }
}

impl Character {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        // race: Race
        let race: Race = crate::util::read_u8_le(r)?.try_into()?;

        // class: Class
        let class: Class = crate::util::read_u8_le(r)?.try_into()?;

        // gender: Gender
        let gender: Gender = crate::util::read_u8_le(r)?.try_into()?;

        // skin: u8
        let skin = crate::util::read_u8_le(r)?;

        // face: u8
        let face = crate::util::read_u8_le(r)?;

        // hairstyle: u8
        let hairstyle = crate::util::read_u8_le(r)?;

        // haircolor: u8
        let haircolor = crate::util::read_u8_le(r)?;

        // facialhair: u8
        let facialhair = crate::util::read_u8_le(r)?;

        // level: u8
        let level = crate::util::read_u8_le(r)?;

        // area: Area
        let area: Area = crate::util::read_u32_le(r)?.try_into()?;

        // map: Map
        let map: Map = map_try_from(crate::util::read_u32_le(r)?)?;

        // position: Vector3d
        let position = Vector3d::read(r)?;

        // guild_id: u32
        let guild_id = crate::util::read_u32_le(r)?;

        // flags: CharacterFlags
        let flags = CharacterFlags::new(crate::util::read_u32_le(r)?);

        // first_login: u8
        let first_login = crate::util::read_u8_le(r)?;

        // pet_display_id: u32
        let pet_display_id = crate::util::read_u32_le(r)?;

        // pet_level: u32
        let pet_level = crate::util::read_u32_le(r)?;

        // pet_familiy: u32
        let pet_familiy = crate::util::read_u32_le(r)?;

        // equipment: CharacterGear[19]
        let mut equipment = [CharacterGear::default(); 19];
        for i in equipment.iter_mut() {
            *i = CharacterGear::read(r)?;
        }

        // first_bag_display_id: u32
        let _first_bag_display_id = crate::util::read_u32_le(r)?;
        // first_bag_display_id is expected to always be 0 (0)

        // first_bag_inventory_id: u8
        let _first_bag_inventory_id = crate::util::read_u8_le(r)?;
        // first_bag_inventory_id is expected to always be 0 (0)

        Ok(Self {
            guid,
            name,
            race,
            class,
            gender,
            skin,
            face,
            hairstyle,
            haircolor,
            facialhair,
            level,
            area,
            map,
            position,
            guild_id,
            flags,
            first_login,
            pet_display_id,
            pet_level,
            pet_familiy,
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
        + 1 // hairstyle: u8
        + 1 // haircolor: u8
        + 1 // facialhair: u8
        + 1 // level: u8
        + 4 // area: Area
        + 4 // map: Map
        + 12 // position: Vector3d
        + 4 // guild_id: u32
        + 4 // flags: CharacterFlags
        + 1 // first_login: u8
        + 4 // pet_display_id: u32
        + 4 // pet_level: u32
        + 4 // pet_familiy: u32
        + 19 * 5 // equipment: CharacterGear[19]
        + 4 // first_bag_display_id: u32
        + 1 // first_bag_inventory_id: u8
    }
}

