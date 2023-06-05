use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/cmsg_summon_response.wowm:7`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/cmsg_summon_response.wowm#L7):
/// ```text
/// cmsg CMSG_SUMMON_RESPONSE = 0x02AC {
///     Guid summoner;
///     Bool agree;
/// }
/// ```
pub struct CMSG_SUMMON_RESPONSE {
    pub summoner: Guid,
    pub agree: bool,
}

impl crate::private::Sealed for CMSG_SUMMON_RESPONSE {}
impl crate::Message for CMSG_SUMMON_RESPONSE {
    const OPCODE: u32 = 0x02ac;

    fn size_without_header(&self) -> u32 {
        9
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // summoner: Guid
        w.write_all(&self.summoner.guid().to_le_bytes())?;

        // agree: Bool
        w.write_all(u8::from(self.agree).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 9 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02AC, size: body_size });
        }

        // summoner: Guid
        let summoner = crate::util::read_guid(&mut r)?;

        // agree: Bool
        let agree = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            summoner,
            agree,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_SUMMON_RESPONSE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_SUMMON_RESPONSE {}

