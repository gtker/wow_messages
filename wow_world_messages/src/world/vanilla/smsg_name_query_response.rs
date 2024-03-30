use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::{
    Class, Gender, Race,
};

/// Response to [`CMSG_NAME_QUERY`](crate::vanilla::CMSG_NAME_QUERY).
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_name_query_response.wowm:2`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_name_query_response.wowm#L2):
/// ```text
/// smsg SMSG_NAME_QUERY_RESPONSE = 0x0051 {
///     Guid guid;
///     CString character_name;
///     CString realm_name;
///     (u32)Race race;
///     (u32)Gender gender;
///     (u32)Class class;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_NAME_QUERY_RESPONSE {
    pub guid: Guid,
    pub character_name: String,
    /// Used for showing cross realm realm names. If this is an empty string it is shown like a regular player on the same realm.
    pub realm_name: String,
    pub race: Race,
    pub gender: Gender,
    pub class: Class,
}

impl crate::private::Sealed for SMSG_NAME_QUERY_RESPONSE {}
impl SMSG_NAME_QUERY_RESPONSE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(22..=532).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // character_name: CString
        let character_name = {
            let character_name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(character_name)?
        };

        // realm_name: CString
        let realm_name = {
            let realm_name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(realm_name)?
        };

        // race: Race
        let race = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        // gender: Gender
        let gender = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        // class: Class
        let class = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        Ok(Self {
            guid,
            character_name,
            realm_name,
            race,
            gender,
            class,
        })
    }

}

impl crate::Message for SMSG_NAME_QUERY_RESPONSE {
    const OPCODE: u32 = 0x0051;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_NAME_QUERY_RESPONSE"
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // character_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.character_name.as_bytes().iter().next_back(), Some(&0_u8), "String `character_name` must not be null-terminated.");
        w.write_all(self.character_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // realm_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.realm_name.as_bytes().iter().next_back(), Some(&0_u8), "String `realm_name` must not be null-terminated.");
        w.write_all(self.realm_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // race: Race
        w.write_all(&u32::from(self.race.as_int()).to_le_bytes())?;

        // gender: Gender
        w.write_all(&u32::from(self.gender.as_int()).to_le_bytes())?;

        // class: Class
        w.write_all(&u32::from(self.class.as_int()).to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(81, "SMSG_NAME_QUERY_RESPONSE", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_NAME_QUERY_RESPONSE {}

impl SMSG_NAME_QUERY_RESPONSE {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + self.character_name.len() + 1 // character_name: CString
        + self.realm_name.len() + 1 // realm_name: CString
        + 4 // race: Race
        + 4 // gender: Gender
        + 4 // class: Class
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_NAME_QUERY_RESPONSE;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    const RAW0: [u8; 30] = [ 0x00, 0x1C, 0x51, 0x00, 0xEF, 0xBE, 0xAD, 0xDE, 0x00,
         0x00, 0x00, 0x00, 0x41, 0x73, 0x64, 0x66, 0x00, 0x00, 0x01, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> SMSG_NAME_QUERY_RESPONSE {
        SMSG_NAME_QUERY_RESPONSE {
            guid: Guid::new(0xDEADBEEF),
            character_name: String::from("Asdf"),
            realm_name: String::from(""),
            race: Race::Human,
            gender: Gender::Male,
            class: Class::Warrior,
        }

    }

    // Generated from `wow_message_parser/wowm/world/queries/smsg_name_query_response.wowm` line 58.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_name_query_response0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_NAME_QUERY_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_NAME_QUERY_RESPONSE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/queries/smsg_name_query_response.wowm` line 58.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_name_query_response0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_NAME_QUERY_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_NAME_QUERY_RESPONSE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/queries/smsg_name_query_response.wowm` line 58.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_name_query_response0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_NAME_QUERY_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_NAME_QUERY_RESPONSE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    const RAW1: [u8; 31] = [ 0x00, 0x1D, 0x51, 0x00, 0xEF, 0xBE, 0xAD, 0xDE, 0x00,
         0x00, 0x00, 0x00, 0x41, 0x73, 0x64, 0x66, 0x00, 0x41, 0x00, 0x01, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected1() -> SMSG_NAME_QUERY_RESPONSE {
        SMSG_NAME_QUERY_RESPONSE {
            guid: Guid::new(0xDEADBEEF),
            character_name: String::from("Asdf"),
            realm_name: String::from("A"),
            race: Race::Human,
            gender: Gender::Male,
            class: Class::Warrior,
        }

    }

    // Generated from `wow_message_parser/wowm/world/queries/smsg_name_query_response.wowm` line 78.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_name_query_response1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW1)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_NAME_QUERY_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_NAME_QUERY_RESPONSE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/queries/smsg_name_query_response.wowm` line 78.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_name_query_response1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_NAME_QUERY_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_NAME_QUERY_RESPONSE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/queries/smsg_name_query_response.wowm` line 78.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_name_query_response1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_NAME_QUERY_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_NAME_QUERY_RESPONSE, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

}

