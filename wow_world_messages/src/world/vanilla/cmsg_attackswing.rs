use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Signals that client has right clicked an opponent and is in the attack stance.
/// Server should reply with [`SMSG_ATTACKSTART`](crate::world::vanilla::SMSG_ATTACKSTART).
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/cmsg_attackswing.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/cmsg_attackswing.wowm#L3):
/// ```text
/// cmsg CMSG_ATTACKSWING = 0x0141 {
///     Guid guid;
/// }
/// ```
pub struct CMSG_ATTACKSWING {
    pub guid: Guid,
}

impl ClientMessage for CMSG_ATTACKSWING {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0141;

    fn client_size(&self) -> u16 {
        14
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            guid,
        })
    }

}

#[cfg(test)]
mod test {
    use super::CMSG_ATTACKSWING;
    use super::*;
    use super::super::*;
    use crate::world::vanilla::opcodes::ClientOpcodeMessage;
    use crate::{Guid, UpdateMask, UpdateContainer, UpdateItem, UpdateCorpse, UpdateGameObject, UpdateDynamicObject, UpdateUnit, UpdatePlayer};
    use crate::{ClientMessage, ServerMessage};

    const RAW0: [u8; 14] = [ 0x00, 0x0C, 0x41, 0x01, 0x00, 0x00, 0x64, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, ];

    // Generated from `wow_message_parser/wowm/world/combat/cmsg_attackswing.wowm` line 10.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMSG_ATTACKSWING0() {
        let expected = CMSG_ATTACKSWING {
            guid: Guid::new(0x64),
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_ATTACKSWING(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_ATTACKSWING, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);

        assert_eq!(8 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/combat/cmsg_attackswing.wowm` line 10.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMSG_ATTACKSWING0() {
        let expected = CMSG_ATTACKSWING {
            guid: Guid::new(0x64),
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_ATTACKSWING(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_ATTACKSWING, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);

        assert_eq!(8 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/combat/cmsg_attackswing.wowm` line 10.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMSG_ATTACKSWING0() {
        let expected = CMSG_ATTACKSWING {
            guid: Guid::new(0x64),
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_ATTACKSWING(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_ATTACKSWING, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);

        assert_eq!(8 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}
