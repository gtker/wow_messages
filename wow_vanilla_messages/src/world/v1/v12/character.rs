use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::v1::v12::{Area, AreaError};
use crate::world::v1::v12::{CharacterFlags};
use crate::world::v1::v12::{CharacterGear, CharacterGearError};
use crate::world::v1::v12::{Class, ClassError};
use crate::world::v1::v12::{Gender, GenderError};
use crate::world::v1::v12::{Map, MapError};
use crate::world::v1::v12::{Race, RaceError};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
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
    pub position_x: f32,
    pub position_y: f32,
    pub position_z: f32,
    pub guild_id: u32,
    pub flags: CharacterFlags,
    pub first_login: u8,
    pub pet_display_id: u32,
    pub pet_level: u32,
    pub pet_familiy: u32,
    pub equipment: [CharacterGear; 19],
}

impl Character {
    pub const FIRST_BAG_DISPLAY_ID_VALUE: u32 = 0x00;

    pub const FIRST_BAG_INVENTORY_ID_VALUE: u8 = 0x00;

}

impl ReadableAndWritable for Character {
    type Error = CharacterError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        // race: Race
        let race = Race::read(r)?;

        // class: Class
        let class = Class::read(r)?;

        // gender: Gender
        let gender = Gender::read(r)?;

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
        let area = Area::read(r)?;

        // map: Map
        let map = Map::read(r)?;

        // position_x: f32
        let position_x = crate::util::read_f32_le(r)?;
        // position_y: f32
        let position_y = crate::util::read_f32_le(r)?;
        // position_z: f32
        let position_z = crate::util::read_f32_le(r)?;
        // guild_id: u32
        let guild_id = crate::util::read_u32_le(r)?;

        // flags: CharacterFlags
        let flags = CharacterFlags::read(r)?;

        // first_login: u8
        let first_login = crate::util::read_u8_le(r)?;

        // pet_display_id: u32
        let pet_display_id = crate::util::read_u32_le(r)?;

        // pet_level: u32
        let pet_level = crate::util::read_u32_le(r)?;

        // pet_familiy: u32
        let pet_familiy = crate::util::read_u32_le(r)?;

        // equipment: CharacterGear[19]
        let mut equipment = Vec::with_capacity(19 as usize);
        for i in 0..19 {
            equipment.push(CharacterGear::read(r)?);
        }
        let equipment = equipment.try_into().unwrap();

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
            position_x,
            position_y,
            position_z,
            guild_id,
            flags,
            first_login,
            pet_display_id,
            pet_level,
            pet_familiy,
            equipment,
        })
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: Guid
        self.guid.write(w)?;

        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // race: Race
        self.race.write(w)?;

        // class: Class
        self.class.write(w)?;

        // gender: Gender
        self.gender.write(w)?;

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
        self.area.write(w)?;

        // map: Map
        self.map.write(w)?;

        // position_x: f32
        w.write_all(&self.position_x.to_le_bytes())?;

        // position_y: f32
        w.write_all(&self.position_y.to_le_bytes())?;

        // position_z: f32
        w.write_all(&self.position_z.to_le_bytes())?;

        // guild_id: u32
        w.write_all(&self.guild_id.to_le_bytes())?;

        // flags: CharacterFlags
        self.flags.write(w)?;

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
            i.write(w)?;
        }

        // first_bag_display_id: u32
        w.write_all(&Self::FIRST_BAG_DISPLAY_ID_VALUE.to_le_bytes())?;

        // first_bag_inventory_id: u8
        w.write_all(&Self::FIRST_BAG_INVENTORY_ID_VALUE.to_le_bytes())?;

        Ok(())
    }

}

impl VariableSized for Character {
    fn size(&self) -> usize {
        8 // guid: Guid
        + self.name.len() + 1 // name: CString and Null Terminator
        + Race::size() // race: Race
        + Class::size() // class: Class
        + Gender::size() // gender: Gender
        + 1 // skin: u8
        + 1 // face: u8
        + 1 // hairstyle: u8
        + 1 // haircolor: u8
        + 1 // facialhair: u8
        + 1 // level: u8
        + Area::size() // area: Area
        + Map::size() // map: Map
        + 4 // position_x: f32
        + 4 // position_y: f32
        + 4 // position_z: f32
        + 4 // guild_id: u32
        + CharacterFlags::size() // flags: CharacterFlags
        + 1 // first_login: u8
        + 4 // pet_display_id: u32
        + 4 // pet_level: u32
        + 4 // pet_familiy: u32
        + 19 * CharacterGear::size() // equipment: CharacterGear[19]
        + 4 // first_bag_display_id: u32
        + 1 // first_bag_inventory_id: u8
    }
}

impl MaximumPossibleSized for Character {
    fn maximum_possible_size() -> usize {
        8 // guid: Guid
        + 256 // name: CString
        + Race::maximum_possible_size() // race: Race
        + Class::maximum_possible_size() // class: Class
        + Gender::maximum_possible_size() // gender: Gender
        + 1 // skin: u8
        + 1 // face: u8
        + 1 // hairstyle: u8
        + 1 // haircolor: u8
        + 1 // facialhair: u8
        + 1 // level: u8
        + Area::maximum_possible_size() // area: Area
        + Map::maximum_possible_size() // map: Map
        + 4 // position_x: f32
        + 4 // position_y: f32
        + 4 // position_z: f32
        + 4 // guild_id: u32
        + CharacterFlags::maximum_possible_size() // flags: CharacterFlags
        + 1 // first_login: u8
        + 4 // pet_display_id: u32
        + 4 // pet_level: u32
        + 4 // pet_familiy: u32
        + 19 * CharacterGear::maximum_possible_size() // equipment: CharacterGear[19]
        + 4 // first_bag_display_id: u32
        + 1 // first_bag_inventory_id: u8
    }
}

#[derive(Debug)]
pub enum CharacterError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
    Area(AreaError),
    CharacterGear(CharacterGearError),
    Class(ClassError),
    Gender(GenderError),
    Map(MapError),
    Race(RaceError),
}

impl std::error::Error for CharacterError {}
impl std::fmt::Display for CharacterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::Area(i) => i.fmt(f),
            Self::CharacterGear(i) => i.fmt(f),
            Self::Class(i) => i.fmt(f),
            Self::Gender(i) => i.fmt(f),
            Self::Map(i) => i.fmt(f),
            Self::Race(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CharacterError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CharacterError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<AreaError> for CharacterError {
    fn from(e: AreaError) -> Self {
        Self::Area(e)
    }
}

impl From<CharacterGearError> for CharacterError {
    fn from(e: CharacterGearError) -> Self {
        Self::CharacterGear(e)
    }
}

impl From<ClassError> for CharacterError {
    fn from(e: ClassError) -> Self {
        Self::Class(e)
    }
}

impl From<GenderError> for CharacterError {
    fn from(e: GenderError) -> Self {
        Self::Gender(e)
    }
}

impl From<MapError> for CharacterError {
    fn from(e: MapError) -> Self {
        Self::Map(e)
    }
}

impl From<RaceError> for CharacterError {
    fn from(e: RaceError) -> Self {
        Self::Race(e)
    }
}

