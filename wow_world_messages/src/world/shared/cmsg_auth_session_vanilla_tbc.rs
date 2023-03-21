use crate::Message;
use std::io::{Read, Write};

use crate::shared::addon_info_vanilla_tbc_wrath::AddonInfo;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Sent after receiving [`SMSG_AUTH_CHALLENGE`](crate::vanilla::SMSG_AUTH_CHALLENGE).
///
/// This message is never encrypted.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/cmsg_auth_session.wowm:10`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/cmsg_auth_session.wowm#L10):
/// ```text
/// cmsg CMSG_AUTH_SESSION = 0x01ED {
///     u32 build;
///     u32 server_id;
///     CString username;
///     u32 client_seed;
///     u8[20] client_proof;
///     u32 decompressed_addon_info_size;
///     AddonInfo[-] addon_info;
/// }
/// ```
pub struct CMSG_AUTH_SESSION {
    pub build: u32,
    /// This is sent to the client in `CMD_REALM_LIST_Server`.
    ///
    pub server_id: u32,
    pub username: String,
    pub client_seed: u32,
    pub client_proof: [u8; 20],
    pub decompressed_addon_info_size: u32,
    pub addon_info: Vec<AddonInfo>,
}

impl crate::Message for CMSG_AUTH_SESSION {
    const OPCODE: u32 = 0x01ed;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // build: u32
        w.write_all(&self.build.to_le_bytes())?;

        // server_id: u32
        w.write_all(&self.server_id.to_le_bytes())?;

