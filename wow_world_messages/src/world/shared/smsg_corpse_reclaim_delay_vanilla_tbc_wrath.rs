use std::io::{Read, Write};

use std::time::Duration;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/resurrect/smsg_corpse_reclaim_delay.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/resurrect/smsg_corpse_reclaim_delay.wowm#L3):
/// ```text
/// smsg SMSG_CORPSE_RECLAIM_DELAY = 0x0269 {
///     Seconds delay;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_CORPSE_RECLAIM_DELAY {
    pub delay: Duration,
}

impl crate::private::Sealed for SMSG_CORPSE_RECLAIM_DELAY {}
impl SMSG_CORPSE_RECLAIM_DELAY {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 4 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // delay: Seconds
        let delay = Duration::from_secs(crate::util::read_u32_le(&mut r)?.into());

        Ok(Self {
            delay,
        })
    }

}

impl crate::Message for SMSG_CORPSE_RECLAIM_DELAY {
    const OPCODE: u32 = 0x0269;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_CORPSE_RECLAIM_DELAY"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_CORPSE_RECLAIM_DELAY {{").unwrap();
        // Members
        writeln!(s, "    delay = {};", self.delay.as_secs()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 6_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 617_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "delay", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // delay: Seconds
        w.write_all((self.delay.as_secs() as u32).to_le_bytes().as_slice())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(617, "SMSG_CORPSE_RECLAIM_DELAY", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_CORPSE_RECLAIM_DELAY {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_CORPSE_RECLAIM_DELAY {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_CORPSE_RECLAIM_DELAY {}

