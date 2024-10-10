use crate::Message;
use crate::ClientMessage;
use std::io::{Read, Write};

use crate::logon::all::{
    Locale, Os, Platform, ProtocolVersion, Version,
};
use std::net::Ipv4Addr;

/// First message sent by the client when attempting to reconnect. The server will respond with [`CMD_AUTH_RECONNECT_CHALLENGE_Server`](crate::logon::version_2::CMD_AUTH_RECONNECT_CHALLENGE_Server).
/// Has the exact same layout as [`CMD_AUTH_LOGON_CHALLENGE_Client`](crate::logon::all::CMD_AUTH_LOGON_CHALLENGE_Client).
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_auth_reconnect/challenge_client.wowm:5`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_auth_reconnect/challenge_client.wowm#L5):
/// ```text
/// clogin CMD_AUTH_RECONNECT_CHALLENGE_Client = 0x02 {
///     ProtocolVersion protocol_version;
///     u16 size = self.size;
///     u32 game_name = "\0WoW";
///     Version version;
///     Platform platform;
///     Os os;
///     Locale locale;
///     i32 utc_timezone_offset;
///     IpAddress client_ip_address;
///     String account_name;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CMD_AUTH_RECONNECT_CHALLENGE_Client {
    /// Determines which version of messages are used for further communication.
    pub protocol_version: ProtocolVersion,
    pub version: Version,
    pub platform: Platform,
    pub os: Os,
    pub locale: Locale,
    /// Offset in minutes from UTC time. 180 would be UTC+3
    pub utc_timezone_offset: i32,
    pub client_ip_address: Ipv4Addr,
    /// Real clients can send a maximum of 16 UTF-8 characters. This is not necessarily 16 bytes since one character can be more than one byte.
    /// Real clients will send a fully uppercased username, and will perform authentication calculations on the uppercased version.
    /// Uppercasing in regards to non-ASCII values is little weird. See `https://docs.rs/wow_srp/latest/wow_srp/normalized_string/index.html` for more info.
    pub account_name: String,
}

impl CMD_AUTH_RECONNECT_CHALLENGE_Client {
    /// The field `game_name` is constantly specified to be:
    ///
    /// | Format | Value |
    /// | ------ | ----- |
    /// | Decimal | `5730135` |
    /// | Hex | `0x576f57` |
    /// | Original | `"\0WoW"` |
    ///
    /// **This field is not in the Rust struct, but is written as this constant value.**
    pub const GAME_NAME_VALUE: u32 = 0x576f57;

}

impl CMD_AUTH_RECONNECT_CHALLENGE_Client {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        // protocol_version: ProtocolVersion
        w.write_all(&(self.protocol_version.as_int().to_le_bytes()))?;

        // size: u16
        w.write_all(&((self.size() - 3) as u16).to_le_bytes())?;

        // game_name: u32
        w.write_all(&Self::GAME_NAME_VALUE.to_le_bytes())?;

        // version: Version
        self.version.write_into_vec(&mut w)?;

        // platform: Platform
        w.write_all(&(self.platform.as_int().to_le_bytes()))?;

        // os: Os
        w.write_all(&(self.os.as_int().to_le_bytes()))?;

        // locale: Locale
        w.write_all(&(self.locale.as_int().to_le_bytes()))?;

        // utc_timezone_offset: i32
        w.write_all(&self.utc_timezone_offset.to_le_bytes())?;

        // client_ip_address: IpAddress
        w.write_all(&self.client_ip_address.octets())?;

        // account_name: String
        w.write_all(&(self.account_name.len() as u8).to_le_bytes())?;
        w.write_all(self.account_name.as_bytes())?;

        Ok(())
    }
}

impl crate::private::Sealed for CMD_AUTH_RECONNECT_CHALLENGE_Client {}

impl CMD_AUTH_RECONNECT_CHALLENGE_Client {
    #[cfg(feature = "sync")]
    fn read_inner<R: std::io::Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // protocol_version: ProtocolVersion
        let protocol_version = crate::util::read_u8_le(&mut r)?.try_into()?;

        // size: u16
        let _size = crate::util::read_u16_le(&mut r)?;
        // size is dynamic size of the object

        // game_name: u32
        let _game_name = crate::util::read_u32_le(&mut r)?;
        // game_name is expected to always be "\0WoW" (5730135)

        // version: Version
        let version = Version::read(&mut r)?;

        // platform: Platform
        let platform = crate::util::read_u32_le(&mut r)?.try_into()?;

        // os: Os
        let os = crate::util::read_u32_le(&mut r)?.try_into()?;

