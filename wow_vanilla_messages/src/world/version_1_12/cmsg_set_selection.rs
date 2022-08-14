use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Sets the current target.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/client_set/cmsg_set_selection.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/client_set/cmsg_set_selection.wowm#L3):
/// ```text
/// cmsg CMSG_SET_SELECTION = 0x013D {
///     Guid target;
/// }
/// ```
pub struct CMSG_SET_SELECTION {
    pub target: Guid,
}

impl ClientMessage for CMSG_SET_SELECTION {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // target: Guid
        w.write_all(&self.target.guid().to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x013d;

    fn client_size(&self) -> u16 {
        14
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // target: Guid
        let target = Guid::read(r)?;

        Ok(Self {
            target,
        })
    }

}

#[cfg(test)]
mod test {
    use super::CMSG_SET_SELECTION;
    use super::*;
    use super::super::*;
    use crate::world::version_1_12::opcodes::ClientOpcodeMessage;
    use crate::{Guid, UpdateMask, UpdateContainer, UpdateItem, UpdateCorpse, UpdateGameObject, UpdateDynamicObject, UpdateUnit, UpdatePlayer};
    use crate::{ClientMessage, ServerMessage};

    const RAW0: [u8; 14] = [ 0x00, 0x0C, 0x3D, 0x01, 0x00, 0x00, 0x06, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, ];

    // Generated from `wow_message_parser/wowm/world/client_set/cmsg_set_selection.wowm` line 9.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMSG_SET_SELECTION0() {
        let expected = CMSG_SET_SELECTION {
            target: Guid::new(0x6),
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_SET_SELECTION(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_SET_SELECTION, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.target, expected.target);

        assert_eq!(8 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/client_set/cmsg_set_selection.wowm` line 9.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMSG_SET_SELECTION0() {
        let expected = CMSG_SET_SELECTION {
            target: Guid::new(0x6),
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_SET_SELECTION(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_SET_SELECTION, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.target, expected.target);

        assert_eq!(8 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/client_set/cmsg_set_selection.wowm` line 9.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMSG_SET_SELECTION0() {
        let expected = CMSG_SET_SELECTION {
            target: Guid::new(0x6),
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_SET_SELECTION(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_SET_SELECTION, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.target, expected.target);

        assert_eq!(8 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}
