use crate::logon::all::Version;
use crate::logon::all::Locale;
use crate::logon::all::Os;
use crate::logon::all::Platform;
use crate::ClientMessage;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// First message sent by the client when attempting to connect. The server will respond with [`CMD_AUTH_LOGON_CHALLENGE_Server`](crate::logon::version_2::CMD_AUTH_LOGON_CHALLENGE_Server).
///
/// Has the exact same layout as [`CMD_AUTH_RECONNECT_CHALLENGE_Client`](crate::logon::all::CMD_AUTH_RECONNECT_CHALLENGE_Client).
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_auth_logon/challenge_client.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_auth_logon/challenge_client.wowm#L3):
/// ```text
/// clogin CMD_AUTH_LOGON_CHALLENGE_Client = 0x00 {
///     u8 protocol_version;
///     u16 size = self.size;
///     u32 game_name = "\0WoW";
///     Version version;
///     Platform platform;
///     Os os;
///     Locale locale;
///     u32 utc_timezone_offset;
///     u32_be client_ip_address;
///     String account_name;
/// }
/// ```
pub struct CMD_AUTH_LOGON_CHALLENGE_Client {
    /// Determines which version of messages are used for further communication.
    ///
    pub protocol_version: u8,
    pub version: Version,
    pub platform: Platform,
    pub os: Os,
    pub locale: Locale,
    /// Offset in minutes from UTC time. 180 would be UTC+3
    ///
    pub utc_timezone_offset: u32,
    pub client_ip_address: u32,
    /// Real clients can send a maximum of 16 UTF-8 characters. This is not necessarily 16 bytes since one character can be more than one byte.
    /// Real clients will send a fully uppercased username, and will perform authentication calculations on the uppercased version.
    /// Uppercasing in regards to non-ASCII values is little weird. See `https://docs.rs/wow_srp/latest/wow_srp/normalized_string/index.html` for more info.
    ///
    pub account_name: String,
}

impl CMD_AUTH_LOGON_CHALLENGE_Client {
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

impl CMD_AUTH_LOGON_CHALLENGE_Client {
    pub(crate) fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        // protocol_version: u8
        w.write_all(&self.protocol_version.to_le_bytes())?;

        // size: u16
        w.write_all(&((self.size() - 3) as u16).to_le_bytes())?;

        // game_name: u32
        w.write_all(&Self::GAME_NAME_VALUE.to_le_bytes())?;

        // version: Version
        self.version.write_into_vec(w)?;

        // platform: Platform
        w.write_all(&(self.platform.as_int() as u32).to_le_bytes())?;

        // os: Os
        w.write_all(&(self.os.as_int() as u32).to_le_bytes())?;

        // locale: Locale
        w.write_all(&(self.locale.as_int() as u32).to_le_bytes())?;

        // utc_timezone_offset: u32
        w.write_all(&self.utc_timezone_offset.to_le_bytes())?;

        // client_ip_address: u32_be
        w.write_all(&self.client_ip_address.to_be_bytes())?;

        // account_name: String
        w.write_all(&(self.account_name.len() as u8).to_le_bytes())?;
        w.write_all(self.account_name.as_bytes())?;

        Ok(())
    }
}

impl ClientMessage for CMD_AUTH_LOGON_CHALLENGE_Client {
    const OPCODE: u8 = 0x00;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // protocol_version: u8
        let protocol_version = crate::util::read_u8_le(r)?;

        // size: u16
        let _size = crate::util::read_u16_le(r)?;
        // size is expected to always be self.size (0)

        // game_name: u32
        let _game_name = crate::util::read_u32_le(r)?;
        // game_name is expected to always be "\0WoW" (5730135)

        // version: Version
        let version = Version::read(r)?;

        // platform: Platform
        let platform: Platform = crate::util::read_u32_le(r)?.into();

        // os: Os
        let os: Os = crate::util::read_u32_le(r)?.into();

        // locale: Locale
        let locale: Locale = crate::util::read_u32_le(r)?.into();

        // utc_timezone_offset: u32
        let utc_timezone_offset = crate::util::read_u32_le(r)?;

        // client_ip_address: u32_be
        let client_ip_address = crate::util::read_u32_be(r)?;

