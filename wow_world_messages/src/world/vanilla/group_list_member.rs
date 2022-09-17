use std::convert::{TryFrom, TryInto};
use crate::Guid;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_group_list.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_group_list.wowm#L8):
/// ```text
/// struct GroupListMember {
///     CString name;
///     Guid guid;
///     Bool is_online;
/// }
/// ```
pub struct GroupListMember {
    pub name: String,
    pub guid: Guid,
    pub is_online: bool,
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
        w.write_all(if self.is_online { &[1] } else { &[0] })?;

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
        Ok(Self {
            name,
            guid,
            is_online,
        })
    }

}

impl GroupListMember {
    pub(crate) fn size(&self) -> usize {
        self.name.len() + 1 // name: CString
        + 8 // guid: Guid
        + 1 // is_online: Bool
    }
}

