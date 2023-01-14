use std::convert::{TryFrom, TryInto};
use crate::world::wrath::InspectTalent;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_talents_info.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_talents_info.wowm#L8):
/// ```text
/// struct TalentInfoSpec {
///     u8 amount_of_talents;
///     InspectTalent[amount_of_talents] talents;
///     u8 amount_of_glyphs;
///     u16[amount_of_glyphs] glyphs;
/// }
/// ```
pub struct TalentInfoSpec {
    pub talents: Vec<InspectTalent>,
    pub glyphs: Vec<u16>,
}

impl TalentInfoSpec {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // amount_of_talents: u8
        w.write_all(&(self.talents.len() as u8).to_le_bytes())?;

        // talents: InspectTalent[amount_of_talents]
        for i in self.talents.iter() {
            i.write_into_vec(w)?;
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
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // amount_of_talents: u8
        let amount_of_talents = crate::util::read_u8_le(r)?;

        // talents: InspectTalent[amount_of_talents]
        let mut talents = Vec::with_capacity(amount_of_talents as usize);
        for i in 0..amount_of_talents {
            talents.push(InspectTalent::read(r)?);
        }

        // amount_of_glyphs: u8
        let amount_of_glyphs = crate::util::read_u8_le(r)?;

        // glyphs: u16[amount_of_glyphs]
        let mut glyphs = Vec::with_capacity(amount_of_glyphs as usize);
        for i in 0..amount_of_glyphs {
            glyphs.push(crate::util::read_u16_le(r)?);
        }

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

