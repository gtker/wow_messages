use std::convert::{TryFrom, TryInto};
use crate::{ServerMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct SMSG_PET_NAME_QUERY_RESPONSE {
    pub pet_number: u32,
    pub name: String,
    pub pet_name_timestamp: u32,
}

impl ServerMessageWrite for SMSG_PET_NAME_QUERY_RESPONSE {}

#[cfg_attr(any(feature = "async_tokio", feature = "async_std"), async_trait)]
impl MessageBody for SMSG_PET_NAME_QUERY_RESPONSE {
    const OPCODE: u16 = 0x0053;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

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

    #[cfg(feature = "async_tokio")]
    async fn tokio_read_body<R: AsyncReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // pet_number: u32
        let pet_number = crate::util::tokio_read_u32_le(r).await?;

        // name: CString
        let name = crate::util::tokio_read_c_string_to_vec(r).await?;
        let name = String::from_utf8(name)?;

        // pet_name_timestamp: u32
        let pet_name_timestamp = crate::util::tokio_read_u32_le(r).await?;

        Ok(Self {
            pet_number,
            name,
            pet_name_timestamp,
        })
    }

    #[cfg(feature = "async_tokio")]
    async fn tokio_write_body<W: AsyncWriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // pet_number: u32
        w.write_all(&self.pet_number.to_le_bytes()).await?;

        // name: CString
        w.write_all(self.name.as_bytes()).await?;
        // Null terminator
        w.write_all(&[0]).await?;

        // pet_name_timestamp: u32
        w.write_all(&self.pet_name_timestamp.to_le_bytes()).await?;

        Ok(())
    }

    #[cfg(feature = "async_std")]
    async fn astd_read_body<R: ReadExt + Unpin + Send>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // pet_number: u32
        let pet_number = crate::util::astd_read_u32_le(r).await?;

        // name: CString
        let name = crate::util::astd_read_c_string_to_vec(r).await?;
        let name = String::from_utf8(name)?;

        // pet_name_timestamp: u32
        let pet_name_timestamp = crate::util::astd_read_u32_le(r).await?;

        Ok(Self {
            pet_number,
            name,
            pet_name_timestamp,
        })
    }

    #[cfg(feature = "async_std")]
    async fn astd_write_body<W: WriteExt + Unpin + Send>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // pet_number: u32
        w.write_all(&self.pet_number.to_le_bytes()).await?;

        // name: CString
        w.write_all(self.name.as_bytes()).await?;
        // Null terminator
        w.write_all(&[0]).await?;

        // pet_name_timestamp: u32
        w.write_all(&self.pet_name_timestamp.to_le_bytes()).await?;

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
    use super::*;
    use super::super::*;
    use crate::world::v1::v12::opcodes::ServerOpcodeMessage;
    use crate::{MessageBody, ClientMessageWrite, ServerMessageWrite, OpcodeMessage};

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
        let t = ServerOpcodeMessage::read_unencrypted(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_PET_NAME_QUERY_RESPONSE(t) => t,
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
