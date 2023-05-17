use std::io::{Read, Write};

use crate::logon::all::Version;
use crate::logon::version_2::{
    Population, RealmCategory, RealmType,
};
use crate::logon::version_8::RealmFlag;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_realm/server.wowm:162`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_realm/server.wowm#L162):
/// ```text
/// struct Realm {
///     RealmType realm_type;
///     Bool locked;
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
    pub locked: bool,
    pub flag: Realm_RealmFlag,
    pub name: String,
    pub address: String,
    pub population: Population,
    pub number_of_characters_on_realm: u8,
    pub category: RealmCategory,
    pub realm_id: u8,
}

impl Realm {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // realm_type: RealmType
        w.write_all(&u8::from(self.realm_type.as_int()).to_le_bytes())?;

        // locked: Bool
        w.write_all(u8::from(self.locked).to_le_bytes().as_slice())?;

        // flag: RealmFlag
        w.write_all(&u8::from(self.flag.as_int()).to_le_bytes())?;

        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().rev().next(), Some(&0_u8), "String `name` must not be null-terminated.");
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // address: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.address.as_bytes().iter().rev().next(), Some(&0_u8), "String `address` must not be null-terminated.");
        w.write_all(self.address.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // population: Population
        w.write_all(&u32::from(self.population.as_int()).to_le_bytes())?;

        // number_of_characters_on_realm: u8
        w.write_all(&self.number_of_characters_on_realm.to_le_bytes())?;

        // category: RealmCategory
        w.write_all(&u8::from(self.category.as_int()).to_le_bytes())?;

        // realm_id: u8
        w.write_all(&self.realm_id.to_le_bytes())?;

        if let Some(if_statement) = &self.flag.specify_build {
            // version: Version
            if_statement.version.write_into_vec(&mut w)?;

        }

        Ok(())
    }
}

impl Realm {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> std::result::Result<Self, crate::errors::ParseError> {
        // realm_type: RealmType
        let realm_type: RealmType = crate::util::read_u8_le(&mut r)?.try_into()?;

        // locked: Bool
        let locked = crate::util::read_u8_le(&mut r)? != 0;

        // flag: RealmFlag
        let flag = RealmFlag::new(crate::util::read_u8_le(&mut r)?);

        // name: CString
        let name = {
            let name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(name)?
        };

        // address: CString
        let address = {
            let address = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(address)?
        };

        // population: Population
        let population: Population = crate::util::read_u32_le(&mut r)?.into();

        // number_of_characters_on_realm: u8
        let number_of_characters_on_realm = crate::util::read_u8_le(&mut r)?;

        // category: RealmCategory
        let category: RealmCategory = crate::util::read_u8_le(&mut r)?.try_into()?;

        // realm_id: u8
        let realm_id = crate::util::read_u8_le(&mut r)?;

