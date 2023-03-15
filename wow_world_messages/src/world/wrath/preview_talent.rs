use std::io::{Read, Write};

use crate::wrath::Talent;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/cmsg_learn_preview_talents.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/cmsg_learn_preview_talents.wowm#L1):
/// ```text
/// struct PreviewTalent {
///     Talent talent;
///     u32 rank;
/// }
/// ```
pub struct PreviewTalent {
    pub talent: Talent,
    pub rank: u32,
}

impl PreviewTalent {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // talent: Talent
        w.write_all(&u32::from(self.talent.as_int()).to_le_bytes())?;

        // rank: u32
        w.write_all(&self.rank.to_le_bytes())?;

        Ok(())
    }
}

impl PreviewTalent {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> std::result::Result<Self, crate::errors::ParseError> {
        // talent: Talent
        let talent: Talent = crate::util::read_u32_le(&mut r)?.try_into()?;

        // rank: u32
        let rank = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            talent,
            rank,
        })
    }

}

