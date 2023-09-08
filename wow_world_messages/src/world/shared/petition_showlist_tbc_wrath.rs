use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/smsg_petition_showlist.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_petition_showlist.wowm#L1):
/// ```text
/// struct PetitionShowlist {
///     u32 index;
///     u32 charter_entry;
///     u32 charter_display_id;
///     u32 guild_charter_cost;
///     u32 unknown1;
///     u32 signatures_required;
/// }
/// ```
pub struct PetitionShowlist {
    pub index: u32,
    /// cmangos/vmangos/mangoszero: statically sets to guild charter item id (5863).
    pub charter_entry: u32,
    /// cmangos/vmangos/mangoszero: statically sets to guild charter display id (16161).
    pub charter_display_id: u32,
    /// cmangos/vmangos/mangoszero: statically set to 1000 (10 silver).
    pub guild_charter_cost: u32,
    /// cmangos/vmangos/mangoszero: statically set to 1
    /// arcemu: charter type? seems to be 0x0 for guilds and 0x1 for arena charters
    pub unknown1: u32,
    pub signatures_required: u32,
}

impl PetitionShowlist {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // index: u32
        w.write_all(&self.index.to_le_bytes())?;

        // charter_entry: u32
        w.write_all(&self.charter_entry.to_le_bytes())?;

        // charter_display_id: u32
        w.write_all(&self.charter_display_id.to_le_bytes())?;

        // guild_charter_cost: u32
        w.write_all(&self.guild_charter_cost.to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // signatures_required: u32
        w.write_all(&self.signatures_required.to_le_bytes())?;

        Ok(())
    }
}

impl PetitionShowlist {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, std::io::Error> {
        // index: u32
        let index = crate::util::read_u32_le(&mut r)?;

        // charter_entry: u32
        let charter_entry = crate::util::read_u32_le(&mut r)?;

        // charter_display_id: u32
        let charter_display_id = crate::util::read_u32_le(&mut r)?;

        // guild_charter_cost: u32
        let guild_charter_cost = crate::util::read_u32_le(&mut r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(&mut r)?;

        // signatures_required: u32
        let signatures_required = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            index,
            charter_entry,
            charter_display_id,
            guild_charter_cost,
            unknown1,
            signatures_required,
        })
    }

}

