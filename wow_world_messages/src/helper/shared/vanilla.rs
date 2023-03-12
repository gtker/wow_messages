#[cfg(feature = "encryption")]
use crate::util::{vanilla_get_encrypted_client, vanilla_get_encrypted_server};
use crate::util::{vanilla_get_unencrypted_client, vanilla_get_unencrypted_server};
use crate::util::{CLIENT_HEADER_LENGTH, SERVER_HEADER_LENGTH};
use crate::Message;
#[cfg(any(feature = "tokio", feature = "async-std"))]
use std::future::Future;
#[cfg(any(feature = "tokio", feature = "async-std"))]
use std::pin::Pin;
#[cfg(feature = "encryption")]
use wow_srp::vanilla_header::EncrypterHalf;

pub trait ServerMessage: Message {
    /// Total size the message takes up including header.
    /// This is not the same value as what goes into the size field
    /// since the size field does not include the size of the size field.
    fn server_size(&self) -> u16 {
        self.size_without_header() as u16 + SERVER_HEADER_LENGTH
    }

    #[cfg(feature = "sync")]
    fn write_unencrypted_server<W: std::io::Write>(&self, mut w: W) -> Result<(), std::io::Error> {
        let size = self.server_size();
        let mut v = Vec::with_capacity(size.into());
        vanilla_get_unencrypted_server(&mut v, Self::OPCODE as u16, size)?;
        self.write_into_vec(&mut v)?;

        assert_eq!(size, v.len() as u16);

        w.write_all(&v)
    }

    #[cfg(all(feature = "sync", feature = "encryption"))]
    fn write_encrypted_server<W: std::io::Write>(
        &self,
        mut w: W,
        e: &mut EncrypterHalf,
    ) -> Result<(), std::io::Error> {
        let size = self.server_size();
        let mut v = Vec::with_capacity(size.into());
        vanilla_get_encrypted_server(&mut v, Self::OPCODE as u16, size, e)?;

        self.write_into_vec(&mut v)?;
        assert_eq!(size, v.len() as u16);

        w.write_all(&v)
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
            let mut v = Vec::with_capacity(size.into());
            vanilla_get_unencrypted_server(&mut v, Self::OPCODE as u16, size)?;
            self.write_into_vec(&mut v)?;

            assert_eq!(size, v.len() as u16);

            w.write_all(&v).await
        })
    }

    #[cfg(all(feature = "tokio", feature = "encryption"))]
    fn tokio_write_encrypted_server<'s, 'e, 'async_trait, W>(
        &'s self,
        mut w: W,
        e: &'e mut EncrypterHalf,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        's: 'async_trait,
        'e: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            let size = self.server_size();
            let mut v = Vec::with_capacity(size.into());
            vanilla_get_encrypted_server(&mut v, Self::OPCODE as u16, size, e)?;
            self.write_into_vec(&mut v)?;
            assert_eq!(size, v.len() as u16);

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
            let mut v = Vec::with_capacity(size.into());
            vanilla_get_unencrypted_server(&mut v, Self::OPCODE as u16, size)?;
            self.write_into_vec(&mut v)?;

            assert_eq!(size, v.len() as u16);

            w.write_all(&v).await
        })
    }

    #[cfg(all(feature = "async-std", feature = "encryption"))]
    fn astd_write_encrypted_server<'s, 'e, 'async_trait, W>(
        &'s self,
        mut w: W,
        e: &'e mut EncrypterHalf,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        's: 'async_trait,
        'e: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            let size = self.server_size();
            let mut v = Vec::with_capacity(size.into());
            vanilla_get_encrypted_server(&mut v, Self::OPCODE as u16, size, e)?;
            self.write_into_vec(&mut v)?;

            assert_eq!(size, v.len() as u16);

            w.write_all(&v).await
        })
    }
}

pub trait ClientMessage: Message {
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
        vanilla_get_unencrypted_client(&mut v, Self::OPCODE as u16, size)?;
        self.write_into_vec(&mut v)?;
        assert_eq!(size, v.len() as u16);

        w.write_all(&v)
    }

    #[cfg(all(feature = "sync", feature = "encryption"))]
    fn write_encrypted_client<W: std::io::Write>(
        &self,
        mut w: W,
        e: &mut EncrypterHalf,
    ) -> Result<(), std::io::Error> {
        let size = self.client_size();
        let mut v = Vec::with_capacity(size.into());
        vanilla_get_encrypted_client(&mut v, Self::OPCODE as u16, size, e)?;
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
            vanilla_get_unencrypted_client(&mut v, Self::OPCODE as u16, size)?;
            self.write_into_vec(&mut v)?;

            assert_eq!(size, v.len() as u16);

            w.write_all(&v).await
        })
    }

    #[cfg(all(feature = "encryption", feature = "tokio"))]
    fn tokio_write_encrypted_client<'s, 'e, 'async_trait, W>(
        &'s self,
        mut w: W,
        e: &'e mut EncrypterHalf,
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
            vanilla_get_encrypted_client(&mut v, Self::OPCODE as u16, size, e)?;
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
            vanilla_get_unencrypted_client(&mut v, Self::OPCODE as u16, size)?;
            self.write_into_vec(&mut v)?;

            assert_eq!(size, v.len() as u16);

            w.write_all(&v).await
        })
    }

    #[cfg(all(feature = "async-std", feature = "encryption"))]
    fn astd_write_encrypted_client<'s, 'e, 'async_trait, W>(
        &'s self,
        mut w: W,
        e: &'e mut EncrypterHalf,
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
            vanilla_get_encrypted_client(&mut v, Self::OPCODE as u16, size, e)?;
            self.write_into_vec(&mut v)?;

            assert_eq!(size, v.len() as u16);

            w.write_all(&v).await
        })
    }
}
