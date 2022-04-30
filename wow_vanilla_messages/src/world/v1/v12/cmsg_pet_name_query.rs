use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ClientMessageWrite, MessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};
#[cfg(any(feature = "async_tokio", feature = "async_std"))]
use async_trait::async_trait;
#[cfg(feature = "async_tokio")]
use tokio::io::{AsyncReadExt, AsyncWriteExt};
#[cfg(feature = "async_std")]
use async_std::io::{ReadExt, WriteExt};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_PET_NAME_QUERY {
    pub pet_number: u32,
    pub guid: Guid,
}

impl ClientMessageWrite for CMSG_PET_NAME_QUERY {}

impl MessageBody for CMSG_PET_NAME_QUERY {
    const OPCODE: u16 = 0x0052;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        Self::size() as u16
    }

    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // pet_number: u32
        let pet_number = crate::util::read_u32_le(r)?;

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            pet_number,
            guid,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // pet_number: u32
        w.write_all(&self.pet_number.to_le_bytes())?;

        // guid: Guid
        self.guid.write(w)?;

        Ok(())
    }
}

impl ConstantSized for CMSG_PET_NAME_QUERY {}

impl MaximumPossibleSized for CMSG_PET_NAME_QUERY {
    fn maximum_possible_size() -> usize {
        4 // pet_number: u32
        + 8 // guid: Guid
    }
}

#[cfg(test)]
mod test {
    use crate::ReadableAndWritable;
    use std::io::Cursor;
    use super::CMSG_PET_NAME_QUERY;
    use crate::ConstantSized;
    use super::*;
    use super::super::*;
    use crate::world::v1::v12::opcodes::ClientOpcodeMessage;
    use crate::{MessageBody, ClientMessageWrite, ServerMessageWrite, OpcodeMessage};

    #[test]
    fn CMSG_PET_NAME_QUERY0() {
        let raw: Vec<u8> = vec![ 0x00, 0x10, 0x52, 0x00, 0x00, 0x00, 0xEF, 0xBE,
             0xAD, 0xDE, 0xEF, 0xBE, 0xAD, 0xDE, 0xDE, 0xCA, 0xFA, 0x00, ];

        let expected = CMSG_PET_NAME_QUERY {
            pet_number: 0xDEADBEEF,
            guid: Guid::new(0xFACADEDEADBEEF),
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::read_unencrypted(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_PET_NAME_QUERY(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PET_NAME_QUERY, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.pet_number, expected.pet_number);
        assert_eq!(t.guid, expected.guid);

        assert_eq!(CMSG_PET_NAME_QUERY::size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write_unencrypted_client(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

}
