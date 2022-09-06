use std::convert::{TryFrom, TryInto};
use crate::logon::version_2::Population;
use crate::logon::version_2::RealmCategory;
use crate::logon::version_8::RealmFlag;
use crate::logon::version_2::RealmType;
use crate::logon::all::Version;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_realm/server.wowm:138`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_realm/server.wowm#L138):
/// ```text
/// struct Realm {
///     RealmType realm_type;
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
    /// vmangos: this is the second column in Cfg_Configs.dbc
    ///
    pub realm_type: RealmType,
    pub locked: u8,
    pub flag: Realm_RealmFlag,
    pub name: String,
    pub address: String,
    pub population: Population,
    pub number_of_characters_on_realm: u8,
    pub category: RealmCategory,
    pub realm_id: u8,
}

impl Realm {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // realm_type: RealmType
        w.write_all(&(self.realm_type.as_int() as u8).to_le_bytes())?;

        // locked: u8
        w.write_all(&self.locked.to_le_bytes())?;

        // flag: RealmFlag
        w.write_all(&(self.flag.as_int() as u8).to_le_bytes())?;

        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // address: CString
        w.write_all(self.address.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // population: Population
        w.write_all(&(self.population.as_int() as u32).to_le_bytes())?;

        // number_of_characters_on_realm: u8
        w.write_all(&self.number_of_characters_on_realm.to_le_bytes())?;

        // category: RealmCategory
        w.write_all(&(self.category.as_int() as u8).to_le_bytes())?;

        // realm_id: u8
        w.write_all(&self.realm_id.to_le_bytes())?;

        if let Some(if_statement) = &self.flag.specify_build {
            // version: Version
            if_statement.version.write_into_vec(w)?;

        }

        Ok(())
    }
}

impl Realm {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // realm_type: RealmType
        let realm_type: RealmType = crate::util::read_u8_le(r)?.try_into()?;

        // locked: u8
        let locked = crate::util::read_u8_le(r)?;

        // flag: RealmFlag
        let flag = RealmFlag::new(crate::util::read_u8_le(r)?);

        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        // address: CString
        let address = crate::util::read_c_string_to_vec(r)?;
        let address = String::from_utf8(address)?;

        // population: Population
        let population: Population = crate::util::read_u32_le(r)?.into();

        // number_of_characters_on_realm: u8
        let number_of_characters_on_realm = crate::util::read_u8_le(r)?;

        // category: RealmCategory
        let category: RealmCategory = crate::util::read_u8_le(r)?.try_into()?;

        // realm_id: u8
        let realm_id = crate::util::read_u8_le(r)?;

        let flag_SPECIFY_BUILD = if flag.is_SPECIFY_BUILD() {
            // version: Version
            let version = Version::read(r)?;

            Some(Realm_RealmFlag_SpecifyBuild {
                version,
            })
        }
        else {
            None
        };

