use crate::Message;
use crate::ServerMessage;
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/login/cmd_xfer.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/login/cmd_xfer.wowm#L3):
/// ```text
/// slogin CMD_XFER_INITIATE = 0x30 {
///     String filename;
///     u64 file_size;
///     u8[16] file_md5;
/// }
/// ```
pub struct CMD_XFER_INITIATE {
    pub filename: String,
    pub file_size: u64,
    pub file_md5: [u8; 16],
}

impl CMD_XFER_INITIATE {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // opcode: u8
        w.write_all(&Self::OPCODE.to_le_bytes())?;

        // filename: String
        w.write_all(&(self.filename.len() as u8).to_le_bytes())?;
        w.write_all(self.filename.as_bytes())?;

        // file_size: u64
        w.write_all(&self.file_size.to_le_bytes())?;

        // file_md5: u8[16]
        for i in self.file_md5.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
}

impl crate::private::Sealed for CMD_XFER_INITIATE {}

impl CMD_XFER_INITIATE {
    #[cfg(feature = "sync")]
    fn read_inner<R: std::io::Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // filename: String
        let filename = {
            let filename = crate::util::read_u8_le(&mut r)?;
            let filename = crate::util::read_fixed_string_to_vec(&mut r, filename as usize)?;
            String::from_utf8(filename)?
        };

        // file_size: u64
        let file_size = crate::util::read_u64_le(&mut r)?;

        // file_md5: u8[16]
        let file_md5 = {
            let mut file_md5 = [0_u8; 16];
            r.read_exact(&mut file_md5)?;
            file_md5
        };

        Ok(Self {
            filename,
            file_size,
            file_md5,
        })
    }

    #[cfg(feature = "tokio")]
    async fn tokio_read_inner<R: tokio::io::AsyncReadExt + Unpin + Send>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // filename: String
        let filename = {
            let filename = crate::util::tokio_read_u8_le(&mut r).await?;
            let filename = crate::util::tokio_read_fixed_string_to_vec(&mut r, filename as usize).await?;
            String::from_utf8(filename)?
        };

        // file_size: u64
        let file_size = crate::util::tokio_read_u64_le(&mut r).await?;

        // file_md5: u8[16]
        let file_md5 = {
            let mut file_md5 = [0_u8; 16];
            r.read_exact(&mut file_md5).await?;
            file_md5
        };

        Ok(Self {
            filename,
            file_size,
            file_md5,
        })
    }

    #[cfg(feature = "async-std")]
    async fn astd_read_inner<R: async_std::io::ReadExt + Unpin + Send>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // filename: String
        let filename = {
            let filename = crate::util::astd_read_u8_le(&mut r).await?;
            let filename = crate::util::astd_read_fixed_string_to_vec(&mut r, filename as usize).await?;
            String::from_utf8(filename)?
        };

        // file_size: u64
        let file_size = crate::util::astd_read_u64_le(&mut r).await?;

        // file_md5: u8[16]
        let file_md5 = {
            let mut file_md5 = [0_u8; 16];
            r.read_exact(&mut file_md5).await?;
            file_md5
        };

        Ok(Self {
            filename,
            file_size,
            file_md5,
        })
    }

}

impl Message for CMD_XFER_INITIATE {
    const OPCODE: u8 = 0x30;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;

        let mut s = String::new();

        writeln!(s, "test CMD_XFER_INITIATE {{").unwrap();
        // Members
        writeln!(s, "    filename = \"{}\";", self.filename).unwrap();
        writeln!(s, "    file_size = {};", self.file_size).unwrap();
        writeln!(s, "    file_md5 = [").unwrap();
        for v in self.file_md5.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "    ];").unwrap();

        writeln!(s, "}} [").unwrap();

        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        writeln!(s, "    {:#04X}, /* opcode */ ", bytes.next().unwrap()).unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, self.filename.len() + 1, "filename", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "file_size", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, self.file_md5.len(), "file_md5", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    login_versions = \"{}\";", std::env::var("WOWM_TEST_CASE_LOGIN_VERSION").unwrap_or("2 3 5 6 7 8".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    #[cfg(feature = "sync")]
    fn read<R: std::io::Read, I: crate::private::Sealed>(r: R) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r).map_err(|kind| crate::errors::ParseError::new(48, "CMD_XFER_INITIATE", kind))
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
        Box::pin(async move {Self::tokio_read_inner(r).await.map_err(|kind| crate::errors::ParseError::new(48, "CMD_XFER_INITIATE", kind))})
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
        Box::pin(async move {Self::astd_read_inner(r).await.map_err(|kind| crate::errors::ParseError::new(48, "CMD_XFER_INITIATE", kind))})
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

impl ServerMessage for CMD_XFER_INITIATE {}
impl CMD_XFER_INITIATE {
    pub(crate) fn size(&self) -> usize {
        self.filename.len() + 1 // filename: String
        + 8 // file_size: u64
        + 16 // file_md5: u8[16]
    }
}

