use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/msg/msg_auction_hello_client.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/msg/msg_auction_hello_client.wowm#L3):
/// ```text
/// cmsg MSG_AUCTION_HELLO_Client = 0x0255 {
///     Guid auctioneer;
/// }
/// ```
pub struct MSG_AUCTION_HELLO_Client {
    pub auctioneer: Guid,
}

impl crate::Message for MSG_AUCTION_HELLO_Client {
    const OPCODE: u32 = 0x0255;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // auctioneer: Guid
        w.write_all(&self.auctioneer.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // auctioneer: Guid
        let auctioneer = Guid::read(r)?;

        Ok(Self {
            auctioneer,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for MSG_AUCTION_HELLO_Client {}

#[cfg(test)]
mod test {
    use super::MSG_AUCTION_HELLO_Client;
    use super::*;
    use super::super::*;
    use crate::world::vanilla::opcodes::ClientOpcodeMessage;
    use crate::Guid;
    use crate::world::vanilla::{ClientMessage, ServerMessage};

    const RAW0: [u8; 14] = [ 0x00, 0x0C, 0x55, 0x02, 0x00, 0x00, 0xEF, 0xBE, 0xAD,
         0xDE, 0x00, 0x00, 0x00, 0x00, ];

    // Generated from `wow_message_parser/wowm/world/auction/msg/msg_auction_hello_client.wowm` line 7.
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

    // Generated from `wow_message_parser/wowm/world/auction/msg/msg_auction_hello_client.wowm` line 7.
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

    // Generated from `wow_message_parser/wowm/world/auction/msg/msg_auction_hello_client.wowm` line 7.
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
