use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::CinematicSequenceId;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/cinematic/smsg_trigger_cinematic.wowm:15`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/cinematic/smsg_trigger_cinematic.wowm#L15):
/// ```text
/// smsg SMSG_TRIGGER_CINEMATIC = 0x00FA {
///     CinematicSequenceId cinematic_sequence_id;
/// }
/// ```
pub struct SMSG_TRIGGER_CINEMATIC {
    pub cinematic_sequence_id: CinematicSequenceId,
}

impl ServerMessage for SMSG_TRIGGER_CINEMATIC {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // cinematic_sequence_id: CinematicSequenceId
        w.write_all(&(self.cinematic_sequence_id.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x00fa;

    fn server_size(&self) -> u16 {
        8
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // cinematic_sequence_id: CinematicSequenceId
        let cinematic_sequence_id: CinematicSequenceId = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            cinematic_sequence_id,
        })
    }

}

