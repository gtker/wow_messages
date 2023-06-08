use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
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

#[cfg(feature = "print-testcase")]
impl SMSG_MOVE_KNOCK_BACK {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_MOVE_KNOCK_BACK {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    movement_counter = {};", self.movement_counter).unwrap();
        writeln!(s, "    {}", if self.v_cos.to_string().contains(".") { self.v_cos.to_string() } else { format!("{}.0", self.v_cos) }).unwrap();
        writeln!(s, "    {}", if self.v_sin.to_string().contains(".") { self.v_sin.to_string() } else { format!("{}.0", self.v_sin) }).unwrap();
        writeln!(s, "    {}", if self.horizontal_speed.to_string().contains(".") { self.horizontal_speed.to_string() } else { format!("{}.0", self.horizontal_speed) }).unwrap();
        writeln!(s, "    {}", if self.vertical_speed.to_string().contains(".") { self.vertical_speed.to_string() } else { format!("{}.0", self.vertical_speed) }).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 239_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, crate::util::packed_guid_size(&self.guid), "guid", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "movement_counter", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "v_cos", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "v_sin", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "horizontal_speed", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "vertical_speed", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_MOVE_KNOCK_BACK {}
impl crate::Message for SMSG_MOVE_KNOCK_BACK {
    const OPCODE: u32 = 0x00ef;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_MOVE_KNOCK_BACK::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        crate::util::write_packed_guid(&self.guid, &mut w)?;

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

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(22..=29).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00EF, size: body_size });
        }

        // guid: PackedGuid
        let guid = crate::util::read_packed_guid(&mut r)?;

        // movement_counter: u32
        let movement_counter = crate::util::read_u32_le(&mut r)?;

        // v_cos: f32
        let v_cos = crate::util::read_f32_le(&mut r)?;

        // v_sin: f32
        let v_sin = crate::util::read_f32_le(&mut r)?;

        // horizontal_speed: f32
        let horizontal_speed = crate::util::read_f32_le(&mut r)?;

        // vertical_speed: f32
        let vertical_speed = crate::util::read_f32_le(&mut r)?;

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

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_MOVE_KNOCK_BACK {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_MOVE_KNOCK_BACK {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_MOVE_KNOCK_BACK {}

impl SMSG_MOVE_KNOCK_BACK {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.guid) // guid: PackedGuid
        + 4 // movement_counter: u32
        + 4 // v_cos: f32
        + 4 // v_sin: f32
        + 4 // horizontal_speed: f32
        + 4 // vertical_speed: f32
    }
}

