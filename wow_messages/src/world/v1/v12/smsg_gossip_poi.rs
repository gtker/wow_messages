use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/unsorted/new2.wowm:723`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/unsorted/new2.wowm#L723):
/// ```text
/// smsg SMSG_GOSSIP_POI = 0x224 {
///     u32 flags;
///     f32 position_x;
///     f32 position_y;
///     u32 icon;
///     u32 data;
///     CString location_name;
/// }
/// ```
pub struct SMSG_GOSSIP_POI {
    pub flags: u32,
    pub position_x: f32,
    pub position_y: f32,
    pub icon: u32,
    pub data: u32,
    pub location_name: String,
}

impl WorldServerMessageWrite for SMSG_GOSSIP_POI {
    const OPCODE: u16 = 0x224;

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
impl WorldMessageBody for SMSG_GOSSIP_POI {
    type Error = SMSG_GOSSIP_POIError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // flags: u32
        let flags = crate::util::read_u32_le(r)?;

        // position_x: f32
        let position_x = crate::util::read_f32_le(r)?;
        // position_y: f32
        let position_y = crate::util::read_f32_le(r)?;
        // icon: u32
        let icon = crate::util::read_u32_le(r)?;

        // data: u32
        let data = crate::util::read_u32_le(r)?;

        // location_name: CString
        let location_name = crate::util::read_c_string_to_vec(r)?;
        let location_name = String::from_utf8(location_name)?;

        Ok(Self {
            flags,
            position_x,
            position_y,
            icon,
            data,
            location_name,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // flags: u32
        w.write_all(&self.flags.to_le_bytes())?;

        // position_x: f32
        w.write_all(&self.position_x.to_le_bytes())?;

        // position_y: f32
        w.write_all(&self.position_y.to_le_bytes())?;

        // icon: u32
        w.write_all(&self.icon.to_le_bytes())?;

        // data: u32
        w.write_all(&self.data.to_le_bytes())?;

        // location_name: CString
        w.write_all(self.location_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
}

impl VariableSized for SMSG_GOSSIP_POI {
    fn size(&self) -> usize {
        4 // flags: u32
        + 4 // position_x: f32
        + 4 // position_y: f32
        + 4 // icon: u32
        + 4 // data: u32
        + self.location_name.len() + 1 // location_name: CString and Null Terminator
    }
}

impl MaximumPossibleSized for SMSG_GOSSIP_POI {
    fn maximum_possible_size() -> usize {
        4 // flags: u32
        + 4 // position_x: f32
        + 4 // position_y: f32
        + 4 // icon: u32
        + 4 // data: u32
        + 256 // location_name: CString
    }
}

#[derive(Debug)]
pub enum SMSG_GOSSIP_POIError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for SMSG_GOSSIP_POIError {}
impl std::fmt::Display for SMSG_GOSSIP_POIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_GOSSIP_POIError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_GOSSIP_POIError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

