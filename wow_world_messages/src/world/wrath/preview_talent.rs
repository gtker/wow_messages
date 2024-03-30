use std::io::{Read, Write};

use crate::wrath::Talent;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/cmsg_learn_preview_talents.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/cmsg_learn_preview_talents.wowm#L1):
/// ```text
/// struct PreviewTalent {
///     Talent talent;
///     u32 rank;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct PreviewTalent {
    pub talent: Talent,
    pub rank: u32,
}

impl PreviewTalent {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // talent: Talent
        w.write_all(&(self.talent.as_int().to_le_bytes()))?;

        // rank: u32
        w.write_all(&self.rank.to_le_bytes())?;

        Ok(())
    }
}

impl PreviewTalent {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // talent: Talent
        let talent = crate::util::read_u32_le(&mut r)?.try_into()?;

        // rank: u32
        let rank = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            talent,
            rank,
        })
    }

}

