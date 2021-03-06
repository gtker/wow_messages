use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Immediately logs out the client of the world and makes it send [`CMSG_CHAR_ENUM`](crate::world::version_1_2::CMSG_CHAR_ENUM).
/// Normally the client will send [`CMSG_LOGOUT_REQUEST`](crate::world::version_1_12::CMSG_LOGOUT_REQUEST) and the server will reply with an [`SMSG_LOGOUT_RESPONSE`](crate::world::version_1_12::SMSG_LOGOUT_RESPONSE) before this message, but sending it unprompted will also immediately send the client to the character screen.
///
/// The client always seems to send 2 [`CMSG_CANCEL_TRADE`](crate::world::version_1_12::CMSG_CANCEL_TRADE) immediately after receiving this mesage, but before sending [`CMSG_CHAR_ENUM`](crate::world::version_1_2::CMSG_CHAR_ENUM).
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_logout_complete.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_logout_complete.wowm#L3):
/// ```text
/// smsg SMSG_LOGOUT_COMPLETE = 0x004D {
/// }
/// ```
pub struct SMSG_LOGOUT_COMPLETE {
}

impl ServerMessage for SMSG_LOGOUT_COMPLETE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        Ok(())
    }
    const OPCODE: u16 = 0x004d;

    fn server_size(&self) -> u16 {
        4
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        Ok(Self {
        })
    }

}

#[cfg(test)]
mod test {
    use super::SMSG_LOGOUT_COMPLETE;
    use super::*;
    use super::super::*;
    use crate::world::version_1_12::opcodes::ServerOpcodeMessage;
    use crate::{Guid, UpdateMask, UpdateContainer, UpdateItem, UpdateCorpse, UpdateGameObject, UpdateDynamicObject, UpdateUnit, UpdatePlayer};
    use crate::{ClientMessage, ServerMessage};

    const RAW0: [u8; 4] = [ 0x00, 0x02, 0x4D, 0x00, ];

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_logout_complete.wowm` line 9.
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

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_logout_complete.wowm` line 9.
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

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_logout_complete.wowm` line 9.
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
