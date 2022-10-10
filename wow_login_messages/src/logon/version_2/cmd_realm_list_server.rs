use std::convert::{TryFrom, TryInto};
use crate::logon::version_2::Realm;
use crate::ServerMessage;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_realm/server.wowm:66`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_realm/server.wowm#L66):
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
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
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
            i.write_into_vec(w)?;
        }

        // footer_padding: u16
        w.write_all(&Self::FOOTER_PADDING_VALUE.to_le_bytes())?;

        Ok(())
    }
}

impl ServerMessage for CMD_REALM_LIST_Server {
    const OPCODE: u8 = 0x10;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // size: u16
        let _size = crate::util::read_u16_le(r)?;
        // size is expected to always be self.size (0)

        // header_padding: u32
        let _header_padding = crate::util::read_u32_le(r)?;
        // header_padding is expected to always be 0 (0)

        // number_of_realms: u8
        let number_of_realms = crate::util::read_u8_le(r)?;

        // realms: Realm[number_of_realms]
        let mut realms = Vec::with_capacity(number_of_realms as usize);
        for i in 0..number_of_realms {
            let o = Realm::read(r)?;
            realms.push(o);
        }

        // footer_padding: u16
        let _footer_padding = crate::util::read_u16_le(r)?;
        // footer_padding is expected to always be 0 (0)

        Ok(Self {
            realms,
        })
    }

    #[cfg(feature = "sync")]
    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let mut v = Vec::with_capacity(self.size() + 1);
        self.write_into_vec(&mut v)?;
        w.write_all(&v)
    }

    #[cfg(feature = "tokio")]
    fn tokio_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, crate::errors::ParseError>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + tokio::io::AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // size: u16
            let _size = crate::util::tokio_read_u16_le(r).await?;
            // size is expected to always be self.size (0)

            // header_padding: u32
            let _header_padding = crate::util::tokio_read_u32_le(r).await?;
            // header_padding is expected to always be 0 (0)

            // number_of_realms: u8
            let number_of_realms = crate::util::tokio_read_u8_le(r).await?;

            // realms: Realm[number_of_realms]
            let mut realms = Vec::with_capacity(number_of_realms as usize);
            for i in 0..number_of_realms {
                let o = Realm::tokio_read(r).await?;
                realms.push(o);
            }

            // footer_padding: u16
            let _footer_padding = crate::util::tokio_read_u16_le(r).await?;
            // footer_padding is expected to always be 0 (0)

            Ok(Self {
                realms,
            })
        })
    }

    #[cfg(feature = "tokio")]
    fn tokio_write<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let mut v = Vec::with_capacity(self.size() + 1);
            self.write_into_vec(&mut v)?;
            w.write_all(&v).await
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_read<'life0, 'async_trait, R>(
        r: &'life0 mut R,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, crate::errors::ParseError>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + async_std::io::ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // size: u16
            let _size = crate::util::astd_read_u16_le(r).await?;
            // size is expected to always be self.size (0)

            // header_padding: u32
            let _header_padding = crate::util::astd_read_u32_le(r).await?;
            // header_padding is expected to always be 0 (0)

            // number_of_realms: u8
            let number_of_realms = crate::util::astd_read_u8_le(r).await?;

            // realms: Realm[number_of_realms]
            let mut realms = Vec::with_capacity(number_of_realms as usize);
            for i in 0..number_of_realms {
                let o = Realm::astd_read(r).await?;
                realms.push(o);
            }

            // footer_padding: u16
            let _footer_padding = crate::util::astd_read_u16_le(r).await?;
            // footer_padding is expected to always be 0 (0)

            Ok(Self {
                realms,
            })
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
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

#[cfg(test)]
mod testversion_2 {
    use super::CMD_REALM_LIST_Server;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_2::opcodes::ServerOpcodeMessage;

    const RAW0: [u8; 26] = [ 0x10, 0x17, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x41, 0x00, 0x41, 0x00, 0x00, 0x00, 0xC8, 0x43,
         0x01, 0x00, 0x02, 0x00, 0x00, ];

    // Generated from `wow_message_parser/wowm/login/cmd_realm/server.wowm` line 76.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_REALM_LIST_Server0() {
        let expected = CMD_REALM_LIST_Server {
            realms: vec![
                Realm {
                    realm_type: RealmType::PlayerVsEnvironment,
                    flag: RealmFlag::empty()
                        ,
                    name: String::from("A"),
                    address: String::from("A"),
                    population: Population::RedFull,
                    number_of_characters_on_realm: 0x1,
                    category: RealmCategory::Default,
                    realm_id: 0x2,
                },
            ],
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_REALM_LIST(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_REALM_LIST, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.realms, expected.realms);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_realm/server.wowm` line 76.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_REALM_LIST_Server0() {
        let expected = CMD_REALM_LIST_Server {
            realms: vec![
                Realm {
                    realm_type: RealmType::PlayerVsEnvironment,
                    flag: RealmFlag::empty()
                        ,
                    name: String::from("A"),
                    address: String::from("A"),
                    population: Population::RedFull,
                    number_of_characters_on_realm: 0x1,
                    category: RealmCategory::Default,
                    realm_id: 0x2,
                },
            ],
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_REALM_LIST(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_REALM_LIST, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.realms, expected.realms);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_realm/server.wowm` line 76.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_REALM_LIST_Server0() {
        let expected = CMD_REALM_LIST_Server {
            realms: vec![
                Realm {
                    realm_type: RealmType::PlayerVsEnvironment,
                    flag: RealmFlag::empty()
                        ,
                    name: String::from("A"),
                    address: String::from("A"),
                    population: Population::RedFull,
                    number_of_characters_on_realm: 0x1,
                    category: RealmCategory::Default,
                    realm_id: 0x2,
                },
            ],
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_REALM_LIST(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_REALM_LIST, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.realms, expected.realms);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    const RAW1: [u8; 26] = [ 0x10, 0x17, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00,
         0x00, 0x00, 0x00, 0x03, 0x41, 0x00, 0x41, 0x00, 0x00, 0x00, 0xC8, 0x43,
         0x01, 0x00, 0x02, 0x00, 0x00, ];

    // Generated from `wow_message_parser/wowm/login/cmd_realm/server.wowm` line 107.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_REALM_LIST_Server1() {
        let expected = CMD_REALM_LIST_Server {
            realms: vec![
                Realm {
                    realm_type: RealmType::PlayerVsEnvironment,
                    flag: RealmFlag::empty()
                        .set_INVALID()
                        .set_OFFLINE()
                        ,
                    name: String::from("A"),
                    address: String::from("A"),
                    population: Population::RedFull,
                    number_of_characters_on_realm: 0x1,
                    category: RealmCategory::Default,
                    realm_id: 0x2,
                },
            ],
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW1)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_REALM_LIST(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_REALM_LIST, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.realms, expected.realms);

        assert_eq!(t.size() + header_size, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_realm/server.wowm` line 107.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_REALM_LIST_Server1() {
        let expected = CMD_REALM_LIST_Server {
            realms: vec![
                Realm {
                    realm_type: RealmType::PlayerVsEnvironment,
                    flag: RealmFlag::empty()
                        .set_INVALID()
                        .set_OFFLINE()
                        ,
                    name: String::from("A"),
                    address: String::from("A"),
                    population: Population::RedFull,
                    number_of_characters_on_realm: 0x1,
                    category: RealmCategory::Default,
                    realm_id: 0x2,
                },
            ],
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_REALM_LIST(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_REALM_LIST, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.realms, expected.realms);

        assert_eq!(t.size() + header_size, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_realm/server.wowm` line 107.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_REALM_LIST_Server1() {
        let expected = CMD_REALM_LIST_Server {
            realms: vec![
                Realm {
                    realm_type: RealmType::PlayerVsEnvironment,
                    flag: RealmFlag::empty()
                        .set_INVALID()
                        .set_OFFLINE()
                        ,
                    name: String::from("A"),
                    address: String::from("A"),
                    population: Population::RedFull,
                    number_of_characters_on_realm: 0x1,
                    category: RealmCategory::Default,
                    realm_id: 0x2,
                },
            ],
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_REALM_LIST(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_REALM_LIST, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.realms, expected.realms);

        assert_eq!(t.size() + header_size, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

}

#[cfg(test)]
mod testversion_3 {
    use super::CMD_REALM_LIST_Server;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_3::opcodes::ServerOpcodeMessage;

    const RAW0: [u8; 26] = [ 0x10, 0x17, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x41, 0x00, 0x41, 0x00, 0x00, 0x00, 0xC8, 0x43,
         0x01, 0x00, 0x02, 0x00, 0x00, ];

    // Generated from `wow_message_parser/wowm/login/cmd_realm/server.wowm` line 76.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_REALM_LIST_Server0() {
        let expected = CMD_REALM_LIST_Server {
            realms: vec![
                Realm {
                    realm_type: RealmType::PlayerVsEnvironment,
                    flag: RealmFlag::empty()
                        ,
                    name: String::from("A"),
                    address: String::from("A"),
                    population: Population::RedFull,
                    number_of_characters_on_realm: 0x1,
                    category: RealmCategory::Default,
                    realm_id: 0x2,
                },
            ],
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_REALM_LIST(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_REALM_LIST, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.realms, expected.realms);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_realm/server.wowm` line 76.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_REALM_LIST_Server0() {
        let expected = CMD_REALM_LIST_Server {
            realms: vec![
                Realm {
                    realm_type: RealmType::PlayerVsEnvironment,
                    flag: RealmFlag::empty()
                        ,
                    name: String::from("A"),
                    address: String::from("A"),
                    population: Population::RedFull,
                    number_of_characters_on_realm: 0x1,
                    category: RealmCategory::Default,
                    realm_id: 0x2,
                },
            ],
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_REALM_LIST(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_REALM_LIST, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.realms, expected.realms);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_realm/server.wowm` line 76.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_REALM_LIST_Server0() {
        let expected = CMD_REALM_LIST_Server {
            realms: vec![
                Realm {
                    realm_type: RealmType::PlayerVsEnvironment,
                    flag: RealmFlag::empty()
                        ,
                    name: String::from("A"),
                    address: String::from("A"),
                    population: Population::RedFull,
                    number_of_characters_on_realm: 0x1,
                    category: RealmCategory::Default,
                    realm_id: 0x2,
                },
            ],
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_REALM_LIST(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_REALM_LIST, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.realms, expected.realms);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    const RAW1: [u8; 26] = [ 0x10, 0x17, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00,
         0x00, 0x00, 0x00, 0x03, 0x41, 0x00, 0x41, 0x00, 0x00, 0x00, 0xC8, 0x43,
         0x01, 0x00, 0x02, 0x00, 0x00, ];

    // Generated from `wow_message_parser/wowm/login/cmd_realm/server.wowm` line 107.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_REALM_LIST_Server1() {
        let expected = CMD_REALM_LIST_Server {
            realms: vec![
                Realm {
                    realm_type: RealmType::PlayerVsEnvironment,
                    flag: RealmFlag::empty()
                        .set_INVALID()
                        .set_OFFLINE()
                        ,
                    name: String::from("A"),
                    address: String::from("A"),
                    population: Population::RedFull,
                    number_of_characters_on_realm: 0x1,
                    category: RealmCategory::Default,
                    realm_id: 0x2,
                },
            ],
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::read(&mut std::io::Cursor::new(&RAW1)).unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_REALM_LIST(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_REALM_LIST, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.realms, expected.realms);

        assert_eq!(t.size() + header_size, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_realm/server.wowm` line 107.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_REALM_LIST_Server1() {
        let expected = CMD_REALM_LIST_Server {
            realms: vec![
                Realm {
                    realm_type: RealmType::PlayerVsEnvironment,
                    flag: RealmFlag::empty()
                        .set_INVALID()
                        .set_OFFLINE()
                        ,
                    name: String::from("A"),
                    address: String::from("A"),
                    population: Population::RedFull,
                    number_of_characters_on_realm: 0x1,
                    category: RealmCategory::Default,
                    realm_id: 0x2,
                },
            ],
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_REALM_LIST(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_REALM_LIST, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.realms, expected.realms);

        assert_eq!(t.size() + header_size, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_realm/server.wowm` line 107.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_REALM_LIST_Server1() {
        let expected = CMD_REALM_LIST_Server {
            realms: vec![
                Realm {
                    realm_type: RealmType::PlayerVsEnvironment,
                    flag: RealmFlag::empty()
                        .set_INVALID()
                        .set_OFFLINE()
                        ,
                    name: String::from("A"),
                    address: String::from("A"),
                    population: Population::RedFull,
                    number_of_characters_on_realm: 0x1,
                    category: RealmCategory::Default,
                    realm_id: 0x2,
                },
            ],
        };

        let header_size = 1;
        let t = ServerOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::CMD_REALM_LIST(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_REALM_LIST, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.realms, expected.realms);

        assert_eq!(t.size() + header_size, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

}

