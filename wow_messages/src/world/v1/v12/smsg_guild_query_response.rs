use std::convert::{TryFrom, TryInto};
use crate::world::helper::{WorldServerMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/add_messages.wowm:58`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/add_messages.wowm):
/// ```text
/// smsg SMSG_GUILD_QUERY_RESPONSE = 0x55 {
///     u32 id;
///     CString name;
///     CString[10] rank_names;
///     u32 emblem_style;
///     u32 emblem_color;
///     u32 border_style;
///     u32 border_color;
///     u32 background_color;
/// }
/// ```
pub struct SMSG_GUILD_QUERY_RESPONSE {
    pub id: u32,
    pub name: String,
    pub rank_names: [String; 10],
    pub emblem_style: u32,
    pub emblem_color: u32,
    pub border_style: u32,
    pub border_color: u32,
    pub background_color: u32,
}

impl WorldServerMessageWrite for SMSG_GUILD_QUERY_RESPONSE {
    const OPCODE: u16 = 0x55;

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
impl WorldMessageBody for SMSG_GUILD_QUERY_RESPONSE {
    type Error = SMSG_GUILD_QUERY_RESPONSEError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        // rank_names: CString[10]
        let mut rank_names = Vec::with_capacity(10 as usize);
        for i in 0..10 {
            let s = crate::util::read_c_string_to_vec(r)?;
            rank_names[i] = String::from_utf8(s)?;
        }
        let rank_names = rank_names.try_into().unwrap();

        // emblem_style: u32
        let emblem_style = crate::util::read_u32_le(r)?;

        // emblem_color: u32
        let emblem_color = crate::util::read_u32_le(r)?;

        // border_style: u32
        let border_style = crate::util::read_u32_le(r)?;

        // border_color: u32
        let border_color = crate::util::read_u32_le(r)?;

        // background_color: u32
        let background_color = crate::util::read_u32_le(r)?;

        Ok(Self {
            id,
            name,
            rank_names,
            emblem_style,
            emblem_color,
            border_style,
            border_color,
            background_color,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // rank_names: CString[10]
        for i in self.rank_names.iter() {
            w.write_all(&i.as_bytes())?;
            w.write_all(&[0])?;
        }

        // emblem_style: u32
        w.write_all(&self.emblem_style.to_le_bytes())?;

        // emblem_color: u32
        w.write_all(&self.emblem_color.to_le_bytes())?;

        // border_style: u32
        w.write_all(&self.border_style.to_le_bytes())?;

        // border_color: u32
        w.write_all(&self.border_color.to_le_bytes())?;

        // background_color: u32
        w.write_all(&self.background_color.to_le_bytes())?;

        Ok(())
    }
}

impl VariableSized for SMSG_GUILD_QUERY_RESPONSE {
    fn size(&self) -> usize {
        4 // id: u32
        + self.name.len() + 1 // name: CString and Null Terminator
        + self.rank_names.iter().fold(0, |acc, x| acc + x.len() + 1) // rank_names: CString[10]
        + 4 // emblem_style: u32
        + 4 // emblem_color: u32
        + 4 // border_style: u32
        + 4 // border_color: u32
        + 4 // background_color: u32
    }
}

impl MaximumPossibleSized for SMSG_GUILD_QUERY_RESPONSE {
    fn maximum_possible_size() -> usize {
        4 // id: u32
        + 256 // name: CString
        + 10 * 256 // rank_names: CString[10]
        + 4 // emblem_style: u32
        + 4 // emblem_color: u32
        + 4 // border_style: u32
        + 4 // border_color: u32
        + 4 // background_color: u32
    }
}

#[derive(Debug)]
pub enum SMSG_GUILD_QUERY_RESPONSEError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for SMSG_GUILD_QUERY_RESPONSEError {}
impl std::fmt::Display for SMSG_GUILD_QUERY_RESPONSEError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for SMSG_GUILD_QUERY_RESPONSEError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for SMSG_GUILD_QUERY_RESPONSEError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

