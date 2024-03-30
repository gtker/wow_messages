use std::io::{Read, Write};

use wow_world_base::shared::timer_type_vanilla_tbc_wrath::TimerType;

/// According to cmangos: 'Default UI handler for this is bugged, args dont match. Gotta do a full update with `SMSG_START_MIRROR_TIMER` to avoid lua errors.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_pause_mirror_timer.wowm:4`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_pause_mirror_timer.wowm#L4):
/// ```text
/// smsg SMSG_PAUSE_MIRROR_TIMER = 0x01DA {
///     TimerType timer;
///     Bool is_frozen;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PAUSE_MIRROR_TIMER {
    pub timer: TimerType,
    pub is_frozen: bool,
}

impl crate::private::Sealed for SMSG_PAUSE_MIRROR_TIMER {}
impl SMSG_PAUSE_MIRROR_TIMER {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 5 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // timer: TimerType
        let timer = crate::util::read_u32_le(&mut r)?.try_into()?;

        // is_frozen: Bool
        let is_frozen = crate::util::read_bool_u8(&mut r)?;

        Ok(Self {
            timer,
            is_frozen,
        })
    }

}

impl crate::Message for SMSG_PAUSE_MIRROR_TIMER {
    const OPCODE: u32 = 0x01da;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_PAUSE_MIRROR_TIMER"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_PAUSE_MIRROR_TIMER {{").unwrap();
        // Members
        writeln!(s, "    timer = {};", self.timer.as_test_case_value()).unwrap();
        writeln!(s, "    is_frozen = {};", if self.is_frozen { "TRUE" } else { "FALSE" }).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 7_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 474_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "timer", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "is_frozen", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        5
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // timer: TimerType
        w.write_all(&(self.timer.as_int().to_le_bytes()))?;

        // is_frozen: Bool
        w.write_all(u8::from(self.is_frozen).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(474, "SMSG_PAUSE_MIRROR_TIMER", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_PAUSE_MIRROR_TIMER {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_PAUSE_MIRROR_TIMER {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_PAUSE_MIRROR_TIMER {}

