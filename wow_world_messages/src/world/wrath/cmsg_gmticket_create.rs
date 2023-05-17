use crate::Message;
use std::io::{Read, Write};

use crate::wrath::{
    Map, Vector3d,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/cmsg_gmticket_create.wowm:21`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/cmsg_gmticket_create.wowm#L21):
/// ```text
/// cmsg CMSG_GMTICKET_CREATE = 0x0205 {
///     Map map;
///     Vector3d position;
///     CString message;
///     Bool needs_response;
///     Bool needs_more_help;
///     u32 num_of_times;
///     u32[num_of_times] times;
///     u32 decompressed_size;
///     u8[-] compressed_data;
/// }
/// ```
pub struct CMSG_GMTICKET_CREATE {
    pub map: Map,
    pub position: Vector3d,
    pub message: String,
    pub needs_response: bool,
    pub needs_more_help: bool,
    pub times: Vec<u32>,
    pub decompressed_size: u32,
    pub compressed_data: Vec<u8>,
}

impl crate::private::Sealed for CMSG_GMTICKET_CREATE {}
impl crate::Message for CMSG_GMTICKET_CREATE {
    const OPCODE: u32 = 0x0205;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&(self.map.as_int().to_le_bytes()))?;

        // position: Vector3d
        self.position.write_into_vec(&mut w)?;

        // message: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.message.as_bytes().iter().rev().next(), Some(&0_u8), "String `message` must not be null-terminated.");
        w.write_all(self.message.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // needs_response: Bool
        w.write_all(u8::from(self.needs_response).to_le_bytes().as_slice())?;

        // needs_more_help: Bool
        w.write_all(u8::from(self.needs_more_help).to_le_bytes().as_slice())?;

        // num_of_times: u32
        w.write_all(&(self.times.len() as u32).to_le_bytes())?;

        // times: u32[num_of_times]
        for i in self.times.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        // decompressed_size: u32
        w.write_all(&self.decompressed_size.to_le_bytes())?;

        // compressed_data: u8[-]
        let mut encoder = flate2::write::ZlibEncoder::new(w, flate2::Compression::default());
        for i in self.compressed_data.iter() {
            encoder.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(27..=10240).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0205, size: body_size });
        }

        // map: Map
        let map: Map = crate::util::read_u32_le(&mut r)?.try_into()?;

        // position: Vector3d
        let position = Vector3d::read(&mut r)?;

        // message: CString
        let message = {
            let message = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(message)?
        };

        // needs_response: Bool
        let needs_response = crate::util::read_u8_le(&mut r)? != 0;

        // needs_more_help: Bool
        let needs_more_help = crate::util::read_u8_le(&mut r)? != 0;

        // num_of_times: u32
        let num_of_times = crate::util::read_u32_le(&mut r)?;

        // times: u32[num_of_times]
        let times = {
            let mut times = Vec::with_capacity(num_of_times as usize);
            for i in 0..num_of_times {
                times.push(crate::util::read_u32_le(&mut r)?);
            }
            times
        };

        // decompressed_size: u32
        let decompressed_size = crate::util::read_u32_le(&mut r)?;

        // compressed_data: u8[-]
        let compressed_data = {
            let mut decoder = &mut flate2::read::ZlibDecoder::new(r);

            let mut current_size = {
                4 // map: Map
                + 12 // position: Vector3d
                + message.len() + 1 // message: CString
                + 1 // needs_response: Bool
                + 1 // needs_more_help: Bool
                + 4 // num_of_times: u32
                + times.len() * core::mem::size_of::<u32>() // times: u32[num_of_times]
                + 4 // decompressed_size: u32
            };
            let mut compressed_data = Vec::with_capacity(body_size as usize - current_size);
            while decoder.total_out() < (decompressed_size as u64) {
                compressed_data.push(crate::util::read_u8_le(&mut decoder)?);
                current_size += 1;
            }
            compressed_data
        };

        Ok(Self {
            map,
            position,
            message,
            needs_response,
            needs_more_help,
            times,
            decompressed_size,
            compressed_data,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_GMTICKET_CREATE {
    #[cfg(feature = "sync")]
    fn write_unencrypted_client<W: std::io::Write>(&self, mut w: W) -> Result<(), std::io::Error> {
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
    fn write_encrypted_client<W: std::io::Write>(
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

impl CMSG_GMTICKET_CREATE {
    pub(crate) fn size(&self) -> usize {
        4 // map: Map
        + 12 // position: Vector3d
        + self.message.len() + 1 // message: CString
        + 1 // needs_response: Bool
        + 1 // needs_more_help: Bool
        + 4 // num_of_times: u32
        + self.times.len() * core::mem::size_of::<u32>() // times: u32[num_of_times]
        + 4 // decompressed_size: u32
        + crate::util::zlib_compressed_size(&self.compressed_data) // compressed_data: u8[-]
    }
}