        // username: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.username.as_bytes().iter().rev().next(), Some(&0_u8), "String `username` must not be null-terminated.");
        w.write_all(self.username.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // client_seed: u32
        w.write_all(&self.client_seed.to_le_bytes())?;

        // client_proof: u8[20]
        for i in self.client_proof.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // decompressed_addon_info_size: u32
        w.write_all(&self.decompressed_addon_info_size.to_le_bytes())?;

        // addon_info: AddonInfo[-]
        let mut encoder = flate2::write::ZlibEncoder::new(w, flate2::Compression::default());
        for i in self.addon_info.iter() {
            i.write_into_vec(&mut encoder)?;
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(37..=65827).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01ED, size: body_size as u32 });
        }

        // build: u32
        let build = crate::util::read_u32_le(&mut r)?;

        // server_id: u32
        let server_id = crate::util::read_u32_le(&mut r)?;

        // username: CString
        let username = {
            let username = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(username)?
        };

        // client_seed: u32
        let client_seed = crate::util::read_u32_le(&mut r)?;

        // client_proof: u8[20]
        let client_proof = {
            let mut client_proof = [0_u8; 20];
            r.read_exact(&mut client_proof)?;
            client_proof
        };

        // decompressed_addon_info_size: u32
        let decompressed_addon_info_size = crate::util::read_u32_le(&mut r)?;

        // addon_info: AddonInfo[-]
        let addon_info = {
            let mut decoder = &mut flate2::read::ZlibDecoder::new(r);

            let mut current_size = {
                4 // build: u32
                + 4 // server_id: u32
                + username.len() + 1 // username: CString
                + 4 // client_seed: u32
                + 20 // client_proof: u8[20]
                + 4 // decompressed_addon_info_size: u32
            };
            let mut addon_info = Vec::with_capacity(body_size as usize - current_size);
            while decoder.total_out() < (decompressed_addon_info_size as u64) {
                addon_info.push(AddonInfo::read(&mut decoder)?);
                current_size += 1;
            }
            addon_info
        };

        Ok(Self {
            build,
            server_id,
            username,
            client_seed,
            client_proof,
            decompressed_addon_info_size,
            addon_info,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_AUTH_SESSION {
    #[cfg(feature = "sync")]
    fn write_unencrypted_client<W: std::io::Write>(&self, mut w: W) -> Result<(), std::io::Error> {
        let mut v = Vec::with_capacity(1024);
        let mut s = &mut v;
        crate::util::vanilla_get_unencrypted_client(&mut s, Self::OPCODE as u16, 0)?;
        self.write_into_vec(&mut s)?;
        let size_len = 2;
        let size = v.len().saturating_sub(size_len);
        let s = size.to_le_bytes();
        v[0] = s[1];
        v[1] = s[0];
        w.write_all(&v)
    }

    #[cfg(all(feature = "sync", feature = "encryption"))]
    fn write_encrypted_client<W: std::io::Write>(
        &self,
        mut w: W,
        e: &mut wow_srp::vanilla_header::EncrypterHalf,
    ) -> Result<(), std::io::Error> {
        let mut v = Vec::with_capacity(1024);
        let mut s = &mut v;
        crate::util::vanilla_get_unencrypted_client(&mut s, Self::OPCODE as u16, 0)?;
        self.write_into_vec(&mut s)?;
        let size_len = 2;
        let size = v.len().saturating_sub(size_len) as u16;
        let header = e.encrypt_client_header(size, Self::OPCODE);
        for (i, e) in header.iter().enumerate() {
            v[i] = *e;
        }
        w.write_all(&v)
    }

    #[cfg(feature = "tokio")]
    fn tokio_write_unencrypted_client<'s, 'async_trait, W>(
        &'s self,
        mut w: W,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        's: 'async_trait,
        Self: Sync + 'async_trait,
     {
        Box::pin(async move {
            let mut v = Vec::with_capacity(1024);
            let mut s = &mut v;
            crate::util::vanilla_get_unencrypted_client(&mut s, Self::OPCODE as u16, 0)?;
            self.write_into_vec(&mut s)?;
            let size_len = 2;
            let size = v.len().saturating_sub(size_len);
            let s = size.to_le_bytes();
            v[0] = s[1];
            v[1] = s[0];
            w.write_all(&v).await
        })
    }

    #[cfg(all(feature = "tokio", feature = "encryption"))]
    fn tokio_write_encrypted_client<'s, 'e, 'async_trait, W>(
        &'s self,
        mut w: W,
        e: &'e mut wow_srp::vanilla_header::EncrypterHalf,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        's: 'async_trait,
        'e: 'async_trait,
        Self: Sync + 'async_trait,
     {
        Box::pin(async move {
            let mut v = Vec::with_capacity(1024);
            let mut s = &mut v;
            crate::util::vanilla_get_unencrypted_client(&mut s, Self::OPCODE as u16, 0)?;
            self.write_into_vec(&mut s)?;
            let size_len = 2;
            let size = v.len().saturating_sub(size_len) as u16;
            let header = e.encrypt_client_header(size, Self::OPCODE);
            for (i, e) in header.iter().enumerate() {
                v[i] = *e;
            }
            w.write_all(&v).await
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write_unencrypted_client<'s, 'async_trait, W>(
        &'s self,
        mut w: W,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        's: 'async_trait,
        Self: Sync + 'async_trait,
     {
        Box::pin(async move {
            let mut v = Vec::with_capacity(1024);
            let mut s = &mut v;
            crate::util::vanilla_get_unencrypted_client(&mut s, Self::OPCODE as u16, 0)?;
            self.write_into_vec(&mut s)?;
            let size_len = 2;
            let size = v.len().saturating_sub(size_len);
            let s = size.to_le_bytes();
            v[0] = s[1];
            v[1] = s[0];
            w.write_all(&v).await
        })
    }

    #[cfg(all(feature = "async-std", feature = "encryption"))]
    fn astd_write_encrypted_client<'s, 'e, 'async_trait, W>(
        &'s self,
        mut w: W,
        e: &'e mut wow_srp::vanilla_header::EncrypterHalf,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        's: 'async_trait,
        'e: 'async_trait,
        Self: Sync + 'async_trait,
     {
        Box::pin(async move {
            let mut v = Vec::with_capacity(1024);
            let mut s = &mut v;
            crate::util::vanilla_get_unencrypted_client(&mut s, Self::OPCODE as u16, 0)?;
            self.write_into_vec(&mut s)?;
            let size_len = 2;
            let size = v.len().saturating_sub(size_len) as u16;
            let header = e.encrypt_client_header(size, Self::OPCODE);
            for (i, e) in header.iter().enumerate() {
                v[i] = *e;
            }
            w.write_all(&v).await
        })
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_AUTH_SESSION {
    #[cfg(feature = "sync")]
    fn write_unencrypted_client<W: std::io::Write>(&self, mut w: W) -> Result<(), std::io::Error> {
        let mut v = Vec::with_capacity(1024);
        let mut s = &mut v;
        crate::util::tbc_get_unencrypted_client(&mut s, Self::OPCODE as u16, 0)?;
        self.write_into_vec(&mut s)?;
        let size_len = 2;
        let size = v.len().saturating_sub(size_len);
        let s = size.to_le_bytes();
        v[0] = s[1];
        v[1] = s[0];
        w.write_all(&v)
    }

    #[cfg(all(feature = "sync", feature = "encryption"))]
    fn write_encrypted_client<W: std::io::Write>(
        &self,
        mut w: W,
        e: &mut wow_srp::tbc_header::EncrypterHalf,
    ) -> Result<(), std::io::Error> {
        let mut v = Vec::with_capacity(1024);
        let mut s = &mut v;
        crate::util::tbc_get_unencrypted_client(&mut s, Self::OPCODE as u16, 0)?;
        self.write_into_vec(&mut s)?;
        let size_len = 2;
        let size = v.len().saturating_sub(size_len) as u16;
        let header = e.encrypt_client_header(size, Self::OPCODE);
        for (i, e) in header.iter().enumerate() {
            v[i] = *e;
        }
        w.write_all(&v)
    }

    #[cfg(feature = "tokio")]
    fn tokio_write_unencrypted_client<'s, 'async_trait, W>(
        &'s self,
        mut w: W,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        's: 'async_trait,
        Self: Sync + 'async_trait,
     {
        Box::pin(async move {
            let mut v = Vec::with_capacity(1024);
            let mut s = &mut v;
            crate::util::tbc_get_unencrypted_client(&mut s, Self::OPCODE as u16, 0)?;
            self.write_into_vec(&mut s)?;
            let size_len = 2;
            let size = v.len().saturating_sub(size_len);
            let s = size.to_le_bytes();
            v[0] = s[1];
            v[1] = s[0];
            w.write_all(&v).await
        })
    }

    #[cfg(all(feature = "tokio", feature = "encryption"))]
    fn tokio_write_encrypted_client<'s, 'e, 'async_trait, W>(
        &'s self,
        mut w: W,
        e: &'e mut wow_srp::tbc_header::EncrypterHalf,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        's: 'async_trait,
        'e: 'async_trait,
        Self: Sync + 'async_trait,
     {
        Box::pin(async move {
            let mut v = Vec::with_capacity(1024);
            let mut s = &mut v;
            crate::util::tbc_get_unencrypted_client(&mut s, Self::OPCODE as u16, 0)?;
            self.write_into_vec(&mut s)?;
            let size_len = 2;
            let size = v.len().saturating_sub(size_len) as u16;
            let header = e.encrypt_client_header(size, Self::OPCODE);
            for (i, e) in header.iter().enumerate() {
                v[i] = *e;
            }
            w.write_all(&v).await
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write_unencrypted_client<'s, 'async_trait, W>(
        &'s self,
        mut w: W,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        's: 'async_trait,
        Self: Sync + 'async_trait,
     {
        Box::pin(async move {
            let mut v = Vec::with_capacity(1024);
            let mut s = &mut v;
            crate::util::tbc_get_unencrypted_client(&mut s, Self::OPCODE as u16, 0)?;
            self.write_into_vec(&mut s)?;
            let size_len = 2;
            let size = v.len().saturating_sub(size_len);
            let s = size.to_le_bytes();
            v[0] = s[1];
            v[1] = s[0];
            w.write_all(&v).await
        })
    }

    #[cfg(all(feature = "async-std", feature = "encryption"))]
    fn astd_write_encrypted_client<'s, 'e, 'async_trait, W>(
        &'s self,
        mut w: W,
        e: &'e mut wow_srp::tbc_header::EncrypterHalf,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        's: 'async_trait,
        'e: 'async_trait,
        Self: Sync + 'async_trait,
     {
        Box::pin(async move {
            let mut v = Vec::with_capacity(1024);
            let mut s = &mut v;
            crate::util::tbc_get_unencrypted_client(&mut s, Self::OPCODE as u16, 0)?;
            self.write_into_vec(&mut s)?;
            let size_len = 2;
            let size = v.len().saturating_sub(size_len) as u16;
            let header = e.encrypt_client_header(size, Self::OPCODE);
            for (i, e) in header.iter().enumerate() {
                v[i] = *e;
            }
            w.write_all(&v).await
        })
    }

}

impl CMSG_AUTH_SESSION {
    pub(crate) fn size(&self) -> usize {
        4 // build: u32
        + 4 // server_id: u32
        + self.username.len() + 1 // username: CString
        + 4 // client_seed: u32
        + 20 // client_proof: u8[20]
        + 4 // decompressed_addon_info_size: u32
        + crate::util::zlib_compressed_size(self.addon_info.iter().fold(Vec::new(), |mut acc, x| { x.write_into_vec(&mut acc).unwrap(); acc } ).as_slice()) // addon_info: AddonInfo[-]
    }
}

#[cfg(all(feature = "vanilla", test))]
mod test_vanilla {
    use super::CMSG_AUTH_SESSION;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ClientOpcodeMessage;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const RAW0: [u8; 174] = [ 0x00, 0xAC, 0xED, 0x01, 0x00, 0x00, 0xF3, 0x16, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x41, 0x00, 0x88, 0x02, 0xD8, 0x49, 0x88,
         0x9D, 0xEF, 0x05, 0x25, 0xBB, 0xC1, 0xAB, 0xA7, 0x8A, 0xDB, 0xA4, 0xFB,
         0xA3, 0xE7, 0x7E, 0x67, 0xAC, 0xEA, 0xC6, 0x56, 0x01, 0x00, 0x00, 0x78,
         0x9C, 0x75, 0xCC, 0xBD, 0x0E, 0xC2, 0x30, 0x0C, 0x04, 0xE0, 0xF2, 0x1E,
         0xBC, 0x0C, 0x61, 0x40, 0x95, 0xC8, 0x42, 0xC3, 0x8C, 0x4C, 0xE2, 0x22,
         0x0B, 0xC7, 0xA9, 0x8C, 0xCB, 0x4F, 0x9F, 0x1E, 0x16, 0x24, 0x06, 0x73,
         0xEB, 0x77, 0x77, 0x81, 0x69, 0x59, 0x40, 0xCB, 0x69, 0x33, 0x67, 0xA3,
         0x26, 0xC7, 0xBE, 0x5B, 0xD5, 0xC7, 0x7A, 0xDF, 0x7D, 0x12, 0xBE, 0x16,
         0xC0, 0x8C, 0x71, 0x24, 0xE4, 0x12, 0x49, 0xA8, 0xC2, 0xE4, 0x95, 0x48,
         0x0A, 0xC9, 0xC5, 0x3D, 0xD8, 0xB6, 0x7A, 0x06, 0x4B, 0xF8, 0x34, 0x0F,
         0x15, 0x46, 0x73, 0x67, 0xBB, 0x38, 0xCC, 0x7A, 0xC7, 0x97, 0x8B, 0xBD,
         0xDC, 0x26, 0xCC, 0xFE, 0x30, 0x42, 0xD6, 0xE6, 0xCA, 0x01, 0xA8, 0xB8,
         0x90, 0x80, 0x51, 0xFC, 0xB7, 0xA4, 0x50, 0x70, 0xB8, 0x12, 0xF3, 0x3F,
         0x26, 0x41, 0xFD, 0xB5, 0x37, 0x90, 0x19, 0x66, 0x8F, ];

    pub(crate) fn expected0() -> CMSG_AUTH_SESSION {
        CMSG_AUTH_SESSION {
            build: 0x16F3,
            server_id: 0x0,
            username: String::from("A"),
            client_seed: 0x49D80288,
            client_proof: [ 0x88, 0x9D, 0xEF, 0x05, 0x25, 0xBB, 0xC1, 0xAB, 0xA7,
                 0x8A, 0xDB, 0xA4, 0xFB, 0xA3, 0xE7, 0x7E, 0x67, 0xAC, 0xEA, 0xC6, ],
            decompressed_addon_info_size: 0x156,
            addon_info: vec![
                AddonInfo {
                    addon_name: String::from("Blizzard_AuctionUI"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0x0,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_BattlefieldMinimap"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0x0,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_BindingUI"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0x0,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_CombatText"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0x0,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_CraftUI"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0x0,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_GMSurveyUI"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0x0,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_InspectUI"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0x0,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_MacroUI"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0x0,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_RaidUI"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0x0,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_TalentUI"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0x0,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_TradeSkillUI"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0x0,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_TrainerUI"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0x0,
                },
            ],
        }

    }

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_auth_session.wowm` line 28.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMSG_AUTH_SESSION0() {
        let expected = expected0();
        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_AUTH_SESSION(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_AUTH_SESSION, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.build, expected.build);
        assert_eq!(t.server_id, expected.server_id);
        assert_eq!(t.username, expected.username);
        assert_eq!(t.client_seed, expected.client_seed);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.decompressed_addon_info_size, expected.decompressed_addon_info_size);
        assert_eq!(t.addon_info, expected.addon_info);

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        let s = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&dest)).unwrap();
        let s = match s {
            ClientOpcodeMessage::CMSG_AUTH_SESSION(s) => s,
            opcode => panic!("incorrect opcode. Expected CMSG_AUTH_SESSION, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t, s);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_auth_session.wowm` line 28.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMSG_AUTH_SESSION0() {
        let expected = expected0();
        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_AUTH_SESSION(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_AUTH_SESSION, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.build, expected.build);
        assert_eq!(t.server_id, expected.server_id);
        assert_eq!(t.username, expected.username);
        assert_eq!(t.client_seed, expected.client_seed);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.decompressed_addon_info_size, expected.decompressed_addon_info_size);
        assert_eq!(t.addon_info, expected.addon_info);

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        let s = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&dest)).await.unwrap();
        let s = match s {
            ClientOpcodeMessage::CMSG_AUTH_SESSION(s) => s,
            opcode => panic!("incorrect opcode. Expected CMSG_AUTH_SESSION, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t, s);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_auth_session.wowm` line 28.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMSG_AUTH_SESSION0() {
        let expected = expected0();
        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_AUTH_SESSION(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_AUTH_SESSION, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.build, expected.build);
        assert_eq!(t.server_id, expected.server_id);
        assert_eq!(t.username, expected.username);
        assert_eq!(t.client_seed, expected.client_seed);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.decompressed_addon_info_size, expected.decompressed_addon_info_size);
        assert_eq!(t.addon_info, expected.addon_info);

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        let s = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&dest)).await.unwrap();
        let s = match s {
            ClientOpcodeMessage::CMSG_AUTH_SESSION(s) => s,
            opcode => panic!("incorrect opcode. Expected CMSG_AUTH_SESSION, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t, s);
    }

}

#[cfg(all(feature = "tbc", test))]
mod test_tbc {
    use super::CMSG_AUTH_SESSION;
    use super::*;
    use super::super::*;
    use crate::tbc::opcodes::ClientOpcodeMessage;
    use crate::tbc::{ClientMessage, ServerMessage};

    const RAW0: [u8; 174] = [ 0x00, 0xAC, 0xED, 0x01, 0x00, 0x00, 0xF3, 0x16, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x41, 0x00, 0x88, 0x02, 0xD8, 0x49, 0x88,
         0x9D, 0xEF, 0x05, 0x25, 0xBB, 0xC1, 0xAB, 0xA7, 0x8A, 0xDB, 0xA4, 0xFB,
         0xA3, 0xE7, 0x7E, 0x67, 0xAC, 0xEA, 0xC6, 0x56, 0x01, 0x00, 0x00, 0x78,
         0x9C, 0x75, 0xCC, 0xBD, 0x0E, 0xC2, 0x30, 0x0C, 0x04, 0xE0, 0xF2, 0x1E,
         0xBC, 0x0C, 0x61, 0x40, 0x95, 0xC8, 0x42, 0xC3, 0x8C, 0x4C, 0xE2, 0x22,
         0x0B, 0xC7, 0xA9, 0x8C, 0xCB, 0x4F, 0x9F, 0x1E, 0x16, 0x24, 0x06, 0x73,
         0xEB, 0x77, 0x77, 0x81, 0x69, 0x59, 0x40, 0xCB, 0x69, 0x33, 0x67, 0xA3,
         0x26, 0xC7, 0xBE, 0x5B, 0xD5, 0xC7, 0x7A, 0xDF, 0x7D, 0x12, 0xBE, 0x16,
         0xC0, 0x8C, 0x71, 0x24, 0xE4, 0x12, 0x49, 0xA8, 0xC2, 0xE4, 0x95, 0x48,
         0x0A, 0xC9, 0xC5, 0x3D, 0xD8, 0xB6, 0x7A, 0x06, 0x4B, 0xF8, 0x34, 0x0F,
         0x15, 0x46, 0x73, 0x67, 0xBB, 0x38, 0xCC, 0x7A, 0xC7, 0x97, 0x8B, 0xBD,
         0xDC, 0x26, 0xCC, 0xFE, 0x30, 0x42, 0xD6, 0xE6, 0xCA, 0x01, 0xA8, 0xB8,
         0x90, 0x80, 0x51, 0xFC, 0xB7, 0xA4, 0x50, 0x70, 0xB8, 0x12, 0xF3, 0x3F,
         0x26, 0x41, 0xFD, 0xB5, 0x37, 0x90, 0x19, 0x66, 0x8F, ];

    pub(crate) fn expected0() -> CMSG_AUTH_SESSION {
        CMSG_AUTH_SESSION {
            build: 0x16F3,
            server_id: 0x0,
            username: String::from("A"),
            client_seed: 0x49D80288,
            client_proof: [ 0x88, 0x9D, 0xEF, 0x05, 0x25, 0xBB, 0xC1, 0xAB, 0xA7,
                 0x8A, 0xDB, 0xA4, 0xFB, 0xA3, 0xE7, 0x7E, 0x67, 0xAC, 0xEA, 0xC6, ],
            decompressed_addon_info_size: 0x156,
            addon_info: vec![
                AddonInfo {
                    addon_name: String::from("Blizzard_AuctionUI"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0x0,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_BattlefieldMinimap"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0x0,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_BindingUI"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0x0,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_CombatText"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0x0,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_CraftUI"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0x0,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_GMSurveyUI"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0x0,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_InspectUI"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0x0,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_MacroUI"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0x0,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_RaidUI"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0x0,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_TalentUI"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0x0,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_TradeSkillUI"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0x0,
                },
                AddonInfo {
                    addon_name: String::from("Blizzard_TrainerUI"),
                    addon_has_signature: 0x1,
                    addon_crc: 0x4C1C776D,
                    addon_extra_crc: 0x0,
                },
            ],
        }

    }

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_auth_session.wowm` line 28.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMSG_AUTH_SESSION0() {
        let expected = expected0();
        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_AUTH_SESSION(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_AUTH_SESSION, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.build, expected.build);
        assert_eq!(t.server_id, expected.server_id);
        assert_eq!(t.username, expected.username);
        assert_eq!(t.client_seed, expected.client_seed);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.decompressed_addon_info_size, expected.decompressed_addon_info_size);
        assert_eq!(t.addon_info, expected.addon_info);

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        let s = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&dest)).unwrap();
        let s = match s {
            ClientOpcodeMessage::CMSG_AUTH_SESSION(s) => s,
            opcode => panic!("incorrect opcode. Expected CMSG_AUTH_SESSION, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t, s);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_auth_session.wowm` line 28.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMSG_AUTH_SESSION0() {
        let expected = expected0();
        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_AUTH_SESSION(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_AUTH_SESSION, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.build, expected.build);
        assert_eq!(t.server_id, expected.server_id);
        assert_eq!(t.username, expected.username);
        assert_eq!(t.client_seed, expected.client_seed);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.decompressed_addon_info_size, expected.decompressed_addon_info_size);
        assert_eq!(t.addon_info, expected.addon_info);

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        let s = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&dest)).await.unwrap();
        let s = match s {
            ClientOpcodeMessage::CMSG_AUTH_SESSION(s) => s,
            opcode => panic!("incorrect opcode. Expected CMSG_AUTH_SESSION, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t, s);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/cmsg_auth_session.wowm` line 28.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMSG_AUTH_SESSION0() {
        let expected = expected0();
        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_AUTH_SESSION(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_AUTH_SESSION, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.build, expected.build);
        assert_eq!(t.server_id, expected.server_id);
        assert_eq!(t.username, expected.username);
        assert_eq!(t.client_seed, expected.client_seed);
        assert_eq!(t.client_proof, expected.client_proof);
        assert_eq!(t.decompressed_addon_info_size, expected.decompressed_addon_info_size);
        assert_eq!(t.addon_info, expected.addon_info);

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        let s = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&dest)).await.unwrap();
        let s = match s {
            ClientOpcodeMessage::CMSG_AUTH_SESSION(s) => s,
            opcode => panic!("incorrect opcode. Expected CMSG_AUTH_SESSION, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t, s);
    }

}

