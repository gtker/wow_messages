use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_inspect_talent.wowm:16`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_inspect_talent.wowm#L16):
/// ```text
/// struct InspectTalent {
///     u32 talent_id;
///     u8 max_rank;
/// }
/// ```
pub struct InspectTalent {
    pub talent_id: u32,
    pub max_rank: u8,
}

impl InspectTalent {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // talent_id: u32
        w.write_all(&self.talent_id.to_le_bytes())?;

        // max_rank: u8
        w.write_all(&self.max_rank.to_le_bytes())?;

        Ok(())
    }
}

impl InspectTalent {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // talent_id: u32
        let talent_id = crate::util::read_u32_le(r)?;

        // max_rank: u8
        let max_rank = crate::util::read_u8_le(r)?;

        Ok(Self {
            talent_id,
            max_rank,
        })
    }

}

