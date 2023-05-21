use std::io::{Read, Write};

use wow_world_base::shared::unit_stand_state_vanilla_tbc_wrath::UnitStandState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_standstate_update.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_standstate_update.wowm#L3):
/// ```text
/// smsg SMSG_STANDSTATE_UPDATE = 0x029D {
///     UnitStandState state;
/// }
/// ```
pub struct SMSG_STANDSTATE_UPDATE {
    pub state: UnitStandState,
}

impl crate::private::Sealed for SMSG_STANDSTATE_UPDATE {}
impl SMSG_STANDSTATE_UPDATE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 1 {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x029D, size: body_size });
        }

        // state: UnitStandState
        let state = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            state,
        })
    }

}

impl crate::Message for SMSG_STANDSTATE_UPDATE {
    const OPCODE: u32 = 0x029d;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_STANDSTATE_UPDATE {{").unwrap();
        // Members
        writeln!(s, "    state = {};", self.state.as_test_case_value()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 3_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 669_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "state", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12 2 3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // state: UnitStandState
        w.write_all(&(self.state.as_int().to_le_bytes()))?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_STANDSTATE_UPDATE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_STANDSTATE_UPDATE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_STANDSTATE_UPDATE {}

