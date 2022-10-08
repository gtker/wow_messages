use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::shared::log_format1_1223::LogFormat;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// According to cmangos/azerothcore/trinitycore/mangostwo. Not present in vmangos.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_procresist.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_procresist.wowm#L8):
/// ```text
/// smsg SMSG_PROCRESIST = 0x0260 {
///     Guid caster;
///     Guid target;
///     u32 id;
///     LogFormat log_format;
/// }
/// ```
pub struct SMSG_PROCRESIST {
    pub caster: Guid,
    pub target: Guid,
    pub id: u32,
    pub log_format: LogFormat,
}

impl crate::Message for SMSG_PROCRESIST {
    const OPCODE: u32 = 0x0260;

    fn size_without_header(&self) -> u32 {
        21
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // caster: Guid
        w.write_all(&self.caster.guid().to_le_bytes())?;

        // target: Guid
        w.write_all(&self.target.guid().to_le_bytes())?;

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

        // caster: Guid
        let caster = Guid::read(r)?;

        // target: Guid
        let target = Guid::read(r)?;

        // id: u32
        let id = crate::util::read_u32_le(r)?;

        // log_format: LogFormat
        let log_format: LogFormat = crate::util::read_u8_le(r)?.try_into()?;

        Ok(Self {
            caster,
            target,
            id,
            log_format,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_PROCRESIST {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_PROCRESIST {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_PROCRESIST {}

