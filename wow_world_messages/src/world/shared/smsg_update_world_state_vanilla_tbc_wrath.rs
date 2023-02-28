use crate::shared::world_state_vanilla_tbc_wrath::WorldState;
use std::io::{Write, Read};

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

impl crate::Message for SMSG_UPDATE_WORLD_STATE {
    const OPCODE: u32 = 0x02c3;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // state: WorldState
        self.state.write_into_vec(&mut w)?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02C3, size: body_size as u32 });
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

