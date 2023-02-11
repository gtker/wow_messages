use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_group_list.wowm:73`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_group_list.wowm#L73):
/// ```text
/// struct GroupListMember {
///     CString name;
///     Guid guid;
///     Bool is_online;
///     u8 group_id;
///     u8 flags;
///     u8 lfg_roles;
/// }
/// ```
pub struct GroupListMember {
    pub name: String,
    pub guid: Guid,
    pub is_online: bool,
    pub group_id: u8,
    /// mangosone: 0x2 main assist, 0x4 main tank
    ///
    pub flags: u8,
    pub lfg_roles: u8,
}

impl GroupListMember {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().rev().next(), Some(&0_u8), "String `name` must not be null-terminated.");
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // is_online: Bool
        w.write_all(u8::from(self.is_online).to_le_bytes().as_slice())?;

        // group_id: u8
        w.write_all(&self.group_id.to_le_bytes())?;

        // flags: u8
        w.write_all(&self.flags.to_le_bytes())?;

        // lfg_roles: u8
        w.write_all(&self.lfg_roles.to_le_bytes())?;

        Ok(())
    }
}

impl GroupListMember {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        // guid: Guid
        let guid = Guid::read(r)?;

        // is_online: Bool
        let is_online = crate::util::read_u8_le(r)? != 0;
        // group_id: u8
        let group_id = crate::util::read_u8_le(r)?;

        // flags: u8
        let flags = crate::util::read_u8_le(r)?;

        // lfg_roles: u8
        let lfg_roles = crate::util::read_u8_le(r)?;

        Ok(Self {
            name,
            guid,
            is_online,
            group_id,
            flags,
            lfg_roles,
        })
    }

}

impl GroupListMember {
    pub(crate) fn size(&self) -> usize {
        self.name.len() + 1 // name: CString
        + 8 // guid: Guid
        + 1 // is_online: Bool
        + 1 // group_id: u8
        + 1 // flags: u8
        + 1 // lfg_roles: u8
    }
}

