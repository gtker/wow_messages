use std::convert::{TryFrom, TryInto};
use crate::world::vanilla::SheathState;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// Says which weapon the client pulls out.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/client_set/cmsg_setsheathed.wowm:11`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/client_set/cmsg_setsheathed.wowm#L11):
/// ```text
/// cmsg CMSG_SETSHEATHED = 0x01E0 {
///     SheathState sheathed;
/// }
/// ```
pub struct CMSG_SETSHEATHED {
    pub sheathed: SheathState,
}

impl crate::Message for CMSG_SETSHEATHED {
    const OPCODE: u32 = 0x01e0;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // sheathed: SheathState
        w.write_all(&(self.sheathed.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // sheathed: SheathState
        let sheathed: SheathState = (crate::util::read_u32_le(r)? as u8).try_into()?;

        Ok(Self {
            sheathed,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_SETSHEATHED {}

#[cfg(test)]
mod test {
    use super::CMSG_SETSHEATHED;
    use crate::world::vanilla::SheathState;
    use super::*;
    use super::super::*;
    use crate::world::vanilla::opcodes::ClientOpcodeMessage;
    use crate::world::vanilla::{ClientMessage, ServerMessage};

    const RAW0: [u8; 10] = [ 0x00, 0x08, 0xE0, 0x01, 0x00, 0x00, 0x01, 0x00, 0x00,
         0x00, ];

    // Generated from `wow_message_parser/wowm/world/client_set/cmsg_setsheathed.wowm` line 17.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMSG_SETSHEATHED0() {
        let expected = CMSG_SETSHEATHED {
            sheathed: SheathState::Melee,
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_SETSHEATHED(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_SETSHEATHED, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.sheathed, expected.sheathed);

        assert_eq!(4 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/client_set/cmsg_setsheathed.wowm` line 17.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMSG_SETSHEATHED0() {
        let expected = CMSG_SETSHEATHED {
            sheathed: SheathState::Melee,
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_SETSHEATHED(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_SETSHEATHED, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.sheathed, expected.sheathed);

        assert_eq!(4 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/client_set/cmsg_setsheathed.wowm` line 17.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMSG_SETSHEATHED0() {
        let expected = CMSG_SETSHEATHED {
            sheathed: SheathState::Melee,
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_SETSHEATHED(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_SETSHEATHED, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.sheathed, expected.sheathed);

        assert_eq!(4 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}
