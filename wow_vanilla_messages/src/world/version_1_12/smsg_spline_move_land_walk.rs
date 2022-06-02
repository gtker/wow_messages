use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_spline_move_land_walk.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_spline_move_land_walk.wowm#L3):
/// ```text
/// smsg SMSG_SPLINE_MOVE_LAND_WALK = 0x030A {
///     PackedGuid guid;
/// }
/// ```
pub struct SMSG_SPLINE_MOVE_LAND_WALK {
    pub guid: Guid,
}

impl ServerMessage for SMSG_SPLINE_MOVE_LAND_WALK {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        w.write_all(&self.guid.packed_guid())?;

        Ok(())
    }
    const OPCODE: u16 = 0x030a;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        Ok(Self {
            guid,
        })
    }

}

impl SMSG_SPLINE_MOVE_LAND_WALK {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: Guid
    }
}

