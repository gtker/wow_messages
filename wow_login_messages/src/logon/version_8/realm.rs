use std::convert::{TryFrom, TryInto};
use crate::logon::version_2::Population;
use crate::logon::version_2::{RealmCategory, RealmCategoryError};
use crate::logon::version_8::{RealmFlag};
use crate::logon::all::Version;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_realm/server.wowm:138`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_realm/server.wowm#L138):
/// ```text
/// struct Realm {
///     u8 realm_type;
///     u8 locked;
///     RealmFlag flag;
///     CString name;
///     CString address;
///     Population population;
///     u8 number_of_characters_on_realm;
///     RealmCategory category;
///     u8 realm_id;
///     if (flag & SPECIFY_BUILD) {
///         Version version;
///     }
/// }
/// ```
pub struct Realm {
    pub realm_type: u8,
    pub locked: u8,
    pub flag: RealmRealmFlag,
    pub name: String,
    pub address: String,
    pub population: Population,
    pub number_of_characters_on_realm: u8,
    pub category: RealmCategory,
    pub realm_id: u8,
}

impl ReadableAndWritable for Realm {
    type Error = RealmError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // realm_type: u8
        let realm_type = crate::util::read_u8_le(r)?;

        // locked: u8
        let locked = crate::util::read_u8_le(r)?;

        // flag: RealmFlag
        let flag = RealmFlag::read(r)?;

        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        // address: CString
        let address = crate::util::read_c_string_to_vec(r)?;
        let address = String::from_utf8(address)?;

        // population: Population
        let population = Population::read(r)?;

        // number_of_characters_on_realm: u8
        let number_of_characters_on_realm = crate::util::read_u8_le(r)?;

        // category: RealmCategory
        let category = RealmCategory::read(r)?;

        // realm_id: u8
        let realm_id = crate::util::read_u8_le(r)?;

        let flag_SPECIFY_BUILD = if flag.is_SPECIFY_BUILD() {
            // version: Version
            let version = Version::read(r)?;

            Some(RealmRealmFlagSPECIFY_BUILD {
                version,
            })
        } else {
            None
        };

        let flag = RealmRealmFlag {
            inner: flag.as_u8(),
            specify_build: flag_SPECIFY_BUILD,
        };

        Ok(Self {
            realm_type,
            locked,
            flag,
            name,
            address,
            population,
            number_of_characters_on_realm,
            category,
            realm_id,
        })
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // realm_type: u8
        w.write_all(&self.realm_type.to_le_bytes())?;

        // locked: u8
        w.write_all(&self.locked.to_le_bytes())?;

        // flag: RealmFlag
        self.flag.write(w)?;

        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // address: CString
        w.write_all(self.address.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // population: Population
        self.population.write(w)?;

        // number_of_characters_on_realm: u8
        w.write_all(&self.number_of_characters_on_realm.to_le_bytes())?;

        // category: RealmCategory
        self.category.write(w)?;

        // realm_id: u8
        w.write_all(&self.realm_id.to_le_bytes())?;

        if let Some(s) = &self.flag.specify_build {
            s.write(w)?;
        }

        Ok(())
    }

}

impl VariableSized for Realm {
    fn size(&self) -> usize {
        1 // realm_type: u8
        + 1 // locked: u8
        + self.flag.size() // flag: RealmFlag and subfields
        + self.name.len() + 1 // name: CString and Null Terminator
        + self.address.len() + 1 // address: CString and Null Terminator
        + Population::size() // population: Population
        + 1 // number_of_characters_on_realm: u8
        + RealmCategory::size() // category: RealmCategory
        + 1 // realm_id: u8
    }
}

impl MaximumPossibleSized for Realm {
    fn maximum_possible_size() -> usize {
        1 // realm_type: u8
        + 1 // locked: u8
        + RealmFlag::maximum_possible_size() // flag: RealmFlag
        + 256 // name: CString
        + 256 // address: CString
        + Population::maximum_possible_size() // population: Population
        + 1 // number_of_characters_on_realm: u8
        + RealmCategory::maximum_possible_size() // category: RealmCategory
        + 1 // realm_id: u8
    }
}

#[derive(Debug)]
pub enum RealmError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
    RealmCategory(RealmCategoryError),
}