        // locale: Locale
        let locale = crate::util::read_u32_le(&mut r)?.try_into()?;

        // utc_timezone_offset: i32
        let utc_timezone_offset = crate::util::read_i32_le(&mut r)?;

        // client_ip_address: IpAddress
        let client_ip_address = Ipv4Addr::from(crate::util::read_u32_be(&mut r)?);

        // account_name: String
        let account_name = {
            let account_name = crate::util::read_u8_le(&mut r)?;
            let account_name = crate::util::read_fixed_string_to_vec(&mut r, account_name as usize)?;
            String::from_utf8(account_name)?
        };

        Ok(Self {
            protocol_version,
            version,
            platform,
            os,
            locale,
            utc_timezone_offset,
            client_ip_address,
            account_name,
        })
    }

    #[cfg(feature = "tokio")]
    async fn tokio_read_inner<R: tokio::io::AsyncReadExt + Unpin + Send>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // protocol_version: ProtocolVersion
        let protocol_version = crate::util::tokio_read_u8_le(&mut r).await?.try_into()?;

        // size: u16
        let _size = crate::util::tokio_read_u16_le(&mut r).await?;
        // size is dynamic size of the object

        // game_name: u32
        let _game_name = crate::util::tokio_read_u32_le(&mut r).await?;
        // game_name is expected to always be "\0WoW" (5730135)

        // version: Version
        let version = Version::tokio_read(&mut r).await?;

        // platform: Platform
        let platform = crate::util::tokio_read_u32_le(&mut r).await?.try_into()?;

        // os: Os
        let os = crate::util::tokio_read_u32_le(&mut r).await?.try_into()?;

        // locale: Locale
        let locale = crate::util::tokio_read_u32_le(&mut r).await?.try_into()?;

        // utc_timezone_offset: i32
        let utc_timezone_offset = crate::util::tokio_read_i32_le(&mut r).await?;

        // client_ip_address: IpAddress
        let client_ip_address = Ipv4Addr::from(crate::util::tokio_read_u32_be(&mut r).await?);

        // account_name: String
        let account_name = {
            let account_name = crate::util::tokio_read_u8_le(&mut r).await?;
            let account_name = crate::util::tokio_read_fixed_string_to_vec(&mut r, account_name as usize).await?;
            String::from_utf8(account_name)?
        };

        Ok(Self {
            protocol_version,
            version,
            platform,
            os,
            locale,
            utc_timezone_offset,
            client_ip_address,
            account_name,
        })
    }

    #[cfg(feature = "async-std")]
    async fn astd_read_inner<R: async_std::io::ReadExt + Unpin + Send>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // protocol_version: ProtocolVersion
        let protocol_version = crate::util::astd_read_u8_le(&mut r).await?.try_into()?;

        // size: u16
        let _size = crate::util::astd_read_u16_le(&mut r).await?;
        // size is dynamic size of the object

        // game_name: u32
        let _game_name = crate::util::astd_read_u32_le(&mut r).await?;
        // game_name is expected to always be "\0WoW" (5730135)

        // version: Version
        let version = Version::astd_read(&mut r).await?;

        // platform: Platform
        let platform = crate::util::astd_read_u32_le(&mut r).await?.try_into()?;

        // os: Os
        let os = crate::util::astd_read_u32_le(&mut r).await?.try_into()?;

        // locale: Locale
        let locale = crate::util::astd_read_u32_le(&mut r).await?.try_into()?;

        // utc_timezone_offset: i32
        let utc_timezone_offset = crate::util::astd_read_i32_le(&mut r).await?;

        // client_ip_address: IpAddress
        let client_ip_address = Ipv4Addr::from(crate::util::astd_read_u32_be(&mut r).await?);

        // account_name: String
        let account_name = {
            let account_name = crate::util::astd_read_u8_le(&mut r).await?;
            let account_name = crate::util::astd_read_fixed_string_to_vec(&mut r, account_name as usize).await?;
            String::from_utf8(account_name)?
        };

        Ok(Self {
            protocol_version,
            version,
            platform,
            os,
            locale,
            utc_timezone_offset,
            client_ip_address,
            account_name,
        })
    }

}

impl Message for CMD_AUTH_RECONNECT_CHALLENGE_Client {
    const OPCODE: u8 = 0x02;

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read, I: crate::private::Sealed>(r: R) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r).map_err(|kind| crate::errors::ParseError::new(2, "CMD_AUTH_RECONNECT_CHALLENGE_Client", kind))
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
        Box::pin(async move {Self::tokio_read_inner(r).await.map_err(|kind| crate::errors::ParseError::new(2, "CMD_AUTH_RECONNECT_CHALLENGE_Client", kind))})
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
        Box::pin(async move {Self::astd_read_inner(r).await.map_err(|kind| crate::errors::ParseError::new(2, "CMD_AUTH_RECONNECT_CHALLENGE_Client", kind))})
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

