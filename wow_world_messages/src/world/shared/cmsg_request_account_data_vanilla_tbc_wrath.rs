use crate:: {
};
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Respond with [`SMSG_UPDATE_ACCOUNT_DATA`](crate::tbc::SMSG_UPDATE_ACCOUNT_DATA)
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/cmsg_request_account_data.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/cmsg_request_account_data.wowm#L1):
/// ```text
/// cmsg CMSG_REQUEST_ACCOUNT_DATA = 0x020A {
///     u32 data_type;
/// }
/// ```
pub struct CMSG_REQUEST_ACCOUNT_DATA {
    /// The type of account data being requested. You can check this against the `CacheMask` to know if this is character-specific data or account-wide data.
    ///
    pub data_type: u32,
}

impl crate::Message for CMSG_REQUEST_ACCOUNT_DATA {
    const OPCODE: u32 = 0x020a;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // data_type: u32
        w.write_all(&self.data_type.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x020A, size: body_size as u32 });
        }

        // data_type: u32
        let data_type = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            data_type,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_REQUEST_ACCOUNT_DATA {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_REQUEST_ACCOUNT_DATA {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_REQUEST_ACCOUNT_DATA {}

#[cfg(all(feature = "vanilla", test))]
mod test_vanilla {
    use super::CMSG_REQUEST_ACCOUNT_DATA;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ClientOpcodeMessage;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const RAW0: [u8; 10] = [ 0x00, 0x08, 0x0A, 0x02, 0x00, 0x00, 0x06, 0x00, 0x00,
         0x00, ];

    // Generated from `wow_message_parser/wowm/world/login_logout/cmsg_request_account_data.wowm` line 10.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMSG_REQUEST_ACCOUNT_DATA0() {
        let expected = CMSG_REQUEST_ACCOUNT_DATA {
            data_type: 0x6,
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_REQUEST_ACCOUNT_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_REQUEST_ACCOUNT_DATA, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.data_type, expected.data_type);

        assert_eq!(4 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/cmsg_request_account_data.wowm` line 10.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMSG_REQUEST_ACCOUNT_DATA0() {
        let expected = CMSG_REQUEST_ACCOUNT_DATA {
            data_type: 0x6,
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_REQUEST_ACCOUNT_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_REQUEST_ACCOUNT_DATA, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.data_type, expected.data_type);

        assert_eq!(4 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/cmsg_request_account_data.wowm` line 10.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMSG_REQUEST_ACCOUNT_DATA0() {
        let expected = CMSG_REQUEST_ACCOUNT_DATA {
            data_type: 0x6,
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_REQUEST_ACCOUNT_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_REQUEST_ACCOUNT_DATA, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.data_type, expected.data_type);

        assert_eq!(4 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "tbc", test))]
mod test_tbc {
    use super::CMSG_REQUEST_ACCOUNT_DATA;
    use super::*;
    use super::super::*;
    use crate::tbc::opcodes::ClientOpcodeMessage;
    use crate::tbc::{ClientMessage, ServerMessage};

    const RAW0: [u8; 10] = [ 0x00, 0x08, 0x0A, 0x02, 0x00, 0x00, 0x06, 0x00, 0x00,
         0x00, ];

    // Generated from `wow_message_parser/wowm/world/login_logout/cmsg_request_account_data.wowm` line 10.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMSG_REQUEST_ACCOUNT_DATA0() {
        let expected = CMSG_REQUEST_ACCOUNT_DATA {
            data_type: 0x6,
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_REQUEST_ACCOUNT_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_REQUEST_ACCOUNT_DATA, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.data_type, expected.data_type);

        assert_eq!(4 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/cmsg_request_account_data.wowm` line 10.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMSG_REQUEST_ACCOUNT_DATA0() {
        let expected = CMSG_REQUEST_ACCOUNT_DATA {
            data_type: 0x6,
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_REQUEST_ACCOUNT_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_REQUEST_ACCOUNT_DATA, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.data_type, expected.data_type);

        assert_eq!(4 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/cmsg_request_account_data.wowm` line 10.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMSG_REQUEST_ACCOUNT_DATA0() {
        let expected = CMSG_REQUEST_ACCOUNT_DATA {
            data_type: 0x6,
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_REQUEST_ACCOUNT_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_REQUEST_ACCOUNT_DATA, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.data_type, expected.data_type);

        assert_eq!(4 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "wrath", test))]
mod test_wrath {
    use super::CMSG_REQUEST_ACCOUNT_DATA;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ClientOpcodeMessage;
    use crate::wrath::{ClientMessage, ServerMessage};

    const RAW0: [u8; 10] = [ 0x00, 0x08, 0x0A, 0x02, 0x00, 0x00, 0x06, 0x00, 0x00,
         0x00, ];

    // Generated from `wow_message_parser/wowm/world/login_logout/cmsg_request_account_data.wowm` line 10.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMSG_REQUEST_ACCOUNT_DATA0() {
        let expected = CMSG_REQUEST_ACCOUNT_DATA {
            data_type: 0x6,
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_REQUEST_ACCOUNT_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_REQUEST_ACCOUNT_DATA, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.data_type, expected.data_type);

        assert_eq!(4 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/cmsg_request_account_data.wowm` line 10.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMSG_REQUEST_ACCOUNT_DATA0() {
        let expected = CMSG_REQUEST_ACCOUNT_DATA {
            data_type: 0x6,
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_REQUEST_ACCOUNT_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_REQUEST_ACCOUNT_DATA, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.data_type, expected.data_type);

        assert_eq!(4 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/cmsg_request_account_data.wowm` line 10.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMSG_REQUEST_ACCOUNT_DATA0() {
        let expected = CMSG_REQUEST_ACCOUNT_DATA {
            data_type: 0x6,
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_REQUEST_ACCOUNT_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_REQUEST_ACCOUNT_DATA, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.data_type, expected.data_type);

        assert_eq!(4 + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

