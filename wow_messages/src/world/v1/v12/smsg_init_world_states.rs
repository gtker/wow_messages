use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Area, AreaError};
use crate::world::v1::v12::{Map, MapError};
use crate::world::v1::v12::WorldState;
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new5.wowm:999`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new5.wowm#L999):
/// ```text
/// smsg SMSG_INIT_WORLD_STATES = 0x2C2 {
///     Map map;
///     Area area;
///     u16 amount_of_states;
///     WorldState[amount_of_states] states;
/// }
/// ```
pub struct SMSG_INIT_WORLD_STATES {
    pub map: Map,
    pub area: Area,
    pub amount_of_states: u16,
    pub states: Vec<WorldState>,
}

impl WorldServerMessageWrite for SMSG_INIT_WORLD_STATES {
    const OPCODE: u16 = 0x2c2;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (self.size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (self.size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_INIT_WORLD_STATES {
    type Error = SMSG_INIT_WORLD_STATESError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // map: Map
        let map = Map::read(r)?;

        // area: Area
        let area = Area::read(r)?;

        // amount_of_states: u16
        let amount_of_states = crate::util::read_u16_le(r)?;

        // states: WorldState[amount_of_states]
        let mut states = Vec::with_capacity(amount_of_states as usize);
        for i in 0..amount_of_states {
            states.push(WorldState::read(r)?);
        }

        Ok(Self {
            map,
            area,
            amount_of_states,
            states,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // map: Map
        self.map.write(w)?;

        // area: Area
        self.area.write(w)?;

        // amount_of_states: u16
        w.write_all(&(self.states.len() as u16).to_le_bytes())?;

        // states: WorldState[amount_of_states]
        for i in self.states.iter() {
            i.write(w)?;
        }

        Ok(())
    }
}

impl VariableSized for SMSG_INIT_WORLD_STATES {
    fn size(&self) -> usize {
        Map::size() // map: Map
        + Area::size() // area: Area
        + 2 // amount_of_states: u16
        + self.states.iter().fold(0, |acc, x| acc + WorldState::size()) // states: WorldState[amount_of_states]
    }
}

impl MaximumPossibleSized for SMSG_INIT_WORLD_STATES {
    fn maximum_possible_size() -> usize {
        Map::maximum_possible_size() // map: Map
        + Area::maximum_possible_size() // area: Area
        + 2 // amount_of_states: u16
        + 65535 * WorldState::maximum_possible_size() // states: WorldState[amount_of_states]
    }
}

#[derive(Debug)]
pub enum SMSG_INIT_WORLD_STATESError {
    Io(std::io::Error),
    Area(AreaError),
    Map(MapError),
}

impl std::error::Error for SMSG_INIT_WORLD_STATESError {}
impl std::fmt::Display for SMSG_INIT_WORLD_STATESError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Area(i) => i.fmt(f),
            Self::Map(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_INIT_WORLD_STATESError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<AreaError> for SMSG_INIT_WORLD_STATESError {
    fn from(e: AreaError) -> Self {
        Self::Area(e)
    }
}

impl From<MapError> for SMSG_INIT_WORLD_STATESError {
    fn from(e: MapError) -> Self {
        Self::Map(e)
    }
}

