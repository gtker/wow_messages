use std::io::{Read, Write};

use crate::wrath::FactionFlag;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/faction/smsg_initialize_factions.wowm:19`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/faction/smsg_initialize_factions.wowm#L19):
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
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // flag: FactionFlag
        w.write_all(&(self.flag.as_int().to_le_bytes()))?;

        // standing: u32
        w.write_all(&self.standing.to_le_bytes())?;

        Ok(())
    }
}

impl FactionInitializer {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> std::result::Result<Self, std::io::Error> {
        // flag: FactionFlag
        let flag = FactionFlag::new(crate::util::read_u8_le(&mut r)?);

        // standing: u32
        let standing = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            flag,
            standing,
        })
    }

}

