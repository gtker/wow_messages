use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::shared::petition_result_tbc_wrath::PetitionResult;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/smsg_petition_sign_results.wowm:30`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_petition_sign_results.wowm#L30):
/// ```text
/// smsg SMSG_PETITION_SIGN_RESULTS = 0x01C1 {
///     Guid petition;
///     Guid owner;
///     PetitionResult result;
/// }
/// ```
pub struct SMSG_PETITION_SIGN_RESULTS {
    pub petition: Guid,
    pub owner: Guid,
    pub result: PetitionResult,
}

impl crate::Message for SMSG_PETITION_SIGN_RESULTS {
    const OPCODE: u32 = 0x01c1;

    fn size_without_header(&self) -> u32 {
        20
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // petition: Guid
        w.write_all(&self.petition.guid().to_le_bytes())?;

        // owner: Guid
        w.write_all(&self.owner.guid().to_le_bytes())?;

        // result: PetitionResult
        w.write_all(&(self.result.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 20 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01C1, size: body_size as u32 });
        }

        // petition: Guid
        let petition = Guid::read(r)?;

        // owner: Guid
        let owner = Guid::read(r)?;

        // result: PetitionResult
        let result: PetitionResult = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            petition,
            owner,
            result,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_PETITION_SIGN_RESULTS {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_PETITION_SIGN_RESULTS {}

