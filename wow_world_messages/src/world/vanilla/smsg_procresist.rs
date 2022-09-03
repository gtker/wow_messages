use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::LogFormat;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_procresist.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_procresist.wowm#L8):
/// ```text
/// smsg SMSG_PROCRESIST = 0x0260 {
///     Guid guid;
///     Guid target_guid;
///     u32 id;
///     LogFormat log_format;
/// }
/// ```
pub struct SMSG_PROCRESIST {
    pub guid: Guid,
    pub target_guid: Guid,
    pub id: u32,
    pub log_format: LogFormat,
}

impl crate::Message for SMSG_PROCRESIST {
    const OPCODE: u32 = 0x0260;

    fn size_without_header(&self) -> u32 {
        21
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // target_guid: Guid
        w.write_all(&self.target_guid.guid().to_le_bytes())?;

        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // log_format: LogFormat
        w.write_all(&(self.log_format.as_int() as u8).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 21 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // guid: Guid
        let guid = Guid::read(r)?;

        // target_guid: Guid
        let target_guid = Guid::read(r)?;

        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // log_format: LogFormat
        let log_format: LogFormat = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            guid,
            target_guid,
            id,
            log_format,
        })
    }

}
impl ServerMessage for SMSG_PROCRESIST {}

