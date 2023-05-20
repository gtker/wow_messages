use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::Area;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_summon_request.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_summon_request.wowm#L3):
/// ```text
/// smsg SMSG_SUMMON_REQUEST = 0x02AB {
///     Guid summoner;
///     Area area;
///     Milliseconds auto_decline_time;
/// }
/// ```
pub struct SMSG_SUMMON_REQUEST {
    pub summoner: Guid,
    pub area: Area,
    pub auto_decline_time: Duration,
}

impl crate::private::Sealed for SMSG_SUMMON_REQUEST {}
impl crate::Message for SMSG_SUMMON_REQUEST {
    const OPCODE: u32 = 0x02ab;

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // summoner: Guid
        w.write_all(&self.summoner.guid().to_le_bytes())?;

        // area: Area
        w.write_all(&(self.area.as_int().to_le_bytes()))?;

        // auto_decline_time: Milliseconds
        w.write_all((self.auto_decline_time.as_millis() as u32).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02AB, size: body_size });
        }

        // summoner: Guid
        let summoner = Guid::read(&mut r)?;

        // area: Area
        let area: Area = crate::util::read_u32_le(&mut r)?.try_into()?;

        // auto_decline_time: Milliseconds
        let auto_decline_time = Duration::from_millis(crate::util::read_u32_le(&mut r)?.into());

        Ok(Self {
            summoner,
            area,
            auto_decline_time,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_SUMMON_REQUEST {}

