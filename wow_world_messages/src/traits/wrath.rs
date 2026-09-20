#[cfg(feature = "encryption")]
use crate::util::{wrath_get_encrypted_client, wrath_get_encrypted_server};
use crate::util::{wrath_get_unencrypted_client, wrath_get_unencrypted_server};
use crate::util::{
    CLIENT_HEADER_LENGTH, LARGE_MESSAGE_THRESHOLD, MAXIMUM_SERVER_HEADER_LENGTH,
    MAXIMUM_SIZE_LENGTH, MINIMUM_SERVER_HEADER_LENGTH, MINIMUM_SIZE_LENGTH,
};
use crate::Message;
#[cfg(any(feature = "tokio", feature = "async-std"))]
use std::future::Future;
#[cfg(any(feature = "tokio", feature = "async-std"))]
use std::pin::Pin;
#[cfg(feature = "encryption")]
use wow_srp::wrath_header::{ClientEncrypterHalf, ServerEncrypterHalf};

pub trait ServerMessage: Message + crate::traits::private::Sealed {
    /// Total size the message takes up including header.
    /// This is not the same value as what goes into the size field
    /// since the size field does not include the size of the size field.
    fn server_size(&self) -> u32 {
        let size = self.size_without_header();
        if size > LARGE_MESSAGE_THRESHOLD {
            size + MAXIMUM_SERVER_HEADER_LENGTH as u32
        } else {
            size + MINIMUM_SERVER_HEADER_LENGTH as u32
        }
    }

    #[cfg(feature = "sync")]
    fn write_unencrypted_server<W: std::io::Write>(&self, mut w: W) -> Result<(), std::io::Error> {
        let size = self.server_size();
        let mut v = Vec::with_capacity(size as usize);
        wrath_get_unencrypted_server(&mut v, Self::OPCODE as u16, size)?;
        self.write_into_vec(&mut v)?;
        assert_eq!(size, v.len() as u32);

        w.write_all(&v)
    }

    #[cfg(all(feature = "sync", feature = "encryption"))]
    fn write_encrypted_server<W: std::io::Write>(
        &self,
        mut w: W,
        e: &mut ServerEncrypterHalf,
    ) -> Result<(), std::io::Error> {
        let size = self.server_size();
        let mut v = Vec::with_capacity(size as usize);
        wrath_get_encrypted_server(&mut v, Self::OPCODE as u16, size, e)?;

        self.write_into_vec(&mut v)?;
        assert_eq!(size, v.len() as u32);

        w.write_all(&v)
    }

    /// Writes this typed message into a caller-provided buffer whose length is
    /// exactly [`Self::server_size`].
    ///
    /// The caller must establish the exact-size contract before calling this
    /// method. The header is encrypted in place through `wow_srp`, and the
    /// generated typed body writer writes directly into the remaining slice;
    /// no packet-sized allocation or fallible `Write` boundary is needed.
    #[cfg(feature = "encryption")]
    fn write_encrypted_server_into_exact(
        &self,
        buffer: &mut [u8],
        e: &mut ServerEncrypterHalf,
    ) {
        let size = self.server_size();
        assert_eq!(buffer.len(), size as usize);

        let size_length = if size > LARGE_MESSAGE_THRESHOLD {
            MAXIMUM_SIZE_LENGTH
        } else {
            MINIMUM_SIZE_LENGTH
        };
        let header = e.encrypt_server_header(size.saturating_sub(size_length), Self::OPCODE as u16);
        let header_size = header.len();
        buffer[..header_size].copy_from_slice(header);

        let mut body = &mut buffer[header_size..];
        self.write_into_vec(&mut body)
            .expect("typed message body must fit its exact-size buffer");
        assert!(body.is_empty());
    }

