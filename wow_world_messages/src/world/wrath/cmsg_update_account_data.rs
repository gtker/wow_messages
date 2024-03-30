use crate::Message;
use std::io::{Read, Write};

/// Respond with [`SMSG_UPDATE_ACCOUNT_DATA_COMPLETE`](crate::wrath::SMSG_UPDATE_ACCOUNT_DATA_COMPLETE)
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/cmsg_update_account_data.wowm:27`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/cmsg_update_account_data.wowm#L27):
/// ```text
/// cmsg CMSG_UPDATE_ACCOUNT_DATA = 0x020B {
///     u32 data_type;
///     u32 unix_time;
///     u8[-] compressed_data;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_UPDATE_ACCOUNT_DATA {
    /// You can check this against the `CacheMask` to find out if this is character-specific data or account-wide data
    pub data_type: u32,
    /// Seconds since unix epoch. The client wants this number back when it requests the ACCOUNT_DATA_TIMES
    pub unix_time: u32,
    /// Compressed account data (macros, keybinds, etc). The server does not actually care about the uncompressed contents. It only needs to send this back to the client. The server acts as a cross-device storage
    pub compressed_data: Vec<u8>,
}

impl crate::private::Sealed for CMSG_UPDATE_ACCOUNT_DATA {}
impl CMSG_UPDATE_ACCOUNT_DATA {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(12..=65547).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // data_type: u32
        let data_type = crate::util::read_u32_le(&mut r)?;

        // unix_time: u32
        let unix_time = crate::util::read_u32_le(&mut r)?;

        // compressed_data: u8[-]
        let compressed_data = {
            let compressed_data_decompressed_size = crate::util::read_u32_le(&mut r)?;

            let mut buf = Vec::with_capacity(compressed_data_decompressed_size as usize);
            let mut decoder = &mut flate2::read::ZlibDecoder::new(r);
            decoder.read_to_end(&mut buf).unwrap();
            let mut r = &buf[..];

            let mut current_size = {
                4 // data_type: u32
                + 4 // unix_time: u32
            };
            current_size += 4; // compressed_data_decompressed_size: u32
            let mut compressed_data = Vec::with_capacity(body_size as usize - current_size);
            while !r.is_empty() {
                compressed_data.push(crate::util::read_u8_le(&mut r)?);
            }
            compressed_data
        };

        Ok(Self {
            data_type,
            unix_time,
            compressed_data,
        })
    }

}

impl crate::Message for CMSG_UPDATE_ACCOUNT_DATA {
    const OPCODE: u32 = 0x020b;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_UPDATE_ACCOUNT_DATA"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_UPDATE_ACCOUNT_DATA {{").unwrap();
        // Members
        writeln!(s, "    data_type = {};", self.data_type).unwrap();
        writeln!(s, "    unix_time = {};", self.unix_time).unwrap();
        writeln!(s, "    compressed_data = [").unwrap();
        for v in self.compressed_data.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "    ];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 523_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "data_type", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unix_time", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.compressed_data.len(), "compressed_data", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // data_type: u32
        w.write_all(&self.data_type.to_le_bytes())?;

        // unix_time: u32
        w.write_all(&self.unix_time.to_le_bytes())?;

        // compressed_data: u8[-]
        let decompressed_size: u32 = 1 * self.compressed_data.len() as u32;
        w.write_all(&decompressed_size.to_le_bytes())?;
        let mut encoder = flate2::write::ZlibEncoder::new(w, flate2::Compression::default());
        for i in self.compressed_data.iter() {
            encoder.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(523, "CMSG_UPDATE_ACCOUNT_DATA", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_UPDATE_ACCOUNT_DATA {
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

impl CMSG_UPDATE_ACCOUNT_DATA {
    pub(crate) fn size(&self) -> usize {
        4 // data_type: u32
        + 4 // unix_time: u32
        + crate::util::zlib_compressed_size(&self.compressed_data) + 4 // compressed_data: u8[-]
    }
}

