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

#[cfg(feature = "print-testcase")]
impl CMSG_REQUEST_ACCOUNT_DATA {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_REQUEST_ACCOUNT_DATA {{").unwrap();
        // Members
        writeln!(s, "    data_type = {};", self.data_type).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 10_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 522_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "data_type");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1 2 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for CMSG_REQUEST_ACCOUNT_DATA {}
impl crate::Message for CMSG_REQUEST_ACCOUNT_DATA {
    const OPCODE: u32 = 0x020a;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // data_type: u32
        w.write_all(&self.data_type.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x020A, size: body_size });
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
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_REQUEST_ACCOUNT_DATA;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ClientOpcodeMessage;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    fn assert(t: &CMSG_REQUEST_ACCOUNT_DATA, expected: &CMSG_REQUEST_ACCOUNT_DATA) {
        assert_eq!(t.data_type, expected.data_type);
    }

    const RAW0: [u8; 10] = [ 0x00, 0x08, 0x0A, 0x02, 0x00, 0x00, 0x06, 0x00, 0x00,
         0x00, ];

    pub(crate) fn expected0() -> CMSG_REQUEST_ACCOUNT_DATA {
        CMSG_REQUEST_ACCOUNT_DATA {
            data_type: 0x6,
        }

    }

    // Generated from `wow_message_parser/wowm/world/login_logout/cmsg_request_account_data.wowm` line 10.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_request_account_data0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_REQUEST_ACCOUNT_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_REQUEST_ACCOUNT_DATA, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/cmsg_request_account_data.wowm` line 10.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_request_account_data0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_REQUEST_ACCOUNT_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_REQUEST_ACCOUNT_DATA, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/cmsg_request_account_data.wowm` line 10.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_request_account_data0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_REQUEST_ACCOUNT_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_REQUEST_ACCOUNT_DATA, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "tbc", test))]
mod test_tbc {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_REQUEST_ACCOUNT_DATA;
    use super::*;
    use super::super::*;
    use crate::tbc::opcodes::ClientOpcodeMessage;
    use crate::tbc::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    fn assert(t: &CMSG_REQUEST_ACCOUNT_DATA, expected: &CMSG_REQUEST_ACCOUNT_DATA) {
        assert_eq!(t.data_type, expected.data_type);
    }

    const RAW0: [u8; 10] = [ 0x00, 0x08, 0x0A, 0x02, 0x00, 0x00, 0x06, 0x00, 0x00,
         0x00, ];

    pub(crate) fn expected0() -> CMSG_REQUEST_ACCOUNT_DATA {
        CMSG_REQUEST_ACCOUNT_DATA {
            data_type: 0x6,
        }

    }

    // Generated from `wow_message_parser/wowm/world/login_logout/cmsg_request_account_data.wowm` line 10.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_request_account_data0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_REQUEST_ACCOUNT_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_REQUEST_ACCOUNT_DATA, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/cmsg_request_account_data.wowm` line 10.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_request_account_data0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_REQUEST_ACCOUNT_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_REQUEST_ACCOUNT_DATA, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/cmsg_request_account_data.wowm` line 10.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_request_account_data0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_REQUEST_ACCOUNT_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_REQUEST_ACCOUNT_DATA, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "wrath", test))]
mod test_wrath {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_REQUEST_ACCOUNT_DATA;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ClientOpcodeMessage;
    use crate::wrath::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    fn assert(t: &CMSG_REQUEST_ACCOUNT_DATA, expected: &CMSG_REQUEST_ACCOUNT_DATA) {
        assert_eq!(t.data_type, expected.data_type);
    }

    const RAW0: [u8; 10] = [ 0x00, 0x08, 0x0A, 0x02, 0x00, 0x00, 0x06, 0x00, 0x00,
         0x00, ];

    pub(crate) fn expected0() -> CMSG_REQUEST_ACCOUNT_DATA {
        CMSG_REQUEST_ACCOUNT_DATA {
            data_type: 0x6,
        }

    }

    // Generated from `wow_message_parser/wowm/world/login_logout/cmsg_request_account_data.wowm` line 10.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_request_account_data0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_REQUEST_ACCOUNT_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_REQUEST_ACCOUNT_DATA, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/cmsg_request_account_data.wowm` line 10.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_request_account_data0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_REQUEST_ACCOUNT_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_REQUEST_ACCOUNT_DATA, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/login_logout/cmsg_request_account_data.wowm` line 10.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_request_account_data0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_REQUEST_ACCOUNT_DATA(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_REQUEST_ACCOUNT_DATA, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(4 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