        // account_name: String
        let account_name = {
            let account_name = crate::util::read_u8_le(r)?;
            let account_name = crate::util::read_fixed_string_to_vec(r, account_name as usize)?;
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
            // protocol_version: u8
            let protocol_version = crate::util::tokio_read_u8_le(r).await?;

            // size: u16
            let _size = crate::util::tokio_read_u16_le(r).await?;
            // size is expected to always be self.size (0)

            // game_name: u32
            let _game_name = crate::util::tokio_read_u32_le(r).await?;
            // game_name is expected to always be "\0WoW" (5730135)

            // version: Version
            let version = Version::tokio_read(r).await?;

            // platform: Platform
            let platform: Platform = crate::util::tokio_read_u32_le(r).await?.into();

            // os: Os
            let os: Os = crate::util::tokio_read_u32_le(r).await?.into();

            // locale: Locale
            let locale: Locale = crate::util::tokio_read_u32_le(r).await?.into();

            // utc_timezone_offset: u32
            let utc_timezone_offset = crate::util::tokio_read_u32_le(r).await?;

            // client_ip_address: u32_be
            let client_ip_address = crate::util::tokio_read_u32_be(r).await?;

            // account_name: String
            let account_name = {
                let account_name = crate::util::tokio_read_u8_le(r).await?;
                let account_name = crate::util::tokio_read_fixed_string_to_vec(r, account_name as usize).await?;
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
            // protocol_version: u8
            let protocol_version = crate::util::astd_read_u8_le(r).await?;

            // size: u16
            let _size = crate::util::astd_read_u16_le(r).await?;
            // size is expected to always be self.size (0)

            // game_name: u32
            let _game_name = crate::util::astd_read_u32_le(r).await?;
            // game_name is expected to always be "\0WoW" (5730135)

            // version: Version
            let version = Version::astd_read(r).await?;

            // platform: Platform
            let platform: Platform = crate::util::astd_read_u32_le(r).await?.into();

            // os: Os
            let os: Os = crate::util::astd_read_u32_le(r).await?.into();

            // locale: Locale
            let locale: Locale = crate::util::astd_read_u32_le(r).await?.into();

            // utc_timezone_offset: u32
            let utc_timezone_offset = crate::util::astd_read_u32_le(r).await?;

            // client_ip_address: u32_be
            let client_ip_address = crate::util::astd_read_u32_be(r).await?;

            // account_name: String
            let account_name = {
                let account_name = crate::util::astd_read_u8_le(r).await?;
                let account_name = crate::util::astd_read_fixed_string_to_vec(r, account_name as usize).await?;
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

impl CMD_AUTH_LOGON_CHALLENGE_Client {
    pub(crate) fn size(&self) -> usize {
        1 // protocol_version: u8
        + 2 // size: u16
        + 4 // game_name: u32
        + 5 // version: Version
        + 4 // platform: Platform
        + 4 // os: Os
        + 4 // locale: Locale
        + 4 // utc_timezone_offset: u32
        + 4 // client_ip_address: u32_be
        + self.account_name.len() + 1 // account_name: String
    }
}

#[cfg(test)]
mod test {
    use super::CMD_AUTH_LOGON_CHALLENGE_Client;
    use crate::all::*;
    use super::*;
    use super::super::*;
    use crate::logon::version_3::opcodes::ClientOpcodeMessage;

    const RAW0: [u8; 35] = [ 0x00, 0x03, 0x1F, 0x00, 0x57, 0x6F, 0x57, 0x00, 0x01,
         0x0C, 0x01, 0xF3, 0x16, 0x36, 0x38, 0x78, 0x00, 0x6E, 0x69, 0x57, 0x00,
         0x42, 0x47, 0x6E, 0x65, 0x3C, 0x00, 0x00, 0x00, 0x7F, 0x00, 0x00, 0x01,
         0x01, 0x41, ];

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_client.wowm` line 27.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMD_AUTH_LOGON_CHALLENGE_Client0() {
        let expected = CMD_AUTH_LOGON_CHALLENGE_Client {
            protocol_version: 0x3,
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
            client_ip_address: 0x7F000001,
            account_name: String::from("A"),
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::read(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.protocol_version, expected.protocol_version);
        assert_eq!(t.version, expected.version);
        assert_eq!(t.platform, expected.platform);
        assert_eq!(t.os, expected.os);
        assert_eq!(t.locale, expected.locale);
        assert_eq!(t.utc_timezone_offset, expected.utc_timezone_offset);
        assert_eq!(t.client_ip_address, expected.client_ip_address);
        assert_eq!(t.account_name, expected.account_name);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_client.wowm` line 27.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMD_AUTH_LOGON_CHALLENGE_Client0() {
        let expected = CMD_AUTH_LOGON_CHALLENGE_Client {
            protocol_version: 0x3,
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
            client_ip_address: 0x7F000001,
            account_name: String::from("A"),
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::tokio_read(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.protocol_version, expected.protocol_version);
        assert_eq!(t.version, expected.version);
        assert_eq!(t.platform, expected.platform);
        assert_eq!(t.os, expected.os);
        assert_eq!(t.locale, expected.locale);
        assert_eq!(t.utc_timezone_offset, expected.utc_timezone_offset);
        assert_eq!(t.client_ip_address, expected.client_ip_address);
        assert_eq!(t.account_name, expected.account_name);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/login/cmd_auth_logon/challenge_client.wowm` line 27.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMD_AUTH_LOGON_CHALLENGE_Client0() {
        let expected = CMD_AUTH_LOGON_CHALLENGE_Client {
            protocol_version: 0x3,
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
            client_ip_address: 0x7F000001,
            account_name: String::from("A"),
        };

        let header_size = 1;
        let t = ClientOpcodeMessage::astd_read(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMD_AUTH_LOGON_CHALLENGE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMD_AUTH_LOGON_CHALLENGE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.protocol_version, expected.protocol_version);
        assert_eq!(t.version, expected.version);
        assert_eq!(t.platform, expected.platform);
        assert_eq!(t.os, expected.os);
        assert_eq!(t.locale, expected.locale);
        assert_eq!(t.utc_timezone_offset, expected.utc_timezone_offset);
        assert_eq!(t.client_ip_address, expected.client_ip_address);
        assert_eq!(t.account_name, expected.account_name);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

