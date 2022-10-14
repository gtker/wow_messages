use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Sent immediately after logging in. Client expects reply in [`SMSG_QUERY_TIME_RESPONSE`](crate::world::vanilla::SMSG_QUERY_TIME_RESPONSE).
///
/// This message and the [`SMSG_QUERY_TIME_RESPONSE`](crate::world::vanilla::SMSG_QUERY_TIME_RESPONSE) reply does not actually appear to set the time. Instead [`SMSG_LOGIN_SETTIMESPEED`](crate::world::vanilla::SMSG_LOGIN_SETTIMESPEED) seems to correctly set the time.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/cmsg_query_time.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/cmsg_query_time.wowm#L3):
/// ```text
/// cmsg CMSG_QUERY_TIME = 0x01CE {
/// }
/// ```
pub struct CMSG_QUERY_TIME {
}

impl crate::Message for CMSG_QUERY_TIME {
    const OPCODE: u32 = 0x01ce;

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
impl crate::world::vanilla::ClientMessage for CMSG_QUERY_TIME {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_QUERY_TIME {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_QUERY_TIME {}

#[cfg(all(feature = "vanilla", test))]
mod test1_12 {
    use super::CMSG_QUERY_TIME;
    use super::*;
    use super::super::*;
    use crate::world::vanilla::opcodes::ClientOpcodeMessage;
    use crate::world::vanilla::{ClientMessage, ServerMessage};

    const RAW0: [u8; 6] = [ 0x00, 0x04, 0xCE, 0x01, 0x00, 0x00, ];

    // Generated from `wow_message_parser/wowm/world/queries/cmsg_query_time.wowm` line 8.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMSG_QUERY_TIME0() {
        let expected = CMSG_QUERY_TIME {
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_QUERY_TIME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_QUERY_TIME, got {opcode:#?}", opcode = opcode),
        };


        assert_eq!(header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/queries/cmsg_query_time.wowm` line 8.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMSG_QUERY_TIME0() {
        let expected = CMSG_QUERY_TIME {
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_QUERY_TIME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_QUERY_TIME, got {opcode:#?}", opcode = opcode),
        };


        assert_eq!(header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/queries/cmsg_query_time.wowm` line 8.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMSG_QUERY_TIME0() {
        let expected = CMSG_QUERY_TIME {
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_QUERY_TIME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_QUERY_TIME, got {opcode:#?}", opcode = opcode),
        };


        assert_eq!(header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "tbc", test))]
mod test2 {
    use super::CMSG_QUERY_TIME;
    use super::*;
    use super::super::*;
    use crate::world::tbc::opcodes::ClientOpcodeMessage;
    use crate::world::tbc::{ClientMessage, ServerMessage};

    const RAW0: [u8; 6] = [ 0x00, 0x04, 0xCE, 0x01, 0x00, 0x00, ];

    // Generated from `wow_message_parser/wowm/world/queries/cmsg_query_time.wowm` line 8.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMSG_QUERY_TIME0() {
        let expected = CMSG_QUERY_TIME {
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_QUERY_TIME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_QUERY_TIME, got {opcode:#?}", opcode = opcode),
        };


        assert_eq!(header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/queries/cmsg_query_time.wowm` line 8.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMSG_QUERY_TIME0() {
        let expected = CMSG_QUERY_TIME {
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_QUERY_TIME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_QUERY_TIME, got {opcode:#?}", opcode = opcode),
        };


        assert_eq!(header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/queries/cmsg_query_time.wowm` line 8.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMSG_QUERY_TIME0() {
        let expected = CMSG_QUERY_TIME {
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_QUERY_TIME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_QUERY_TIME, got {opcode:#?}", opcode = opcode),
        };


        assert_eq!(header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "wrath", test))]
mod test3 {
    use super::CMSG_QUERY_TIME;
    use super::*;
    use super::super::*;
    use crate::world::wrath::opcodes::ClientOpcodeMessage;
    use crate::world::wrath::{ClientMessage, ServerMessage};

    const RAW0: [u8; 6] = [ 0x00, 0x04, 0xCE, 0x01, 0x00, 0x00, ];

    // Generated from `wow_message_parser/wowm/world/queries/cmsg_query_time.wowm` line 8.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMSG_QUERY_TIME0() {
        let expected = CMSG_QUERY_TIME {
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_QUERY_TIME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_QUERY_TIME, got {opcode:#?}", opcode = opcode),
        };


        assert_eq!(header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/queries/cmsg_query_time.wowm` line 8.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMSG_QUERY_TIME0() {
        let expected = CMSG_QUERY_TIME {
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_QUERY_TIME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_QUERY_TIME, got {opcode:#?}", opcode = opcode),
        };


        assert_eq!(header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/queries/cmsg_query_time.wowm` line 8.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMSG_QUERY_TIME0() {
        let expected = CMSG_QUERY_TIME {
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_QUERY_TIME(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_QUERY_TIME, got {opcode:#?}", opcode = opcode),
        };


        assert_eq!(header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

