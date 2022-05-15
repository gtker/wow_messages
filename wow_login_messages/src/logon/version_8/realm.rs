use std::convert::{TryFrom, TryInto};
use crate::logon::version_2::Population;
use crate::logon::version_2::{RealmCategory, RealmCategoryError};
use crate::logon::version_8::{RealmFlag};
use crate::logon::all::Version;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
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

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // realm_type: u8
        let realm_type = crate::util::read_u8_le(r)?;

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
        }
        else {
            None
        };

        let flag = RealmRealmFlag {
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

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // realm_type: u8
        w.write_all(&self.realm_type.to_le_bytes())?;

        // locked: u8
        w.write_all(&self.locked.to_le_bytes())?;

        // flag: RealmFlag
        crate::util::write_u8_le(w, self.flag.as_int() as u8)?;

        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // address: CString
        w.write_all(self.address.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // population: Population
        crate::util::write_u32_le(w, self.population.as_int() as u32)?;

        // number_of_characters_on_realm: u8
        w.write_all(&self.number_of_characters_on_realm.to_le_bytes())?;

        // category: RealmCategory
        crate::util::write_u8_le(w, self.category.as_int() as u8)?;

        // realm_id: u8
        w.write_all(&self.realm_id.to_le_bytes())?;

        if let Some(if_statement) = &self.flag.specify_build {
            // version: Version
            if_statement.version.write(w)?;

        }

        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // realm_type: u8
            let realm_type = crate::util::tokio_read_u8_le(r).await?;

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
            let population = Population::tokio_read(r).await?;

            // number_of_characters_on_realm: u8
            let number_of_characters_on_realm = crate::util::tokio_read_u8_le(r).await?;

            // category: RealmCategory
            let category = RealmCategory::tokio_read(r).await?;

            // realm_id: u8
            let realm_id = crate::util::tokio_read_u8_le(r).await?;

            let flag_SPECIFY_BUILD = if flag.is_SPECIFY_BUILD() {
                // version: Version
                let version = Version::tokio_read(r).await?;

                Some(RealmRealmFlagSPECIFY_BUILD {
                    version,
                })
            }
            else {
                None
            };

            let flag = RealmRealmFlag {
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
        })
    }

    #[cfg(feature = "async_tokio")]
    fn tokio_write<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // realm_type: u8
            w.write_all(&self.realm_type.to_le_bytes()).await?;

            // locked: u8
            w.write_all(&self.locked.to_le_bytes()).await?;

            // flag: RealmFlag
            crate::util::tokio_write_u8_le(w, self.flag.as_int() as u8).await?;

            // name: CString
            w.write_all(self.name.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            // address: CString
            w.write_all(self.address.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            // population: Population
            crate::util::tokio_write_u32_le(w, self.population.as_int() as u32).await?;

            // number_of_characters_on_realm: u8
            w.write_all(&self.number_of_characters_on_realm.to_le_bytes()).await?;

            // category: RealmCategory
            crate::util::tokio_write_u8_le(w, self.category.as_int() as u8).await?;

            // realm_id: u8
            w.write_all(&self.realm_id.to_le_bytes()).await?;

            if let Some(if_statement) = &self.flag.specify_build {
                // version: Version
                if_statement.version.tokio_write(w).await?;

            }

            Ok(())
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // realm_type: u8
            let realm_type = crate::util::astd_read_u8_le(r).await?;

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
            let population = Population::astd_read(r).await?;

            // number_of_characters_on_realm: u8
            let number_of_characters_on_realm = crate::util::astd_read_u8_le(r).await?;

            // category: RealmCategory
            let category = RealmCategory::astd_read(r).await?;

            // realm_id: u8
            let realm_id = crate::util::astd_read_u8_le(r).await?;

            let flag_SPECIFY_BUILD = if flag.is_SPECIFY_BUILD() {
                // version: Version
                let version = Version::astd_read(r).await?;

                Some(RealmRealmFlagSPECIFY_BUILD {
                    version,
                })
            }
            else {
                None
            };

            let flag = RealmRealmFlag {
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
        })
    }

    #[cfg(feature = "async_std")]
    fn astd_write<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // realm_type: u8
            w.write_all(&self.realm_type.to_le_bytes()).await?;

            // locked: u8
            w.write_all(&self.locked.to_le_bytes()).await?;

            // flag: RealmFlag
            crate::util::astd_write_u8_le(w, self.flag.as_int() as u8).await?;

            // name: CString
            w.write_all(self.name.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            // address: CString
            w.write_all(self.address.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            // population: Population
            crate::util::astd_write_u32_le(w, self.population.as_int() as u32).await?;

            // number_of_characters_on_realm: u8
            w.write_all(&self.number_of_characters_on_realm.to_le_bytes()).await?;

            // category: RealmCategory
            crate::util::astd_write_u8_le(w, self.category.as_int() as u8).await?;

            // realm_id: u8
            w.write_all(&self.realm_id.to_le_bytes()).await?;

            if let Some(if_statement) = &self.flag.specify_build {
                // version: Version
                if_statement.version.astd_write(w).await?;

            }

            Ok(())
        })
    }

}

