use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::PetitionResult;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/smsg_petition_sign_results.wowm:22`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_petition_sign_results.wowm#L22):
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

impl crate::private::Sealed for SMSG_PETITION_SIGN_RESULTS {}
impl crate::Message for SMSG_PETITION_SIGN_RESULTS {
    const OPCODE: u32 = 0x01c1;

    fn size_without_header(&self) -> u32 {
        20
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // petition: Guid
        w.write_all(&self.petition.guid().to_le_bytes())?;

        // owner: Guid
        w.write_all(&self.owner.guid().to_le_bytes())?;

        // result: PetitionResult
        w.write_all(&u32::from(self.result.as_int()).to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 20 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01C1, size: body_size as u32 });
        }

        // petition: Guid
        let petition = Guid::read(&mut r)?;

        // owner: Guid
        let owner = Guid::read(&mut r)?;

        // result: PetitionResult
        let result: PetitionResult = crate::util::read_u32_le(&mut r)?.try_into()?;

        Ok(Self {
            petition,
            owner,
            result,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_PETITION_SIGN_RESULTS {}

