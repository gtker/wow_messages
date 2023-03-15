use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::PreviewTalent;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/cmsg_learn_preview_talents_pet.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/cmsg_learn_preview_talents_pet.wowm#L1):
/// ```text
/// cmsg CMSG_LEARN_PREVIEW_TALENTS_PET = 0x04C2 {
///     Guid pet;
///     u32 amount_of_talents;
///     PreviewTalent[amount_of_talents] talents;
/// }
/// ```
pub struct CMSG_LEARN_PREVIEW_TALENTS_PET {
    pub pet: Guid,
    pub talents: Vec<PreviewTalent>,
}

impl crate::Message for CMSG_LEARN_PREVIEW_TALENTS_PET {
    const OPCODE: u32 = 0x04c2;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // pet: Guid
        w.write_all(&self.pet.guid().to_le_bytes())?;

        // amount_of_talents: u32
        w.write_all(&(self.talents.len() as u32).to_le_bytes())?;

        // talents: PreviewTalent[amount_of_talents]
        for i in self.talents.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(12..=10240).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04C2, size: body_size as u32 });
        }

        // pet: Guid
        let pet = Guid::read(&mut r)?;

        // amount_of_talents: u32
        let amount_of_talents = crate::util::read_u32_le(&mut r)?;

        // talents: PreviewTalent[amount_of_talents]
        let talents = {
            let mut talents = Vec::with_capacity(amount_of_talents as usize);
            for i in 0..amount_of_talents {
                talents.push(PreviewTalent::read(&mut r)?);
            }
            talents
        };

        Ok(Self {
            pet,
            talents,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_LEARN_PREVIEW_TALENTS_PET {}

impl CMSG_LEARN_PREVIEW_TALENTS_PET {
    pub(crate) fn size(&self) -> usize {
        8 // pet: Guid
        + 4 // amount_of_talents: u32
        + self.talents.len() * 8 // talents: PreviewTalent[amount_of_talents]
    }
}

