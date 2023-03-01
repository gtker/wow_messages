use crate:: {
    Guid,
};
use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_lfg_role_check_update.wowm:14`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_lfg_role_check_update.wowm#L14):
/// ```text
/// struct LfgRole {
///     Guid guid;
///     Bool ready;
///     u32 roles;
///     u8 level;
/// }
/// ```
pub struct LfgRole {
    pub guid: Guid,
    pub ready: bool,
    pub roles: u32,
    pub level: u8,
}

impl LfgRole {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // ready: Bool
        w.write_all(u8::from(self.ready).to_le_bytes().as_slice())?;

        // roles: u32
        w.write_all(&self.roles.to_le_bytes())?;

        // level: u8
        w.write_all(&self.level.to_le_bytes())?;

        Ok(())
    }
}

impl LfgRole {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> std::result::Result<Self, std::io::Error> {
        // guid: Guid
        let guid = Guid::read(&mut r)?;

        // ready: Bool
        let ready = crate::util::read_u8_le(&mut r)? != 0;

        // roles: u32
        let roles = crate::util::read_u32_le(&mut r)?;

        // level: u8
        let level = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            guid,
            ready,
            roles,
            level,
        })
    }

}

