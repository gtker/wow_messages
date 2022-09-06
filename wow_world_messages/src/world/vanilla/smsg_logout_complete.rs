use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Immediately logs out the client of the world and makes it send [`CMSG_CHAR_ENUM`](crate::world::vanilla::CMSG_CHAR_ENUM).
/// Normally the client will send [`CMSG_LOGOUT_REQUEST`](crate::world::vanilla::CMSG_LOGOUT_REQUEST) and the server will reply with an [`SMSG_LOGOUT_RESPONSE`](crate::world::vanilla::SMSG_LOGOUT_RESPONSE) before this message, but sending it unprompted will also immediately send the client to the character screen.
///
/// The client always seems to send 2 [`CMSG_CANCEL_TRADE`](crate::world::vanilla::CMSG_CANCEL_TRADE) immediately after receiving this mesage, but before sending [`CMSG_CHAR_ENUM`](crate::world::vanilla::CMSG_CHAR_ENUM).
/// Even if 'Exit Game' is selected the client will still send a [`CMSG_CHAR_ENUM`](crate::world::vanilla::CMSG_CHAR_ENUM) immediately before closing the connection, despite it not needing to see the character list.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_logout_complete.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_logout_complete.wowm#L3):
/// ```text
/// smsg SMSG_LOGOUT_COMPLETE = 0x004D {
/// }
/// ```
pub struct SMSG_LOGOUT_COMPLETE {
}

impl crate::Message for SMSG_LOGOUT_COMPLETE {
    const OPCODE: u32 = 0x004d;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_LOGOUT_COMPLETE {}

#[cfg(test)]
mod test {
    use super::SMSG_LOGOUT_COMPLETE;
    use super::*;
    use super::super::*;
    use crate::world::vanilla::opcodes::ServerOpcodeMessage;
    use crate::world::vanilla::{ClientMessage, ServerMessage};

    const RAW0: [u8; 4] = [ 0x00, 0x02, 0x4D, 0x00, ];

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_logout_complete.wowm` line 10.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_LOGOUT_COMPLETE0() {
        let expected = SMSG_LOGOUT_COMPLETE {
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOGOUT_COMPLETE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOGOUT_COMPLETE, got {opcode:#?}", opcode = opcode),
        };


        assert_eq!(header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_logout_complete.wowm` line 10.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_LOGOUT_COMPLETE0() {
        let expected = SMSG_LOGOUT_COMPLETE {
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOGOUT_COMPLETE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOGOUT_COMPLETE, got {opcode:#?}", opcode = opcode),
        };


        assert_eq!(header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_logout_complete.wowm` line 10.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_LOGOUT_COMPLETE0() {
        let expected = SMSG_LOGOUT_COMPLETE {
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_LOGOUT_COMPLETE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_LOGOUT_COMPLETE, got {opcode:#?}", opcode = opcode),
        };


        assert_eq!(header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}