impl ClientMessage for CMD_AUTH_RECONNECT_CHALLENGE_Client {}
impl CMD_AUTH_RECONNECT_CHALLENGE_Client {
    pub(crate) fn size(&self) -> usize {
        1 // protocol_version: ProtocolVersion
        + 2 // size: u16
        + 4 // game_name: u32
        + 5 // version: Version
        + 4 // platform: Platform
        + 4 // os: Os
        + 4 // locale: Locale
        + 4 // utc_timezone_offset: i32
        + 4 // client_ip_address: IpAddress
        + self.account_name.len() + 1 // account_name: String
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMD_AUTH_RECONNECT_CHALLENGE_Client;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_3::opcodes::ClientOpcodeMessage;

    const HEADER_SIZE: usize = 1;
    const RAW0: [u8; 35] = [ 0x02, 0x02, 0x1F, 0x00, 0x57, 0x6F, 0x57, 0x00, 0x01,
         0x0C, 0x01, 0xF3, 0x16, 0x36, 0x38, 0x78, 0x00, 0x6E, 0x69, 0x57, 0x00,
         0x42, 0x47, 0x6E, 0x65, 0x3C, 0x00, 0x00, 0x00, 0x7F, 0x00, 0x00, 0x01,
         0x01, 0x41, ];

    pub(crate) fn expected0() -> CMD_AUTH_RECONNECT_CHALLENGE_Client {
        CMD_AUTH_RECONNECT_CHALLENGE_Client {
            protocol_version: ProtocolVersion::Two,
            version: Version {
                major: 0x1,
                minor: 0xC,
                patch: 0x1,
                build: 0x16F3,
            },
            platform: Platform::X86,
            os: Os::Windows,
            locale: Locale::EnGb,
            utc_timezone_offset: 0x3C,
            client_ip_address: Ipv4Addr::from(0x7F000001_u32),
            account_name: String::from("A"),
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/challenge_client.wowm` line 23.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_auth_reconnect_challenge_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_RECONNECT_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_CHALLENGE, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/challenge_client.wowm` line 23.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_auth_reconnect_challenge_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_RECONNECT_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_CHALLENGE, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/challenge_client.wowm` line 23.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_auth_reconnect_challenge_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_RECONNECT_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_CHALLENGE, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    const RAW1: [u8; 50] = [ 0x02, 0x02, 0x2E, 0x00, 0x57, 0x6F, 0x57, 0x00, 0x01,
         0x0C, 0x01, 0xF3, 0x16, 0x36, 0x38, 0x78, 0x00, 0x6E, 0x69, 0x57, 0x00,
         0x42, 0x47, 0x6E, 0x65, 0x3C, 0x00, 0x00, 0x00, 0x7F, 0x00, 0x00, 0x01,
         0x10, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4A, 0x4B,
         0x4C, 0x4D, 0x4E, 0x4F, 0x50, ];

    pub(crate) fn expected1() -> CMD_AUTH_RECONNECT_CHALLENGE_Client {
        CMD_AUTH_RECONNECT_CHALLENGE_Client {
            protocol_version: ProtocolVersion::Two,
            version: Version {
                major: 0x1,
                minor: 0xC,
                patch: 0x1,
                build: 0x16F3,
            },
            platform: Platform::X86,
            os: Os::Windows,
            locale: Locale::EnGb,
            utc_timezone_offset: 0x3C,
            client_ip_address: Ipv4Addr::from(0x7F000001_u32),
            account_name: String::from("ABCDEFGHIJKLMNOP"),
        }

    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/challenge_client.wowm` line 54.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmd_auth_reconnect_challenge_client1() {
        let expected = expected1();
        let t = ClientOpcodeMessage::read(&mut std::io::Cursor::new(&RAW1)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_RECONNECT_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_CHALLENGE, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/challenge_client.wowm` line 54.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmd_auth_reconnect_challenge_client1() {
        let expected = expected1();
        let t = ClientOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_RECONNECT_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_CHALLENGE, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_reconnect/challenge_client.wowm` line 54.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmd_auth_reconnect_challenge_client1() {
        let expected = expected1();
        let t = ClientOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_RECONNECT_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_RECONNECT_CHALLENGE, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

}

