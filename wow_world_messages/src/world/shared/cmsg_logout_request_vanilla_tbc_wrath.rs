use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Sent by client after pressing 'Logout' or 'Exit Game'.
/// Server should reply with [`SMSG_LOGOUT_RESPONSE`](crate::vanilla::SMSG_LOGOUT_RESPONSE).
///
/// Spamming the 'Logout' and 'Exit Game' buttons does not send multiple messages.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/cmsg_logout_request.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/cmsg_logout_request.wowm#L3):
/// ```text
/// cmsg CMSG_LOGOUT_REQUEST = 0x004B {
/// }
/// ```
pub struct CMSG_LOGOUT_REQUEST {
}

#[cfg(feature = "print-testcase")]
impl CMSG_LOGOUT_REQUEST {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_LOGOUT_REQUEST {{").unwrap();
        // Members

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 6_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 75_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();



        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1 2 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for CMSG_LOGOUT_REQUEST {}
impl crate::Message for CMSG_LOGOUT_REQUEST {
    const OPCODE: u32 = 0x004b;

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x004B, size: body_size });
        }

        Ok(Self {
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_LOGOUT_REQUEST {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_LOGOUT_REQUEST {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_LOGOUT_REQUEST {}

#[cfg(all(feature = "vanilla", test))]
mod test_vanilla {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_LOGOUT_REQUEST;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ClientOpcodeMessage;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    const RAW0: [u8; 6] = [ 0x00, 0x04, 0x4B, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> CMSG_LOGOUT_REQUEST {
        CMSG_LOGOUT_REQUEST {
        }

    }

    // Generated from `wow_message_parser/wowm/world/login_logout/cmsg_logout_request.wowm` line 9.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_logout_request0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        match t {
            ClientOpcodeMessage::CMSG_LOGOUT_REQUEST => {}
            opcode => panic!("incorrect opcode. Expected CMSG_LOGOUT_REQUEST, got {opcode:#?}"),
        };

        assert_eq!(HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/cmsg_logout_request.wowm` line 9.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_logout_request0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        match t {
            ClientOpcodeMessage::CMSG_LOGOUT_REQUEST => {}
            opcode => panic!("incorrect opcode. Expected CMSG_LOGOUT_REQUEST, got {opcode:#?}"),
        };

        assert_eq!(HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/cmsg_logout_request.wowm` line 9.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_logout_request0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        match t {
            ClientOpcodeMessage::CMSG_LOGOUT_REQUEST => {}
            opcode => panic!("incorrect opcode. Expected CMSG_LOGOUT_REQUEST, got {opcode:#?}"),
        };

        assert_eq!(HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "tbc", test))]
mod test_tbc {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_LOGOUT_REQUEST;
    use super::*;
    use super::super::*;
    use crate::tbc::opcodes::ClientOpcodeMessage;
    use crate::tbc::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    const RAW0: [u8; 6] = [ 0x00, 0x04, 0x4B, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> CMSG_LOGOUT_REQUEST {
        CMSG_LOGOUT_REQUEST {
        }

    }

    // Generated from `wow_message_parser/wowm/world/login_logout/cmsg_logout_request.wowm` line 9.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_logout_request0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        match t {
            ClientOpcodeMessage::CMSG_LOGOUT_REQUEST => {}
            opcode => panic!("incorrect opcode. Expected CMSG_LOGOUT_REQUEST, got {opcode:#?}"),
        };

        assert_eq!(HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/cmsg_logout_request.wowm` line 9.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_logout_request0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        match t {
            ClientOpcodeMessage::CMSG_LOGOUT_REQUEST => {}
            opcode => panic!("incorrect opcode. Expected CMSG_LOGOUT_REQUEST, got {opcode:#?}"),
        };

        assert_eq!(HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/cmsg_logout_request.wowm` line 9.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_logout_request0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        match t {
            ClientOpcodeMessage::CMSG_LOGOUT_REQUEST => {}
            opcode => panic!("incorrect opcode. Expected CMSG_LOGOUT_REQUEST, got {opcode:#?}"),
        };

        assert_eq!(HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "wrath", test))]
mod test_wrath {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_LOGOUT_REQUEST;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ClientOpcodeMessage;
    use crate::wrath::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    const RAW0: [u8; 6] = [ 0x00, 0x04, 0x4B, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> CMSG_LOGOUT_REQUEST {
        CMSG_LOGOUT_REQUEST {
        }

    }

    // Generated from `wow_message_parser/wowm/world/login_logout/cmsg_logout_request.wowm` line 9.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_logout_request0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        match t {
            ClientOpcodeMessage::CMSG_LOGOUT_REQUEST => {}
            opcode => panic!("incorrect opcode. Expected CMSG_LOGOUT_REQUEST, got {opcode:#?}"),
        };

        assert_eq!(HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/cmsg_logout_request.wowm` line 9.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_logout_request0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        match t {
            ClientOpcodeMessage::CMSG_LOGOUT_REQUEST => {}
            opcode => panic!("incorrect opcode. Expected CMSG_LOGOUT_REQUEST, got {opcode:#?}"),
        };

        assert_eq!(HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/cmsg_logout_request.wowm` line 9.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_logout_request0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        match t {
            ClientOpcodeMessage::CMSG_LOGOUT_REQUEST => {}
            opcode => panic!("incorrect opcode. Expected CMSG_LOGOUT_REQUEST, got {opcode:#?}"),
        };

        assert_eq!(HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

