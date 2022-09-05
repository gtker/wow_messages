use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::RaidTargetIndex;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/raid_target.wowm:21`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/raid_target.wowm#L21):
/// ```text
/// struct RaidTargetUpdate {
///     RaidTargetIndex index;
///     Guid guid;
/// }
/// ```
pub struct RaidTargetUpdate {
    pub index: RaidTargetIndex,
    pub guid: Guid,
}

impl RaidTargetUpdate {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // index: RaidTargetIndex
        w.write_all(&(self.index.as_int() as u8).to_le_bytes())?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
}

impl RaidTargetUpdate {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // index: RaidTargetIndex
        let index: RaidTargetIndex = crate::util::read_u8_le(r)?.try_into()?;

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            index,
            guid,
        })
    }

}