impl std::error::Error for RealmError {}
impl std::fmt::Display for RealmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::RealmCategory(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for RealmError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for RealmError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<RealmCategoryError> for RealmError {
    fn from(e: RealmCategoryError) -> Self {
        Self::RealmCategory(e)
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct RealmRealmFlag {
    inner: u8,
    specify_build: Option<RealmRealmFlagSPECIFY_BUILD>,
}

impl From<&RealmRealmFlag> for RealmFlag {
    fn from(e: &RealmRealmFlag) -> Self {
        Self::new(e.inner)
    }
}

impl RealmRealmFlag {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: RealmFlag = self.into();
        a.write(w)?;
        Ok(())
    }

    pub const fn empty() -> Self {
        Self {
            inner: 0,
            specify_build: None,
        }
    }

    pub const fn new_NONE() -> Self {
        Self {
            inner: RealmFlag::NONE,
            specify_build: None,
        }
    }

    pub fn set_NONE(&mut self) -> Self {
        self.inner |= RealmFlag::NONE;
        self.clone()
    }

    pub fn clear_NONE(&mut self) -> Self {
        self.inner &= RealmFlag::NONE.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_INVALID() -> Self {
        Self {
            inner: RealmFlag::INVALID,
            specify_build: None,
        }
    }

    pub fn set_INVALID(&mut self) -> Self {
        self.inner |= RealmFlag::INVALID;
        self.clone()
    }

    pub fn clear_INVALID(&mut self) -> Self {
        self.inner &= RealmFlag::INVALID.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_OFFLINE() -> Self {
        Self {
            inner: RealmFlag::OFFLINE,
            specify_build: None,
        }
    }

    pub fn set_OFFLINE(&mut self) -> Self {
        self.inner |= RealmFlag::OFFLINE;
        self.clone()
    }

    pub fn clear_OFFLINE(&mut self) -> Self {
        self.inner &= RealmFlag::OFFLINE.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_SPECIFY_BUILD(specify_build: RealmRealmFlagSPECIFY_BUILD) -> Self {
        Self {
            inner: RealmFlag::SPECIFY_BUILD,
            specify_build: Some(specify_build),
        }
    }

    pub fn set_SPECIFY_BUILD(&mut self, specify_build: RealmRealmFlagSPECIFY_BUILD) -> Self {
        self.inner |= RealmFlag::SPECIFY_BUILD;
        self.specify_build = Some(specify_build);
        self.clone()
    }

    pub fn clear_SPECIFY_BUILD(&mut self) -> Self {
        self.inner &= RealmFlag::SPECIFY_BUILD.reverse_bits();
        self.specify_build = None;
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_FORCE_BLUE_RECOMMENDED() -> Self {
        Self {
            inner: RealmFlag::FORCE_BLUE_RECOMMENDED,
            specify_build: None,
        }
    }

    pub fn set_FORCE_BLUE_RECOMMENDED(&mut self) -> Self {
        self.inner |= RealmFlag::FORCE_BLUE_RECOMMENDED;
        self.clone()
    }

    pub fn clear_FORCE_BLUE_RECOMMENDED(&mut self) -> Self {
        self.inner &= RealmFlag::FORCE_BLUE_RECOMMENDED.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_FORCE_GREEN_RECOMMENDED() -> Self {
        Self {
            inner: RealmFlag::FORCE_GREEN_RECOMMENDED,
            specify_build: None,
        }
    }

    pub fn set_FORCE_GREEN_RECOMMENDED(&mut self) -> Self {
        self.inner |= RealmFlag::FORCE_GREEN_RECOMMENDED;
        self.clone()
    }

    pub fn clear_FORCE_GREEN_RECOMMENDED(&mut self) -> Self {
        self.inner &= RealmFlag::FORCE_GREEN_RECOMMENDED.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub const fn new_FORCE_RED_FULL() -> Self {
        Self {
            inner: RealmFlag::FORCE_RED_FULL,
            specify_build: None,
        }
    }

    pub fn set_FORCE_RED_FULL(&mut self) -> Self {
        self.inner |= RealmFlag::FORCE_RED_FULL;
        self.clone()
    }

    pub fn clear_FORCE_RED_FULL(&mut self) -> Self {
        self.inner &= RealmFlag::FORCE_RED_FULL.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

}
impl VariableSized for RealmRealmFlag {
    fn size(&self) -> usize {
        1 // inner: RealmFlag (u8)
        + {
            if let Some(s) = &self.specify_build {
                s.size()
            } else {
                0
            }
        }
    }
}

impl MaximumPossibleSized for RealmRealmFlag {
    fn maximum_possible_size() -> usize {
        1 // inner: RealmFlag (u8)
        + RealmRealmFlagSPECIFY_BUILD::maximum_possible_size() // SPECIFY_BUILD enumerator
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct RealmRealmFlagSPECIFY_BUILD {
    pub version: Version,
}

impl VariableSized for RealmRealmFlagSPECIFY_BUILD {
    fn size(&self) -> usize {
        Version::size() // version: Version
    }
}

impl MaximumPossibleSized for RealmRealmFlagSPECIFY_BUILD {
    fn maximum_possible_size() -> usize {
        Version::maximum_possible_size() // version: Version
    }
}

impl RealmRealmFlagSPECIFY_BUILD {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        self.version.write(w)?;

        Ok(())
    }
}

