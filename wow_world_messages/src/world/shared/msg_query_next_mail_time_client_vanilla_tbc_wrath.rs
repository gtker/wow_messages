use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Sent when the client enters the world.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/msg_query_next_mail_time_client.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/msg_query_next_mail_time_client.wowm#L1):
/// ```text
/// cmsg MSG_QUERY_NEXT_MAIL_TIME_Client = 0x0284 {
/// }
/// ```
pub struct MSG_QUERY_NEXT_MAIL_TIME_Client {
}

#[cfg(feature = "print-testcase")]
impl MSG_QUERY_NEXT_MAIL_TIME_Client {
    pub fn to_test_case_string(&self) -> Option<String> {
        None
    }

}

impl crate::private::Sealed for MSG_QUERY_NEXT_MAIL_TIME_Client {}
impl crate::Message for MSG_QUERY_NEXT_MAIL_TIME_Client {
    const OPCODE: u32 = 0x0284;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        MSG_QUERY_NEXT_MAIL_TIME_Client::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0284, size: body_size });
        }

        Ok(Self {
        })
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for MSG_QUERY_NEXT_MAIL_TIME_Client {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for MSG_QUERY_NEXT_MAIL_TIME_Client {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for MSG_QUERY_NEXT_MAIL_TIME_Client {}

#[cfg(all(feature = "vanilla", test))]
mod test_vanilla {
    #![allow(clippy::missing_const_for_fn)]
    use super::MSG_QUERY_NEXT_MAIL_TIME_Client;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ClientOpcodeMessage;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    const RAW0: [u8; 6] = [ 0x00, 0x04, 0x84, 0x02, 0x00, 0x00, ];

    pub(crate) fn expected0() -> MSG_QUERY_NEXT_MAIL_TIME_Client {
        MSG_QUERY_NEXT_MAIL_TIME_Client {
        }

    }

    // Generated from `wow_message_parser/wowm/world/queries/msg_query_next_mail_time_client.wowm` line 7.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn msg_query_next_mail_time_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        match t {
            ClientOpcodeMessage::MSG_QUERY_NEXT_MAIL_TIME => {}
            opcode => panic!("incorrect opcode. Expected MSG_QUERY_NEXT_MAIL_TIME, got {opcode:#?}"),
        };

        assert_eq!(HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/queries/msg_query_next_mail_time_client.wowm` line 7.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_msg_query_next_mail_time_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        match t {
            ClientOpcodeMessage::MSG_QUERY_NEXT_MAIL_TIME => {}
            opcode => panic!("incorrect opcode. Expected MSG_QUERY_NEXT_MAIL_TIME, got {opcode:#?}"),
        };

        assert_eq!(HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/queries/msg_query_next_mail_time_client.wowm` line 7.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_msg_query_next_mail_time_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        match t {
            ClientOpcodeMessage::MSG_QUERY_NEXT_MAIL_TIME => {}
            opcode => panic!("incorrect opcode. Expected MSG_QUERY_NEXT_MAIL_TIME, got {opcode:#?}"),
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
    use super::MSG_QUERY_NEXT_MAIL_TIME_Client;
    use super::*;
    use super::super::*;
    use crate::tbc::opcodes::ClientOpcodeMessage;
    use crate::tbc::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    const RAW0: [u8; 6] = [ 0x00, 0x04, 0x84, 0x02, 0x00, 0x00, ];

    pub(crate) fn expected0() -> MSG_QUERY_NEXT_MAIL_TIME_Client {
        MSG_QUERY_NEXT_MAIL_TIME_Client {
        }

    }

    // Generated from `wow_message_parser/wowm/world/queries/msg_query_next_mail_time_client.wowm` line 7.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn msg_query_next_mail_time_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        match t {
            ClientOpcodeMessage::MSG_QUERY_NEXT_MAIL_TIME => {}
            opcode => panic!("incorrect opcode. Expected MSG_QUERY_NEXT_MAIL_TIME, got {opcode:#?}"),
        };

        assert_eq!(HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/queries/msg_query_next_mail_time_client.wowm` line 7.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_msg_query_next_mail_time_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        match t {
            ClientOpcodeMessage::MSG_QUERY_NEXT_MAIL_TIME => {}
            opcode => panic!("incorrect opcode. Expected MSG_QUERY_NEXT_MAIL_TIME, got {opcode:#?}"),
        };

        assert_eq!(HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/queries/msg_query_next_mail_time_client.wowm` line 7.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_msg_query_next_mail_time_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        match t {
            ClientOpcodeMessage::MSG_QUERY_NEXT_MAIL_TIME => {}
            opcode => panic!("incorrect opcode. Expected MSG_QUERY_NEXT_MAIL_TIME, got {opcode:#?}"),
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
    use super::MSG_QUERY_NEXT_MAIL_TIME_Client;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ClientOpcodeMessage;
    use crate::wrath::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    const RAW0: [u8; 6] = [ 0x00, 0x04, 0x84, 0x02, 0x00, 0x00, ];

    pub(crate) fn expected0() -> MSG_QUERY_NEXT_MAIL_TIME_Client {
        MSG_QUERY_NEXT_MAIL_TIME_Client {
        }

    }

    // Generated from `wow_message_parser/wowm/world/queries/msg_query_next_mail_time_client.wowm` line 7.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn msg_query_next_mail_time_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        match t {
            ClientOpcodeMessage::MSG_QUERY_NEXT_MAIL_TIME => {}
            opcode => panic!("incorrect opcode. Expected MSG_QUERY_NEXT_MAIL_TIME, got {opcode:#?}"),
        };

        assert_eq!(HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/queries/msg_query_next_mail_time_client.wowm` line 7.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_msg_query_next_mail_time_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        match t {
            ClientOpcodeMessage::MSG_QUERY_NEXT_MAIL_TIME => {}
            opcode => panic!("incorrect opcode. Expected MSG_QUERY_NEXT_MAIL_TIME, got {opcode:#?}"),
        };

        assert_eq!(HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/queries/msg_query_next_mail_time_client.wowm` line 7.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_msg_query_next_mail_time_client0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        match t {
            ClientOpcodeMessage::MSG_QUERY_NEXT_MAIL_TIME => {}
            opcode => panic!("incorrect opcode. Expected MSG_QUERY_NEXT_MAIL_TIME, got {opcode:#?}"),
        };

        assert_eq!(HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

