use std::io::{Read, Write};

use crate::Guid;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_move_set_collision_hgt.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_move_set_collision_hgt.wowm#L1):
/// ```text
/// smsg SMSG_MOVE_SET_COLLISION_HGT = 0x0516 {
///     PackedGuid unit;
///     u32 packet_counter;
///     f32 collision_height;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct SMSG_MOVE_SET_COLLISION_HGT {
    pub unit: Guid,
    pub packet_counter: u32,
    pub collision_height: f32,
}

impl crate::private::Sealed for SMSG_MOVE_SET_COLLISION_HGT {}
impl SMSG_MOVE_SET_COLLISION_HGT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(9..=17).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // unit: PackedGuid
        let unit = crate::util::read_packed_guid(&mut r)?;

        // packet_counter: u32
        let packet_counter = crate::util::read_u32_le(&mut r)?;

        // collision_height: f32
        let collision_height = crate::util::read_f32_le(&mut r)?;

        Ok(Self {
            unit,
            packet_counter,
            collision_height,
        })
    }

}

impl crate::Message for SMSG_MOVE_SET_COLLISION_HGT {
    const OPCODE: u32 = 0x0516;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_MOVE_SET_COLLISION_HGT"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_MOVE_SET_COLLISION_HGT {{").unwrap();
        // Members
        writeln!(s, "    unit = {};", self.unit.guid()).unwrap();
        writeln!(s, "    packet_counter = {};", self.packet_counter).unwrap();
        writeln!(s, "    collision_height = {};", if self.collision_height.to_string().contains('.') { self.collision_height.to_string() } else { format!("{}.0", self.collision_height) }).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1302_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.unit), "unit", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "packet_counter", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "collision_height", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // unit: PackedGuid
        crate::util::write_packed_guid(&self.unit, &mut w)?;

        // packet_counter: u32
        w.write_all(&self.packet_counter.to_le_bytes())?;

        // collision_height: f32
        w.write_all(&self.collision_height.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1302, "SMSG_MOVE_SET_COLLISION_HGT", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_MOVE_SET_COLLISION_HGT {}

impl SMSG_MOVE_SET_COLLISION_HGT {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.unit) // unit: PackedGuid
        + 4 // packet_counter: u32
        + 4 // collision_height: f32
    }
}

