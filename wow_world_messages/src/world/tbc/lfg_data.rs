use std::convert::{TryFrom, TryInto};
use crate::world::tbc::LfgType;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_lfg_update.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_lfg_update.wowm#L8):
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
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // entry: u16
        w.write_all(&self.entry.to_le_bytes())?;

        // lfg_type: LfgType
        w.write_all(&(self.lfg_type.as_int() as u16).to_le_bytes())?;

        Ok(())
    }
}

impl LfgData {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // entry: u16
        let entry = crate::util::read_u16_le(r)?;

        // lfg_type: LfgType
        let lfg_type: LfgType = (crate::util::read_u16_le(r)? as u8).try_into()?;

        Ok(Self {
            entry,
            lfg_type,
        })
    }

}

