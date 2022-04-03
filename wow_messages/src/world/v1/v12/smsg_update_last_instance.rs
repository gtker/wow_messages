use std::convert::{TryFrom, TryInto};
use crate::world::v1::v12::{Map, MapError};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/new4.wowm:504`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/new4.wowm#L504):
/// ```text
/// smsg SMSG_UPDATE_LAST_INSTANCE = 0x320 {
///     Map map;
/// }
/// ```
pub struct SMSG_UPDATE_LAST_INSTANCE {
    pub map: Map,
}

impl WorldServerMessageWrite for SMSG_UPDATE_LAST_INSTANCE {
    const OPCODE: u16 = 0x320;

    fn write_unencrypted_server<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        crate::util::write_u16_be(w, (Self::size() + 2) as u16)?;
        crate::util::write_u16_le(w, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_server<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u16
        e.write_encrypted_server_header(w, (Self::size() + 2) as u16, <Self as WorldServerMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for SMSG_UPDATE_LAST_INSTANCE {
    type Error = SMSG_UPDATE_LAST_INSTANCEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // map: Map
        let map = Map::read(r)?;

        Ok(Self {
            map,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // map: Map
        self.map.write(w)?;

        Ok(())
    }
}

impl ConstantSized for SMSG_UPDATE_LAST_INSTANCE {
    fn size() -> usize {
        Self::maximum_possible_size()
    }
}

impl MaximumPossibleSized for SMSG_UPDATE_LAST_INSTANCE {
    fn maximum_possible_size() -> usize {
        Map::size() // map: Map
    }
}

#[derive(Debug)]
pub enum SMSG_UPDATE_LAST_INSTANCEError {
    Io(std::io::Error),
    Map(MapError),
}

impl std::error::Error for SMSG_UPDATE_LAST_INSTANCEError {}
impl std::fmt::Display for SMSG_UPDATE_LAST_INSTANCEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::Map(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_UPDATE_LAST_INSTANCEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<MapError> for SMSG_UPDATE_LAST_INSTANCEError {
    fn from(e: MapError) -> Self {
        Self::Map(e)
    }
}

