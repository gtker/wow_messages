use crate::Guid;
use crate::tbc::Area;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_summon_request.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_summon_request.wowm#L3):
/// ```text
/// smsg SMSG_SUMMON_REQUEST = 0x02AB {
///     Guid summoner;
///     Area area;
///     u32 auto_decline_time_in_msecs;
/// }
/// ```
pub struct SMSG_SUMMON_REQUEST {
    pub summoner: Guid,
    pub area: Area,
    pub auto_decline_time_in_msecs: u32,
}

impl crate::Message for SMSG_SUMMON_REQUEST {
    const OPCODE: u32 = 0x02ab;

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // summoner: Guid
        w.write_all(&self.summoner.guid().to_le_bytes())?;

        // area: Area
        w.write_all(&(self.area.as_int() as u32).to_le_bytes())?;

        // auto_decline_time_in_msecs: u32
        w.write_all(&self.auto_decline_time_in_msecs.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02AB, size: body_size as u32 });
        }

        // summoner: Guid
        let summoner = Guid::read(r)?;

        // area: Area
        let area: Area = crate::util::read_u32_le(r)?.try_into()?;

        // auto_decline_time_in_msecs: u32
        let auto_decline_time_in_msecs = crate::util::read_u32_le(r)?;

        Ok(Self {
            summoner,
            area,
            auto_decline_time_in_msecs,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_SUMMON_REQUEST {}

