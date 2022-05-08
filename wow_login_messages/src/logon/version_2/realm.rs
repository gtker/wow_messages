use std::convert::{TryFrom, TryInto};
use crate::logon::version_2::Population;
use crate::logon::version_2::{RealmCategory, RealmCategoryError};
use crate::logon::version_2::{RealmFlag};
use crate::logon::version_2::{RealmType, RealmTypeError};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct Realm {
    pub realm_type: RealmType,
    pub flag: RealmFlag,
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
        // realm_type: RealmType
        let realm_type = RealmType::read(r)?;

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

        Ok(Self {
            realm_type,
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
        // realm_type: RealmType
        self.realm_type.write(w)?;

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
            // realm_type: RealmType
            let realm_type = RealmType::tokio_read(r).await?;

            // flag: RealmFlag
            let flag = RealmFlag::tokio_read(r).await?;

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

            Ok(Self {
                realm_type,
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
            // realm_type: RealmType
            self.realm_type.tokio_write(w).await?;

            // flag: RealmFlag
            self.flag.tokio_write(w).await?;

            // name: CString
            w.write_all(self.name.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            // address: CString
            w.write_all(self.address.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            // population: Population
            self.population.tokio_write(w).await?;

            // number_of_characters_on_realm: u8
            w.write_all(&self.number_of_characters_on_realm.to_le_bytes()).await?;

            // category: RealmCategory
            self.category.tokio_write(w).await?;

            // realm_id: u8
            w.write_all(&self.realm_id.to_le_bytes()).await?;

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
            // realm_type: RealmType
            let realm_type = RealmType::astd_read(r).await?;

            // flag: RealmFlag
            let flag = RealmFlag::astd_read(r).await?;

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

            Ok(Self {
                realm_type,
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
            // realm_type: RealmType
            self.realm_type.astd_write(w).await?;

            // flag: RealmFlag
            self.flag.astd_write(w).await?;

            // name: CString
            w.write_all(self.name.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            // address: CString
            w.write_all(self.address.as_bytes()).await?;
            // Null terminator
            w.write_all(&[0]).await?;

            // population: Population
            self.population.astd_write(w).await?;

            // number_of_characters_on_realm: u8
            w.write_all(&self.number_of_characters_on_realm.to_le_bytes()).await?;

            // category: RealmCategory
            self.category.astd_write(w).await?;

            // realm_id: u8
            w.write_all(&self.realm_id.to_le_bytes()).await?;

            Ok(())
        })
    }

}

impl VariableSized for Realm {
    fn size(&self) -> usize {
        0
        + 4 // realm_type: RealmType
        + 1 // flag: RealmFlag
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
        + 4 // realm_type: RealmType
        + 1 // flag: RealmFlag
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
    RealmType(RealmTypeError),
}

impl std::error::Error for RealmError {}
impl std::fmt::Display for RealmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::RealmCategory(i) => i.fmt(f),
            Self::RealmType(i) => i.fmt(f),
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

impl From<RealmTypeError> for RealmError {
    fn from(e: RealmTypeError) -> Self {
        Self::RealmType(e)
    }
}

