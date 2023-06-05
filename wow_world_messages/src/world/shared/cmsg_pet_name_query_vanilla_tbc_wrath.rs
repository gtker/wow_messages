use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/cmsg_pet_name_query.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/cmsg_pet_name_query.wowm#L1):
/// ```text
/// cmsg CMSG_PET_NAME_QUERY = 0x0052 {
///     u32 pet_number;
///     Guid guid;
/// }
/// ```
pub struct CMSG_PET_NAME_QUERY {
    pub pet_number: u32,
    pub guid: Guid,
}

#[cfg(feature = "print-testcase")]
impl CMSG_PET_NAME_QUERY {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_PET_NAME_QUERY {{").unwrap();
        // Members
        writeln!(s, "    pet_number = {};", self.pet_number).unwrap();
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 16_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 82_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "pet_number", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1 2 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for CMSG_PET_NAME_QUERY {}
impl crate::Message for CMSG_PET_NAME_QUERY {
    const OPCODE: u32 = 0x0052;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        CMSG_PET_NAME_QUERY::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // pet_number: u32
        w.write_all(&self.pet_number.to_le_bytes())?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0052, size: body_size });
        }

        // pet_number: u32
        let pet_number = crate::util::read_u32_le(&mut r)?;

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        Ok(Self {
            pet_number,
            guid,
        })
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_PET_NAME_QUERY {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_PET_NAME_QUERY {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_PET_NAME_QUERY {}

#[cfg(all(feature = "vanilla", test))]
mod test_vanilla {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_PET_NAME_QUERY;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ClientOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    fn assert(t: &CMSG_PET_NAME_QUERY, expected: &CMSG_PET_NAME_QUERY) {
        assert_eq!(t.pet_number, expected.pet_number);
        assert_eq!(t.guid, expected.guid);
    }

    const RAW0: [u8; 18] = [ 0x00, 0x10, 0x52, 0x00, 0x00, 0x00, 0xEF, 0xBE, 0xAD,
         0xDE, 0xEF, 0xBE, 0xAD, 0xDE, 0xDE, 0xCA, 0xFA, 0x00, ];

    pub(crate) fn expected0() -> CMSG_PET_NAME_QUERY {
        CMSG_PET_NAME_QUERY {
            pet_number: 0xDEADBEEF,
            guid: Guid::new(0xFACADEDEADBEEF),
        }

    }

    // Generated from `wow_message_parser/wowm/world/queries/cmsg_pet_name_query.wowm` line 8.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_pet_name_query0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PET_NAME_QUERY(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PET_NAME_QUERY, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(12 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/queries/cmsg_pet_name_query.wowm` line 8.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_pet_name_query0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PET_NAME_QUERY(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PET_NAME_QUERY, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(12 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/queries/cmsg_pet_name_query.wowm` line 8.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_pet_name_query0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PET_NAME_QUERY(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PET_NAME_QUERY, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(12 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "tbc", test))]
mod test_tbc {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_PET_NAME_QUERY;
    use super::*;
    use super::super::*;
    use crate::tbc::opcodes::ClientOpcodeMessage;
    use crate::Guid;
    use crate::tbc::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    fn assert(t: &CMSG_PET_NAME_QUERY, expected: &CMSG_PET_NAME_QUERY) {
        assert_eq!(t.pet_number, expected.pet_number);
        assert_eq!(t.guid, expected.guid);
    }

    const RAW0: [u8; 18] = [ 0x00, 0x10, 0x52, 0x00, 0x00, 0x00, 0xEF, 0xBE, 0xAD,
         0xDE, 0xEF, 0xBE, 0xAD, 0xDE, 0xDE, 0xCA, 0xFA, 0x00, ];

    pub(crate) fn expected0() -> CMSG_PET_NAME_QUERY {
        CMSG_PET_NAME_QUERY {
            pet_number: 0xDEADBEEF,
            guid: Guid::new(0xFACADEDEADBEEF),
        }

    }

    // Generated from `wow_message_parser/wowm/world/queries/cmsg_pet_name_query.wowm` line 8.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_pet_name_query0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PET_NAME_QUERY(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PET_NAME_QUERY, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(12 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/queries/cmsg_pet_name_query.wowm` line 8.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_pet_name_query0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PET_NAME_QUERY(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PET_NAME_QUERY, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(12 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/queries/cmsg_pet_name_query.wowm` line 8.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_pet_name_query0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PET_NAME_QUERY(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PET_NAME_QUERY, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(12 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "wrath", test))]
mod test_wrath {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_PET_NAME_QUERY;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ClientOpcodeMessage;
    use crate::Guid;
    use crate::wrath::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    fn assert(t: &CMSG_PET_NAME_QUERY, expected: &CMSG_PET_NAME_QUERY) {
        assert_eq!(t.pet_number, expected.pet_number);
        assert_eq!(t.guid, expected.guid);
    }

    const RAW0: [u8; 18] = [ 0x00, 0x10, 0x52, 0x00, 0x00, 0x00, 0xEF, 0xBE, 0xAD,
         0xDE, 0xEF, 0xBE, 0xAD, 0xDE, 0xDE, 0xCA, 0xFA, 0x00, ];

    pub(crate) fn expected0() -> CMSG_PET_NAME_QUERY {
        CMSG_PET_NAME_QUERY {
            pet_number: 0xDEADBEEF,
            guid: Guid::new(0xFACADEDEADBEEF),
        }

    }

    // Generated from `wow_message_parser/wowm/world/queries/cmsg_pet_name_query.wowm` line 8.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_pet_name_query0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PET_NAME_QUERY(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PET_NAME_QUERY, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(12 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/queries/cmsg_pet_name_query.wowm` line 8.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_pet_name_query0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PET_NAME_QUERY(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PET_NAME_QUERY, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(12 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/queries/cmsg_pet_name_query.wowm` line 8.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_pet_name_query0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PET_NAME_QUERY(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PET_NAME_QUERY, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(12 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

