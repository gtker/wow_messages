use std::io::{Read, Write};

use crate::tbc::LfgType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/msg_looking_for_group.wowm:13`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/msg_looking_for_group.wowm#L13):
/// ```text
/// cmsg MSG_LOOKING_FOR_GROUP_Client = 0x01FF {
///     (u32)LfgType lfg_type;
///     u32 entry;
///     u32 unknown;
/// }
/// ```
pub struct MSG_LOOKING_FOR_GROUP_Client {
    pub lfg_type: LfgType,
    /// entry from LfgDunggeons.dbc
    ///
    pub entry: u32,
    pub unknown: u32,
}

impl crate::private::Sealed for MSG_LOOKING_FOR_GROUP_Client {}
impl crate::Message for MSG_LOOKING_FOR_GROUP_Client {
    const OPCODE: u32 = 0x01ff;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // lfg_type: LfgType
        w.write_all(&u32::from(self.lfg_type.as_int()).to_le_bytes())?;

        // entry: u32
        w.write_all(&self.entry.to_le_bytes())?;

        // unknown: u32
        w.write_all(&self.unknown.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01FF, size: body_size });
        }

        // lfg_type: LfgType
        let lfg_type: LfgType = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        // entry: u32
        let entry = crate::util::read_u32_le(&mut r)?;

        // unknown: u32
        let unknown = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            lfg_type,
            entry,
            unknown,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for MSG_LOOKING_FOR_GROUP_Client {}