    #[cfg(feature = "tokio")]
    fn tokio_write_unencrypted_server<'s, 'async_trait, W>(
        &'s self,
        mut w: W,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        's: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            let size = self.server_size();
            let mut v = Vec::with_capacity(size as usize);
            wrath_get_unencrypted_server(&mut v, Self::OPCODE as u16, size)?;
            self.write_into_vec(&mut v)?;
            assert_eq!(size, v.len() as u32);

            w.write_all(&v).await
        })
    }

    #[cfg(all(feature = "tokio", feature = "encryption"))]
    fn tokio_write_encrypted_server<'s, 'e, 'async_trait, W>(
        &'s self,
        mut w: W,
        e: &'e mut ServerEncrypterHalf,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        's: 'async_trait,
        'e: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            let size = self.server_size();
            let mut v = Vec::with_capacity(size as usize);
            wrath_get_encrypted_server(&mut v, Self::OPCODE as u16, size, e)?;
            self.write_into_vec(&mut v)?;
            assert_eq!(size, v.len() as u32);

            w.write_all(&v).await
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write_unencrypted_server<'s, 'async_trait, W>(
        &'s self,
        mut w: W,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        's: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            let size = self.server_size();
            let mut v = Vec::with_capacity(size as usize);
            wrath_get_unencrypted_server(&mut v, Self::OPCODE as u16, size)?;
            self.write_into_vec(&mut v)?;
            assert_eq!(size, v.len() as u32);

            w.write_all(&v).await
        })
    }

    #[cfg(all(feature = "async-std", feature = "encryption"))]
    fn astd_write_encrypted_server<'s, 'e, 'async_trait, W>(
        &'s self,
        mut w: W,
        e: &'e mut ServerEncrypterHalf,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        's: 'async_trait,
        'e: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            let size = self.server_size();
            let mut v = Vec::with_capacity(size as usize);
            wrath_get_encrypted_server(&mut v, Self::OPCODE as u16, size, e)?;
            self.write_into_vec(&mut v)?;
            assert_eq!(size, v.len() as u32);

            w.write_all(&v).await
        })
    }
}

pub trait ClientMessage: Message + crate::traits::private::Sealed {
    /// Total size the message takes up including header.
    /// This is not the same value as what goes into the size field
    /// since the size field does not include the size of the size field.
    fn client_size(&self) -> u16 {
        self.size_without_header() as u16 + CLIENT_HEADER_LENGTH
    }

    #[cfg(feature = "sync")]
    fn write_unencrypted_client<W: std::io::Write>(&self, mut w: W) -> Result<(), std::io::Error> {
        let size = self.client_size();
        let mut v = Vec::with_capacity(size.into());
        wrath_get_unencrypted_client(&mut v, Self::OPCODE as u16, size)?;
        self.write_into_vec(&mut v)?;
        assert_eq!(size, v.len() as u16);

        w.write_all(&v)
    }

    #[cfg(all(feature = "sync", feature = "encryption"))]
    fn write_encrypted_client<W: std::io::Write>(
        &self,
        mut w: W,
        e: &mut ClientEncrypterHalf,
    ) -> Result<(), std::io::Error> {
        let size = self.client_size();
        let mut v = Vec::with_capacity(size.into());
        wrath_get_encrypted_client(&mut v, Self::OPCODE as u16, size, e)?;
        self.write_into_vec(&mut v)?;
        assert_eq!(size, v.len() as u16);

        w.write_all(&v)
    }

    #[cfg(feature = "tokio")]
    fn tokio_write_unencrypted_client<'s, 'async_trait, W>(
        &'s self,
        mut w: W,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        's: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            let size = self.client_size();
            let mut v = Vec::with_capacity(size.into());
            wrath_get_unencrypted_client(&mut v, Self::OPCODE as u16, size)?;
            self.write_into_vec(&mut v)?;
            assert_eq!(size, v.len() as u16);

            w.write_all(&v).await
        })
    }

    #[cfg(all(feature = "tokio", feature = "encryption"))]
    fn tokio_write_encrypted_client<'s, 'e, 'async_trait, W>(
        &'s self,
        mut w: W,
        e: &'e mut ClientEncrypterHalf,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        's: 'async_trait,
        'e: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            let size = self.client_size();
            let mut v = Vec::with_capacity(size.into());
            wrath_get_encrypted_client(&mut v, Self::OPCODE as u16, size, e)?;
            self.write_into_vec(&mut v)?;
            assert_eq!(size, v.len() as u16);

            w.write_all(&v).await
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write_unencrypted_client<'s, 'async_trait, W>(
        &'s self,
        mut w: W,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        's: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            let size = self.client_size();
            let mut v = Vec::with_capacity(size.into());
            wrath_get_unencrypted_client(&mut v, Self::OPCODE as u16, size)?;
            self.write_into_vec(&mut v)?;
            assert_eq!(size, v.len() as u16);

            w.write_all(&v).await
        })
    }

    #[cfg(all(feature = "async-std", feature = "encryption"))]
    fn astd_write_encrypted_client<'s, 'e, 'async_trait, W>(
        &'s self,
        mut w: W,
        e: &'e mut ClientEncrypterHalf,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        's: 'async_trait,
        'e: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            let size = self.client_size();
            let mut v = Vec::with_capacity(size.into());
            wrath_get_encrypted_client(&mut v, Self::OPCODE as u16, size, e)?;
            self.write_into_vec(&mut v)?;
            assert_eq!(size, v.len() as u16);

            w.write_all(&v).await
        })
    }
}
