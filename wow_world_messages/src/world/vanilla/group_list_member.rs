use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_group_list.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_group_list.wowm#L8):
/// ```text
/// struct GroupListMember {
///     CString name;
///     Guid guid;
///     Bool is_online;
///     u8 flags;
/// }
/// ```
pub struct GroupListMember {
    pub name: String,
    pub guid: Guid,
    pub is_online: bool,
    /// mangoszero/cmangos/vmangos: own flags (groupid | (assistant?0x80:0))
    pub flags: u8,
}

impl GroupListMember {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
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

        // flags: u8
        w.write_all(&self.flags.to_le_bytes())?;

        Ok(())
    }
}

impl GroupListMember {
    pub(crate) fn read<R: Read>(mut r: R) -> Result<Self, crate::errors::ParseError> {
        // name: CString
        let name = {
            let name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(name)?
        };

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // is_online: Bool
        let is_online = crate::util::read_u8_le(&mut r)? != 0;

        // flags: u8
        let flags = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            name,
            guid,
            is_online,
            flags,
        })
    }

}

impl GroupListMember {
    pub(crate) fn size(&self) -> usize {
        self.name.len() + 1 // name: CString
        + 8 // guid: Guid
        + 1 // is_online: Bool
        + 1 // flags: u8
    }
}

