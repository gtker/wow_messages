use crate:: {
    Guid,
};
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/auction/msg/msg_auction_hello_server.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/auction/msg/msg_auction_hello_server.wowm#L1):
/// ```text
/// smsg MSG_AUCTION_HELLO_Server = 0x0255 {
///     Guid auctioneer;
///     u32 auction_house_id;
/// }
/// ```
pub struct MSG_AUCTION_HELLO_Server {
    pub auctioneer: Guid,
    pub auction_house_id: u32,
}

impl crate::Message for MSG_AUCTION_HELLO_Server {
    const OPCODE: u32 = 0x0255;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // auctioneer: Guid
        w.write_all(&self.auctioneer.guid().to_le_bytes())?;

        // auction_house_id: u32
        w.write_all(&self.auction_house_id.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0255, size: body_size as u32 });
        }

        // auctioneer: Guid
        let auctioneer = Guid::read(&mut r)?;

        // auction_house_id: u32
        let auction_house_id = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            auctioneer,
            auction_house_id,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for MSG_AUCTION_HELLO_Server {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for MSG_AUCTION_HELLO_Server {}

#[cfg(all(feature = "vanilla", test))]
mod test_vanilla {
    use super::MSG_AUCTION_HELLO_Server;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const RAW0: [u8; 16] = [ 0x00, 0x0E, 0x55, 0x02, 0xDE, 0xCA, 0xFA, 0xEF, 0xBE,
         0xAD, 0xDE, 0x00, 0x12, 0x00, 0x00, 0x00, ];

    // Generated from `wow_message_parser/wowm/world/auction/msg/msg_auction_hello_server.wowm` line 8.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn MSG_AUCTION_HELLO_Server0() {
        let expected = MSG_AUCTION_HELLO_Server {
            auctioneer: Guid::new(0xDEADBEEFFACADE),
            auction_house_id: 0x12,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::MSG_AUCTION_HELLO(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_AUCTION_HELLO, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.auctioneer, expected.auctioneer);
        assert_eq!(t.auction_house_id, expected.auction_house_id);

        assert_eq!(12 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/auction/msg/msg_auction_hello_server.wowm` line 8.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_MSG_AUCTION_HELLO_Server0() {
        let expected = MSG_AUCTION_HELLO_Server {
            auctioneer: Guid::new(0xDEADBEEFFACADE),
            auction_house_id: 0x12,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::MSG_AUCTION_HELLO(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_AUCTION_HELLO, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.auctioneer, expected.auctioneer);
        assert_eq!(t.auction_house_id, expected.auction_house_id);

        assert_eq!(12 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/auction/msg/msg_auction_hello_server.wowm` line 8.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_MSG_AUCTION_HELLO_Server0() {
        let expected = MSG_AUCTION_HELLO_Server {
            auctioneer: Guid::new(0xDEADBEEFFACADE),
            auction_house_id: 0x12,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::MSG_AUCTION_HELLO(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_AUCTION_HELLO, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.auctioneer, expected.auctioneer);
        assert_eq!(t.auction_house_id, expected.auction_house_id);

        assert_eq!(12 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "tbc", test))]
mod test_tbc {
    use super::MSG_AUCTION_HELLO_Server;
    use super::*;
    use super::super::*;
    use crate::tbc::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::tbc::{ClientMessage, ServerMessage};

    const RAW0: [u8; 16] = [ 0x00, 0x0E, 0x55, 0x02, 0xDE, 0xCA, 0xFA, 0xEF, 0xBE,
         0xAD, 0xDE, 0x00, 0x12, 0x00, 0x00, 0x00, ];

    // Generated from `wow_message_parser/wowm/world/auction/msg/msg_auction_hello_server.wowm` line 8.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn MSG_AUCTION_HELLO_Server0() {
        let expected = MSG_AUCTION_HELLO_Server {
            auctioneer: Guid::new(0xDEADBEEFFACADE),
            auction_house_id: 0x12,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::MSG_AUCTION_HELLO(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_AUCTION_HELLO, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.auctioneer, expected.auctioneer);
        assert_eq!(t.auction_house_id, expected.auction_house_id);

        assert_eq!(12 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/auction/msg/msg_auction_hello_server.wowm` line 8.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_MSG_AUCTION_HELLO_Server0() {
        let expected = MSG_AUCTION_HELLO_Server {
            auctioneer: Guid::new(0xDEADBEEFFACADE),
            auction_house_id: 0x12,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::MSG_AUCTION_HELLO(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_AUCTION_HELLO, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.auctioneer, expected.auctioneer);
        assert_eq!(t.auction_house_id, expected.auction_house_id);

        assert_eq!(12 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/auction/msg/msg_auction_hello_server.wowm` line 8.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_MSG_AUCTION_HELLO_Server0() {
        let expected = MSG_AUCTION_HELLO_Server {
            auctioneer: Guid::new(0xDEADBEEFFACADE),
            auction_house_id: 0x12,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::MSG_AUCTION_HELLO(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_AUCTION_HELLO, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.auctioneer, expected.auctioneer);
        assert_eq!(t.auction_house_id, expected.auction_house_id);

        assert_eq!(12 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

