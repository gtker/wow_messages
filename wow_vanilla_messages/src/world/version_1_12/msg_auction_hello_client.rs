use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct MSG_AUCTION_HELLO_Client {
    pub auctioneer: Guid,
}

impl ClientMessage for MSG_AUCTION_HELLO_Client {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // auctioneer: Guid
        w.write_all(&self.auctioneer.guid().to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0255;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        8
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // auctioneer: Guid
        let auctioneer = Guid::read(r)?;

        Ok(Self {
            auctioneer,
        })
    }

}

#[cfg(test)]
mod test {
    use super::MSG_AUCTION_HELLO_Client;
    use super::*;
    use super::super::*;
    use crate::world::version_1_12::opcodes::ClientOpcodeMessage;
    use crate::{Guid, UpdateMask, UpdateContainer, UpdateItem, UpdateCorpse, UpdateGameObject, UpdateDynamicObject, UpdateUnit, UpdatePlayer};
    use crate::{ClientMessage, ServerMessage};

    const RAW0: [u8; 14] = [ 0x00, 0x0C, 0x55, 0x02, 0x00, 0x00, 0xEF, 0xBE, 0xAD,
         0xDE, 0x00, 0x00, 0x00, 0x00, ];

    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn MSG_AUCTION_HELLO_Client0() {
        let expected = MSG_AUCTION_HELLO_Client {
            auctioneer: Guid::new(0xDEADBEEF),
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::MSG_AUCTION_HELLO(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_AUCTION_HELLO, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.auctioneer, expected.auctioneer);

        assert_eq!(8 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_MSG_AUCTION_HELLO_Client0() {
        let expected = MSG_AUCTION_HELLO_Client {
            auctioneer: Guid::new(0xDEADBEEF),
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::MSG_AUCTION_HELLO(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_AUCTION_HELLO, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.auctioneer, expected.auctioneer);

        assert_eq!(8 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_MSG_AUCTION_HELLO_Client0() {
        let expected = MSG_AUCTION_HELLO_Client {
            auctioneer: Guid::new(0xDEADBEEF),
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::MSG_AUCTION_HELLO(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_AUCTION_HELLO, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.auctioneer, expected.auctioneer);

        assert_eq!(8 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}
