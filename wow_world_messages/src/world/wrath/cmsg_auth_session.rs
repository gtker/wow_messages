use crate::Message;
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Sent after receiving [`SMSG_AUTH_CHALLENGE`](crate::wrath::SMSG_AUTH_CHALLENGE).
///
/// This message is never encrypted.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/cmsg_auth_session.wowm:143`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/cmsg_auth_session.wowm#L143):
/// ```text
/// cmsg CMSG_AUTH_SESSION = 0x01ED {
///     u32 client_build;
///     u32 login_server_id;
///     CString username;
///     u32 login_server_type;
///     u32 client_seed;
///     u32 region_id;
///     u32 battleground_id;
///     u32 realm_id;
///     u64 dos_response;
///     u8[20] client_proof;
///     u8[-] addon_info;
/// }
/// ```
pub struct CMSG_AUTH_SESSION {
    pub client_build: u32,
    pub login_server_id: u32,
    pub username: String,
    pub login_server_type: u32,
    pub client_seed: u32,
    pub region_id: u32,
    pub battleground_id: u32,
    pub realm_id: u32,
    /// Purpose and exact meaning of name unknown.
    /// TrinityCore has this name but never uses the variable afterwards.
    pub dos_response: u64,
    pub client_proof: [u8; 20],
    pub addon_info: Vec<u8>,
}

impl crate::private::Sealed for CMSG_AUTH_SESSION {}
impl CMSG_AUTH_SESSION {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(61..=65851).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // client_build: u32
        let client_build = crate::util::read_u32_le(&mut r)?;

        // login_server_id: u32
        let login_server_id = crate::util::read_u32_le(&mut r)?;

        // username: CString
        let username = {
            let username = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(username)?
        };

        // login_server_type: u32
        let login_server_type = crate::util::read_u32_le(&mut r)?;

        // client_seed: u32
        let client_seed = crate::util::read_u32_le(&mut r)?;

        // region_id: u32
        let region_id = crate::util::read_u32_le(&mut r)?;

        // battleground_id: u32
        let battleground_id = crate::util::read_u32_le(&mut r)?;

        // realm_id: u32
        let realm_id = crate::util::read_u32_le(&mut r)?;

        // dos_response: u64
        let dos_response = crate::util::read_u64_le(&mut r)?;

        // client_proof: u8[20]
        let client_proof = {
            let mut client_proof = [0_u8; 20];
            r.read_exact(&mut client_proof)?;
            client_proof
        };

        // addon_info: u8[-]
        let addon_info = {
            let addon_info_decompressed_size = crate::util::read_u32_le(&mut r)?;

            let mut decoder = &mut flate2::read::ZlibDecoder::new(r);

            let mut current_size = {
                4 // client_build: u32
                + 4 // login_server_id: u32
                + username.len() + 1 // username: CString
                + 4 // login_server_type: u32
                + 4 // client_seed: u32
                + 4 // region_id: u32
                + 4 // battleground_id: u32
                + 4 // realm_id: u32
                + 8 // dos_response: u64
                + 20 // client_proof: u8[20]
            };
            let mut addon_info = Vec::with_capacity(body_size as usize - current_size);
            while decoder.total_out() < (addon_info_decompressed_size as u64) {
                addon_info.push(crate::util::read_u8_le(&mut decoder)?);
                current_size += 1;
            }
            addon_info
        };

        Ok(Self {
            client_build,
            login_server_id,
            username,
            login_server_type,
            client_seed,
            region_id,
            battleground_id,
            realm_id,
            dos_response,
            client_proof,
            addon_info,
        })
    }

}

