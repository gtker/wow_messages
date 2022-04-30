use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use crate::AsyncReadWrite;
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct MSG_AUCTION_HELLO_Server {
    pub auctioneer: Guid,
    pub auction_house_id: u32,
}

impl WorldServerMessageWrite for MSG_AUCTION_HELLO_Server {
    const OPCODE: u16 = 0x255;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (Self::size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (Self::size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for MSG_AUCTION_HELLO_Server {
    type Error = std::io::Error;

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

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // auctioneer: Guid
        self.auctioneer.write(w)?;

        // auction_house_id: u32
        w.write_all(&self.auction_house_id.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for MSG_AUCTION_HELLO_Server {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for MSG_AUCTION_HELLO_Server {
    fn maximum_possible_size() -> usize {
        8 // auctioneer: Guid
        + 4 // auction_house_id: u32
    }
}

#[cfg(test)]
mod test {
    use crate::ReadableAndWritable;
    use std::io::Cursor;
    use super::MSG_AUCTION_HELLO_Server;
    use crate::ConstantSized;
    use super::*;
    use super::super::*;
    use crate::world::v1::v12::opcodes::WorldServerOpcodeMessage;
    use crate::{WorldMessageBody, WorldClientMessageWrite, WorldServerMessageWrite, WorldMessage};

    #[test]
    fn MSG_AUCTION_HELLO_Server0() {
        let raw: Vec<u8> = vec![ 0x00, 0x0E, 0x55, 0x02, 0xDE, 0xCA, 0xFA, 0xEF,
             0xBE, 0xAD, 0xDE, 0x00, 0x12, 0x00, 0x00, 0x00, ];

        let expected = MSG_AUCTION_HELLO_Server {
            auctioneer: Guid::new(0xDEADBEEFFACADE),
            auction_house_id: 0x12,
        };

        let header_size = 2 + 2;
        let t = WorldServerOpcodeMessage::read_unencrypted(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            WorldServerOpcodeMessage::MSG_AUCTION_HELLO(t) => t,
            opcode => panic!("incorrect opcode. Expected MSG_AUCTION_HELLO, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.auctioneer, expected.auctioneer);
        assert_eq!(t.auction_house_id, expected.auction_house_id);

        assert_eq!(MSG_AUCTION_HELLO_Server::size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write_unencrypted_server(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

}
