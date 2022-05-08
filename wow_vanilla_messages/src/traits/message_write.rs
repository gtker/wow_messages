use crate::MessageBody;
#[cfg(feature = "sync")]
use std::io::Write;
use wow_srp::header_crypto::Encrypter;

#[cfg(feature = "async_std")]
use async_std::io::WriteExt;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::AsyncWriteExt;

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
pub trait ServerMessageWrite: MessageBody {
    const OPCODE_LENGTH: u16 = 2;

    #[cfg(feature = "sync")]
    fn write_unencrypted_server<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
        crate::util::write_u16_be(
            w,
            self.size_without_size_or_opcode_fields() + Self::OPCODE_LENGTH,
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
            self.size_without_size_or_opcode_fields() + Self::OPCODE_LENGTH,
            <Self as MessageBody>::OPCODE,
        )?;

        self.write_body(w)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_unencrypted_server<W: AsyncWriteExt + Unpin + Send>(
        &self,
        w: &mut W,
    ) -> Result<(), std::io::Error> {
        crate::util::tokio_write_u16_be(
            w,
            self.size_without_size_or_opcode_fields() + Self::OPCODE_LENGTH,
        )
        .await?;
        crate::util::tokio_write_u16_le(w, <Self as MessageBody>::OPCODE).await?;

        self.tokio_write_body(w).await?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_encrypted_server<W: AsyncWriteExt + Unpin + Send, E: Encrypter + Send>(
        &self,
        w: &mut W,
        e: &mut E,
    ) -> Result<(), std::io::Error> {
        let header = e.encrypt_server_header(
            self.size_without_size_or_opcode_fields() + Self::OPCODE_LENGTH,
            <Self as MessageBody>::OPCODE,
        );
        w.write_all(&header);

        self.tokio_write_body(w).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_unencrypted_server<W: WriteExt + Unpin + Send>(
        &self,
        w: &mut W,
    ) -> Result<(), std::io::Error> {
        crate::util::astd_write_u16_be(
            w,
            self.size_without_size_or_opcode_fields() + Self::OPCODE_LENGTH,
        )
        .await?;
        crate::util::astd_write_u16_le(w, <Self as MessageBody>::OPCODE).await?;

        self.astd_write_body(w).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_encrypted_server<W: WriteExt + Unpin + Send, E: Encrypter + Send>(
        &self,
        w: &mut W,
        e: &mut E,
    ) -> Result<(), std::io::Error> {
        let header = e.encrypt_server_header(
            self.size_without_size_or_opcode_fields() + Self::OPCODE_LENGTH,
            <Self as MessageBody>::OPCODE,
        );
        w.write_all(&header).await;

        self.astd_write_body(w).await?;
        Ok(())
    }
}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
pub trait ClientMessageWrite: MessageBody {
    const OPCODE_LENGTH: u16 = 4;

    #[cfg(feature = "sync")]
    fn write_unencrypted_client<W: Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
        crate::util::write_u16_be(
            w,
            (self.size_without_size_or_opcode_fields() + Self::OPCODE_LENGTH),
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
            (self.size_without_size_or_opcode_fields() + Self::OPCODE_LENGTH),
            <Self as MessageBody>::OPCODE as u32,
        )?;

        self.write_body(w)?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_unencrypted_client<W: AsyncWriteExt + Unpin + Send>(
        &self,
        w: &mut W,
    ) -> Result<(), std::io::Error> {
        crate::util::tokio_write_u16_be(
            w,
            self.size_without_size_or_opcode_fields() + Self::OPCODE_LENGTH,
        )
        .await?;
        crate::util::tokio_write_u32_le(w, <Self as MessageBody>::OPCODE as u32).await?;

        self.tokio_write_body(w).await?;
        Ok(())
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_encrypted_client<W: AsyncWriteExt + Unpin + Send, E: Encrypter + Send>(
        &self,
        w: &mut W,
        e: &mut E,
    ) -> Result<(), std::io::Error> {
        let header = e.encrypt_client_header(
            self.size_without_size_or_opcode_fields() + 4,
            <Self as MessageBody>::OPCODE as u32,
        );
        w.write_all(&header);

        self.tokio_write_body(w).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_unencrypted_client<W: WriteExt + Unpin + Send>(
        &self,
        w: &mut W,
    ) -> Result<(), std::io::Error> {
        crate::util::astd_write_u16_be(w, self.size_without_size_or_opcode_fields() + 4).await?;
        crate::util::astd_write_u32_le(w, <Self as MessageBody>::OPCODE as u32).await?;

        self.astd_write_body(w).await?;
        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_encrypted_client<W: WriteExt + Unpin + Send, E: Encrypter + Send>(
        &self,
        w: &mut W,
        e: &mut E,
    ) -> Result<(), std::io::Error> {
        let header = e.encrypt_client_header(
            self.size_without_size_or_opcode_fields() + 4,
            <Self as MessageBody>::OPCODE as u32,
        );
        w.write_all(&header).await;

        self.astd_write_body(w).await?;
        Ok(())
    }
}
