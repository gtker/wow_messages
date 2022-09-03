use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_move_knock_back.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_move_knock_back.wowm#L3):
/// ```text
/// smsg SMSG_MOVE_KNOCK_BACK = 0x00EF {
///     PackedGuid guid;
///     u32 movement_counter;
///     f32 v_cos;
///     f32 v_sin;
///     f32 horizontal_speed;
///     f32 vertical_speed;
/// }
/// ```
pub struct SMSG_MOVE_KNOCK_BACK {
    pub guid: Guid,
    /// mangoszero: Sequence
    /// mangoszero sets to 0
    ///
    pub movement_counter: u32,
    /// cmangos/mangoszero/vmangos: x direction
    ///
    pub v_cos: f32,
    /// cmangos/mangoszero/vmangos: y direction
    ///
    pub v_sin: f32,
    /// cmangos/mangoszero/vmangos: Horizontal speed
    ///
    pub horizontal_speed: f32,
    /// cmangos/mangoszero/vmangos: Z Movement speed (vertical)
    ///
    pub vertical_speed: f32,
}

impl crate::Message for SMSG_MOVE_KNOCK_BACK {
    const OPCODE: u32 = 0x00ef;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(w);

        // movement_counter: u32
        w.write_all(&self.movement_counter.to_le_bytes())?;

        // v_cos: f32
        w.write_all(&self.v_cos.to_le_bytes())?;

        // v_sin: f32
        w.write_all(&self.v_sin.to_le_bytes())?;

        // horizontal_speed: f32
        w.write_all(&self.horizontal_speed.to_le_bytes())?;

        // vertical_speed: f32
        w.write_all(&self.vertical_speed.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        // movement_counter: u32
        let movement_counter = crate::util::read_u32_le(r)?;

        // v_cos: f32
        let v_cos = crate::util::read_f32_le(r)?;
        // v_sin: f32
        let v_sin = crate::util::read_f32_le(r)?;
        // horizontal_speed: f32
        let horizontal_speed = crate::util::read_f32_le(r)?;
        // vertical_speed: f32
        let vertical_speed = crate::util::read_f32_le(r)?;
        Ok(Self {
            guid,
            movement_counter,
            v_cos,
            v_sin,
            horizontal_speed,
            vertical_speed,
        })
    }

}
impl ServerMessage for SMSG_MOVE_KNOCK_BACK {}

impl SMSG_MOVE_KNOCK_BACK {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: Guid
        + 4 // movement_counter: u32
        + 4 // v_cos: f32
        + 4 // v_sin: f32
        + 4 // horizontal_speed: f32
        + 4 // vertical_speed: f32
    }
}