        let flag = Realm_RealmFlag {
            inner: flag.as_int(),
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

    #[cfg(feature = "tokio")]
    pub(crate) async fn tokio_read<R: tokio::io::AsyncReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // realm_type: RealmType
        let realm_type: RealmType = crate::util::tokio_read_u8_le(r).await?.try_into()?;

        // locked: u8
        let locked = crate::util::tokio_read_u8_le(r).await?;

        // flag: RealmFlag
        let flag = RealmFlag::new(crate::util::tokio_read_u8_le(r).await?);

        // name: CString
        let name = crate::util::tokio_read_c_string_to_vec(r).await?;
        let name = String::from_utf8(name)?;

        // address: CString
        let address = crate::util::tokio_read_c_string_to_vec(r).await?;
        let address = String::from_utf8(address)?;

        // population: Population
        let population: Population = crate::util::tokio_read_u32_le(r).await?.into();

        // number_of_characters_on_realm: u8
        let number_of_characters_on_realm = crate::util::tokio_read_u8_le(r).await?;

        // category: RealmCategory
        let category: RealmCategory = crate::util::tokio_read_u8_le(r).await?.try_into()?;

        // realm_id: u8
        let realm_id = crate::util::tokio_read_u8_le(r).await?;

        let flag_SPECIFY_BUILD = if flag.is_SPECIFY_BUILD() {
            // version: Version
            let version = Version::tokio_read(r).await?;

            Some(Realm_RealmFlag_SpecifyBuild {
                version,
            })
        }
        else {
            None
        };

        let flag = Realm_RealmFlag {
            inner: flag.as_int(),
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

    #[cfg(feature = "async-std")]
    pub(crate) async fn astd_read<R: async_std::io::ReadExt + Unpin + Send>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // realm_type: RealmType
        let realm_type: RealmType = crate::util::astd_read_u8_le(r).await?.try_into()?;

        // locked: u8
        let locked = crate::util::astd_read_u8_le(r).await?;

        // flag: RealmFlag
        let flag = RealmFlag::new(crate::util::astd_read_u8_le(r).await?);

        // name: CString
        let name = crate::util::astd_read_c_string_to_vec(r).await?;
        let name = String::from_utf8(name)?;

        // address: CString
        let address = crate::util::astd_read_c_string_to_vec(r).await?;
        let address = String::from_utf8(address)?;

        // population: Population
        let population: Population = crate::util::astd_read_u32_le(r).await?.into();

        // number_of_characters_on_realm: u8
        let number_of_characters_on_realm = crate::util::astd_read_u8_le(r).await?;

        // category: RealmCategory
        let category: RealmCategory = crate::util::astd_read_u8_le(r).await?.try_into()?;

        // realm_id: u8
        let realm_id = crate::util::astd_read_u8_le(r).await?;

        let flag_SPECIFY_BUILD = if flag.is_SPECIFY_BUILD() {
            // version: Version
            let version = Version::astd_read(r).await?;

            Some(Realm_RealmFlag_SpecifyBuild {
                version,
            })
        }
        else {
            None
        };

        let flag = Realm_RealmFlag {
            inner: flag.as_int(),
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

}

impl Realm {
    pub(crate) fn size(&self) -> usize {
        1 // realm_type: RealmType
        + 1 // locked: u8
        + self.flag.size() // flag: Realm_RealmFlag
        + self.name.len() + 1 // name: CString
        + self.address.len() + 1 // address: CString
        + 4 // population: Population
        + 1 // number_of_characters_on_realm: u8
        + 1 // category: RealmCategory
        + 1 // realm_id: u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Realm_RealmFlag {
    inner: u8,
    specify_build: Option<Realm_RealmFlag_SpecifyBuild>,
}

impl Realm_RealmFlag {
    pub const fn empty() -> Self {
        Self {
            inner: 0,
            specify_build: None,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
        && self.specify_build.is_none()
    }

    pub const fn new_INVALID() -> Self {
        Self {
            inner: RealmFlag::INVALID,
            specify_build: None,
        }
    }

    pub fn set_INVALID(mut self) -> Self {
        self.inner |= RealmFlag::INVALID;
        self
    }

    pub const fn get_INVALID(&self) -> bool {
        (self.inner & RealmFlag::INVALID) != 0
    }

    pub fn clear_INVALID(mut self) -> Self {
        self.inner &= RealmFlag::INVALID.reverse_bits();
        self
    }

    pub const fn new_OFFLINE() -> Self {
        Self {
            inner: RealmFlag::OFFLINE,
            specify_build: None,
        }
    }

    pub fn set_OFFLINE(mut self) -> Self {
        self.inner |= RealmFlag::OFFLINE;
        self
    }

    pub const fn get_OFFLINE(&self) -> bool {
        (self.inner & RealmFlag::OFFLINE) != 0
    }

    pub fn clear_OFFLINE(mut self) -> Self {
        self.inner &= RealmFlag::OFFLINE.reverse_bits();
        self
    }

    pub const fn new_SPECIFY_BUILD(specify_build: Realm_RealmFlag_SpecifyBuild) -> Self {
        Self {
            inner: RealmFlag::SPECIFY_BUILD,
            specify_build: Some(specify_build),
        }
    }

    pub fn set_SPECIFY_BUILD(mut self, specify_build: Realm_RealmFlag_SpecifyBuild) -> Self {
        self.inner |= RealmFlag::SPECIFY_BUILD;
        self.specify_build = Some(specify_build);
        self
    }

    pub const fn get_SPECIFY_BUILD(&self) -> Option<&Realm_RealmFlag_SpecifyBuild> {
        self.specify_build.as_ref()
    }

    pub fn clear_SPECIFY_BUILD(mut self) -> Self {
        self.inner &= RealmFlag::SPECIFY_BUILD.reverse_bits();
        self.specify_build = None;
        self
    }

    pub const fn new_FORCE_BLUE_RECOMMENDED() -> Self {
        Self {
            inner: RealmFlag::FORCE_BLUE_RECOMMENDED,
            specify_build: None,
        }
    }

    pub fn set_FORCE_BLUE_RECOMMENDED(mut self) -> Self {
        self.inner |= RealmFlag::FORCE_BLUE_RECOMMENDED;
        self
    }

    pub const fn get_FORCE_BLUE_RECOMMENDED(&self) -> bool {
        (self.inner & RealmFlag::FORCE_BLUE_RECOMMENDED) != 0
    }

    pub fn clear_FORCE_BLUE_RECOMMENDED(mut self) -> Self {
        self.inner &= RealmFlag::FORCE_BLUE_RECOMMENDED.reverse_bits();
        self
    }

    pub const fn new_FORCE_GREEN_RECOMMENDED() -> Self {
        Self {
            inner: RealmFlag::FORCE_GREEN_RECOMMENDED,
            specify_build: None,
        }
    }

    pub fn set_FORCE_GREEN_RECOMMENDED(mut self) -> Self {
        self.inner |= RealmFlag::FORCE_GREEN_RECOMMENDED;
        self
    }

    pub const fn get_FORCE_GREEN_RECOMMENDED(&self) -> bool {
        (self.inner & RealmFlag::FORCE_GREEN_RECOMMENDED) != 0
    }

    pub fn clear_FORCE_GREEN_RECOMMENDED(mut self) -> Self {
        self.inner &= RealmFlag::FORCE_GREEN_RECOMMENDED.reverse_bits();
        self
    }

    pub const fn new_FORCE_RED_FULL() -> Self {
        Self {
            inner: RealmFlag::FORCE_RED_FULL,
            specify_build: None,
        }
    }

    pub fn set_FORCE_RED_FULL(mut self) -> Self {
        self.inner |= RealmFlag::FORCE_RED_FULL;
        self
    }

    pub const fn get_FORCE_RED_FULL(&self) -> bool {
        (self.inner & RealmFlag::FORCE_RED_FULL) != 0
    }

    pub fn clear_FORCE_RED_FULL(mut self) -> Self {
        self.inner &= RealmFlag::FORCE_RED_FULL.reverse_bits();
        self
    }

    pub(crate) const fn as_int(&self) -> u8 {
        self.inner
    }

}
impl Realm_RealmFlag {
    pub(crate) fn size(&self) -> usize {
        1 // inner
        + {
            if let Some(s) = &self.specify_build {
                s.size()
            } else {
                0
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Realm_RealmFlag_SpecifyBuild {
    pub version: Version,
}

impl Realm_RealmFlag_SpecifyBuild {
    pub(crate) fn size(&self) -> usize {
        5 // version: Version
    }
}

