use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_summon_request.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_summon_request.wowm#L3):
/// ```text
/// smsg SMSG_SUMMON_REQUEST = 0x02AB {
///     Guid summoner_guid;
///     u32 zone_id;
///     u32 auto_decline_time_in_msecs;
/// }
/// ```
pub struct SMSG_SUMMON_REQUEST {
    pub summoner_guid: Guid,
    pub zone_id: u32,
    pub auto_decline_time_in_msecs: u32,
}

impl ServerMessage for SMSG_SUMMON_REQUEST {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // summoner_guid: Guid
        w.write_all(&self.summoner_guid.guid().to_le_bytes())?;

        // zone_id: u32
        w.write_all(&self.zone_id.to_le_bytes())?;

        // auto_decline_time_in_msecs: u32
        w.write_all(&self.auto_decline_time_in_msecs.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x02ab;

    fn server_size(&self) -> u16 {
        20
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // summoner_guid: Guid
        let summoner_guid = Guid::read(r)?;

        // zone_id: u32
        let zone_id = crate::util::read_u32_le(r)?;

        // auto_decline_time_in_msecs: u32
        let auto_decline_time_in_msecs = crate::util::read_u32_le(r)?;

        Ok(Self {
            summoner_guid,
            zone_id,
            auto_decline_time_in_msecs,
        })
    }

}

