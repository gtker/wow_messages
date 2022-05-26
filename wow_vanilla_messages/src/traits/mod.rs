use std::future::Future;
use std::pin::Pin;
use wow_srp::header_crypto::Encrypter;

const SERVER_OPCODE_LENGTH: u16 = 2;
const CLIENT_OPCODE_LENGTH: u16 = 4;

pub trait ServerMessage: Sized {
    #[cfg(feature = "sync")]
    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
        crate::util::write_u16_be(
            w,
            self.size_without_size_or_opcode_fields() + SERVER_OPCODE_LENGTH,
        )?;
        crate::util::write_u16_le(w, Self::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    #[cfg(feature = "sync")]
    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(
        &self,
        w: &mut W,
        e: &mut E,
    ) -> Result<(), std::io::Error> {
        e.write_encrypted_server_header(
            w,
            self.size_without_size_or_opcode_fields() + SERVER_OPCODE_LENGTH,
            Self::OPCODE,
        )?;

        self.write_body(w)?;
        Ok(())
    }

    #[cfg(feature = "tokio")]
    fn tokio_write_unencrypted_server<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            crate::util::tokio_write_u16_be(
                w,
                self.size_without_size_or_opcode_fields() + SERVER_OPCODE_LENGTH,
            )
            .await?;
            crate::util::tokio_write_u16_le(w, Self::OPCODE).await?;

            self.tokio_write_body(w).await?;
            Ok(())
        })
    }

    #[cfg(feature = "tokio")]
    fn tokio_write_encrypted_server<'life0, 'life1, 'life2, 'async_trait, W, E>(
        &'life0 self,
        w: &'life1 mut W,
        e: &'life2 mut E,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        E: 'async_trait + Encrypter + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            let header = e.encrypt_server_header(
                self.size_without_size_or_opcode_fields() + SERVER_OPCODE_LENGTH,
                Self::OPCODE,
            );
            w.write_all(&header).await;

            self.tokio_write_body(w).await?;
            Ok(())
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write_unencrypted_server<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            crate::util::astd_write_u16_be(
                w,
                self.size_without_size_or_opcode_fields() + SERVER_OPCODE_LENGTH,
            )
            .await?;
            crate::util::astd_write_u16_le(w, Self::OPCODE).await?;

            self.astd_write_body(w).await?;
            Ok(())
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write_encrypted_server<'life0, 'life1, 'life2, 'async_trait, W, E>(
        &'life0 self,
        w: &'life1 mut W,
        e: &'life2 mut E,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        E: 'async_trait + Encrypter + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            let header = e.encrypt_server_header(
                self.size_without_size_or_opcode_fields() + SERVER_OPCODE_LENGTH,
                Self::OPCODE,
            );
            w.write_all(&header);

            self.astd_write_body(w).await?;
            Ok(())
        })
    }

    const OPCODE: u16;

    fn size_without_size_or_opcode_fields(&self) -> u16;

    type Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> Result<Self, Self::Error>;

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> Result<(), std::io::Error>;

    #[cfg(feature = "async-std")]
    fn astd_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> Pin<Box<dyn Future<Output = Result<Self, Self::Error>> + Send + 'async_trait>>
    where
        R: 'async_trait + async_std::io::ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait;

    #[cfg(feature = "async-std")]
    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait;

    #[cfg(feature = "tokio")]
    fn tokio_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> Pin<Box<dyn Future<Output = Result<Self, Self::Error>> + Send + 'async_trait>>
    where
        R: 'async_trait + tokio::io::AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait;

    #[cfg(feature = "tokio")]
    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait;
}

pub trait ClientMessage: Sized {
    #[cfg(feature = "sync")]
    fn write_unencrypted_client<W: std::io::Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
        crate::util::write_u16_be(
            w,
            (self.size_without_size_or_opcode_fields() + CLIENT_OPCODE_LENGTH),
        )?;
        crate::util::write_u32_le(w, Self::OPCODE as u32)?;

        self.write_body(w)?;
        Ok(())
    }

    #[cfg(feature = "sync")]
    fn write_encrypted_client<W: std::io::Write, E: Encrypter>(
        &self,
        w: &mut W,
        e: &mut E,
    ) -> Result<(), std::io::Error> {
        e.write_encrypted_client_header(
            w,
            (self.size_without_size_or_opcode_fields() + CLIENT_OPCODE_LENGTH),
            Self::OPCODE as u32,
        )?;

        self.write_body(w)?;
        Ok(())
    }

    #[cfg(feature = "tokio")]
    fn tokio_write_unencrypted_client<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            crate::util::tokio_write_u16_be(
                w,
                self.size_without_size_or_opcode_fields() + CLIENT_OPCODE_LENGTH,
            )
            .await?;
            crate::util::tokio_write_u32_le(w, Self::OPCODE as u32).await?;

            self.tokio_write_body(w).await?;
            Ok(())
        })
    }

    #[cfg(feature = "tokio")]
    fn tokio_write_encrypted_client<'life0, 'life1, 'life2, 'async_trait, W, E>(
        &'life0 self,
        w: &'life1 mut W,
        e: &'life2 mut E,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        E: 'async_trait + Encrypter + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            let header = e.encrypt_client_header(
                self.size_without_size_or_opcode_fields() + 4,
                Self::OPCODE as u32,
            );
            w.write_all(&header);

            self.tokio_write_body(w).await?;
            Ok(())
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write_unencrypted_client<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            crate::util::astd_write_u16_be(
                w,
                self.size_without_size_or_opcode_fields() + CLIENT_OPCODE_LENGTH,
            )
            .await?;
            crate::util::astd_write_u32_le(w, Self::OPCODE as u32).await?;

            self.astd_write_body(w).await?;
            Ok(())
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write_encrypted_client<'life0, 'life1, 'life2, 'async_trait, W, E>(
        &'life0 self,
        w: &'life1 mut W,
        e: &'life2 mut E,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        E: 'async_trait + Encrypter + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            let header = e.encrypt_client_header(
                self.size_without_size_or_opcode_fields() + CLIENT_OPCODE_LENGTH,
                Self::OPCODE as u32,
            );
            w.write_all(&header);

            self.astd_write_body(w).await?;
            Ok(())
        })
    }

    const OPCODE: u16;

    fn size_without_size_or_opcode_fields(&self) -> u16;

    type Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> Result<Self, Self::Error>;

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> Result<(), std::io::Error>;

    #[cfg(feature = "async-std")]
    fn astd_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> Pin<Box<dyn Future<Output = Result<Self, Self::Error>> + Send + 'async_trait>>
    where
        R: 'async_trait + async_std::io::ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait;

    #[cfg(feature = "async-std")]
    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait;

    #[cfg(feature = "tokio")]
    fn tokio_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> Pin<Box<dyn Future<Output = Result<Self, Self::Error>> + Send + 'async_trait>>
    where
        R: 'async_trait + tokio::io::AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait;

    #[cfg(feature = "tokio")]
    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait;
}
