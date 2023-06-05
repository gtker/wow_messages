use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_move_gravity_enable.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_move_gravity_enable.wowm#L1):
/// ```text
/// smsg SMSG_MOVE_GRAVITY_ENABLE = 0x04D0 {
///     PackedGuid unit;
///     u32 movement_counter;
/// }
/// ```
pub struct SMSG_MOVE_GRAVITY_ENABLE {
    pub unit: Guid,
    pub movement_counter: u32,
}

#[cfg(feature = "print-testcase")]
impl SMSG_MOVE_GRAVITY_ENABLE {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_MOVE_GRAVITY_ENABLE {{").unwrap();
        // Members
        writeln!(s, "    unit = {};", self.unit.guid()).unwrap();
        writeln!(s, "    movement_counter = {};", self.movement_counter).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 1232_u32.to_le_bytes();
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
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_MOVE_GRAVITY_ENABLE {}
impl crate::Message for SMSG_MOVE_GRAVITY_ENABLE {
    const OPCODE: u32 = 0x04d0;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // unit: PackedGuid
        crate::util::write_packed_guid(&self.unit, &mut w)?;

        // movement_counter: u32
        w.write_all(&self.movement_counter.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(6..=13).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04D0, size: body_size });
        }

        // unit: PackedGuid
        let unit = crate::util::read_packed_guid(&mut r)?;

        // movement_counter: u32
        let movement_counter = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            unit,
            movement_counter,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_MOVE_GRAVITY_ENABLE {}

impl SMSG_MOVE_GRAVITY_ENABLE {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.unit) // unit: PackedGuid
        + 4 // movement_counter: u32
    }
}

