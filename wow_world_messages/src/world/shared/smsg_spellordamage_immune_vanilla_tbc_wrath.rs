use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spellordamage_immune.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spellordamage_immune.wowm#L3):
/// ```text
/// smsg SMSG_SPELLORDAMAGE_IMMUNE = 0x0263 {
///     Guid caster;
///     Guid target;
///     u32 id;
///     Bool debug_log_format;
/// }
/// ```
pub struct SMSG_SPELLORDAMAGE_IMMUNE {
    pub caster: Guid,
    pub target: Guid,
    pub id: u32,
    pub debug_log_format: bool,
}

impl crate::Message for SMSG_SPELLORDAMAGE_IMMUNE {
    const OPCODE: u32 = 0x0263;

    fn size_without_header(&self) -> u32 {
        21
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // caster: Guid
        w.write_all(&self.caster.guid().to_le_bytes())?;

        // target: Guid
        w.write_all(&self.target.guid().to_le_bytes())?;

        // id: u32
        w.write_all(&self.id.to_le_bytes())?;

        // debug_log_format: Bool
        w.write_all(u8::from(self.debug_log_format).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 21 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0263, size: body_size as u32 });
        }

        // caster: Guid
        let caster = Guid::read(&mut r)?;

        // target: Guid
        let target = Guid::read(&mut r)?;

        // id: u32
        let id = crate::util::read_u32_le(&mut r)?;

        // debug_log_format: Bool
        let debug_log_format = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            caster,
            target,
            id,
            debug_log_format,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_SPELLORDAMAGE_IMMUNE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_SPELLORDAMAGE_IMMUNE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SPELLORDAMAGE_IMMUNE {}

