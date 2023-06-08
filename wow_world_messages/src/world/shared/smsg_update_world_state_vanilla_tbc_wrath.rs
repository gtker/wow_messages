use std::io::{Read, Write};

use crate::shared::world_state_vanilla_tbc_wrath::WorldState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/world/smsg_update_world_state.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/world/smsg_update_world_state.wowm#L3):
/// ```text
/// smsg SMSG_UPDATE_WORLD_STATE = 0x02C3 {
///     WorldState state;
/// }
/// ```
pub struct SMSG_UPDATE_WORLD_STATE {
    pub state: WorldState,
}

impl crate::private::Sealed for SMSG_UPDATE_WORLD_STATE {}
impl crate::Message for SMSG_UPDATE_WORLD_STATE {
    const OPCODE: u32 = 0x02c3;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_UPDATE_WORLD_STATE {{").unwrap();
        // Members
        // state: WorldState
        writeln!(s, "    state = {{").unwrap();
        // Members
        writeln!(s, "        state = {};", self.state.state).unwrap();
        writeln!(s, "        value = {};", self.state.value).unwrap();

        writeln!(s, "    }};").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 10_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 707_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        writeln!(s, "    /* state: WorldState start */").unwrap();
        crate::util::write_bytes(&mut s, &mut bytes, 4, "state", "        ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "value", "        ");
        writeln!(s, "    /* state: WorldState end */").unwrap();


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12 2 3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // state: WorldState
        self.state.write_into_vec(&mut w)?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02C3, size: body_size });
        }

        // state: WorldState
        let state = WorldState::read(&mut r)?;

        Ok(Self {
            state,
        })
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_UPDATE_WORLD_STATE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_UPDATE_WORLD_STATE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_UPDATE_WORLD_STATE {}

