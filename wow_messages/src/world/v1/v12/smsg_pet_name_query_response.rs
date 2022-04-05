use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/add_messages.wowm:133`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/add_messages.wowm#L133):
/// ```text
/// smsg SMSG_PET_NAME_QUERY_RESPONSE = 0x53 {
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

impl WorldServerMessageWrite for SMSG_PET_NAME_QUERY_RESPONSE {
    const OPCODE: u16 = 0x53;

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
impl WorldMessageBody for SMSG_PET_NAME_QUERY_RESPONSE {
    type Error = SMSG_PET_NAME_QUERY_RESPONSEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
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

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // pet_number: u32
        w.write_all(&self.pet_number.to_le_bytes())?;

        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // pet_name_timestamp: u32
        w.write_all(&self.pet_name_timestamp.to_le_bytes())?;

        Ok(())
    }
}

impl VariableSized for SMSG_PET_NAME_QUERY_RESPONSE {
    fn size(&self) -> usize {
        4 // pet_number: u32
        + self.name.len() + 1 // name: CString and Null Terminator
        + 4 // pet_name_timestamp: u32
    }
}

impl MaximumPossibleSized for SMSG_PET_NAME_QUERY_RESPONSE {
    fn maximum_possible_size() -> usize {
        4 // pet_number: u32
        + 256 // name: CString
        + 4 // pet_name_timestamp: u32
    }
}

#[derive(Debug)]
pub enum SMSG_PET_NAME_QUERY_RESPONSEError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for SMSG_PET_NAME_QUERY_RESPONSEError {}
impl std::fmt::Display for SMSG_PET_NAME_QUERY_RESPONSEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_PET_NAME_QUERY_RESPONSEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_PET_NAME_QUERY_RESPONSEError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

#[cfg(test)]
mod test {
    use crate::ReadableAndWritable;
    use std::io::Cursor;
    use super::SMSG_PET_NAME_QUERY_RESPONSE;
    use crate::VariableSized;
    use crate::world::v1::v12::opcodes::WorldServerOpcodeMessage;
    use crate::world::helper::{WorldMessageBody, WorldClientMessageWrite, WorldServerMessageWrite, WorldMessage};

    // Generated from `wow_message_parser/wowm/world/unsorted/add_messages.wowm` line 139.
    #[test]
    fn SMSG_PET_NAME_QUERY_RESPONSE0() {
        let raw: Vec<u8> = vec![ 0x00, 0x11, 0x53, 0x00, 0xEF, 0xBE, 0xAD, 0xDE,
             0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x00, 0xDE, 0xCA, 0xFA, 0x00, ];

        let expected = SMSG_PET_NAME_QUERY_RESPONSE {
            pet_number: 0xDEADBEEF,
            name: String::from("ABCDEF"),
            pet_name_timestamp: 0xFACADE,
        };

        let header_size = 2 + 2;
        let t = WorldServerOpcodeMessage::read_unencrypted(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            WorldServerOpcodeMessage::SMSG_PET_NAME_QUERY_RESPONSE(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_PET_NAME_QUERY_RESPONSE, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.pet_number, expected.pet_number);
        assert_eq!(t.name, expected.name);
        assert_eq!(t.pet_name_timestamp, expected.pet_name_timestamp);

        assert_eq!(t.size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write_unencrypted_server(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

}
