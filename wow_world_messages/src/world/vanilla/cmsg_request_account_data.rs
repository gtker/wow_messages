use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
/// cmangos/vmangos/mangoszero ignore the data in this.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/cmsg_request_account_data.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/cmsg_request_account_data.wowm#L3):
/// ```text
/// cmsg CMSG_REQUEST_ACCOUNT_DATA = 0x020A {
///     u32 block;
/// }
/// ```
pub struct CMSG_REQUEST_ACCOUNT_DATA {
    /// The exact purpose is unknown, but [`CMSG_UPDATE_ACCOUNT_DATA`](crate::world::vanilla::CMSG_UPDATE_ACCOUNT_DATA) and [`SMSG_ACCOUNT_DATA_TIMES`](crate::world::vanilla::SMSG_ACCOUNT_DATA_TIMES) possibly also operate on these blocks.
    ///
    pub block: u32,
}

impl crate::Message for CMSG_REQUEST_ACCOUNT_DATA {
    const OPCODE: u32 = 0x020a;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // block: u32
        w.write_all(&self.block.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // block: u32
        let block = crate::util::read_u32_le(r)?;

        Ok(Self {
            block,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ClientMessage for CMSG_REQUEST_ACCOUNT_DATA {}

#[cfg(test)]
mod test {
    use super::CMSG_REQUEST_ACCOUNT_DATA;
    use super::*;
    use super::super::*;
    use crate::world::vanilla::opcodes::ClientOpcodeMessage;
    use crate::world::vanilla::{ClientMessage, ServerMessage};

    const RAW0: [u8; 10] = [ 0x00, 0x08, 0x0A, 0x02, 0x00, 0x00, 0x06, 0x00, 0x00,
         0x00, ];

    // Generated from `wow_message_parser/wowm/world/login_logout/cmsg_request_account_data.wowm` line 11.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMSG_REQUEST_ACCOUNT_DATA0() {
        let expected = CMSG_REQUEST_ACCOUNT_DATA {
            block: 0x6,
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_REQUEST_ACCOUNT_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_REQUEST_ACCOUNT_DATA, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.block, expected.block);

        assert_eq!(4 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/cmsg_request_account_data.wowm` line 11.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMSG_REQUEST_ACCOUNT_DATA0() {
        let expected = CMSG_REQUEST_ACCOUNT_DATA {
            block: 0x6,
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_REQUEST_ACCOUNT_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_REQUEST_ACCOUNT_DATA, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.block, expected.block);

        assert_eq!(4 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/cmsg_request_account_data.wowm` line 11.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMSG_REQUEST_ACCOUNT_DATA0() {
        let expected = CMSG_REQUEST_ACCOUNT_DATA {
            block: 0x6,
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_REQUEST_ACCOUNT_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_REQUEST_ACCOUNT_DATA, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.block, expected.block);

        assert_eq!(4 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}
