use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/resurrect/smsg_resurrect_request.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/resurrect/smsg_resurrect_request.wowm#L3):
/// ```text
/// smsg SMSG_RESURRECT_REQUEST = 0x015B {
///     Guid guid;
///     SizedCString name;
///     u8 caster_is_spirit_healer;
///     u8 respect_resurrection_timer;
/// }
/// ```
pub struct SMSG_RESURRECT_REQUEST {
    pub guid: Guid,
    pub name: String,
    pub caster_is_spirit_healer: u8,
    pub respect_resurrection_timer: u8,
}

impl ServerMessage for SMSG_RESURRECT_REQUEST {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // name: SizedCString
        w.write_all(&((self.name.len() + 1) as u32).to_le_bytes())?;
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

    fn server_size(&self) -> u16 {
        (self.size() + 4) as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // guid: Guid
        let guid = Guid::read(r)?;

        // name: SizedCString
        let name = crate::util::read_u32_le(r)?;
        let name = crate::util::read_sized_c_string_to_vec(r, name)?;
        let name = String::from_utf8(name)?;;
        // caster_is_spirit_healer: u8
        let caster_is_spirit_healer = crate::util::read_u8_le(r)?;

        // respect_resurrection_timer: u8
        let respect_resurrection_timer = crate::util::read_u8_le(r)?;

        Ok(Self {
            guid,
            name,
            caster_is_spirit_healer,
            respect_resurrection_timer,
        })
    }

}

impl SMSG_RESURRECT_REQUEST {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + self.name.len() + 5 // name: SizedCString
        + 1 // caster_is_spirit_healer: u8
        + 1 // respect_resurrection_timer: u8
    }
}