impl VariableSized for Realm {
    fn size(&self) -> usize {
        0
        + 1 // realm_type: u8
        + 1 // locked: u8
        + self.flag.size() // flag: RealmRealmFlag
        + self.name.len() + 1 // name: CString
        + self.address.len() + 1 // address: CString
        + 4 // population: Population
        + 1 // number_of_characters_on_realm: u8
        + 1 // category: RealmCategory
        + 1 // realm_id: u8
    }
}

impl MaximumPossibleSized for Realm {
    fn maximum_possible_size() -> usize {
        0
        + 1 // realm_type: u8
        + 1 // locked: u8
        + 6 // flag: RealmRealmFlag
        + 256 // name: CString
        + 256 // address: CString
        + 4 // population: Population
        + 1 // number_of_characters_on_realm: u8
        + 1 // category: RealmCategory
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

impl RealmRealmFlag {
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

    pub const fn get_NONE(&self) -> bool {
        // Underlying value is 0
        self.inner == RealmFlag::NONE
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

    pub const fn get_INVALID(&self) -> bool {
        (self.inner & RealmFlag::INVALID) != 0
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

    pub const fn get_OFFLINE(&self) -> bool {
        (self.inner & RealmFlag::OFFLINE) != 0
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

    pub const fn get_SPECIFY_BUILD(&self) -> Option<&RealmRealmFlagSPECIFY_BUILD> {
        self.specify_build.as_ref()
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

    pub const fn get_FORCE_BLUE_RECOMMENDED(&self) -> bool {
        (self.inner & RealmFlag::FORCE_BLUE_RECOMMENDED) != 0
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

    pub const fn get_FORCE_GREEN_RECOMMENDED(&self) -> bool {
        (self.inner & RealmFlag::FORCE_GREEN_RECOMMENDED) != 0
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

    pub const fn get_FORCE_RED_FULL(&self) -> bool {
        (self.inner & RealmFlag::FORCE_RED_FULL) != 0
    }

    pub fn clear_FORCE_RED_FULL(&mut self) -> Self {
        self.inner &= RealmFlag::FORCE_RED_FULL.reverse_bits();
        // TODO: Cloning like this is not conductive to good performance but it is
        // temporarily necessary due to test syntax
        self.clone()
    }

    pub(crate) const fn as_int(&self) -> u8 {
        self.inner
    }

}
impl VariableSized for RealmRealmFlag {
    fn size(&self) -> usize {
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

impl MaximumPossibleSized for RealmRealmFlag {
    fn maximum_possible_size() -> usize {
        1 // inner
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
        5 // version: Version
    }
}

