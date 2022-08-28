use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::Class;
use crate::world::vanilla::Gender;
use crate::world::vanilla::Race;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Response to [`CMSG_NAME_QUERY`](crate::world::vanilla::CMSG_NAME_QUERY).
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_name_query_response.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_name_query_response.wowm#L3):
/// ```text
/// smsg SMSG_NAME_QUERY_RESPONSE = 0x0051 {
///     Guid guid;
///     CString character_name;
///     CString realm_name;
///     Race race;
///     Gender gender;
///     Class class;
/// }
/// ```
pub struct SMSG_NAME_QUERY_RESPONSE {
    pub guid: Guid,
    pub character_name: String,
    /// Used for showing cross realm realm names. If this is an empty string it is shown like a regular player on the same realm.
    ///
    pub realm_name: String,
    pub race: Race,
    pub gender: Gender,
    pub class: Class,
}

impl ServerMessage for SMSG_NAME_QUERY_RESPONSE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // character_name: CString
        w.write_all(self.character_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // realm_name: CString
        w.write_all(self.realm_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // race: Race
        w.write_all(&(self.race.as_int() as u32).to_le_bytes())?;

        // gender: Gender
        w.write_all(&(self.gender.as_int() as u32).to_le_bytes())?;

        // class: Class
        w.write_all(&(self.class.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0051;

    fn server_size(&self) -> u16 {
        (self.size() + 4) as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // character_name: CString
        let character_name = crate::util::read_c_string_to_vec(r)?;
        let character_name = String::from_utf8(character_name)?;

        // realm_name: CString
        let realm_name = crate::util::read_c_string_to_vec(r)?;
        let realm_name = String::from_utf8(realm_name)?;

        // race: Race
        let race: Race = (crate::util::read_u32_le(r)? as u8).try_into()?;

        // gender: Gender
        let gender: Gender = (crate::util::read_u32_le(r)? as u8).try_into()?;

        // class: Class
        let class: Class = (crate::util::read_u32_le(r)? as u8).try_into()?;

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
    use super::SMSG_NAME_QUERY_RESPONSE;
    use crate::world::vanilla::Class;
    use crate::world::vanilla::Gender;
    use crate::world::vanilla::Race;
    use super::*;
    use super::super::*;
    use crate::world::vanilla::opcodes::ServerOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{UpdateMask, UpdateContainer, UpdateItem, UpdateCorpse, UpdateGameObject, UpdateDynamicObject, UpdateUnit, UpdatePlayer};
    use crate::{ClientMessage, ServerMessage};

    const RAW0: [u8; 30] = [ 0x00, 0x1C, 0x51, 0x00, 0xEF, 0xBE, 0xAD, 0xDE, 0x00,
         0x00, 0x00, 0x00, 0x41, 0x73, 0x64, 0x66, 0x00, 0x00, 0x01, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, ];

    // Generated from `wow_message_parser/wowm/world/queries/smsg_name_query_response.wowm` line 16.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_NAME_QUERY_RESPONSE0() {
        let expected = SMSG_NAME_QUERY_RESPONSE {
            guid: Guid::new(0xDEADBEEF),
            character_name: String::from("Asdf"),
            realm_name: String::from(""),
            race: Race::Human,
            gender: Gender::Male,
            class: Class::Warrior,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_NAME_QUERY_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_NAME_QUERY_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.character_name, expected.character_name);
        assert_eq!(t.realm_name, expected.realm_name);
        assert_eq!(t.race, expected.race);
        assert_eq!(t.gender, expected.gender);
        assert_eq!(t.class, expected.class);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/queries/smsg_name_query_response.wowm` line 16.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_NAME_QUERY_RESPONSE0() {
        let expected = SMSG_NAME_QUERY_RESPONSE {
            guid: Guid::new(0xDEADBEEF),
            character_name: String::from("Asdf"),
            realm_name: String::from(""),
            race: Race::Human,
            gender: Gender::Male,
            class: Class::Warrior,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_NAME_QUERY_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_NAME_QUERY_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.character_name, expected.character_name);
        assert_eq!(t.realm_name, expected.realm_name);
        assert_eq!(t.race, expected.race);
        assert_eq!(t.gender, expected.gender);
        assert_eq!(t.class, expected.class);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/queries/smsg_name_query_response.wowm` line 16.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_NAME_QUERY_RESPONSE0() {
        let expected = SMSG_NAME_QUERY_RESPONSE {
            guid: Guid::new(0xDEADBEEF),
            character_name: String::from("Asdf"),
            realm_name: String::from(""),
            race: Race::Human,
            gender: Gender::Male,
            class: Class::Warrior,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_NAME_QUERY_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_NAME_QUERY_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.character_name, expected.character_name);
        assert_eq!(t.realm_name, expected.realm_name);
        assert_eq!(t.race, expected.race);
        assert_eq!(t.gender, expected.gender);
        assert_eq!(t.class, expected.class);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    const RAW1: [u8; 31] = [ 0x00, 0x1D, 0x51, 0x00, 0xEF, 0xBE, 0xAD, 0xDE, 0x00,
         0x00, 0x00, 0x00, 0x41, 0x73, 0x64, 0x66, 0x00, 0x41, 0x00, 0x01, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, ];

    // Generated from `wow_message_parser/wowm/world/queries/smsg_name_query_response.wowm` line 34.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn SMSG_NAME_QUERY_RESPONSE1() {
        let expected = SMSG_NAME_QUERY_RESPONSE {
            guid: Guid::new(0xDEADBEEF),
            character_name: String::from("Asdf"),
            realm_name: String::from("A"),
            race: Race::Human,
            gender: Gender::Male,
            class: Class::Warrior,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW1)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_NAME_QUERY_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_NAME_QUERY_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.character_name, expected.character_name);
        assert_eq!(t.realm_name, expected.realm_name);
        assert_eq!(t.race, expected.race);
        assert_eq!(t.gender, expected.gender);
        assert_eq!(t.class, expected.class);

        assert_eq!(t.size() + header_size, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/queries/smsg_name_query_response.wowm` line 34.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_SMSG_NAME_QUERY_RESPONSE1() {
        let expected = SMSG_NAME_QUERY_RESPONSE {
            guid: Guid::new(0xDEADBEEF),
            character_name: String::from("Asdf"),
            realm_name: String::from("A"),
            race: Race::Human,
            gender: Gender::Male,
            class: Class::Warrior,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_NAME_QUERY_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_NAME_QUERY_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.character_name, expected.character_name);
        assert_eq!(t.realm_name, expected.realm_name);
        assert_eq!(t.race, expected.race);
        assert_eq!(t.gender, expected.gender);
        assert_eq!(t.class, expected.class);

        assert_eq!(t.size() + header_size, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/queries/smsg_name_query_response.wowm` line 34.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_SMSG_NAME_QUERY_RESPONSE1() {
        let expected = SMSG_NAME_QUERY_RESPONSE {
            guid: Guid::new(0xDEADBEEF),
            character_name: String::from("Asdf"),
            realm_name: String::from("A"),
            race: Race::Human,
            gender: Gender::Male,
            class: Class::Warrior,
        };

        let header_size = 2 + 2;
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_NAME_QUERY_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_NAME_QUERY_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.character_name, expected.character_name);
        assert_eq!(t.realm_name, expected.realm_name);
        assert_eq!(t.race, expected.race);
        assert_eq!(t.gender, expected.gender);
        assert_eq!(t.class, expected.class);

        assert_eq!(t.size() + header_size, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

}
