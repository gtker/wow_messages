use std::io::{Read, Write};

use crate::wrath::{
    InspectTalent, Talent,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_inspect_talent.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_inspect_talent.wowm#L9):
/// ```text
/// struct InspectTalentSpec {
///     u8 amount_of_talents;
///     InspectTalent[amount_of_talents] talents;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct InspectTalentSpec {
    pub talents: Vec<InspectTalent>,
}

impl InspectTalentSpec {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // amount_of_talents: u8
        w.write_all(&(self.talents.len() as u8).to_le_bytes())?;

        // talents: InspectTalent[amount_of_talents]
        for i in self.talents.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
}

impl InspectTalentSpec {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // amount_of_talents: u8
        let amount_of_talents = crate::util::read_u8_le(&mut r)?;

        // talents: InspectTalent[amount_of_talents]
        let talents = {
            let mut talents = Vec::with_capacity(amount_of_talents as usize);
            for _ in 0..amount_of_talents {
                talents.push(InspectTalent::read(&mut r)?);
            }
            talents
        };

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

