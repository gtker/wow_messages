use crate:: {
    Guid,
};
use crate::shared::petition_signature_vanilla_tbc_wrath::PetitionSignature;
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/smsg_petition_show_signatures.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_petition_show_signatures.wowm#L8):
/// ```text
/// smsg SMSG_PETITION_SHOW_SIGNATURES = 0x01BF {
///     Guid item;
///     Guid owner;
///     u32 petition;
///     u8 amount_of_signatures;
///     PetitionSignature[amount_of_signatures] signatures;
/// }
/// ```
pub struct SMSG_PETITION_SHOW_SIGNATURES {
    pub item: Guid,
    pub owner: Guid,
    pub petition: u32,
    pub signatures: Vec<PetitionSignature>,
}

impl crate::Message for SMSG_PETITION_SHOW_SIGNATURES {
    const OPCODE: u32 = 0x01bf;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // item: Guid
        w.write_all(&self.item.guid().to_le_bytes())?;

        // owner: Guid
        w.write_all(&self.owner.guid().to_le_bytes())?;

        // petition: u32
        w.write_all(&self.petition.to_le_bytes())?;

        // amount_of_signatures: u8
        w.write_all(&(self.signatures.len() as u8).to_le_bytes())?;

        // signatures: PetitionSignature[amount_of_signatures]
        for i in self.signatures.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(21..=3093).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01BF, size: body_size as u32 });
        }

        // item: Guid
        let item = Guid::read(&mut r)?;

        // owner: Guid
        let owner = Guid::read(&mut r)?;

        // petition: u32
        let petition = crate::util::read_u32_le(&mut r)?;

        // amount_of_signatures: u8
        let amount_of_signatures = crate::util::read_u8_le(&mut r)?;

        // signatures: PetitionSignature[amount_of_signatures]
        let signatures = {
            let mut signatures = Vec::with_capacity(amount_of_signatures as usize);
            for i in 0..amount_of_signatures {
                signatures.push(PetitionSignature::read(&mut r)?);
            }
            signatures
        };

        Ok(Self {
            item,
            owner,
            petition,
            signatures,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_PETITION_SHOW_SIGNATURES {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_PETITION_SHOW_SIGNATURES {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_PETITION_SHOW_SIGNATURES {}

impl SMSG_PETITION_SHOW_SIGNATURES {
    pub(crate) fn size(&self) -> usize {
        8 // item: Guid
        + 8 // owner: Guid
        + 4 // petition: u32
        + 1 // amount_of_signatures: u8
        + self.signatures.len() * 12 // signatures: PetitionSignature[amount_of_signatures]
    }
}

