use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::shared::petition_signature_vanilla_tbc_wrath::PetitionSignature;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/smsg_petition_show_signatures.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_petition_show_signatures.wowm#L8):
/// ```text
/// smsg SMSG_PETITION_SHOW_SIGNATURES = 0x01BF {
///     Guid item_guid;
///     Guid owner_guid;
///     u32 petition_guid;
///     u8 amount_of_signatures;
///     PetitionSignature[amount_of_signatures] signatures;
/// }
/// ```
pub struct SMSG_PETITION_SHOW_SIGNATURES {
    pub item_guid: Guid,
    pub owner_guid: Guid,
    pub petition_guid: u32,
    pub signatures: Vec<PetitionSignature>,
}

impl crate::Message for SMSG_PETITION_SHOW_SIGNATURES {
    const OPCODE: u32 = 0x01bf;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // item_guid: Guid
        w.write_all(&self.item_guid.guid().to_le_bytes())?;

        // owner_guid: Guid
        w.write_all(&self.owner_guid.guid().to_le_bytes())?;

        // petition_guid: u32
        w.write_all(&self.petition_guid.to_le_bytes())?;

        // amount_of_signatures: u8
        w.write_all(&(self.signatures.len() as u8).to_le_bytes())?;

        // signatures: PetitionSignature[amount_of_signatures]
        for i in self.signatures.iter() {
            i.write_into_vec(w)?;
        }

        assert_eq!(self.size() as usize, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // item_guid: Guid
        let item_guid = Guid::read(r)?;

        // owner_guid: Guid
        let owner_guid = Guid::read(r)?;

        // petition_guid: u32
        let petition_guid = crate::util::read_u32_le(r)?;

        // amount_of_signatures: u8
        let amount_of_signatures = crate::util::read_u8_le(r)?;

        // signatures: PetitionSignature[amount_of_signatures]
        let mut signatures = Vec::with_capacity(amount_of_signatures as usize);
        for i in 0..amount_of_signatures {
            signatures.push(PetitionSignature::read(r)?);
        }

        Ok(Self {
            item_guid,
            owner_guid,
            petition_guid,
            signatures,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_PETITION_SHOW_SIGNATURES {}

#[cfg(feature = "tbc")]
impl crate::world::tbc::ServerMessage for SMSG_PETITION_SHOW_SIGNATURES {}

#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_PETITION_SHOW_SIGNATURES {}

impl SMSG_PETITION_SHOW_SIGNATURES {
    pub(crate) fn size(&self) -> usize {
        8 // item_guid: Guid
        + 8 // owner_guid: Guid
        + 4 // petition_guid: u32
        + 1 // amount_of_signatures: u8
        + self.signatures.len() * 12 // signatures: PetitionSignature[amount_of_signatures]
    }
}

