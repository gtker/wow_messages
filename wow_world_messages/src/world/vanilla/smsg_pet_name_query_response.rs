use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_pet_name_query_response.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_pet_name_query_response.wowm#L1):
/// ```text
/// smsg SMSG_PET_NAME_QUERY_RESPONSE = 0x0053 {
///     u32 pet_number;
///     CString name;
///     u32 pet_name_timestamp;
/// }
/// ```
pub struct SMSG_PET_NAME_QUERY_RESPONSE {
    pub pet_number: u32,
    pub name: String,
    pub pet_name_timestamp: u32,
}

impl crate::Message for SMSG_PET_NAME_QUERY_RESPONSE {
    const OPCODE: u32 = 0x0053;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // pet_number: u32
        w.write_all(&self.pet_number.to_le_bytes())?;

        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().rev().next(), Some(&0_u8), "String `name` must not be null-terminated.");
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // pet_name_timestamp: u32
        w.write_all(&self.pet_name_timestamp.to_le_bytes())?;

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(9..=264).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0053, size: body_size as u32 });
        }

        // pet_number: u32
        let pet_number = crate::util::read_u32_le(r)?;

        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        // pet_name_timestamp: u32
        let pet_name_timestamp = crate::util::read_u32_le(r)?;

        Ok(Self {
            pet_number,
            name,
            pet_name_timestamp,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_PET_NAME_QUERY_RESPONSE {}

impl SMSG_PET_NAME_QUERY_RESPONSE {
    pub(crate) fn size(&self) -> usize {
        4 // pet_number: u32
        + self.name.len() + 1 // name: CString
        + 4 // pet_name_timestamp: u32
    }
}

#[cfg(test)]
mod test {
    use super::SMSG_PET_NAME_QUERY_RESPONSE;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const RAW0: [u8; 19] = [ 0x00, 0x11, 0x53, 0x00, 0xEF, 0xBE, 0xAD, 0xDE, 0x41,
         0x42, 0x43, 0x44, 0x45, 0x46, 0x00, 0xDE, 0xCA, 0xFA, 0x00, ];

    // Generated from `wow_message_parser/wowm/world/queries/smsg_pet_name_query_response.wowm` line 9.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_PET_NAME_QUERY_RESPONSE0() {
        let expected = SMSG_PET_NAME_QUERY_RESPONSE {
            pet_number: 0xDEADBEEF,
            name: String::from("ABCDEF"),
            pet_name_timestamp: 0xFACADE,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_PET_NAME_QUERY_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_PET_NAME_QUERY_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.pet_number, expected.pet_number);
        assert_eq!(t.name, expected.name);
        assert_eq!(t.pet_name_timestamp, expected.pet_name_timestamp);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/queries/smsg_pet_name_query_response.wowm` line 9.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_PET_NAME_QUERY_RESPONSE0() {
        let expected = SMSG_PET_NAME_QUERY_RESPONSE {
            pet_number: 0xDEADBEEF,
            name: String::from("ABCDEF"),
            pet_name_timestamp: 0xFACADE,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_PET_NAME_QUERY_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_PET_NAME_QUERY_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.pet_number, expected.pet_number);
        assert_eq!(t.name, expected.name);
        assert_eq!(t.pet_name_timestamp, expected.pet_name_timestamp);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/queries/smsg_pet_name_query_response.wowm` line 9.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_PET_NAME_QUERY_RESPONSE0() {
        let expected = SMSG_PET_NAME_QUERY_RESPONSE {
            pet_number: 0xDEADBEEF,
            name: String::from("ABCDEF"),
            pet_name_timestamp: 0xFACADE,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_PET_NAME_QUERY_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_PET_NAME_QUERY_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.pet_number, expected.pet_number);
        assert_eq!(t.name, expected.name);
        assert_eq!(t.pet_name_timestamp, expected.pet_name_timestamp);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

