use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{BattlefieldPortAction, BattlefieldPortActionError};
use crate::world::v1::v12::{Map, MapError};
use crate::world::helper::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/cmsg_battlefield_port.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/cmsg_battlefield_port.wowm#L3):
/// ```text
/// cmsg CMSG_BATTLEFIELD_PORT = 0x2D5 {
///     Map map;
///     BattlefieldPortAction action;
/// }
/// ```
pub struct CMSG_BATTLEFIELD_PORT {
    pub map: Map,
    pub action: BattlefieldPortAction,
}

impl WorldClientMessageWrite for CMSG_BATTLEFIELD_PORT {
    const OPCODE: u32 = 0x2d5;

    fn write_unencrypted_client<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        crate::util::write_u16_be(w, (Self::size() + 4) as u16)?;
        crate::util::write_u32_le(w, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_client<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        e.write_encrypted_client_header(w, (Self::size() + 4) as u16, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for CMSG_BATTLEFIELD_PORT {
    type Error = CMSG_BATTLEFIELD_PORTError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // map: Map
        let map = Map::read(r)?;

        // action: BattlefieldPortAction
        let action = BattlefieldPortAction::read(r)?;

        Ok(Self {
            map,
            action,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // map: Map
        self.map.write(w)?;

        // action: BattlefieldPortAction
        self.action.write(w)?;

        Ok(())
    }
}

impl ConstantSized for CMSG_BATTLEFIELD_PORT {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for CMSG_BATTLEFIELD_PORT {
    fn maximum_possible_size() -> usize {
        Map::size() // map: Map
        + BattlefieldPortAction::size() // action: BattlefieldPortAction
    }
}

#[derive(Debug)]
pub enum CMSG_BATTLEFIELD_PORTError {
    Io(std::io::Error),
    BattlefieldPortAction(BattlefieldPortActionError),
    Map(MapError),
}

impl std::error::Error for CMSG_BATTLEFIELD_PORTError {}
impl std::fmt::Display for CMSG_BATTLEFIELD_PORTError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::BattlefieldPortAction(i) => i.fmt(f),
            Self::Map(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_BATTLEFIELD_PORTError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<BattlefieldPortActionError> for CMSG_BATTLEFIELD_PORTError {
    fn from(e: BattlefieldPortActionError) -> Self {
        Self::BattlefieldPortAction(e)
    }
}

impl From<MapError> for CMSG_BATTLEFIELD_PORTError {
    fn from(e: MapError) -> Self {
        Self::Map(e)
    }
}

