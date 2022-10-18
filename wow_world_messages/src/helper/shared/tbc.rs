use crate::Message;
use std::future::Future;
use std::pin::Pin;
use wow_srp::tbc_header::EncrypterHalf;

const SERVER_OPCODE_LENGTH: u16 = 2;
const SIZE_LENGTH: u16 = 2;
const SERVER_HEADER_LENGTH: u16 = 4;
const CLIENT_OPCODE_LENGTH: u16 = 4;
const CLIENT_HEADER_LENGTH: u16 = 6;

fn get_unencrypted_server(opcode: u16, size: u16) -> Vec<u8> {
    let mut v = Vec::with_capacity(size as usize);

    let size = (size - SIZE_LENGTH).to_be_bytes();
    let opcode = opcode.to_le_bytes();

    let mut header = [0_u8; SERVER_HEADER_LENGTH as usize];
    header[0] = size[0];
    header[1] = size[1];
    header[2] = opcode[0];
    header[3] = opcode[1];
    v.extend_from_slice(&header);

    v
}

#[cfg(feature = "encryption")]
fn get_encrypted_server(opcode: u16, size: u16, e: &mut EncrypterHalf) -> Vec<u8> {
    let mut v = Vec::with_capacity(size as usize);

    v.extend_from_slice(&e.encrypt_server_header(size - SIZE_LENGTH, opcode));

    v
}

pub trait ServerMessage: Message {
    /// Total size the message takes up including header.
    /// This is not the same value as what goes into the size field
    /// since the size field does not include the size of the size field.
    fn server_size(&self) -> u16 {
        self.size_without_header() as u16 + SERVER_HEADER_LENGTH
    }

    #[cfg(feature = "sync")]
    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
        let mut v = get_unencrypted_server(Self::OPCODE as u16, self.server_size());
        self.write_into_vec(&mut v)?;

        w.write_all(&v)
    }

    #[cfg(all(feature = "sync", feature = "encryption"))]
    fn write_encrypted_server<W: std::io::Write>(
        &self,
        w: &mut W,
        e: &mut EncrypterHalf,
    ) -> Result<(), std::io::Error> {
        let mut v = get_encrypted_server(Self::OPCODE as u16, self.server_size(), e);

        self.write_into_vec(&mut v)?;

        w.write_all(&v)
    }

    #[cfg(feature = "tokio")]
    fn tokio_write_unencrypted_server<'s, 'w, 'async_trait, W>(
        &'s self,
        w: &'w mut W,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        's: 'async_trait,
        'w: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            let mut v = get_unencrypted_server(Self::OPCODE as u16, self.server_size());
            self.write_into_vec(&mut v)?;

            w.write_all(&v).await
        })
    }

    #[cfg(all(feature = "tokio", feature = "encryption"))]
    fn tokio_write_encrypted_server<'s, 'w, 'e, 'async_trait, W>(
        &'s self,
        w: &'w mut W,
        e: &'e mut EncrypterHalf,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        's: 'async_trait,
        'w: 'async_trait,
        'e: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            let mut v = get_encrypted_server(Self::OPCODE as u16, self.server_size(), e);
            self.write_into_vec(&mut v)?;

            w.write_all(&v).await
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write_unencrypted_server<'s, 'w, 'async_trait, W>(
        &'s self,
        w: &'w mut W,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        's: 'async_trait,
        'w: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            let mut v = get_unencrypted_server(Self::OPCODE as u16, self.server_size());
            self.write_into_vec(&mut v)?;

            w.write_all(&v).await
        })
    }

    #[cfg(all(feature = "async-std", feature = "encryption"))]
    fn astd_write_encrypted_server<'s, 'w, 'e, 'async_trait, W>(
        &'s self,
        w: &'w mut W,
        e: &'e mut EncrypterHalf,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        's: 'async_trait,
        'w: 'async_trait,
        'e: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            let mut v = get_encrypted_server(Self::OPCODE as u16, self.server_size(), e);
            self.write_into_vec(&mut v)?;

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
        self.write_into_vec(&mut v)?;

        w.write_all(&v)
    }

    #[cfg(all(feature = "sync", feature = "encryption"))]
    fn write_encrypted_client<W: std::io::Write>(
        &self,
        w: &mut W,
        e: &mut EncrypterHalf,
    ) -> Result<(), std::io::Error> {
        let mut v = get_encrypted_client(Self::OPCODE as u16, self.client_size(), e);
        self.write_into_vec(&mut v)?;

        w.write_all(&v)
    }

    #[cfg(feature = "tokio")]
    fn tokio_write_unencrypted_client<'s, 'w, 'async_trait, W>(
        &'s self,
        w: &'w mut W,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        's: 'async_trait,
        'w: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            let mut v = get_unencrypted_client(Self::OPCODE as u16, self.client_size());
            self.write_into_vec(&mut v)?;

            w.write_all(&v).await
        })
    }

    #[cfg(all(feature = "encryption", feature = "tokio"))]
    fn tokio_write_encrypted_client<'s, 'w, 'e, 'async_trait, W>(
        &'s self,
        w: &'w mut W,
        e: &'e mut EncrypterHalf,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        's: 'async_trait,
        'w: 'async_trait,
        'e: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            let mut v = get_encrypted_client(Self::OPCODE as u16, self.client_size(), e);
            self.write_into_vec(&mut v)?;

            w.write_all(&v).await
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write_unencrypted_client<'s, 'w, 'async_trait, W>(
        &'s self,
        w: &'w mut W,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        's: 'async_trait,
        'w: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            let mut v = get_unencrypted_client(Self::OPCODE as u16, self.client_size());
            self.write_into_vec(&mut v)?;

            w.write_all(&v).await
        })
    }

    #[cfg(all(feature = "async-std", feature = "encryption"))]
    fn astd_write_encrypted_client<'s, 'w, 'e, 'async_trait, W>(
        &'s self,
        w: &'w mut W,
        e: &'e mut EncrypterHalf,
    ) -> Pin<Box<dyn Future<Output = Result<(), std::io::Error>> + Send + 'async_trait>>
    where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        's: 'async_trait,
        'w: 'async_trait,
        'e: 'async_trait,
        Self: Sync + 'async_trait,
    {
        Box::pin(async move {
            let mut v = get_encrypted_client(Self::OPCODE as u16, self.client_size(), e);
            self.write_into_vec(&mut v)?;

            w.write_all(&v).await
        })
    }
}

fn get_unencrypted_client(opcode: u16, size: u16) -> Vec<u8> {
    let mut v = Vec::with_capacity(size as usize);

    let size = (size - SIZE_LENGTH).to_be_bytes();
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

#[cfg(feature = "encryption")]
fn get_encrypted_client(opcode: u16, size: u16, e: &mut EncrypterHalf) -> Vec<u8> {
    let mut v = Vec::with_capacity(size as usize);

    v.extend_from_slice(&e.encrypt_client_header(size - SIZE_LENGTH, opcode as u32));

    v
}
