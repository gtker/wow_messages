use crate::Message;
use crate::ServerMessage;
use std::io::{Read, Write};

use crate::logon::version_5::Realm;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_realm/server.wowm:151`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_realm/server.wowm#L151):
/// ```text
/// slogin CMD_REALM_LIST_Server = 0x10 {
///     u16 size = self.size;
///     u32 header_padding = 0;
///     u16 number_of_realms;
///     Realm[number_of_realms] realms;
///     u16 footer_padding = 0;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
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

        // number_of_realms: u16
        w.write_all(&(self.realms.len() as u16).to_le_bytes())?;

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
    fn read_inner<R: std::io::Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // size: u16
        let _size = crate::util::read_u16_le(&mut r)?;
        // size is dynamic size of the object

        // header_padding: u32
        let _header_padding = crate::util::read_u32_le(&mut r)?;
        // header_padding is expected to always be 0 (0)

        // number_of_realms: u16
        let number_of_realms = crate::util::read_u16_le(&mut r)?;

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
        // size is dynamic size of the object

        // header_padding: u32
        let _header_padding = crate::util::tokio_read_u32_le(&mut r).await?;
        // header_padding is expected to always be 0 (0)

        // number_of_realms: u16
        let number_of_realms = crate::util::tokio_read_u16_le(&mut r).await?;

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
        // size is dynamic size of the object

        // header_padding: u32
        let _header_padding = crate::util::astd_read_u32_le(&mut r).await?;
        // header_padding is expected to always be 0 (0)

        // number_of_realms: u16
        let number_of_realms = crate::util::astd_read_u16_le(&mut r).await?;

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

impl Message for CMD_REALM_LIST_Server {
    const OPCODE: u8 = 0x10;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read, I: crate::private::Sealed>(r: R) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r).map_err(|kind| crate::errors::ParseError::new(16, "CMD_REALM_LIST_Server", kind))
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, mut w: W) -> Result<(), std::io::Error> {
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

impl ServerMessage for CMD_REALM_LIST_Server {}
impl CMD_REALM_LIST_Server {
    pub(crate) fn size(&self) -> usize {
        2 // size: u16
        + 4 // header_padding: u32
        + 2 // number_of_realms: u16
        + self.realms.iter().fold(0, |acc, x| acc + x.size()) // realms: Realm[number_of_realms]
        + 2 // footer_padding: u16
    }
}

#[cfg(test)]
mod test_version_6 {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMD_REALM_LIST_Server;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_6::opcodes::ServerOpcodeMessage;

    const HEADER_SIZE: usize = 1;
    const RAW0: [u8; 84] = [ 0x10, 0x51, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00,
         0x00, 0x00, 0x00, 0x54, 0x65, 0x73, 0x74, 0x20, 0x52, 0x65, 0x61, 0x6C,
         0x6D, 0x32, 0x00, 0x6C, 0x6F, 0x63, 0x61, 0x6C, 0x68, 0x6F, 0x73, 0x74,
         0x3A, 0x38, 0x30, 0x38, 0x35, 0x00, 0x00, 0x00, 0x48, 0x43, 0x03, 0x01,
         0x01, 0x00, 0x00, 0x00, 0x54, 0x65, 0x73, 0x74, 0x20, 0x52, 0x65, 0x61,
         0x6C, 0x6D, 0x00, 0x6C, 0x6F, 0x63, 0x61, 0x6C, 0x68, 0x6F, 0x73, 0x74,
         0x3A, 0x38, 0x30, 0x38, 0x35, 0x00, 0x00, 0x00, 0x48, 0x43, 0x03, 0x02,
         0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> CMD_REALM_LIST_Server {
        CMD_REALM_LIST_Server {
            realms: vec![
                Realm {
                    realm_type: RealmType::PlayerVsEnvironment,
                    locked: false, 
                    flag: RealmFlag::empty()
                        ,
                    name: String::from("Test Realm2"),
                    address: String::from("localhost:8085"),
                    population: Population::GreenRecommended,
                    number_of_characters_on_realm: 0x3,
                    category: RealmCategory::One,
                    realm_id: 0x1,
                },
                Realm {
                    realm_type: RealmType::PlayerVsEnvironment,
                    locked: false, 
                    flag: RealmFlag::empty()
                        ,
                    name: String::from("Test Realm"),
                    address: String::from("localhost:8085"),
                    population: Population::GreenRecommended,
                    number_of_characters_on_realm: 0x3,
                    category: RealmCategory::Two,
                    realm_id: 0x0,
                },
            ],
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_realm/server.wowm` line 161.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_realm_list_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_REALM_LIST(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_REALM_LIST, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_realm/server.wowm` line 161.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_realm_list_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_REALM_LIST(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_REALM_LIST, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_realm/server.wowm` line 161.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_realm_list_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_REALM_LIST(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_REALM_LIST, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(test)]
mod test_version_7 {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMD_REALM_LIST_Server;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_7::opcodes::ServerOpcodeMessage;

    const HEADER_SIZE: usize = 1;
    const RAW0: [u8; 84] = [ 0x10, 0x51, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02, 0x00,
         0x00, 0x00, 0x00, 0x54, 0x65, 0x73, 0x74, 0x20, 0x52, 0x65, 0x61, 0x6C,
         0x6D, 0x32, 0x00, 0x6C, 0x6F, 0x63, 0x61, 0x6C, 0x68, 0x6F, 0x73, 0x74,
         0x3A, 0x38, 0x30, 0x38, 0x35, 0x00, 0x00, 0x00, 0x48, 0x43, 0x03, 0x01,
         0x01, 0x00, 0x00, 0x00, 0x54, 0x65, 0x73, 0x74, 0x20, 0x52, 0x65, 0x61,
         0x6C, 0x6D, 0x00, 0x6C, 0x6F, 0x63, 0x61, 0x6C, 0x68, 0x6F, 0x73, 0x74,
         0x3A, 0x38, 0x30, 0x38, 0x35, 0x00, 0x00, 0x00, 0x48, 0x43, 0x03, 0x02,
         0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> CMD_REALM_LIST_Server {
        CMD_REALM_LIST_Server {
            realms: vec![
                Realm {
                    realm_type: RealmType::PlayerVsEnvironment,
                    locked: false, 
                    flag: RealmFlag::empty()
                        ,
                    name: String::from("Test Realm2"),
                    address: String::from("localhost:8085"),
                    population: Population::GreenRecommended,
                    number_of_characters_on_realm: 0x3,
                    category: RealmCategory::One,
                    realm_id: 0x1,
                },
                Realm {
                    realm_type: RealmType::PlayerVsEnvironment,
                    locked: false, 
                    flag: RealmFlag::empty()
                        ,
                    name: String::from("Test Realm"),
                    address: String::from("localhost:8085"),
                    population: Population::GreenRecommended,
                    number_of_characters_on_realm: 0x3,
                    category: RealmCategory::Two,
                    realm_id: 0x0,
                },
            ],
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_realm/server.wowm` line 161.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_realm_list_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_REALM_LIST(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_REALM_LIST, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_realm/server.wowm` line 161.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_realm_list_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_REALM_LIST(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_REALM_LIST, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_realm/server.wowm` line 161.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_realm_list_server0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_REALM_LIST(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_REALM_LIST, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

