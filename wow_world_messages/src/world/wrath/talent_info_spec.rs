use std::io::{Read, Write};

use crate::wrath::{
    InspectTalent, Talent,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_talents_info.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_talents_info.wowm#L8):
/// ```text
/// struct TalentInfoSpec {
///     u8 amount_of_talents;
///     InspectTalent[amount_of_talents] talents;
///     u8 amount_of_glyphs;
///     u16[amount_of_glyphs] glyphs;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct TalentInfoSpec {
    pub talents: Vec<InspectTalent>,
    pub glyphs: Vec<u16>,
}

impl TalentInfoSpec {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // amount_of_talents: u8
        w.write_all(&(self.talents.len() as u8).to_le_bytes())?;

        // talents: InspectTalent[amount_of_talents]
        for i in self.talents.iter() {
            i.write_into_vec(&mut w)?;
        }

        // amount_of_glyphs: u8
        w.write_all(&(self.glyphs.len() as u8).to_le_bytes())?;

        // glyphs: u16[amount_of_glyphs]
        for i in self.glyphs.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
}

impl TalentInfoSpec {
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

        // amount_of_glyphs: u8
        let amount_of_glyphs = crate::util::read_u8_le(&mut r)?;

        // glyphs: u16[amount_of_glyphs]
        let glyphs = {
            let mut glyphs = Vec::with_capacity(amount_of_glyphs as usize);
            for _ in 0..amount_of_glyphs {
                glyphs.push(crate::util::read_u16_le(&mut r)?);
            }
            glyphs
        };

        Ok(Self {
            talents,
            glyphs,
        })
    }

}

impl TalentInfoSpec {
    pub(crate) fn size(&self) -> usize {
        1 // amount_of_talents: u8
        + self.talents.len() * 5 // talents: InspectTalent[amount_of_talents]
        + 1 // amount_of_glyphs: u8
        + self.glyphs.len() * core::mem::size_of::<u16>() // glyphs: u16[amount_of_glyphs]
    }
}