impl crate::Message for CMSG_AUTH_SESSION {
    const OPCODE: u32 = 0x01ed;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_AUTH_SESSION"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_AUTH_SESSION {{").unwrap();
        // Members
        writeln!(s, "    client_build = {};", self.client_build).unwrap();
        writeln!(s, "    login_server_id = {};", self.login_server_id).unwrap();
        writeln!(s, "    username = \"{}\";", self.username).unwrap();
        writeln!(s, "    login_server_type = {};", self.login_server_type).unwrap();
        writeln!(s, "    client_seed = {};", self.client_seed).unwrap();
        writeln!(s, "    region_id = {};", self.region_id).unwrap();
        writeln!(s, "    battleground_id = {};", self.battleground_id).unwrap();
        writeln!(s, "    realm_id = {};", self.realm_id).unwrap();
        writeln!(s, "    dos_response = {};", self.dos_response).unwrap();
        write!(s, "    client_proof = [").unwrap();
        for v in self.client_proof.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "];").unwrap();
        write!(s, "    addon_info = [").unwrap();
        for v in self.addon_info.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 493_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "client_build", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "login_server_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.username.len() + 1, "username", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "login_server_type", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "client_seed", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "region_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "battleground_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "realm_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "dos_response", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.client_proof.len(), "client_proof", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.addon_info.len(), "addon_info", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // client_build: u32
        w.write_all(&self.client_build.to_le_bytes())?;

        // login_server_id: u32
        w.write_all(&self.login_server_id.to_le_bytes())?;

        // username: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.username.as_bytes().iter().next_back(), Some(&0_u8), "String `username` must not be null-terminated.");
        w.write_all(self.username.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // login_server_type: u32
        w.write_all(&self.login_server_type.to_le_bytes())?;

        // client_seed: u32
        w.write_all(&self.client_seed.to_le_bytes())?;

        // region_id: u32
        w.write_all(&self.region_id.to_le_bytes())?;

        // battleground_id: u32
        w.write_all(&self.battleground_id.to_le_bytes())?;

        // realm_id: u32
        w.write_all(&self.realm_id.to_le_bytes())?;

        // dos_response: u64
        w.write_all(&self.dos_response.to_le_bytes())?;

        // client_proof: u8[20]
        for i in self.client_proof.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // addon_info: u8[-]
        let decompressed_size: u32 = 1 * self.addon_info.len() as u32;
        w.write_all(&decompressed_size.to_le_bytes())?;
        let mut encoder = flate2::write::ZlibEncoder::new(w, flate2::Compression::default());
        for i in self.addon_info.iter() {
            encoder.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(493, "CMSG_AUTH_SESSION", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_AUTH_SESSION {
    #[cfg(feature = "sync")]
    fn write_unencrypted_client<W: Write>(&self, mut w: W) -> Result<(), std::io::Error> {
        let mut v = Vec::with_capacity(1024);
        let mut s = &mut v;
        crate::util::wrath_get_unencrypted_client(&mut s, Self::OPCODE as u16, 0)?;
        self.write_into_vec(&mut s)?;
        let size_len = 2;
        let size = v.len().saturating_sub(size_len);
        let s = size.to_le_bytes();
        v[0] = s[1];
        v[1] = s[0];
        if size > 0x7FFF {
            v[2] = s[2];
        }
        w.write_all(&v)
    }

    #[cfg(all(feature = "sync", feature = "encryption"))]
    fn write_encrypted_client<W: Write>(
        &self,
        mut w: W,
        e: &mut wow_srp::wrath_header::ClientEncrypterHalf,
    ) -> Result<(), std::io::Error> {
        let mut v = Vec::with_capacity(1024);
        let mut s = &mut v;
        crate::util::wrath_get_unencrypted_client(&mut s, Self::OPCODE as u16, 0)?;
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
            crate::util::wrath_get_unencrypted_client(&mut s, Self::OPCODE as u16, 0)?;
            self.write_into_vec(&mut s)?;
            let size_len = 2;
            let size = v.len().saturating_sub(size_len);
            let s = size.to_le_bytes();
            v[0] = s[1];
            v[1] = s[0];
            if size > 0x7FFF {
                v[2] = s[2];
            }
            w.write_all(&v).await
        })
    }

    #[cfg(all(feature = "tokio", feature = "encryption"))]
    fn tokio_write_encrypted_client<'s, 'e, 'async_trait, W>(
        &'s self,
        mut w: W,
        e: &'e mut wow_srp::wrath_header::ClientEncrypterHalf,
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
            crate::util::wrath_get_unencrypted_client(&mut s, Self::OPCODE as u16, 0)?;
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
            crate::util::wrath_get_unencrypted_client(&mut s, Self::OPCODE as u16, 0)?;
            self.write_into_vec(&mut s)?;
            let size_len = 2;
            let size = v.len().saturating_sub(size_len);
            let s = size.to_le_bytes();
            v[0] = s[1];
            v[1] = s[0];
            if size > 0x7FFF {
                v[2] = s[2];
            }
            w.write_all(&v).await
        })
    }

    #[cfg(all(feature = "async-std", feature = "encryption"))]
    fn astd_write_encrypted_client<'s, 'e, 'async_trait, W>(
        &'s self,
        mut w: W,
        e: &'e mut wow_srp::wrath_header::ClientEncrypterHalf,
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
            crate::util::wrath_get_unencrypted_client(&mut s, Self::OPCODE as u16, 0)?;
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
        4 // client_build: u32
        + 4 // login_server_id: u32
        + self.username.len() + 1 // username: CString
        + 4 // login_server_type: u32
        + 4 // client_seed: u32
        + 4 // region_id: u32
        + 4 // battleground_id: u32
        + 4 // realm_id: u32
        + 8 // dos_response: u64
        + 20 // client_proof: u8[20]
        + crate::util::zlib_compressed_size(&self.addon_info) + 4 // addon_info: u8[-]
    }
}

