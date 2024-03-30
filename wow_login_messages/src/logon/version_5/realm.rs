use std::io::{Read, Write};

use crate::all::population::Population;
use crate::logon::version_2::{
    RealmCategory, RealmFlag, RealmType,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_realm/server.wowm:67`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_realm/server.wowm#L67):
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
/// }
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct Realm {
    pub realm_type: RealmType,
    pub locked: bool,
    pub flag: RealmFlag,
    pub name: String,
    pub address: String,
    pub population: Population,
    pub number_of_characters_on_realm: u8,
    pub category: RealmCategory,
    pub realm_id: u8,
}

impl Realm {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // realm_type: RealmType
        w.write_all(&(self.realm_type.as_int().to_le_bytes()))?;

        // locked: Bool
        w.write_all(u8::from(self.locked).to_le_bytes().as_slice())?;

        // flag: RealmFlag
        w.write_all(&(self.flag.as_int().to_le_bytes()))?;

        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().next_back(), Some(&0_u8), "String `name` must not be null-terminated.");
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // address: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.address.as_bytes().iter().next_back(), Some(&0_u8), "String `address` must not be null-terminated.");
        w.write_all(self.address.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // population: Population
        w.write_all(&self.population.as_int().to_le_bytes())?;

        // number_of_characters_on_realm: u8
        w.write_all(&self.number_of_characters_on_realm.to_le_bytes())?;

        // category: RealmCategory
        w.write_all(&(self.category.as_int().to_le_bytes()))?;

        // realm_id: u8
        w.write_all(&self.realm_id.to_le_bytes())?;

        Ok(())
    }
}

impl Realm {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // realm_type: RealmType
        let realm_type = crate::util::read_u8_le(&mut r)?.try_into()?;

        // locked: Bool
        let locked = crate::util::read_bool_u8(&mut r)?;

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
        let population = crate::util::read_f32_le(&mut r)?.into();

        // number_of_characters_on_realm: u8
        let number_of_characters_on_realm = crate::util::read_u8_le(&mut r)?;

        // category: RealmCategory
        let category = crate::util::read_u8_le(&mut r)?.try_into()?;

        // realm_id: u8
        let realm_id = crate::util::read_u8_le(&mut r)?;

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
    pub(crate) async fn tokio_read<R: tokio::io::AsyncReadExt + Unpin + Send>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // realm_type: RealmType
        let realm_type = crate::util::tokio_read_u8_le(&mut r).await?.try_into()?;

        // locked: Bool
        let locked = crate::util::tokio_read_bool_u8(&mut r).await?;

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
        let population = crate::util::tokio_read_f32_le(&mut r).await?.into();

        // number_of_characters_on_realm: u8
        let number_of_characters_on_realm = crate::util::tokio_read_u8_le(&mut r).await?;

        // category: RealmCategory
        let category = crate::util::tokio_read_u8_le(&mut r).await?.try_into()?;

        // realm_id: u8
        let realm_id = crate::util::tokio_read_u8_le(&mut r).await?;

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
    pub(crate) async fn astd_read<R: async_std::io::ReadExt + Unpin + Send>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // realm_type: RealmType
        let realm_type = crate::util::astd_read_u8_le(&mut r).await?.try_into()?;

        // locked: Bool
        let locked = crate::util::astd_read_bool_u8(&mut r).await?;

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
        let population = crate::util::astd_read_f32_le(&mut r).await?.into();

        // number_of_characters_on_realm: u8
        let number_of_characters_on_realm = crate::util::astd_read_u8_le(&mut r).await?;

        // category: RealmCategory
        let category = crate::util::astd_read_u8_le(&mut r).await?.try_into()?;

        // realm_id: u8
        let realm_id = crate::util::astd_read_u8_le(&mut r).await?;

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
        + 1 // flag: RealmFlag
        + self.name.len() + 1 // name: CString
        + self.address.len() + 1 // address: CString
        + 4 // population: Population
        + 1 // number_of_characters_on_realm: u8
        + 1 // category: RealmCategory
        + 1 // realm_id: u8
    }
}

