use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Data for which tutorials the client has passed.
///
/// All bits set means that all tutorials have been passed.
/// Must be sent after [`SMSG_LOGIN_VERIFY_WORLD`](crate::world::version_1_12::SMSG_LOGIN_VERIFY_WORLD) otherwise the client will SEGFAULT.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_tutorial_flags.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_tutorial_flags.wowm#L3):
/// ```text
/// smsg SMSG_TUTORIAL_FLAGS = 0x00FD {
///     u32 tutorial_data0;
///     u32 tutorial_data1;
///     u32 tutorial_data2;
///     u32 tutorial_data3;
///     u32 tutorial_data4;
///     u32 tutorial_data5;
///     u32 tutorial_data6;
///     u32 tutorial_data7;
/// }
/// ```
pub struct SMSG_TUTORIAL_FLAGS {
    pub tutorial_data0: u32,
    pub tutorial_data1: u32,
    pub tutorial_data2: u32,
    pub tutorial_data3: u32,
    pub tutorial_data4: u32,
    pub tutorial_data5: u32,
    pub tutorial_data6: u32,
    pub tutorial_data7: u32,
}

impl ServerMessage for SMSG_TUTORIAL_FLAGS {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // tutorial_data0: u32
        w.write_all(&self.tutorial_data0.to_le_bytes())?;

        // tutorial_data1: u32
        w.write_all(&self.tutorial_data1.to_le_bytes())?;

        // tutorial_data2: u32
        w.write_all(&self.tutorial_data2.to_le_bytes())?;

        // tutorial_data3: u32
        w.write_all(&self.tutorial_data3.to_le_bytes())?;

        // tutorial_data4: u32
        w.write_all(&self.tutorial_data4.to_le_bytes())?;

        // tutorial_data5: u32
        w.write_all(&self.tutorial_data5.to_le_bytes())?;

        // tutorial_data6: u32
        w.write_all(&self.tutorial_data6.to_le_bytes())?;

        // tutorial_data7: u32
        w.write_all(&self.tutorial_data7.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x00fd;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        32
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 32 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // tutorial_data0: u32
        let tutorial_data0 = crate::util::read_u32_le(r)?;

        // tutorial_data1: u32
        let tutorial_data1 = crate::util::read_u32_le(r)?;

        // tutorial_data2: u32
        let tutorial_data2 = crate::util::read_u32_le(r)?;

        // tutorial_data3: u32
        let tutorial_data3 = crate::util::read_u32_le(r)?;

        // tutorial_data4: u32
        let tutorial_data4 = crate::util::read_u32_le(r)?;

        // tutorial_data5: u32
        let tutorial_data5 = crate::util::read_u32_le(r)?;

        // tutorial_data6: u32
        let tutorial_data6 = crate::util::read_u32_le(r)?;

        // tutorial_data7: u32
        let tutorial_data7 = crate::util::read_u32_le(r)?;

        Ok(Self {
            tutorial_data0,
            tutorial_data1,
            tutorial_data2,
            tutorial_data3,
            tutorial_data4,
            tutorial_data5,
            tutorial_data6,
            tutorial_data7,
        })
    }

}

#[cfg(test)]
mod test {
    use super::SMSG_TUTORIAL_FLAGS;
    use super::*;
    use super::super::*;
    use crate::world::version_1_12::opcodes::ServerOpcodeMessage;
    use crate::{Guid, UpdateMask, UpdateContainer, UpdateItem, UpdateCorpse, UpdateGameObject, UpdateDynamicObject, UpdateUnit, UpdatePlayer};
    use crate::{ClientMessage, ServerMessage};

    const RAW0: [u8; 36] = [ 0x00, 0x22, 0xFD, 0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
         0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
         0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
         0xFF, 0xFF, 0xFF, ];

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_tutorial_flags.wowm` line 18.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_TUTORIAL_FLAGS0() {
        let expected = SMSG_TUTORIAL_FLAGS {
            tutorial_data0: 0xFFFFFFFF,
            tutorial_data1: 0xFFFFFFFF,
            tutorial_data2: 0xFFFFFFFF,
            tutorial_data3: 0xFFFFFFFF,
            tutorial_data4: 0xFFFFFFFF,
            tutorial_data5: 0xFFFFFFFF,
            tutorial_data6: 0xFFFFFFFF,
            tutorial_data7: 0xFFFFFFFF,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_TUTORIAL_FLAGS(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_TUTORIAL_FLAGS, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.tutorial_data0, expected.tutorial_data0);
        assert_eq!(t.tutorial_data1, expected.tutorial_data1);
        assert_eq!(t.tutorial_data2, expected.tutorial_data2);
        assert_eq!(t.tutorial_data3, expected.tutorial_data3);
        assert_eq!(t.tutorial_data4, expected.tutorial_data4);
        assert_eq!(t.tutorial_data5, expected.tutorial_data5);
        assert_eq!(t.tutorial_data6, expected.tutorial_data6);
        assert_eq!(t.tutorial_data7, expected.tutorial_data7);

        assert_eq!(32 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_tutorial_flags.wowm` line 18.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_TUTORIAL_FLAGS0() {
        let expected = SMSG_TUTORIAL_FLAGS {
            tutorial_data0: 0xFFFFFFFF,
            tutorial_data1: 0xFFFFFFFF,
            tutorial_data2: 0xFFFFFFFF,
            tutorial_data3: 0xFFFFFFFF,
            tutorial_data4: 0xFFFFFFFF,
            tutorial_data5: 0xFFFFFFFF,
            tutorial_data6: 0xFFFFFFFF,
            tutorial_data7: 0xFFFFFFFF,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_TUTORIAL_FLAGS(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_TUTORIAL_FLAGS, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.tutorial_data0, expected.tutorial_data0);
        assert_eq!(t.tutorial_data1, expected.tutorial_data1);
        assert_eq!(t.tutorial_data2, expected.tutorial_data2);
        assert_eq!(t.tutorial_data3, expected.tutorial_data3);
        assert_eq!(t.tutorial_data4, expected.tutorial_data4);
        assert_eq!(t.tutorial_data5, expected.tutorial_data5);
        assert_eq!(t.tutorial_data6, expected.tutorial_data6);
        assert_eq!(t.tutorial_data7, expected.tutorial_data7);

        assert_eq!(32 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/character_screen/smsg_tutorial_flags.wowm` line 18.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_TUTORIAL_FLAGS0() {
        let expected = SMSG_TUTORIAL_FLAGS {
            tutorial_data0: 0xFFFFFFFF,
            tutorial_data1: 0xFFFFFFFF,
            tutorial_data2: 0xFFFFFFFF,
            tutorial_data3: 0xFFFFFFFF,
            tutorial_data4: 0xFFFFFFFF,
            tutorial_data5: 0xFFFFFFFF,
            tutorial_data6: 0xFFFFFFFF,
            tutorial_data7: 0xFFFFFFFF,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_TUTORIAL_FLAGS(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_TUTORIAL_FLAGS, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.tutorial_data0, expected.tutorial_data0);
        assert_eq!(t.tutorial_data1, expected.tutorial_data1);
        assert_eq!(t.tutorial_data2, expected.tutorial_data2);
        assert_eq!(t.tutorial_data3, expected.tutorial_data3);
        assert_eq!(t.tutorial_data4, expected.tutorial_data4);
        assert_eq!(t.tutorial_data5, expected.tutorial_data5);
        assert_eq!(t.tutorial_data6, expected.tutorial_data6);
        assert_eq!(t.tutorial_data7, expected.tutorial_data7);

        assert_eq!(32 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}
