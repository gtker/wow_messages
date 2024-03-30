use std::io::{Read, Write};

use crate::wrath::Talent;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_inspect_talent.wowm:16`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_inspect_talent.wowm#L16):
/// ```text
/// struct InspectTalent {
///     Talent talent;
///     u8 max_rank;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct InspectTalent {
    pub talent: Talent,
    pub max_rank: u8,
}

impl InspectTalent {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // talent: Talent
        w.write_all(&(self.talent.as_int().to_le_bytes()))?;

        // max_rank: u8
        w.write_all(&self.max_rank.to_le_bytes())?;

        Ok(())
    }
}

impl InspectTalent {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // talent: Talent
        let talent = crate::util::read_u32_le(&mut r)?.try_into()?;

        // max_rank: u8
        let max_rank = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            talent,
            max_rank,
        })
    }

}

