use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/smsg_pet_update_combo_points.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/smsg_pet_update_combo_points.wowm#L1):
/// ```text
/// smsg SMSG_PET_UPDATE_COMBO_POINTS = 0x0492 {
///     PackedGuid unit;
///     PackedGuid target;
///     u8 combo_points;
/// }
/// ```
pub struct SMSG_PET_UPDATE_COMBO_POINTS {
    pub unit: Guid,
    pub target: Guid,
    pub combo_points: u8,
}

impl crate::private::Sealed for SMSG_PET_UPDATE_COMBO_POINTS {}
impl crate::Message for SMSG_PET_UPDATE_COMBO_POINTS {
    const OPCODE: u32 = 0x0492;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // unit: PackedGuid
        self.unit.write_packed_guid_into_vec(&mut w)?;

        // target: PackedGuid
        self.target.write_packed_guid_into_vec(&mut w)?;

        // combo_points: u8
        w.write_all(&self.combo_points.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(5..=19).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0492, size: body_size });
        }

        // unit: PackedGuid
        let unit = Guid::read_packed(&mut r)?;

        // target: PackedGuid
        let target = Guid::read_packed(&mut r)?;

        // combo_points: u8
        let combo_points = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            unit,
            target,
            combo_points,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_PET_UPDATE_COMBO_POINTS {}

impl SMSG_PET_UPDATE_COMBO_POINTS {
    pub(crate) const fn size(&self) -> usize {
        self.unit.size() // unit: PackedGuid
        + self.target.size() // target: PackedGuid
        + 1 // combo_points: u8
    }
}

