use std::convert::{TryFrom, TryInto};
use crate::shared::faction_flag_vanilla_tbc::FactionFlag;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/faction/smsg_initialize_factions.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/faction/smsg_initialize_factions.wowm#L1):
/// ```text
/// struct FactionInitializer {
///     FactionFlag flag;
///     u32 standing;
/// }
/// ```
pub struct FactionInitializer {
    pub flag: FactionFlag,
    pub standing: u32,
}

impl FactionInitializer {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // flag: FactionFlag
        w.write_all(&(self.flag.as_int() as u8).to_le_bytes())?;

        // standing: u32
        w.write_all(&self.standing.to_le_bytes())?;

        Ok(())
    }
}

impl FactionInitializer {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, std::io::Error> {
        // flag: FactionFlag
        let flag = FactionFlag::new(crate::util::read_u8_le(r)?);

        // standing: u32
        let standing = crate::util::read_u32_le(r)?;

        Ok(Self {
            flag,
            standing,
        })
    }

}

