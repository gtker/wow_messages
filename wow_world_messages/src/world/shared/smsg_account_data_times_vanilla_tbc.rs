use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// The purpose of this message is unknown, but it is required in order to prevent the chat box from being a white rectangle that is unable to show text.
/// Sending this causes the client to send [`CMSG_UPDATE_ACCOUNT_DATA`](crate::vanilla::CMSG_UPDATE_ACCOUNT_DATA) messages.
/// [`CMSG_UPDATE_ACCOUNT_DATA`](crate::vanilla::CMSG_UPDATE_ACCOUNT_DATA) and [`CMSG_REQUEST_ACCOUNT_DATA`](crate::vanilla::CMSG_REQUEST_ACCOUNT_DATA) act on blocks numbered 0 to 7. The 32 u32s in this message could possibly actually be 8 sets of u8`16` but it could also be a variable sized message.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/smsg_account_data_times.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/smsg_account_data_times.wowm#L1):
/// ```text
/// smsg SMSG_ACCOUNT_DATA_TIMES = 0x0209 {
///     u32[32] data;
/// }
/// ```
pub struct SMSG_ACCOUNT_DATA_TIMES {
    /// cmangos/vmangos/mangoszero sets to all zeros
    ///
    pub data: [u32; 32],
}

impl crate::Message for SMSG_ACCOUNT_DATA_TIMES {
    const OPCODE: u32 = 0x0209;

    fn size_without_header(&self) -> u32 {
        128
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // data: u32[32]
        for i in self.data.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 128 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0209, size: body_size as u32 });
        }

        // data: u32[32]
        let data = {
            let mut data = [u32::default(); 32];
            for i in data.iter_mut() {
                *i = crate::util::read_u32_le(&mut r)?;
            }
            data
        };

        Ok(Self {
            data,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_ACCOUNT_DATA_TIMES {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_ACCOUNT_DATA_TIMES {}

#[cfg(all(feature = "vanilla", test))]
mod test_vanilla {
    use super::SMSG_ACCOUNT_DATA_TIMES;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    const RAW0: [u8; 132] = [ 0x00, 0x82, 0x09, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> SMSG_ACCOUNT_DATA_TIMES {
        SMSG_ACCOUNT_DATA_TIMES {
            data: [ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, ],
        }

    }

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_account_data_times.wowm` line 13.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_ACCOUNT_DATA_TIMES0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ACCOUNT_DATA_TIMES(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ACCOUNT_DATA_TIMES, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.data, expected.data);

        assert_eq!(128 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_account_data_times.wowm` line 13.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_ACCOUNT_DATA_TIMES0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ACCOUNT_DATA_TIMES(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ACCOUNT_DATA_TIMES, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.data, expected.data);

        assert_eq!(128 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_account_data_times.wowm` line 13.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_ACCOUNT_DATA_TIMES0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ACCOUNT_DATA_TIMES(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ACCOUNT_DATA_TIMES, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.data, expected.data);

        assert_eq!(128 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "tbc", test))]
mod test_tbc {
    use super::SMSG_ACCOUNT_DATA_TIMES;
    use super::*;
    use super::super::*;
    use crate::tbc::opcodes::ServerOpcodeMessage;
    use crate::tbc::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    const RAW0: [u8; 132] = [ 0x00, 0x82, 0x09, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> SMSG_ACCOUNT_DATA_TIMES {
        SMSG_ACCOUNT_DATA_TIMES {
            data: [ 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, ],
        }

    }

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_account_data_times.wowm` line 13.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_ACCOUNT_DATA_TIMES0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ACCOUNT_DATA_TIMES(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ACCOUNT_DATA_TIMES, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.data, expected.data);

        assert_eq!(128 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_account_data_times.wowm` line 13.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_ACCOUNT_DATA_TIMES0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ACCOUNT_DATA_TIMES(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ACCOUNT_DATA_TIMES, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.data, expected.data);

        assert_eq!(128 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/smsg_account_data_times.wowm` line 13.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_ACCOUNT_DATA_TIMES0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_ACCOUNT_DATA_TIMES(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_ACCOUNT_DATA_TIMES, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.data, expected.data);

        assert_eq!(128 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

