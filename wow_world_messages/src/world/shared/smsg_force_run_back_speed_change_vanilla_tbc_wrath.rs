use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_force_run_back_speed_change.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_force_run_back_speed_change.wowm#L3):
/// ```text
/// smsg SMSG_FORCE_RUN_BACK_SPEED_CHANGE = 0x00E4 {
///     PackedGuid guid;
///     u32 move_event;
///     f32 speed;
/// }
/// ```
pub struct SMSG_FORCE_RUN_BACK_SPEED_CHANGE {
    pub guid: Guid,
    /// cmangos/mangoszero/vmangos: set to 0
    /// cmangos/mangoszero/vmangos: moveEvent, NUM_PMOVE_EVTS = 0x39
    ///
    pub move_event: u32,
    pub speed: f32,
}

#[cfg(feature = "print-testcase")]
impl SMSG_FORCE_RUN_BACK_SPEED_CHANGE {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_FORCE_RUN_BACK_SPEED_CHANGE {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();
        writeln!(s, "    move_event = {};", self.move_event).unwrap();
        writeln!(s, "    {}", if self.speed.to_string().contains(".") { self.speed.to_string() } else { format!("{}.0", self.speed) }).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 228_u32.to_le_bytes();
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
        writeln!(s, "    versions = \"1.12 2 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_FORCE_RUN_BACK_SPEED_CHANGE {}
impl crate::Message for SMSG_FORCE_RUN_BACK_SPEED_CHANGE {
    const OPCODE: u32 = 0x00e4;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: PackedGuid
        crate::util::write_packed_guid(&self.guid, &mut w)?;

        // move_event: u32
        w.write_all(&self.move_event.to_le_bytes())?;

        // speed: f32
        w.write_all(&self.speed.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(10..=17).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x00E4, size: body_size });
        }

        // guid: PackedGuid
        let guid = crate::util::read_packed_guid(&mut r)?;

        // move_event: u32
        let move_event = crate::util::read_u32_le(&mut r)?;

        // speed: f32
        let speed = crate::util::read_f32_le(&mut r)?;

        Ok(Self {
            guid,
            move_event,
            speed,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_FORCE_RUN_BACK_SPEED_CHANGE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_FORCE_RUN_BACK_SPEED_CHANGE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_FORCE_RUN_BACK_SPEED_CHANGE {}

impl SMSG_FORCE_RUN_BACK_SPEED_CHANGE {
    pub(crate) const fn size(&self) -> usize {
        crate::util::packed_guid_size(&self.guid) // guid: PackedGuid
        + 4 // move_event: u32
        + 4 // speed: f32
    }
}

