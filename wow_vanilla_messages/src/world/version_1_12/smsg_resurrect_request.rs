use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/resurrect/smsg_resurrect_request.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/resurrect/smsg_resurrect_request.wowm#L3):
/// ```text
/// smsg SMSG_RESURRECT_REQUEST = 0x015B {
///     Guid guid;
///     u32 name_length;
///     CString name;
///     u8 caster_is_spirit_healer;
///     u8 respect_resurrection_timer;
/// }
/// ```
pub struct SMSG_RESURRECT_REQUEST {
    pub guid: Guid,
    pub name_length: u32,
    pub name: String,
    pub caster_is_spirit_healer: u8,
    pub respect_resurrection_timer: u8,
}

impl ServerMessage for SMSG_RESURRECT_REQUEST {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // name_length: u32
        w.write_all(&self.name_length.to_le_bytes())?;

        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // caster_is_spirit_healer: u8
        w.write_all(&self.caster_is_spirit_healer.to_le_bytes())?;

        // respect_resurrection_timer: u8
        w.write_all(&self.respect_resurrection_timer.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x015b;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // name_length: u32
        let name_length = crate::util::read_u32_le(r)?;

        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        // caster_is_spirit_healer: u8
        let caster_is_spirit_healer = crate::util::read_u8_le(r)?;

        // respect_resurrection_timer: u8
        let respect_resurrection_timer = crate::util::read_u8_le(r)?;

        Ok(Self {
            guid,
            name_length,
            name,
            caster_is_spirit_healer,
            respect_resurrection_timer,
        })
    }

}

impl SMSG_RESURRECT_REQUEST {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + 4 // name_length: u32
        + self.name.len() + 1 // name: CString
        + 1 // caster_is_spirit_healer: u8
        + 1 // respect_resurrection_timer: u8
    }
}

