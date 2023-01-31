use std::convert::{TryFrom, TryInto};
use crate::wrath::InspectTalent;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_inspect_talent.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_inspect_talent.wowm#L9):
/// ```text
/// struct InspectTalentSpec {
///     u8 amount_of_talents;
///     InspectTalent[amount_of_talents] talents;
/// }
/// ```
pub struct InspectTalentSpec {
    pub talents: Vec<InspectTalent>,
}

impl InspectTalentSpec {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // amount_of_talents: u8
        w.write_all(&(self.talents.len() as u8).to_le_bytes())?;

        // talents: InspectTalent[amount_of_talents]
        for i in self.talents.iter() {
            i.write_into_vec(w)?;
        }

        Ok(())
    }
}

impl InspectTalentSpec {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // amount_of_talents: u8
        let amount_of_talents = crate::util::read_u8_le(r)?;

        // talents: InspectTalent[amount_of_talents]
        let mut talents = Vec::with_capacity(amount_of_talents as usize);
        for i in 0..amount_of_talents {
            talents.push(InspectTalent::read(r)?);
        }

        Ok(Self {
            talents,
        })
    }

}

impl InspectTalentSpec {
    pub(crate) fn size(&self) -> usize {
        1 // amount_of_talents: u8
        + self.talents.len() * 5 // talents: InspectTalent[amount_of_talents]
    }
}

