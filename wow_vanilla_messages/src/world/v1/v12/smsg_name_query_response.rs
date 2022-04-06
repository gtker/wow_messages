use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Class, ClassError};
use crate::world::v1::v12::{Gender, GenderError};
use crate::world::v1::v12::{Race, RaceError};
use crate::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_name_query_response.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_name_query_response.wowm#L3):
/// ```text
/// smsg SMSG_NAME_QUERY_RESPONSE = 0x51 {
///     u64 guid;
///     CString character_name;
///     CString realm_name;
///     Race race;
///     Gender gender;
///     Class class;
/// }
/// ```
pub struct SMSG_NAME_QUERY_RESPONSE {
    pub guid: u64,
    pub character_name: String,
    pub realm_name: String,
    pub race: Race,
    pub gender: Gender,
    pub class: Class,
}

impl WorldServerMessageWrite for SMSG_NAME_QUERY_RESPONSE {
    const OPCODE: u16 = 0x51;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (self.size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (self.size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_NAME_QUERY_RESPONSE {
    type Error = SMSG_NAME_QUERY_RESPONSEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // guid: u64
        let guid = crate::util::read_u64_le(r)?;

        // character_name: CString
        let character_name = crate::util::read_c_string_to_vec(r)?;
        let character_name = String::from_utf8(character_name)?;

        // realm_name: CString
        let realm_name = crate::util::read_c_string_to_vec(r)?;
        let realm_name = String::from_utf8(realm_name)?;

        // race: Race
        let race = Race::read_u32_le(r)?;

        // gender: Gender
        let gender = Gender::read_u32_le(r)?;

        // class: Class
        let class = Class::read_u32_le(r)?;

        Ok(Self {
            guid,
            character_name,
            realm_name,
            race,
            gender,
            class,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // guid: u64
        w.write_all(&self.guid.to_le_bytes())?;

        // character_name: CString
        w.write_all(self.character_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // realm_name: CString
        w.write_all(self.realm_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // race: Race
        self.race.write_u32_le(w)?;

        // gender: Gender
        self.gender.write_u32_le(w)?;

        // class: Class
        self.class.write_u32_le(w)?;

        Ok(())
    }
}

impl VariableSized for SMSG_NAME_QUERY_RESPONSE {
    fn size(&self) -> usize {
        8 // guid: u64
        + self.character_name.len() + 1 // character_name: CString and Null Terminator
        + self.realm_name.len() + 1 // realm_name: CString and Null Terminator
        + 4 // race: Race upcasted to u32
        + 4 // gender: Gender upcasted to u32
        + 4 // class: Class upcasted to u32
    }
}

impl MaximumPossibleSized for SMSG_NAME_QUERY_RESPONSE {
    fn maximum_possible_size() -> usize {
        8 // guid: u64
        + 256 // character_name: CString
        + 256 // realm_name: CString
        + Race::maximum_possible_size() // race: Race
        + Gender::maximum_possible_size() // gender: Gender
        + Class::maximum_possible_size() // class: Class
    }
}

#[derive(Debug)]
pub enum SMSG_NAME_QUERY_RESPONSEError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
    Class(ClassError),
    Gender(GenderError),
    Race(RaceError),
}

impl std::error::Error for SMSG_NAME_QUERY_RESPONSEError {}
impl std::fmt::Display for SMSG_NAME_QUERY_RESPONSEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::Class(i) => i.fmt(f),
            Self::Gender(i) => i.fmt(f),
            Self::Race(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_NAME_QUERY_RESPONSEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_NAME_QUERY_RESPONSEError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<ClassError> for SMSG_NAME_QUERY_RESPONSEError {
    fn from(e: ClassError) -> Self {
        Self::Class(e)
    }
}

impl From<GenderError> for SMSG_NAME_QUERY_RESPONSEError {
    fn from(e: GenderError) -> Self {
        Self::Gender(e)
    }
}

impl From<RaceError> for SMSG_NAME_QUERY_RESPONSEError {
    fn from(e: RaceError) -> Self {
        Self::Race(e)
    }
}

#[cfg(test)]
mod test {
    use crate::ReadableAndWritable;
    use std::io::Cursor;
    use super::SMSG_NAME_QUERY_RESPONSE;
    use crate::VariableSized;
    use crate::world::v1::v12::Class;
    use crate::world::v1::v12::Gender;
    use crate::world::v1::v12::Race;
    use crate::world::v1::v12::opcodes::WorldServerOpcodeMessage;
    use crate::{WorldMessageBody, WorldClientMessageWrite, WorldServerMessageWrite, WorldMessage};

    // Generated from `wow_message_parser/wowm/world/queries/smsg_name_query_response.wowm` line 12.
    #[test]
    fn SMSG_NAME_QUERY_RESPONSE0() {
        let raw: Vec<u8> = vec![ 0x00, 0x1C, 0x51, 0x00, 0xEF, 0xBE, 0xAD, 0xDE,
             0x00, 0x00, 0x00, 0x00, 0x41, 0x73, 0x64, 0x66, 0x00, 0x00, 0x01, 0x00,
             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, ];

        let expected = SMSG_NAME_QUERY_RESPONSE {
            guid: 0xDEADBEEF,
            character_name: String::from("Asdf"),
            realm_name: String::from(""),
            race: Race::HUMAN,
            gender: Gender::MALE,
            class: Class::WARRIOR,
        };

        let header_size = 2 + 2;
        let t = WorldServerOpcodeMessage::read_unencrypted(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            WorldServerOpcodeMessage::SMSG_NAME_QUERY_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_NAME_QUERY_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.character_name, expected.character_name);
        assert_eq!(t.realm_name, expected.realm_name);
        assert_eq!(t.race, expected.race);
        assert_eq!(t.gender, expected.gender);
        assert_eq!(t.class, expected.class);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write_unencrypted_server(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

    // Generated from `wow_message_parser/wowm/world/queries/smsg_name_query_response.wowm` line 30.
    #[test]
    fn SMSG_NAME_QUERY_RESPONSE1() {
        let raw: Vec<u8> = vec![ 0x00, 0x1D, 0x51, 0x00, 0xEF, 0xBE, 0xAD, 0xDE,
             0x00, 0x00, 0x00, 0x00, 0x41, 0x73, 0x64, 0x66, 0x00, 0x41, 0x00, 0x01,
             0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, ];

        let expected = SMSG_NAME_QUERY_RESPONSE {
            guid: 0xDEADBEEF,
            character_name: String::from("Asdf"),
            realm_name: String::from("A"),
            race: Race::HUMAN,
            gender: Gender::MALE,
            class: Class::WARRIOR,
        };

        let header_size = 2 + 2;
        let t = WorldServerOpcodeMessage::read_unencrypted(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            WorldServerOpcodeMessage::SMSG_NAME_QUERY_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_NAME_QUERY_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.guid, expected.guid);
        assert_eq!(t.character_name, expected.character_name);
        assert_eq!(t.realm_name, expected.realm_name);
        assert_eq!(t.race, expected.race);
        assert_eq!(t.gender, expected.gender);
        assert_eq!(t.class, expected.class);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write_unencrypted_server(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

}
