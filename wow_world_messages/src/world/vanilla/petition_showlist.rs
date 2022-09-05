use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Copy, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/smsg_petition_showlist.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_petition_showlist.wowm#L3):
/// ```text
/// struct PetitionShowlist {
///     u32 index;
///     u32 charter_entry = 5863;
///     u32 charter_display_id = 16161;
///     u32 guild_charter_cost;
///     u32 unknown1;
///     u32 unknown2;
/// }
/// ```
pub struct PetitionShowlist {
    pub index: u32,
    /// cmangos/vmangos/mangoszero: statically set to 1000 (10 silver).
    ///
    pub guild_charter_cost: u32,
    /// cmangos/vmangos/mangoszero: statically set to 1
    ///
    pub unknown1: u32,
    /// cmangos/vmangos/mangoszero: statically set to 9
    /// cmangos/vmangos/mangoszero: required signs?
    ///
    pub unknown2: u32,
}

impl PetitionShowlist {
    /// The field `charter_entry` is constantly specified to be:
    ///
    /// | Format | Value |
    /// | ------ | ----- |
    /// | Decimal | `5863` |
    /// | Hex | `0x16e7` |
    /// | Original | `5863` |
    ///
    /// **This field is not in the Rust struct, but is written as this constant value.**
    pub const CHARTER_ENTRY_VALUE: u32 = 0x16e7;

    /// The field `charter_display_id` is constantly specified to be:
    ///
    /// | Format | Value |
    /// | ------ | ----- |
    /// | Decimal | `16161` |
    /// | Hex | `0x3f21` |
    /// | Original | `16161` |
    ///
    /// **This field is not in the Rust struct, but is written as this constant value.**
    pub const CHARTER_DISPLAY_ID_VALUE: u32 = 0x3f21;

}

impl PetitionShowlist {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // index: u32
        w.write_all(&self.index.to_le_bytes())?;

        // charter_entry: u32
        w.write_all(&Self::CHARTER_ENTRY_VALUE.to_le_bytes())?;

        // charter_display_id: u32
        w.write_all(&Self::CHARTER_DISPLAY_ID_VALUE.to_le_bytes())?;

        // guild_charter_cost: u32
        w.write_all(&self.guild_charter_cost.to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: u32
        w.write_all(&self.unknown2.to_le_bytes())?;

        Ok(())
    }
}

impl PetitionShowlist {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // index: u32
        let index = crate::util::read_u32_le(r)?;

        // charter_entry: u32
        let _charter_entry = crate::util::read_u32_le(r)?;
        // charter_entry is expected to always be 5863 (5863)

        // charter_display_id: u32
        let _charter_display_id = crate::util::read_u32_le(r)?;
        // charter_display_id is expected to always be 16161 (16161)

        // guild_charter_cost: u32
        let guild_charter_cost = crate::util::read_u32_le(r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        // unknown2: u32
        let unknown2 = crate::util::read_u32_le(r)?;

        Ok(Self {
            index,
            guild_charter_cost,
            unknown1,
            unknown2,
        })
    }

}

