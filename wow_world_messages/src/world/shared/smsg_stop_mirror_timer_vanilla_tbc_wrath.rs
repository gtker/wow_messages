use std::io::{Read, Write};

use wow_world_base::shared::timer_type_vanilla_vanilla_tbc_wrath::TimerType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_stop_mirror_timer.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_stop_mirror_timer.wowm#L3):
/// ```text
/// smsg SMSG_STOP_MIRROR_TIMER = 0x01DB {
///     TimerType timer;
/// }
/// ```
pub struct SMSG_STOP_MIRROR_TIMER {
    pub timer: TimerType,
}

impl crate::private::Sealed for SMSG_STOP_MIRROR_TIMER {}
impl SMSG_STOP_MIRROR_TIMER {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01DB, size: body_size });
        }

        // timer: TimerType
        let timer = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            timer,
        })
    }

}

impl crate::Message for SMSG_STOP_MIRROR_TIMER {
    const OPCODE: u32 = 0x01db;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_STOP_MIRROR_TIMER {{").unwrap();
        // Members
        writeln!(s, "    timer = {};", self.timer.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 6_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 475_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "timer", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // timer: TimerType
        w.write_all(&(self.timer.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_STOP_MIRROR_TIMER {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_STOP_MIRROR_TIMER {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_STOP_MIRROR_TIMER {}

