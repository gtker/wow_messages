use crate::MessageBody;
use std::future::Future;
#[cfg(feature = "sync")]
use std::io::Write;
use std::pin::Pin;
use wow_srp::header_crypto::Encrypter;

#[cfg(feature = "async-std")]
use async_std::io::WriteExt;
#[cfg(feature = "tokio")]
use tokio::io::AsyncWriteExt;

const SERVER_OPCODE_LENGTH: u16 = 2;
const CLIENT_OPCODE_LENGTH: u16 = 4;

pub trait ServerMessageWrite: MessageBody {
    #[cfg(feature = "sync")]
    fn write_unencrypted_server<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
        crate::util::write_u16_be(
            w,
            self.size_without_size_or_opcode_fields() + SERVER_OPCODE_LENGTH,
        )?;
        crate::util::write_u16_le(w, <Self as MessageBody>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    #[cfg(feature = "sync")]
    fn write_encrypted_server<W: Write, E: Encrypter>(
        &self,
        w: &mut W,
        e: &mut E,
    ) -> Result<(), std::io::Error> {
        e.write_encrypted_server_header(
            w,
            self.size_without_size_or_opcode_fields() + SERVER_OPCODE_LENGTH,
            <Self as MessageBody>::OPCODE,
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
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
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
            crate::util::tokio_write_u16_le(w, <Self as MessageBody>::OPCODE).await?;

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
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        E: 'async_trait + Encrypter + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            let header = e.encrypt_server_header(
                self.size_without_size_or_opcode_fields() + SERVER_OPCODE_LENGTH,
                <Self as MessageBody>::OPCODE,
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
        W: 'async_trait + WriteExt + Unpin + Send,
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
            crate::util::astd_write_u16_le(w, <Self as MessageBody>::OPCODE).await?;

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
        W: 'async_trait + WriteExt + Unpin + Send,
        E: 'async_trait + Encrypter + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            let header = e.encrypt_server_header(
                self.size_without_size_or_opcode_fields() + SERVER_OPCODE_LENGTH,
                <Self as MessageBody>::OPCODE,
            );
            w.write_all(&header);

            self.astd_write_body(w).await?;
            Ok(())
        })
    }
}

pub trait ClientMessageWrite: MessageBody {
    #[cfg(feature = "sync")]
    fn write_unencrypted_client<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
        crate::util::write_u16_be(
            w,
            (self.size_without_size_or_opcode_fields() + CLIENT_OPCODE_LENGTH),
        )?;
        crate::util::write_u32_le(w, <Self as MessageBody>::OPCODE as u32)?;

        self.write_body(w)?;
        Ok(())
    }

    #[cfg(feature = "sync")]
    fn write_encrypted_client<W: Write, E: Encrypter>(
        &self,
        w: &mut W,
        e: &mut E,
    ) -> Result<(), std::io::Error> {
        e.write_encrypted_client_header(
            w,
            (self.size_without_size_or_opcode_fields() + CLIENT_OPCODE_LENGTH),
            <Self as MessageBody>::OPCODE as u32,
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
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
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
            crate::util::tokio_write_u32_le(w, <Self as MessageBody>::OPCODE as u32).await?;

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
        W: 'async_trait + AsyncWriteExt + Unpin + Send,
        E: 'async_trait + Encrypter + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            let header = e.encrypt_client_header(
                self.size_without_size_or_opcode_fields() + 4,
                <Self as MessageBody>::OPCODE as u32,
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
        W: 'async_trait + WriteExt + Unpin + Send,
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
            crate::util::astd_write_u32_le(w, <Self as MessageBody>::OPCODE as u32).await?;

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
        W: 'async_trait + WriteExt + Unpin + Send,
        E: 'async_trait + Encrypter + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            let header = e.encrypt_client_header(
                self.size_without_size_or_opcode_fields() + CLIENT_OPCODE_LENGTH,
                <Self as MessageBody>::OPCODE as u32,
            );
            w.write_all(&header);

            self.astd_write_body(w).await?;
            Ok(())
        })
    }
}
