use std::io::{Read, Write};

use wow_world_base::shared::timer_type_vanilla_tbc_wrath::TimerType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_start_mirror_timer.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_start_mirror_timer.wowm#L3):
/// ```text
/// smsg SMSG_START_MIRROR_TIMER = 0x01D9 {
///     TimerType timer;
///     u32 time_remaining;
///     u32 duration;
///     u32 scale;
///     Bool is_frozen;
///     Spell id;
/// }
/// ```
pub struct SMSG_START_MIRROR_TIMER {
    pub timer: TimerType,
    pub time_remaining: u32,
    pub duration: u32,
    pub scale: u32,
    pub is_frozen: bool,
    pub id: u32,
}

impl crate::private::Sealed for SMSG_START_MIRROR_TIMER {}
impl SMSG_START_MIRROR_TIMER {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 21 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // timer: TimerType
        let timer = crate::util::read_u32_le(&mut r)?.try_into()?;

        // time_remaining: u32
        let time_remaining = crate::util::read_u32_le(&mut r)?;

        // duration: u32
        let duration = crate::util::read_u32_le(&mut r)?;

        // scale: u32
        let scale = crate::util::read_u32_le(&mut r)?;

        // is_frozen: Bool
        let is_frozen = crate::util::read_u8_le(&mut r)? != 0;

        // id: Spell
        let id = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            timer,
            time_remaining,
            duration,
            scale,
            is_frozen,
            id,
        })
    }

}

impl crate::Message for SMSG_START_MIRROR_TIMER {
    const OPCODE: u32 = 0x01d9;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_START_MIRROR_TIMER"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_START_MIRROR_TIMER {{").unwrap();
        // Members
        writeln!(s, "    timer = {};", self.timer.as_test_case_value()).unwrap();
        writeln!(s, "    time_remaining = {};", self.time_remaining).unwrap();
        writeln!(s, "    duration = {};", self.duration).unwrap();
        writeln!(s, "    scale = {};", self.scale).unwrap();
        writeln!(s, "    is_frozen = {};", if self.is_frozen { "TRUE" } else { "FALSE" }).unwrap();
        writeln!(s, "    id = {};", self.id).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 23_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 473_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "timer", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_remaining", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "duration", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "scale", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "is_frozen", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "id", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        21
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // timer: TimerType
        w.write_all(&(self.timer.as_int().to_le_bytes()))?;

        // time_remaining: u32
        w.write_all(&self.time_remaining.to_le_bytes())?;

        // duration: u32
        w.write_all(&self.duration.to_le_bytes())?;

        // scale: u32
        w.write_all(&self.scale.to_le_bytes())?;

        // is_frozen: Bool
        w.write_all(u8::from(self.is_frozen).to_le_bytes().as_slice())?;

        // id: Spell
        w.write_all(&self.id.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(473, "SMSG_START_MIRROR_TIMER", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_START_MIRROR_TIMER {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_START_MIRROR_TIMER {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_START_MIRROR_TIMER {}

