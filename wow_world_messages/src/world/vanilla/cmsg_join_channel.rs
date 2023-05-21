use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/cmsg_join_channel.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/cmsg_join_channel.wowm#L1):
/// ```text
/// cmsg CMSG_JOIN_CHANNEL = 0x0097 {
///     CString channel_name;
///     CString channel_password;
/// }
/// ```
pub struct CMSG_JOIN_CHANNEL {
    pub channel_name: String,
    pub channel_password: String,
}

impl crate::private::Sealed for CMSG_JOIN_CHANNEL {}
impl CMSG_JOIN_CHANNEL {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(2..=512).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // channel_name: CString
        let channel_name = {
            let channel_name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(channel_name)?
        };

        // channel_password: CString
        let channel_password = {
            let channel_password = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(channel_password)?
        };

        Ok(Self {
            channel_name,
            channel_password,
        })
    }

}

impl crate::Message for CMSG_JOIN_CHANNEL {
    const OPCODE: u32 = 0x0097;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // channel_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.channel_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `channel_name` must not be null-terminated.");
        w.write_all(self.channel_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // channel_password: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.channel_password.as_bytes().iter().rev().next(), Some(&0_u8), "String `channel_password` must not be null-terminated.");
        w.write_all(self.channel_password.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(151, "CMSG_JOIN_CHANNEL", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_JOIN_CHANNEL {}

impl CMSG_JOIN_CHANNEL {
    pub(crate) fn size(&self) -> usize {
        self.channel_name.len() + 1 // channel_name: CString
        + self.channel_password.len() + 1 // channel_password: CString
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_JOIN_CHANNEL;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ClientOpcodeMessage;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    fn assert(t: &CMSG_JOIN_CHANNEL, expected: &CMSG_JOIN_CHANNEL) {
        assert_eq!(t.channel_name, expected.channel_name);
        assert_eq!(t.channel_password, expected.channel_password);
    }

    const RAW0: [u8; 31] = [ 0x00, 0x1D, 0x97, 0x00, 0x00, 0x00, 0x47, 0x65, 0x6E,
         0x65, 0x72, 0x61, 0x6C, 0x20, 0x2D, 0x20, 0x45, 0x6C, 0x77, 0x79, 0x6E,
         0x6E, 0x20, 0x46, 0x6F, 0x72, 0x65, 0x73, 0x74, 0x00, 0x00, ];

    pub(crate) fn expected0() -> CMSG_JOIN_CHANNEL {
        CMSG_JOIN_CHANNEL {
            channel_name: String::from("General - Elwynn Forest"),
            channel_password: String::from(""),
        }

    }

    // Generated from `wow_message_parser/wowm/world/chat/cmsg_join_channel.wowm` line 8.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_join_channel0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_JOIN_CHANNEL(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_JOIN_CHANNEL, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/chat/cmsg_join_channel.wowm` line 8.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_join_channel0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_JOIN_CHANNEL(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_JOIN_CHANNEL, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/chat/cmsg_join_channel.wowm` line 8.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_join_channel0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_JOIN_CHANNEL(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_JOIN_CHANNEL, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    const RAW1: [u8; 36] = [ 0x00, 0x22, 0x97, 0x00, 0x00, 0x00, 0x4C, 0x6F, 0x63,
         0x61, 0x6C, 0x44, 0x65, 0x66, 0x65, 0x6E, 0x73, 0x65, 0x20, 0x2D, 0x20,
         0x45, 0x6C, 0x77, 0x79, 0x6E, 0x6E, 0x20, 0x46, 0x6F, 0x72, 0x65, 0x73,
         0x74, 0x00, 0x00, ];

    pub(crate) fn expected1() -> CMSG_JOIN_CHANNEL {
        CMSG_JOIN_CHANNEL {
            channel_name: String::from("LocalDefense - Elwynn Forest"),
            channel_password: String::from(""),
        }

    }

    // Generated from `wow_message_parser/wowm/world/chat/cmsg_join_channel.wowm` line 20.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_join_channel1() {
        let expected = expected1();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW1)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_JOIN_CHANNEL(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_JOIN_CHANNEL, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/chat/cmsg_join_channel.wowm` line 20.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_join_channel1() {
        let expected = expected1();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_JOIN_CHANNEL(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_JOIN_CHANNEL, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/chat/cmsg_join_channel.wowm` line 20.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_join_channel1() {
        let expected = expected1();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_JOIN_CHANNEL(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_JOIN_CHANNEL, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

}

