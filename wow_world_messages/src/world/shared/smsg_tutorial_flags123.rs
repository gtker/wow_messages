use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Data for which tutorials the client has passed.
///
/// All bits set means that all tutorials have been passed.
/// Must be sent after [`SMSG_LOGIN_VERIFY_WORLD`](crate::world::vanilla::SMSG_LOGIN_VERIFY_WORLD) otherwise the client will SEGFAULT.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_tutorial_flags.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_tutorial_flags.wowm#L3):
/// ```text
/// smsg SMSG_TUTORIAL_FLAGS = 0x00FD {
///     u32[8] tutorial_data;
/// }
/// ```
pub struct SMSG_TUTORIAL_FLAGS {
    pub tutorial_data: [u32; 8],
}

impl crate::Message for SMSG_TUTORIAL_FLAGS {
    const OPCODE: u32 = 0x00fd;

    fn size_without_header(&self) -> u32 {
        32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // tutorial_data: u32[8]
        for i in self.tutorial_data.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 32 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // tutorial_data: u32[8]
        let mut tutorial_data = [u32::default(); 8];
        for i in tutorial_data.iter_mut() {
            *i = crate::util::read_u32_le(r)?;
        }

        Ok(Self {
            tutorial_data,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_TUTORIAL_FLAGS {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_TUTORIAL_FLAGS {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_TUTORIAL_FLAGS {}

#[cfg(test)]
mod test {
    use super::SMSG_TUTORIAL_FLAGS;
    use super::*;
    use super::super::*;
    use crate::world::vanilla::opcodes::ServerOpcodeMessage;
    use crate::world::vanilla::{ClientMessage, ServerMessage};

    const RAW0: [u8; 36] = [ 0x00, 0x22, 0xFD, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
         0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
         0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
         0xFF, 0xFF, 0xFF, ];

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_tutorial_flags.wowm` line 11.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_TUTORIAL_FLAGS0() {
        let expected = SMSG_TUTORIAL_FLAGS {
            tutorial_data: [ 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF,
                 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, ],
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_TUTORIAL_FLAGS(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_TUTORIAL_FLAGS, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.tutorial_data, expected.tutorial_data);

        assert_eq!(32 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_tutorial_flags.wowm` line 11.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_TUTORIAL_FLAGS0() {
        let expected = SMSG_TUTORIAL_FLAGS {
            tutorial_data: [ 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF,
                 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, ],
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_TUTORIAL_FLAGS(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_TUTORIAL_FLAGS, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.tutorial_data, expected.tutorial_data);

        assert_eq!(32 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_tutorial_flags.wowm` line 11.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_TUTORIAL_FLAGS0() {
        let expected = SMSG_TUTORIAL_FLAGS {
            tutorial_data: [ 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF,
                 0xFFFFFFFF, 0xFFFFFFFF, 0xFFFFFFFF, ],
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_TUTORIAL_FLAGS(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_TUTORIAL_FLAGS, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.tutorial_data, expected.tutorial_data);

        assert_eq!(32 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}
