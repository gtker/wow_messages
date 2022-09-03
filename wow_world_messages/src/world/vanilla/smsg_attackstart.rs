use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_attackstart.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_attackstart.wowm#L3):
/// ```text
/// smsg SMSG_ATTACKSTART = 0x0143 {
///     Guid attacker_guid;
///     Guid victim_guid;
/// }
/// ```
pub struct SMSG_ATTACKSTART {
    pub attacker_guid: Guid,
    pub victim_guid: Guid,
}

impl crate::Message for SMSG_ATTACKSTART {
    const OPCODE: u32 = 0x0143;

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // attacker_guid: Guid
        w.write_all(&self.attacker_guid.guid().to_le_bytes())?;

        // victim_guid: Guid
        w.write_all(&self.victim_guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // attacker_guid: Guid
        let attacker_guid = Guid::read(r)?;

        // victim_guid: Guid
        let victim_guid = Guid::read(r)?;

        Ok(Self {
            attacker_guid,
            victim_guid,
        })
    }

}
impl ServerMessage for SMSG_ATTACKSTART {}

