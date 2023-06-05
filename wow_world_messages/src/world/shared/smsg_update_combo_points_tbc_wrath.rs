use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_update_combo_points.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_update_combo_points.wowm#L1):
/// ```text
/// smsg SMSG_UPDATE_COMBO_POINTS = 0x039D {
///     PackedGuid target;
///     u8 combo_points;
/// }
/// ```
pub struct SMSG_UPDATE_COMBO_POINTS {
    pub target: Guid,
    pub combo_points: u8,
}

#[cfg(feature = "print-testcase")]
impl SMSG_UPDATE_COMBO_POINTS {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_UPDATE_COMBO_POINTS {{").unwrap();
        // Members
        writeln!(s, "    target = {};", self.target.guid()).unwrap();
        writeln!(s, "    combo_points = {};", self.combo_points).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 925_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"2.4.3 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_UPDATE_COMBO_POINTS {}
impl crate::Message for SMSG_UPDATE_COMBO_POINTS {
    const OPCODE: u32 = 0x039d;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // target: PackedGuid
        crate::util::write_packed_guid(&self.target, &mut w)?;

        // combo_points: u8
        w.write_all(&self.combo_points.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(3..=10).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x039D, size: body_size });
        }

        // target: PackedGuid
        let target = crate::util::read_packed_guid(&mut r)?;

        // combo_points: u8
        let combo_points = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            target,
            combo_points,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_UPDATE_COMBO_POINTS {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_UPDATE_COMBO_POINTS {}

impl SMSG_UPDATE_COMBO_POINTS {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.target) // target: PackedGuid
        + 1 // combo_points: u8
    }
}