        let flag_SPECIFY_BUILD = if flag.is_SPECIFY_BUILD() {
            // version: Version
            let version = Version::read(&mut r)?;

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
    pub(crate) async fn tokio_read<R: tokio::io::AsyncReadExt + Unpin + Send>(mut r: R) -> std::result::Result<Self, crate::errors::ParseError> {
        // realm_type: RealmType
        let realm_type: RealmType = crate::util::tokio_read_u8_le(&mut r).await?.try_into()?;

        // locked: Bool
        let locked = crate::util::tokio_read_u8_le(&mut r).await? != 0;

        // flag: RealmFlag
        let flag = RealmFlag::new(crate::util::tokio_read_u8_le(&mut r).await?);

        // name: CString
        let name = {
            let name = crate::util::tokio_read_c_string_to_vec(&mut r).await?;
            String::from_utf8(name)?
        };

        // address: CString
        let address = {
            let address = crate::util::tokio_read_c_string_to_vec(&mut r).await?;
            String::from_utf8(address)?
        };

        // population: Population
        let population: Population = crate::util::tokio_read_u32_le(&mut r).await?.into();

        // number_of_characters_on_realm: u8
        let number_of_characters_on_realm = crate::util::tokio_read_u8_le(&mut r).await?;

        // category: RealmCategory
        let category: RealmCategory = crate::util::tokio_read_u8_le(&mut r).await?.try_into()?;

        // realm_id: u8
        let realm_id = crate::util::tokio_read_u8_le(&mut r).await?;

        let flag_SPECIFY_BUILD = if flag.is_SPECIFY_BUILD() {
            // version: Version
            let version = Version::tokio_read(&mut r).await?;

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
    pub(crate) async fn astd_read<R: async_std::io::ReadExt + Unpin + Send>(mut r: R) -> std::result::Result<Self, crate::errors::ParseError> {
        // realm_type: RealmType
        let realm_type: RealmType = crate::util::astd_read_u8_le(&mut r).await?.try_into()?;

        // locked: Bool
        let locked = crate::util::astd_read_u8_le(&mut r).await? != 0;

        // flag: RealmFlag
        let flag = RealmFlag::new(crate::util::astd_read_u8_le(&mut r).await?);

        // name: CString
        let name = {
            let name = crate::util::astd_read_c_string_to_vec(&mut r).await?;
            String::from_utf8(name)?
        };

        // address: CString
        let address = {
            let address = crate::util::astd_read_c_string_to_vec(&mut r).await?;
            String::from_utf8(address)?
        };

        // population: Population
        let population: Population = crate::util::astd_read_u32_le(&mut r).await?.into();

        // number_of_characters_on_realm: u8
        let number_of_characters_on_realm = crate::util::astd_read_u8_le(&mut r).await?;

        // category: RealmCategory
        let category: RealmCategory = crate::util::astd_read_u8_le(&mut r).await?.try_into()?;

        // realm_id: u8
        let realm_id = crate::util::astd_read_u8_le(&mut r).await?;

        let flag_SPECIFY_BUILD = if flag.is_SPECIFY_BUILD() {
            // version: Version
            let version = Version::astd_read(&mut r).await?;

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
        + 1 // locked: Bool
        + self.flag.size() // flag: Realm_RealmFlag
        + self.name.len() + 1 // name: CString
        + self.address.len() + 1 // address: CString
        + 4 // population: Population
        + 1 // number_of_characters_on_realm: u8
        + 1 // category: RealmCategory
        + 1 // realm_id: u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Realm_RealmFlag {
    inner: u8,
    specify_build: Option<Realm_RealmFlag_SpecifyBuild>,
}

impl Realm_RealmFlag {
    pub const fn new(inner: u8, specify_build: Option<Realm_RealmFlag_SpecifyBuild>,) -> Self {
        Self {
            inner,
            specify_build, 
        }
    }

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

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_INVALID(mut self) -> Self {
        self.inner |= RealmFlag::INVALID;
        self
    }

    pub const fn get_INVALID(&self) -> bool {
        (self.inner & RealmFlag::INVALID) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
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

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_OFFLINE(mut self) -> Self {
        self.inner |= RealmFlag::OFFLINE;
        self
    }

    pub const fn get_OFFLINE(&self) -> bool {
        (self.inner & RealmFlag::OFFLINE) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
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

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_SPECIFY_BUILD(mut self, specify_build: Realm_RealmFlag_SpecifyBuild) -> Self {
        self.inner |= RealmFlag::SPECIFY_BUILD;
        self.specify_build = Some(specify_build);
        self
    }

    pub const fn get_SPECIFY_BUILD(&self) -> Option<&Realm_RealmFlag_SpecifyBuild> {
        self.specify_build.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
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

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_FORCE_BLUE_RECOMMENDED(mut self) -> Self {
        self.inner |= RealmFlag::FORCE_BLUE_RECOMMENDED;
        self
    }

    pub const fn get_FORCE_BLUE_RECOMMENDED(&self) -> bool {
        (self.inner & RealmFlag::FORCE_BLUE_RECOMMENDED) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
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

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_FORCE_GREEN_RECOMMENDED(mut self) -> Self {
        self.inner |= RealmFlag::FORCE_GREEN_RECOMMENDED;
        self
    }

    pub const fn get_FORCE_GREEN_RECOMMENDED(&self) -> bool {
        (self.inner & RealmFlag::FORCE_GREEN_RECOMMENDED) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
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

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_FORCE_RED_FULL(mut self) -> Self {
        self.inner |= RealmFlag::FORCE_RED_FULL;
        self
    }

    pub const fn get_FORCE_RED_FULL(&self) -> bool {
        (self.inner & RealmFlag::FORCE_RED_FULL) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_FORCE_RED_FULL(mut self) -> Self {
        self.inner &= RealmFlag::FORCE_RED_FULL.reverse_bits();
        self
    }

    pub(crate) const fn as_int(&self) -> u8 {
        self.inner
    }

}
impl Realm_RealmFlag {
    pub(crate) const fn size(&self) -> usize {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Realm_RealmFlag_SpecifyBuild {
    pub version: Version,
}

impl Realm_RealmFlag_SpecifyBuild {
    pub(crate) const fn size(&self) -> usize {
        5 // version: Version
    }
}

