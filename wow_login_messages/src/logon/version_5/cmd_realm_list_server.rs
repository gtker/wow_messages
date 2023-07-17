use crate::ServerMessage;
use std::io::{Read, Write};

use crate::logon::version_2::{
    Population, RealmCategory, RealmFlag, RealmType,
};
use crate::logon::version_5::Realm;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_realm/server.wowm:90`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_realm/server.wowm#L90):
/// ```text
/// slogin CMD_REALM_LIST_Server = 0x10 {
///     u16 size = self.size;
///     u32 header_padding = 0;
///     u8 number_of_realms;
///     Realm[number_of_realms] realms;
///     u16 footer_padding = 0;
/// }
/// ```
pub struct CMD_REALM_LIST_Server {
    pub realms: Vec<Realm>,
}

impl CMD_REALM_LIST_Server {
    /// The field `header_padding` is constantly specified to be:
    ///
    /// | Format | Value |
    /// | ------ | ----- |
    /// | Decimal | `0` |
    /// | Hex | `0x00` |
    /// | Original | `0` |
    ///
    /// **This field is not in the Rust struct, but is written as this constant value.**
    pub const HEADER_PADDING_VALUE: u32 = 0x00;

    /// The field `footer_padding` is constantly specified to be:
    ///
    /// | Format | Value |
    /// | ------ | ----- |
    /// | Decimal | `0` |
    /// | Hex | `0x00` |
    /// | Original | `0` |
    ///
    /// **This field is not in the Rust struct, but is written as this constant value.**
    pub const FOOTER_PADDING_VALUE: u16 = 0x00;

}

impl CMD_REALM_LIST_Server {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        // size: u16
        w.write_all(&((self.size() - 2) as u16).to_le_bytes())?;

        // header_padding: u32
        w.write_all(&Self::HEADER_PADDING_VALUE.to_le_bytes())?;

        // number_of_realms: u8
        w.write_all(&(self.realms.len() as u8).to_le_bytes())?;

        // realms: Realm[number_of_realms]
        for i in self.realms.iter() {
            i.write_into_vec(&mut w)?;
        }

        // footer_padding: u16
        w.write_all(&Self::FOOTER_PADDING_VALUE.to_le_bytes())?;

        Ok(())
    }
}

impl crate::private::Sealed for CMD_REALM_LIST_Server {}

impl CMD_REALM_LIST_Server {
    #[cfg(feature = "sync")]
    fn read_inner<R: Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // size: u16
        let _size = crate::util::read_u16_le(&mut r)?;
        // size is expected to always be self.size (0)

        // header_padding: u32
        let _header_padding = crate::util::read_u32_le(&mut r)?;
        // header_padding is expected to always be 0 (0)

        // number_of_realms: u8
        let number_of_realms = crate::util::read_u8_le(&mut r)?;

        // realms: Realm[number_of_realms]
        let realms = {
            let mut realms = Vec::with_capacity(number_of_realms as usize);
            for _ in 0..number_of_realms {
                realms.push(Realm::read(&mut r)?);
            }
            realms
        };

        // footer_padding: u16
        let _footer_padding = crate::util::read_u16_le(&mut r)?;
        // footer_padding is expected to always be 0 (0)

        Ok(Self {
            realms,
        })
    }

    #[cfg(feature = "tokio")]
    async fn tokio_read_inner<R: tokio::io::AsyncReadExt + Unpin + Send>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // size: u16
        let _size = crate::util::tokio_read_u16_le(&mut r).await?;
        // size is expected to always be self.size (0)

        // header_padding: u32
        let _header_padding = crate::util::tokio_read_u32_le(&mut r).await?;
        // header_padding is expected to always be 0 (0)

        // number_of_realms: u8
        let number_of_realms = crate::util::tokio_read_u8_le(&mut r).await?;

        // realms: Realm[number_of_realms]
        let realms = {
            let mut realms = Vec::with_capacity(number_of_realms as usize);
            for _ in 0..number_of_realms {
                realms.push(Realm::tokio_read(&mut r).await?);
            }
            realms
        };

        // footer_padding: u16
        let _footer_padding = crate::util::tokio_read_u16_le(&mut r).await?;
        // footer_padding is expected to always be 0 (0)

        Ok(Self {
            realms,
        })
    }

    #[cfg(feature = "async-std")]
    async fn astd_read_inner<R: async_std::io::ReadExt + Unpin + Send>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // size: u16
        let _size = crate::util::astd_read_u16_le(&mut r).await?;
        // size is expected to always be self.size (0)

        // header_padding: u32
        let _header_padding = crate::util::astd_read_u32_le(&mut r).await?;
        // header_padding is expected to always be 0 (0)

        // number_of_realms: u8
        let number_of_realms = crate::util::astd_read_u8_le(&mut r).await?;

        // realms: Realm[number_of_realms]
        let realms = {
            let mut realms = Vec::with_capacity(number_of_realms as usize);
            for _ in 0..number_of_realms {
                realms.push(Realm::astd_read(&mut r).await?);
            }
            realms
        };

        // footer_padding: u16
        let _footer_padding = crate::util::astd_read_u16_le(&mut r).await?;
        // footer_padding is expected to always be 0 (0)

        Ok(Self {
            realms,
        })
    }

}

