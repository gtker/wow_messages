use std::io::{Read, Write};

use crate::vanilla::{
    Area, Map, WorldState,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/world/smsg_init_world_states.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/world/smsg_init_world_states.wowm#L1):
/// ```text
/// smsg SMSG_INIT_WORLD_STATES = 0x02C2 {
///     Map map;
///     Area area;
///     u16 amount_of_states;
///     WorldState[amount_of_states] states;
/// }
/// ```
pub struct SMSG_INIT_WORLD_STATES {
    pub map: Map,
    pub area: Area,
    pub states: Vec<WorldState>,
}

impl crate::private::Sealed for SMSG_INIT_WORLD_STATES {}
impl crate::Message for SMSG_INIT_WORLD_STATES {
    const OPCODE: u32 = 0x02c2;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // map: Map
        w.write_all(&(self.map.as_int().to_le_bytes()))?;

        // area: Area
        w.write_all(&(self.area.as_int().to_le_bytes()))?;

        // amount_of_states: u16
        w.write_all(&(self.states.len() as u16).to_le_bytes())?;

        // states: WorldState[amount_of_states]
        for i in self.states.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(10..=524298).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02C2, size: body_size });
        }

        // map: Map
        let map = crate::util::read_u32_le(&mut r)?.try_into()?;

        // area: Area
        let area = crate::util::read_u32_le(&mut r)?.try_into()?;

        // amount_of_states: u16
        let amount_of_states = crate::util::read_u16_le(&mut r)?;

        // states: WorldState[amount_of_states]
        let states = {
            let mut states = Vec::with_capacity(amount_of_states as usize);
            for _ in 0..amount_of_states {
                states.push(WorldState::read(&mut r)?);
            }
            states
        };

        Ok(Self {
            map,
            area,
            states,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_INIT_WORLD_STATES {}

impl SMSG_INIT_WORLD_STATES {
    pub(crate) fn size(&self) -> usize {
        4 // map: Map
        + 4 // area: Area
        + 2 // amount_of_states: u16
        + self.states.len() * 8 // states: WorldState[amount_of_states]
    }
}

