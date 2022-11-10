use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/cmsg_summon_response.wowm:7`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/cmsg_summon_response.wowm#L7):
/// ```text
/// cmsg CMSG_SUMMON_RESPONSE = 0x02AC {
///     Guid summoner_guid;
///     Bool agree;
/// }
/// ```
pub struct CMSG_SUMMON_RESPONSE {
    pub summoner_guid: Guid,
    pub agree: bool,
}

impl crate::Message for CMSG_SUMMON_RESPONSE {
    const OPCODE: u32 = 0x02ac;

    fn size_without_header(&self) -> u32 {
        9
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // summoner_guid: Guid
        w.write_all(&self.summoner_guid.guid().to_le_bytes())?;

        // agree: Bool
        w.write_all(u8::from(self.agree).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 9 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02AC, size: body_size as u32 });
        }

        // summoner_guid: Guid
        let summoner_guid = Guid::read(r)?;

        // agree: Bool
        let agree = crate::util::read_u8_le(r)? != 0;
        Ok(Self {
            summoner_guid,
            agree,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ClientMessage for CMSG_SUMMON_RESPONSE {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ClientMessage for CMSG_SUMMON_RESPONSE {}

