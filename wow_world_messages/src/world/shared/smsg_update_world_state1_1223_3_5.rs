use std::convert::{TryFrom, TryInto};
use crate::world::shared::world_state1_1223::WorldState;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
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

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // state: WorldState
        self.state.write_into_vec(w)?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // state: WorldState
        let state = WorldState::read(r)?;

        Ok(Self {
            state,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_UPDATE_WORLD_STATE {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_UPDATE_WORLD_STATE {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_UPDATE_WORLD_STATE {}

