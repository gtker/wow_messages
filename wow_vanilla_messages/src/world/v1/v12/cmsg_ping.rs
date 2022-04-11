use std::convert::{TryFrom, TryInto};
use crate::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
pub struct CMSG_PING {
    pub sequence_id: u32,
    pub round_time_in_ms: u32,
}

impl WorldClientMessageWrite for CMSG_PING {
    const OPCODE: u32 = 0x1dc;

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
impl WorldMessageBody for CMSG_PING {
    type Error = std::io::Error;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // sequence_id: u32
        let sequence_id = crate::util::read_u32_le(r)?;

        // round_time_in_ms: u32
        let round_time_in_ms = crate::util::read_u32_le(r)?;

        Ok(Self {
            sequence_id,
            round_time_in_ms,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // sequence_id: u32
        w.write_all(&self.sequence_id.to_le_bytes())?;

        // round_time_in_ms: u32
        w.write_all(&self.round_time_in_ms.to_le_bytes())?;

        Ok(())
    }
}

impl ConstantSized for CMSG_PING {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_PING {
    fn maximum_possible_size() -> usize {
        4 // sequence_id: u32
        + 4 // round_time_in_ms: u32
    }
}

#[cfg(test)]
mod test {
    use crate::ReadableAndWritable;
    use std::io::Cursor;
    use super::CMSG_PING;
    use crate::ConstantSized;
    use super::*;
    use super::super::*;
    use crate::world::v1::v12::opcodes::WorldClientOpcodeMessage;
    use crate::{WorldMessageBody, WorldClientMessageWrite, WorldServerMessageWrite, WorldMessage};

    #[test]
    fn CMSG_PING0() {
        let raw: Vec<u8> = vec![ 0x00, 0x0C, 0xDC, 0x01, 0x00, 0x00, 0xEF, 0xBE,
             0xAD, 0xDE, 0xDE, 0xCA, 0xFA, 0x00, ];

        let expected = CMSG_PING {
            sequence_id: 0xDEADBEEF,
            round_time_in_ms: 0xFACADE,
        };

        let header_size = 2 + 4;
        let t = WorldClientOpcodeMessage::read_unencrypted(&mut Cursor::new(&raw)).unwrap();
        let t = match t {
            WorldClientOpcodeMessage::CMSG_PING(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_PING, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.sequence_id, expected.sequence_id);
        assert_eq!(t.round_time_in_ms, expected.round_time_in_ms);

        assert_eq!(CMSG_PING::size() + header_size, raw.len());

        let mut dest = Vec::with_capacity(raw.len());
        expected.write_unencrypted_client(&mut Cursor::new(&mut dest));

        assert_eq!(dest, raw);
    }

}
