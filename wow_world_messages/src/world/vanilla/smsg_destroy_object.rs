use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Immediately removes an object from the presence of the player.
///
/// Used by vmangos for logout.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gameobject/smsg_destroy_object.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gameobject/smsg_destroy_object.wowm#L3):
/// ```text
/// smsg SMSG_DESTROY_OBJECT = 0x00AA {
///     Guid guid;
/// }
/// ```
pub struct SMSG_DESTROY_OBJECT {
    pub guid: Guid,
}

impl crate::Message for SMSG_DESTROY_OBJECT {
    const OPCODE: u32 = 0x00aa;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
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
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_DESTROY_OBJECT {}

#[cfg(test)]
mod test {
    use super::SMSG_DESTROY_OBJECT;
    use super::*;
    use super::super::*;
    use crate::world::vanilla::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::world::vanilla::{ClientMessage, ServerMessage};

    const RAW0: [u8; 12] = [ 0x00, 0x0A, 0xAA, 0x00, 0x06, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, ];

    // Generated from `wow_message_parser/wowm/world/gameobject/smsg_destroy_object.wowm` line 10.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_DESTROY_OBJECT0() {
        let expected = SMSG_DESTROY_OBJECT {
            guid: Guid::new(0x6),
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_DESTROY_OBJECT(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_DESTROY_OBJECT, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);

        assert_eq!(8 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/gameobject/smsg_destroy_object.wowm` line 10.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_DESTROY_OBJECT0() {
        let expected = SMSG_DESTROY_OBJECT {
            guid: Guid::new(0x6),
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_DESTROY_OBJECT(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_DESTROY_OBJECT, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);

        assert_eq!(8 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/gameobject/smsg_destroy_object.wowm` line 10.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_DESTROY_OBJECT0() {
        let expected = SMSG_DESTROY_OBJECT {
            guid: Guid::new(0x6),
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_DESTROY_OBJECT(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_DESTROY_OBJECT, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);

        assert_eq!(8 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}
