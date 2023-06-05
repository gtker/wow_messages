use std::io::{Read, Write};

use crate::tbc::LfgType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_lfg_update.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_lfg_update.wowm#L8):
/// ```text
/// struct LfgData {
///     u16 entry;
///     (u16)LfgType lfg_type;
/// }
/// ```
pub struct LfgData {
    pub entry: u16,
    pub lfg_type: LfgType,
}

impl LfgData {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // entry: u16
        w.write_all(&self.entry.to_le_bytes())?;

        // lfg_type: LfgType
        w.write_all(&u16::from(self.lfg_type.as_int()).to_le_bytes())?;

        Ok(())
    }
}

impl LfgData {
    pub(crate) fn read<R: Read>(mut r: R) -> Result<Self, crate::errors::ParseError> {
        // entry: u16
        let entry = crate::util::read_u16_le(&mut r)?;

        // lfg_type: LfgType
        let lfg_type = (crate::util::read_u16_le(&mut r)? as u8).try_into()?;

        Ok(Self {
            entry,
            lfg_type,
        })
    }

}

