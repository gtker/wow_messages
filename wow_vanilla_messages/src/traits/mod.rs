use std::future::Future;
use std::pin::Pin;
use wow_srp::header_crypto::Encrypter;

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

fn get_encrypted_server(opcode: u16, size: u16, e: &mut impl Encrypter) -> Vec<u8> {
    let mut v = Vec::with_capacity(size as usize);

    v.extend_from_slice(&e.encrypt_server_header(size - SIZE_LENGTH, opcode));

    v
}

pub trait ServerMessage: Sized {
    #[doc(hidden)]
    const OPCODE: u16;

    /// Total size the message takes up including header.
    /// This is not the same value as what goes into the size field
    /// since the size field does not include the size of the size field.
    fn server_size(&self) -> u16;

    #[doc(hidden)]
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error>;

    #[cfg(feature = "sync")]
    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
        let mut v = get_unencrypted_server(Self::OPCODE, self.server_size());
        self.write_into_vec(&mut v);

        w.write_all(&v)
    }

    #[cfg(feature = "sync")]
    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(
        &self,
        w: &mut W,
        e: &mut E,
    ) -> Result<(), std::io::Error> {
        let mut v = get_encrypted_server(Self::OPCODE, self.server_size(), e);

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
            let mut v = get_unencrypted_server(Self::OPCODE, self.server_size());
            self.write_into_vec(&mut v);

            w.write_all(&v).await
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
            let mut v = get_encrypted_server(Self::OPCODE, self.server_size(), e);
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
            let mut v = get_unencrypted_server(Self::OPCODE, self.server_size());
            self.write_into_vec(&mut v);

            w.write_all(&v).await
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
            let mut v = get_encrypted_server(Self::OPCODE, self.server_size(), e);
            self.write_into_vec(&mut v);

            w.write_all(&v).await
        })
    }

    #[doc(hidden)]
    fn read_body(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError>;
}

pub trait ClientMessage: Sized {
    #[doc(hidden)]
    const OPCODE: u16;

    /// Total size the message takes up including header.
    /// This is not the same value as what goes into the size field
    /// since the size field does not include the size of the size field.
    fn client_size(&self) -> u16;

    #[doc(hidden)]
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error>;

    #[cfg(feature = "sync")]
    fn write_unencrypted_client<W: std::io::Write>(&self, w: &mut W) -> Result<(), std::io::Error> {
        let mut v = get_unencrypted_client(Self::OPCODE, self.client_size());
        self.write_into_vec(&mut v);

        w.write_all(&v)
    }

    #[cfg(feature = "sync")]
    fn write_encrypted_client<W: std::io::Write, E: Encrypter>(
        &self,
        w: &mut W,
        e: &mut E,
    ) -> Result<(), std::io::Error> {
        let mut v = get_encrypted_client(Self::OPCODE, self.client_size(), e);
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
            let mut v = get_unencrypted_client(Self::OPCODE, self.client_size());
            self.write_into_vec(&mut v);

            w.write_all(&v).await
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
            let mut v = get_encrypted_client(Self::OPCODE, self.client_size(), e);
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
            let mut v = get_unencrypted_client(Self::OPCODE, self.client_size());
            self.write_into_vec(&mut v);

            w.write_all(&v).await
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
            let mut v = get_encrypted_client(Self::OPCODE, self.client_size(), e);
            self.write_into_vec(&mut v);

            w.write_all(&v).await
        })
    }

    #[doc(hidden)]
    fn read_body(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError>;
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

fn get_encrypted_client(opcode: u16, size: u16, e: &mut impl Encrypter) -> Vec<u8> {
    let mut v = Vec::with_capacity(size as usize);

    v.extend_from_slice(&e.encrypt_client_header(size - SIZE_LENGTH, opcode as u32));

    v
}
