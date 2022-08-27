use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_spline_move_set_run_mode.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_spline_move_set_run_mode.wowm#L3):
/// ```text
/// smsg SMSG_SPLINE_MOVE_SET_RUN_MODE = 0x030D {
///     PackedGuid guid;
/// }
/// ```
pub struct SMSG_SPLINE_MOVE_SET_RUN_MODE {
    pub guid: Guid,
}

impl ServerMessage for SMSG_SPLINE_MOVE_SET_RUN_MODE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(w);

        Ok(())
    }
    const OPCODE: u16 = 0x030d;

    fn server_size(&self) -> u16 {
        (self.size() + 4) as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        Ok(Self {
            guid,
        })
    }

}

impl SMSG_SPLINE_MOVE_SET_RUN_MODE {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: Guid
    }
}

