use std::io::{Read, Write};

use crate::tbc::Faction;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/faction/smsg_set_faction_standing.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/faction/smsg_set_faction_standing.wowm#L1):
/// ```text
/// struct FactionStanding {
///     Faction faction;
///     u32 standing;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct FactionStanding {
    pub faction: Faction,
    pub standing: u32,
}

impl FactionStanding {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // faction: Faction
        w.write_all(&(self.faction.as_int().to_le_bytes()))?;

        // standing: u32
        w.write_all(&self.standing.to_le_bytes())?;

        Ok(())
    }
}

impl FactionStanding {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // faction: Faction
        let faction = crate::util::read_u16_le(&mut r)?.try_into()?;

        // standing: u32
        let standing = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            faction,
            standing,
        })
    }

}

