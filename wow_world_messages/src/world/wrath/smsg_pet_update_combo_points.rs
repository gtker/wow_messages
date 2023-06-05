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

#[cfg(feature = "print-testcase")]
impl SMSG_PET_UPDATE_COMBO_POINTS {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_PET_UPDATE_COMBO_POINTS {{").unwrap();
        // Members
        writeln!(s, "    unit = {};", self.unit.guid()).unwrap();
        writeln!(s, "    target = {};", self.target.guid()).unwrap();
        writeln!(s, "    combo_points = {};", self.combo_points).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1170_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.unit), "unit", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.target), "target", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "combo_points", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_PET_UPDATE_COMBO_POINTS {}
impl crate::Message for SMSG_PET_UPDATE_COMBO_POINTS {
    const OPCODE: u32 = 0x0492;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_PET_UPDATE_COMBO_POINTS::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // unit: PackedGuid
        crate::util::write_packed_guid(&self.unit, &mut w)?;

        // target: PackedGuid
        crate::util::write_packed_guid(&self.target, &mut w)?;

        // combo_points: u8
        w.write_all(&self.combo_points.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(5..=19).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0492, size: body_size });
        }

        // unit: PackedGuid
        let unit = crate::util::read_packed_guid(&mut r)?;

        // target: PackedGuid
        let target = crate::util::read_packed_guid(&mut r)?;

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
        crate::util::packed_guid_size(&self.unit) // unit: PackedGuid
        + crate::util::packed_guid_size(&self.target) // target: PackedGuid
        + 1 // combo_points: u8
    }
}

