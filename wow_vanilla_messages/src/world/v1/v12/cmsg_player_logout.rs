use std::convert::{TryFrom, TryInto};
use crate::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_PLAYER_LOGOUT {
}

impl WorldClientMessageWrite for CMSG_PLAYER_LOGOUT {
    const OPCODE: u32 = 0x4a;

    fn write_unencrypted_client<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        crate::util::write_u16_be(w, (Self::size() + 4) as u16)?;
        crate::util::write_u32_le(w, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_client<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        e.write_encrypted_client_header(w, (Self::size() + 4) as u16, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for CMSG_PLAYER_LOGOUT {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        Ok(Self {
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        Ok(())
    }
}

impl ConstantSized for CMSG_PLAYER_LOGOUT {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_PLAYER_LOGOUT {
    fn maximum_possible_size() -> usize {
        0
    }
}

#[cfg(test)]
mod test {
    use crate::ReadableAndWritable;
    use std::io::Cursor;
    use super::CMSG_PLAYER_LOGOUT;
    use crate::ConstantSized;
    use super::*;
    use super::super::*;
    use crate::world::v1::v12::opcodes::WorldClientOpcodeMessage;
    use crate::{WorldMessageBody, WorldClientMessageWrite, WorldServerMessageWrite, WorldMessage};

    #[test]
    fn CMSG_PLAYER_LOGOUT0() {
        let raw: Vec<u8> = vec![ 0x00, 0x04, 0x4A, 0x00, 0x00, 0x00, ];

        let expected = CMSG_PLAYER_LOGOUT {
        };

        let header_size = 2 + 4;
        let t = WorldClientOpcodeMessage::read_unencrypted(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            WorldClientOpcodeMessage::CMSG_PLAYER_LOGOUT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PLAYER_LOGOUT, got {opcode:#?}", opcode = opcode),
        };


        assert_eq!(CMSG_PLAYER_LOGOUT::size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write_unencrypted_client(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

}
