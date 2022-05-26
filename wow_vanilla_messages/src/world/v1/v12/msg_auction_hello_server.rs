use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ServerMessage, MessageBody};
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct MSG_AUCTION_HELLO_Server {
    pub auctioneer: Guid,
    pub auction_house_id: u32,
}

impl ServerMessage for MSG_AUCTION_HELLO_Server {}

impl MSG_AUCTION_HELLO_Server {
    pub(crate) fn as_bytes(&self) -> Result<[u8; 12], std::io::Error> {
        let mut array_w = [0u8; 12];
        let mut w = array_w.as_mut_slice();
        // auctioneer: Guid
        w.write_all(&self.auctioneer.guid().to_le_bytes())?;

        // auction_house_id: u32
        w.write_all(&self.auction_house_id.to_le_bytes())?;

        Ok(array_w)
    }
}

impl MessageBody for MSG_AUCTION_HELLO_Server {
    const OPCODE: u16 = 0x0255;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        12
    }

    type Error = std::io::Error;

    #[cfg(feature = "sync")]
    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // auctioneer: Guid
        let auctioneer = Guid::read(r)?;

        // auction_house_id: u32
        let auction_house_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            auctioneer,
            auction_house_id,
        })
    }

    #[cfg(feature = "sync")]
    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let inner = self.as_bytes()?;
        w.write_all(&inner)
    }

    #[cfg(feature = "tokio")]
    fn tokio_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + AsyncReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // auctioneer: Guid
            let auctioneer = Guid::tokio_read(r).await?;

            // auction_house_id: u32
            let auction_house_id = crate::util::tokio_read_u32_le(r).await?;

            Ok(Self {
                auctioneer,
                auction_house_id,
            })
        })
    }

    #[cfg(feature = "tokio")]
    fn tokio_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + tokio::io::AsyncWriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let inner = self.as_bytes()?;
            w.write_all(&inner).await
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_read_body<'life0, 'async_trait, R>(
        r: &'life0 mut R,
        body_size: u32,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<Self, Self::Error>>
            + Send + 'async_trait,
    >> where
        R: 'async_trait + ReadExt + Unpin + Send,
        'life0: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            // auctioneer: Guid
            let auctioneer = Guid::astd_read(r).await?;

            // auction_house_id: u32
            let auction_house_id = crate::util::astd_read_u32_le(r).await?;

            Ok(Self {
                auctioneer,
                auction_house_id,
            })
        })
    }

    #[cfg(feature = "async-std")]
    fn astd_write_body<'life0, 'life1, 'async_trait, W>(
        &'life0 self,
        w: &'life1 mut W,
    ) -> core::pin::Pin<Box<
        dyn core::future::Future<Output = std::result::Result<(), std::io::Error>>
            + Send + 'async_trait
    >> where
        W: 'async_trait + async_std::io::WriteExt + Unpin + Send,
        'life0: 'async_trait,
        'life1: 'async_trait,
        Self: 'async_trait,
     {
        Box::pin(async move {
            let inner = self.as_bytes()?;
            w.write_all(&inner).await
        })
    }

}

#[cfg(test)]
mod test {
    use super::MSG_AUCTION_HELLO_Server;
    use super::*;
    use super::super::*;
    use crate::world::v1::v12::opcodes::ServerOpcodeMessage;
    use crate::{MessageBody, ClientMessage, ServerMessage};

    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn MSG_AUCTION_HELLO_Server0() {
        let raw: Vec<u8> = vec![ 0x00, 0x0E, 0x55, 0x02, 0xDE, 0xCA, 0xFA, 0xEF,
             0xBE, 0xAD, 0xDE, 0x00, 0x12, 0x00, 0x00, 0x00, ];

        let expected = MSG_AUCTION_HELLO_Server {
            auctioneer: Guid::new(0xDEADBEEFFACADE),
            auction_house_id: 0x12,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&raw)).unwrap();
        let t = match t {
            ServerOpcodeMessage::MSG_AUCTION_HELLO(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_AUCTION_HELLO, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.auctioneer, expected.auctioneer);
        assert_eq!(t.auction_house_id, expected.auction_house_id);

        assert_eq!(12 + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_MSG_AUCTION_HELLO_Server0() {
        let raw: Vec<u8> = vec![ 0x00, 0x0E, 0x55, 0x02, 0xDE, 0xCA, 0xFA, 0xEF,
             0xBE, 0xAD, 0xDE, 0x00, 0x12, 0x00, 0x00, 0x00, ];

        let expected = MSG_AUCTION_HELLO_Server {
            auctioneer: Guid::new(0xDEADBEEFFACADE),
            auction_house_id: 0x12,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::MSG_AUCTION_HELLO(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_AUCTION_HELLO, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.auctioneer, expected.auctioneer);
        assert_eq!(t.auction_house_id, expected.auction_house_id);

        assert_eq!(12 + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, raw);
    }

    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_MSG_AUCTION_HELLO_Server0() {
        let raw: Vec<u8> = vec![ 0x00, 0x0E, 0x55, 0x02, 0xDE, 0xCA, 0xFA, 0xEF,
             0xBE, 0xAD, 0xDE, 0x00, 0x12, 0x00, 0x00, 0x00, ];

        let expected = MSG_AUCTION_HELLO_Server {
            auctioneer: Guid::new(0xDEADBEEFFACADE),
            auction_house_id: 0x12,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&raw)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::MSG_AUCTION_HELLO(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_AUCTION_HELLO, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.auctioneer, expected.auctioneer);
        assert_eq!(t.auction_house_id, expected.auction_house_id);

        assert_eq!(12 + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, raw);
    }

}
