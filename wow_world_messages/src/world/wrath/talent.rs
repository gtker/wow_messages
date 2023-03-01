use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/cmsg_learn_preview_talents.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/cmsg_learn_preview_talents.wowm#L1):
/// ```text
/// struct Talent {
///     u32 talent;
///     u32 rank;
/// }
/// ```
pub struct Talent {
    pub talent: u32,
    pub rank: u32,
}

impl Talent {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // talent: u32
        w.write_all(&self.talent.to_le_bytes())?;

        // rank: u32
        w.write_all(&self.rank.to_le_bytes())?;

        Ok(())
    }
}

impl Talent {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> std::result::Result<Self, std::io::Error> {
        // talent: u32
        let talent = crate::util::read_u32_le(&mut r)?;

        // rank: u32
        let rank = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            talent,
            rank,
        })
    }

}

