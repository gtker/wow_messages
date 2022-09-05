use crate::Message;
#[cfg(feature = "async-std")]
use async_std::io::WriteExt;
use std::future::Future;
use std::io::Write;
use std::pin::Pin;
#[cfg(feature = "tokio")]
use tokio::io::AsyncWriteExt;
use wow_srp::wrath_header::{ClientEncrypterHalf, ServerEncrypterHalf};

const SERVER_OPCODE_LENGTH: u16 = 2;
const MINIMUM_SIZE_LENGTH: u32 = 2;
const MAXIMUM_SIZE_LENGTH: u32 = 2;
const MINIMUM_SERVER_HEADER_LENGTH: u16 = 4;
const MAXIMUM_SERVER_HEADER_LENGTH: u16 = 5;
const CLIENT_OPCODE_LENGTH: u16 = 4;
const CLIENT_HEADER_LENGTH: u16 = 6;

const LARGE_MESSAGE_THRESHOLD: u32 = 0x7FFF;

fn get_unencrypted_server(opcode: u16, size: u32) -> Vec<u8> {
    let mut v = Vec::with_capacity(size as usize);

    if size > LARGE_MESSAGE_THRESHOLD {
        let size = (size - MAXIMUM_SIZE_LENGTH).to_be_bytes();
        let opcode = opcode.to_le_bytes();

        let mut header = [0_u8; MAXIMUM_SERVER_HEADER_LENGTH as usize];
        header[0] = size[1] | 0x80;
        header[1] = size[2];
        header[2] = size[3];
        header[3] = opcode[0];
        header[4] = opcode[1];

        v.extend_from_slice(&header);
    } else {
        let size = ((size - MINIMUM_SIZE_LENGTH) as u16).to_be_bytes();
        let opcode = opcode.to_le_bytes();

        let mut header = [0_u8; MINIMUM_SERVER_HEADER_LENGTH as usize];
        header[0] = size[0];
        header[1] = size[1];
        header[2] = opcode[0];
        header[3] = opcode[1];

        v.extend_from_slice(&header);
    }

    v
}

fn get_encrypted_server(opcode: u16, size: u32, e: &mut ServerEncrypterHalf) -> Vec<u8> {
    let mut v = Vec::with_capacity(size as usize);

    let size_length = if size > LARGE_MESSAGE_THRESHOLD {
        MAXIMUM_SIZE_LENGTH
    } else {
        MINIMUM_SIZE_LENGTH
    };

    v.extend_from_slice(e.encrypt_server_header(size - size_length, opcode));

    v
}

pub trait ServerMessage: Message {
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
    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
        let mut v = get_unencrypted_server(Self::OPCODE as u16, self.server_size());
        self.write_into_vec(&mut v);

        w.write_all(&v)
    }

    #[cfg(feature = "sync")]
    fn write_encrypted_server<W: std::io::Write>(
        &self,
        w: &mut W,
        e: &mut ServerEncrypterHalf,
    ) -> Result<(), std::io::Error> {
        let mut v = get_encrypted_server(Self::OPCODE as u16, self.server_size(), e);

        self.write_into_vec(&mut v);

        w.write_all(&v)
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
            let mut v = get_unencrypted_server(Self::OPCODE as u16, self.server_size());
            self.write_into_vec(&mut v);

            w.write_all(&v).await
        })
    }

    #[cfg(feature = "tokio")]
    fn tokio_write_encrypted_server<'life0, 'life1, 'life2, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
        e: &'life2 mut ServerEncrypterHalf,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            let mut v = get_encrypted_server(Self::OPCODE as u16, self.server_size(), e);
            self.write_into_vec(&mut v);

            w.write_all(&v).await
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
            let mut v = get_unencrypted_server(Self::OPCODE as u16, self.server_size());
            self.write_into_vec(&mut v);

            w.write_all(&v).await
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write_encrypted_server<'life0, 'life1, 'life2, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
        e: &'life2 mut ServerEncrypterHalf,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            let mut v = get_encrypted_server(Self::OPCODE as u16, self.server_size(), e);
            self.write_into_vec(&mut v);

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
    fn write_unencrypted_client<W: std::io::Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
        let mut v = get_unencrypted_client(Self::OPCODE as u16, self.client_size());
        self.write_into_vec(&mut v);

        w.write_all(&v)
    }

    #[cfg(feature = "sync")]
    fn write_encrypted_client<W: std::io::Write>(
        &self,
        w: &mut W,
        e: &mut ClientEncrypterHalf,
    ) -> Result<(), std::io::Error> {
        let mut v = get_encrypted_client(Self::OPCODE as u16, self.client_size(), e);
        self.write_into_vec(&mut v);

        w.write_all(&v)
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
            let mut v = get_unencrypted_client(Self::OPCODE as u16, self.client_size());
            self.write_into_vec(&mut v);

            w.write_all(&v).await
        })
    }

    #[cfg(feature = "tokio")]
    fn tokio_write_encrypted_client<'life0, 'life1, 'life2, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
        e: &'life2 mut ClientEncrypterHalf,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            let mut v = get_encrypted_client(Self::OPCODE as u16, self.client_size(), e);
            self.write_into_vec(&mut v);

            w.write_all(&v).await
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
            let mut v = get_unencrypted_client(Self::OPCODE as u16, self.client_size());
            self.write_into_vec(&mut v);

            w.write_all(&v).await
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write_encrypted_client<'life0, 'life1, 'life2, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
        e: &'life2 mut ClientEncrypterHalf,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        'life2: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            let mut v = get_encrypted_client(Self::OPCODE as u16, self.client_size(), e);
            self.write_into_vec(&mut v);

            w.write_all(&v).await
        })
    }
}

fn get_unencrypted_client(opcode: u16, size: u16) -> Vec<u8> {
    let mut v = Vec::with_capacity(size as usize);

    let size = (size - MINIMUM_SIZE_LENGTH as u16).to_be_bytes();
    let opcode = (opcode as u32).to_le_bytes();

    let mut header = [0_u8; CLIENT_HEADER_LENGTH as usize];
    header[0] = size[0];
    header[1] = size[1];
    header[2] = opcode[0];
    header[3] = opcode[1];
    header[4] = opcode[2];
    header[5] = opcode[3];

    v.extend_from_slice(&header);

    v
}

fn get_encrypted_client(opcode: u16, size: u16, e: &mut ClientEncrypterHalf) -> Vec<u8> {
    let mut v = Vec::with_capacity(size as usize);

    v.extend_from_slice(&e.encrypt_client_header(size - MINIMUM_SIZE_LENGTH as u16, opcode as u32));

    v
}
