use std::convert::{TryFrom, TryInto};
use crate::helper::Guid;
use crate::{WorldClientMessageWrite, WorldMessageBody};
use wow_srp::header_crypto::Encrypter;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/cmsg_petition_buy.wowm:5`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/cmsg_petition_buy.wowm#L5):
/// ```text
/// cmsg CMSG_PETITION_BUY = 0x1BD {
///     Guid npc;
///     u32 skip1;
///     u64 skip2;
///     CString name;
///     u32 skip3;
///     u32 skip4;
///     u32 skip5;
///     u32 skip6;
///     u32 skip7;
///     u32 skip8;
///     u32 skip9;
///     u32 skip10;
///     u32 skip11;
///     u32 skip12;
///     u16 skip13;
///     u8 skip14;
///     u32 index;
///     u32 skip15;
/// }
/// ```
pub struct CMSG_PETITION_BUY {
    pub npc: Guid,
    pub skip1: u32,
    pub skip2: u64,
    pub name: String,
    pub skip3: u32,
    pub skip4: u32,
    pub skip5: u32,
    pub skip6: u32,
    pub skip7: u32,
    pub skip8: u32,
    pub skip9: u32,
    pub skip10: u32,
    pub skip11: u32,
    pub skip12: u32,
    pub skip13: u16,
    pub skip14: u8,
    pub index: u32,
    pub skip15: u32,
}

impl WorldClientMessageWrite for CMSG_PETITION_BUY {
    const OPCODE: u32 = 0x1bd;

    fn write_unencrypted_client<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        crate::util::write_u16_be(w, (self.size() + 4) as u16)?;
        crate::util::write_u32_le(w, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }

    fn write_encrypted_client<W: std::io::Write, E: Encrypter>(&self, w: &mut W, e: &mut E) -> std::result::Result<(), std::io::Error> {
        // size: u16_be, and opcode: u32
        e.write_encrypted_client_header(w, (self.size() + 4) as u16, <Self as WorldClientMessageWrite>::OPCODE)?;

        self.write_body(w)?;
        Ok(())
    }
}
impl WorldMessageBody for CMSG_PETITION_BUY {
    type Error = CMSG_PETITION_BUYError;

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, Self::Error> {
        // npc: Guid
        let npc = Guid::read(r)?;

        // skip1: u32
        let skip1 = crate::util::read_u32_le(r)?;

        // skip2: u64
        let skip2 = crate::util::read_u64_le(r)?;

        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        // skip3: u32
        let skip3 = crate::util::read_u32_le(r)?;

        // skip4: u32
        let skip4 = crate::util::read_u32_le(r)?;

        // skip5: u32
        let skip5 = crate::util::read_u32_le(r)?;

        // skip6: u32
        let skip6 = crate::util::read_u32_le(r)?;

        // skip7: u32
        let skip7 = crate::util::read_u32_le(r)?;

        // skip8: u32
        let skip8 = crate::util::read_u32_le(r)?;

        // skip9: u32
        let skip9 = crate::util::read_u32_le(r)?;

        // skip10: u32
        let skip10 = crate::util::read_u32_le(r)?;

        // skip11: u32
        let skip11 = crate::util::read_u32_le(r)?;

        // skip12: u32
        let skip12 = crate::util::read_u32_le(r)?;

        // skip13: u16
        let skip13 = crate::util::read_u16_le(r)?;

        // skip14: u8
        let skip14 = crate::util::read_u8_le(r)?;

        // index: u32
        let index = crate::util::read_u32_le(r)?;

        // skip15: u32
        let skip15 = crate::util::read_u32_le(r)?;

        Ok(Self {
            npc,
            skip1,
            skip2,
            name,
            skip3,
            skip4,
            skip5,
            skip6,
            skip7,
            skip8,
            skip9,
            skip10,
            skip11,
            skip12,
            skip13,
            skip14,
            index,
            skip15,
        })
    }

    fn write_body<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // npc: Guid
        self.npc.write(w)?;

        // skip1: u32
        w.write_all(&self.skip1.to_le_bytes())?;

        // skip2: u64
        w.write_all(&self.skip2.to_le_bytes())?;

        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // skip3: u32
        w.write_all(&self.skip3.to_le_bytes())?;

        // skip4: u32
        w.write_all(&self.skip4.to_le_bytes())?;

        // skip5: u32
        w.write_all(&self.skip5.to_le_bytes())?;

        // skip6: u32
        w.write_all(&self.skip6.to_le_bytes())?;

        // skip7: u32
        w.write_all(&self.skip7.to_le_bytes())?;

        // skip8: u32
        w.write_all(&self.skip8.to_le_bytes())?;

        // skip9: u32
        w.write_all(&self.skip9.to_le_bytes())?;

        // skip10: u32
        w.write_all(&self.skip10.to_le_bytes())?;

        // skip11: u32
        w.write_all(&self.skip11.to_le_bytes())?;

        // skip12: u32
        w.write_all(&self.skip12.to_le_bytes())?;

        // skip13: u16
        w.write_all(&self.skip13.to_le_bytes())?;

        // skip14: u8
        w.write_all(&self.skip14.to_le_bytes())?;

        // index: u32
        w.write_all(&self.index.to_le_bytes())?;

        // skip15: u32
        w.write_all(&self.skip15.to_le_bytes())?;

        Ok(())
    }
}

impl VariableSized for CMSG_PETITION_BUY {
    fn size(&self) -> usize {
        8 // npc: Guid
        + 4 // skip1: u32
        + 8 // skip2: u64
        + self.name.len() + 1 // name: CString and Null Terminator
        + 4 // skip3: u32
        + 4 // skip4: u32
        + 4 // skip5: u32
        + 4 // skip6: u32
        + 4 // skip7: u32
        + 4 // skip8: u32
        + 4 // skip9: u32
        + 4 // skip10: u32
        + 4 // skip11: u32
        + 4 // skip12: u32
        + 2 // skip13: u16
        + 1 // skip14: u8
        + 4 // index: u32
        + 4 // skip15: u32
    }
}

impl MaximumPossibleSized for CMSG_PETITION_BUY {
    fn maximum_possible_size() -> usize {
        8 // npc: Guid
        + 4 // skip1: u32
        + 8 // skip2: u64
        + 256 // name: CString
        + 4 // skip3: u32
        + 4 // skip4: u32
        + 4 // skip5: u32
        + 4 // skip6: u32
        + 4 // skip7: u32
        + 4 // skip8: u32
        + 4 // skip9: u32
        + 4 // skip10: u32
        + 4 // skip11: u32
        + 4 // skip12: u32
        + 2 // skip13: u16
        + 1 // skip14: u8
        + 4 // index: u32
        + 4 // skip15: u32
    }
}

#[derive(Debug)]
pub enum CMSG_PETITION_BUYError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for CMSG_PETITION_BUYError {}
impl std::fmt::Display for CMSG_PETITION_BUYError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for CMSG_PETITION_BUYError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for CMSG_PETITION_BUYError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