impl ServerMessage for CMD_REALM_LIST_Server {
    const OPCODE: u8 = 0x10;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;

        let mut s = String::new();

        writeln!(s, "test CMD_REALM_LIST_Server {{").unwrap();
        // Members
        writeln!(s, "    number_of_realms = {};", self.realms.len()).unwrap();
        write!(s, "    realms = [").unwrap();
        for v in self.realms.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        realm_type = {};", v.realm_type.as_test_case_value()).unwrap();
            writeln!(s, "        locked = {};", if v.locked { "TRUE" } else { "FALSE" }).unwrap();
            writeln!(s, "        flag = {};", v.flag.as_test_case_value()).unwrap();
            writeln!(s, "        name = \"{}\";", v.name).unwrap();
            writeln!(s, "        address = \"{}\";", v.address).unwrap();
            writeln!(s, "        population = {};", v.population.as_test_case_value()).unwrap();
            writeln!(s, "        number_of_characters_on_realm = {};", v.number_of_characters_on_realm).unwrap();
            writeln!(s, "        category = {};", v.category.as_test_case_value()).unwrap();
            writeln!(s, "        realm_id = {};", v.realm_id).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        writeln!(s, "    {:#04X}, /* opcode */ ", bytes.next().unwrap()).unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, 2, "size", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "header_padding", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "number_of_realms", "    ");
        if !self.realms.is_empty() {
            writeln!(s, "    /* realms: Realm[number_of_realms] start */").unwrap();
            for (i, v) in self.realms.iter().enumerate() {
                writeln!(s, "    /* realms: Realm[number_of_realms] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 1, "realm_type", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "locked", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "flag", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, v.name.len() + 1, "name", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, v.address.len() + 1, "address", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "population", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "number_of_characters_on_realm", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "category", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "realm_id", "        ");
                writeln!(s, "    /* realms: Realm[number_of_realms] {i} end */").unwrap();
            }
            writeln!(s, "    /* realms: Realm[number_of_realms] end */").unwrap();
        }
        crate::util::write_bytes(&mut s, &mut bytes, 2, "footer_padding", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    login_versions = \"{}\";", std::env::var("WOWM_TEST_CASE_LOGIN_VERSION").unwrap_or("5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    #[cfg(feature = "sync")]
    fn read<R: Read, I: crate::private::Sealed>(r: R) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r).map_err(|kind| crate::errors::ParseError::new(16, "CMD_REALM_LIST_Server", kind))
    }

    #[cfg(feature = "sync")]
    fn write<W: Write>(&self, mut w: W) -> Result<(), std::io::Error> {
        let mut v = Vec::with_capacity(self.size() + 1);
        self.write_into_vec(&mut v)?;
        w.write_all(&v)
    }

    #[cfg(feature = "tokio")]
    fn tokio_read<'async_trait, R, I: crate::private::Sealed>(
        r: R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = Result<Self, crate::errors::ParseError>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + tokio::io::AsyncReadExt + Unpin + Send,
        Self: 'async_trait,
     {
        Box::pin(async move {Self::tokio_read_inner(r).await.map_err(|kind| crate::errors::ParseError::new(16, "CMD_REALM_LIST_Server", kind))})
    }

    #[cfg(feature = "tokio")]
    fn tokio_write<'life0, 'async_trait, W>(
        &'life0 self,
        mut w: W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let mut v = Vec::with_capacity(self.size() + 1);
            self.write_into_vec(&mut v)?;
            w.write_all(&v).await
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_read<'async_trait, R, I: crate::private::Sealed>(
        r: R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = Result<Self, crate::errors::ParseError>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + async_std::io::ReadExt + Unpin + Send,
        Self: 'async_trait,
     {
        Box::pin(async move {Self::astd_read_inner(r).await.map_err(|kind| crate::errors::ParseError::new(16, "CMD_REALM_LIST_Server", kind))})
    }

    #[cfg(feature = "async-std")]
    fn astd_write<'life0, 'async_trait, W>(
        &'life0 self,
        mut w: W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let mut v = Vec::with_capacity(self.size() + 1);
            self.write_into_vec(&mut v)?;
            w.write_all(&v).await
        })
    }

}

impl CMD_REALM_LIST_Server {
    pub(crate) fn size(&self) -> usize {
        2 // size: u16
        + 4 // header_padding: u32
        + 1 // number_of_realms: u8
        + self.realms.iter().fold(0, |acc, x| acc + x.size()) // realms: Realm[number_of_realms]
        + 2 // footer_padding: u16
    }
}

