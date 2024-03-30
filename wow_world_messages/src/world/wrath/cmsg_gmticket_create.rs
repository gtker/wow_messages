use crate::Message;
use std::io::{Read, Write};

use crate::wrath::{
    Map, Vector3d,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/cmsg_gmticket_create.wowm:19`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/cmsg_gmticket_create.wowm#L19):
/// ```text
/// cmsg CMSG_GMTICKET_CREATE = 0x0205 {
///     Map map;
///     Vector3d position;
///     CString message;
///     Bool needs_response;
///     Bool needs_more_help;
///     u32 num_of_times;
///     u32[num_of_times] times;
///     u8[-] compressed_data;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct CMSG_GMTICKET_CREATE {
    pub map: Map,
    pub position: Vector3d,
    pub message: String,
    pub needs_response: bool,
    pub needs_more_help: bool,
    pub times: Vec<u32>,
    pub compressed_data: Vec<u8>,
}

impl crate::private::Sealed for CMSG_GMTICKET_CREATE {}
impl CMSG_GMTICKET_CREATE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(27..=10240).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // map: Map
        let map = crate::util::read_u32_le(&mut r)?.try_into()?;

        // position: Vector3d
        let position = crate::util::vanilla_tbc_wrath_vector3d_read(&mut r)?;

        // message: CString
        let message = {
            let message = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(message)?
        };

        // needs_response: Bool
        let needs_response = crate::util::read_bool_u8(&mut r)?;

        // needs_more_help: Bool
        let needs_more_help = crate::util::read_bool_u8(&mut r)?;

        // num_of_times: u32
        let num_of_times = crate::util::read_u32_le(&mut r)?;

        // times: u32[num_of_times]
        let times = {
            let mut times = Vec::with_capacity(num_of_times as usize);

            let allocation_size = u64::from(num_of_times) * 4;
            if allocation_size > crate::errors::MAX_ALLOCATION_SIZE_WRATH {
                return Err(crate::errors::ParseErrorKind::AllocationTooLargeError(allocation_size));
            }

            for _ in 0..num_of_times {
                times.push(crate::util::read_u32_le(&mut r)?);
            }
            times
        };

        // compressed_data: u8[-]
        let compressed_data = {
            let compressed_data_decompressed_size = crate::util::read_u32_le(&mut r)?;

            let mut buf = Vec::with_capacity(compressed_data_decompressed_size as usize);
            let mut decoder = &mut flate2::read::ZlibDecoder::new(r);
            decoder.read_to_end(&mut buf).unwrap();
            let mut r = &buf[..];

            let mut current_size = {
                4 // map: Map
                + 12 // position: Vector3d
                + message.len() + 1 // message: CString
                + 1 // needs_response: Bool
                + 1 // needs_more_help: Bool
                + 4 // num_of_times: u32
                + times.len() * core::mem::size_of::<u32>() // times: u32[num_of_times]
            };
            current_size += 4; // compressed_data_decompressed_size: u32
            let mut compressed_data = Vec::with_capacity(body_size as usize - current_size);
            while !r.is_empty() {
                compressed_data.push(crate::util::read_u8_le(&mut r)?);
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
            compressed_data,
        })
    }

}

impl crate::Message for CMSG_GMTICKET_CREATE {
    const OPCODE: u32 = 0x0205;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_GMTICKET_CREATE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_GMTICKET_CREATE {{").unwrap();
        // Members
        writeln!(s, "    map = {};", self.map.as_test_case_value()).unwrap();
        // position: Vector3d
        writeln!(s, "    position = {{").unwrap();
        // Members
        writeln!(s, "        x = {};", if self.position.x.to_string().contains('.') { self.position.x.to_string() } else { format!("{}.0", self.position.x) }).unwrap();
        writeln!(s, "        y = {};", if self.position.y.to_string().contains('.') { self.position.y.to_string() } else { format!("{}.0", self.position.y) }).unwrap();
        writeln!(s, "        z = {};", if self.position.z.to_string().contains('.') { self.position.z.to_string() } else { format!("{}.0", self.position.z) }).unwrap();

        writeln!(s, "    }};").unwrap();
        writeln!(s, "    message = \"{}\";", self.message).unwrap();
        writeln!(s, "    needs_response = {};", if self.needs_response { "TRUE" } else { "FALSE" }).unwrap();
        writeln!(s, "    needs_more_help = {};", if self.needs_more_help { "TRUE" } else { "FALSE" }).unwrap();
        writeln!(s, "    num_of_times = {};", self.times.len()).unwrap();
        writeln!(s, "    times = [").unwrap();
        for v in self.times.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "    ];").unwrap();
        writeln!(s, "    compressed_data = [").unwrap();
        for v in self.compressed_data.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "    ];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 517_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "map", "    ");
        writeln!(s, "    /* position: Vector3d start */").unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, 4, "x", "        ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "y", "        ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "z", "        ");
        writeln!(s, "    /* position: Vector3d end */").unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, self.message.len() + 1, "message", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "needs_response", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "needs_more_help", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "num_of_times", "    ");
        if !self.times.is_empty() {
            writeln!(s, "    /* times: u32[num_of_times] start */").unwrap();
            for (i, v) in self.times.iter().enumerate() {
                crate::util::write_bytes(&mut s, &mut bytes, 4, &format!("times {i}"), "    ");
            }
            writeln!(s, "    /* times: u32[num_of_times] end */").unwrap();
        }
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
        // map: Map
        w.write_all(&(self.map.as_int().to_le_bytes()))?;

        // position: Vector3d
        crate::util::vanilla_tbc_wrath_vector3d_write_into_vec(&self.position, &mut w)?;

        // message: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.message.as_bytes().iter().next_back(), Some(&0_u8), "String `message` must not be null-terminated.");
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
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(517, "CMSG_GMTICKET_CREATE", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_GMTICKET_CREATE {
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

impl CMSG_GMTICKET_CREATE {
    pub(crate) fn size(&self) -> usize {
        4 // map: Map
        + 12 // position: Vector3d
        + self.message.len() + 1 // message: CString
        + 1 // needs_response: Bool
        + 1 // needs_more_help: Bool
        + 4 // num_of_times: u32
        + self.times.len() * core::mem::size_of::<u32>() // times: u32[num_of_times]
        + crate::util::zlib_compressed_size(&self.compressed_data) + 4 // compressed_data: u8[-]
    }
}

