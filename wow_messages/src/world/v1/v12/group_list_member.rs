use std::convert::{TryFrom, TryInto};
use crate::world::helper::Guid;
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_group_list.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_group_list.wowm#L8):
/// ```text
/// struct GroupListMember {
///     CString name;
///     Guid guid;
///     u8 is_online;
/// }
/// ```
pub struct GroupListMember {
    pub name: String,
    pub guid: Guid,
    pub is_online: u8,
}

impl ReadableAndWritable for GroupListMember {
    type Error = GroupListMemberError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        // guid: Guid
        let guid = Guid::read(r)?;

        // is_online: u8
        let is_online = crate::util::read_u8_le(r)?;

        Ok(Self {
            name,
            guid,
            is_online,
        })
    }

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // guid: Guid
        self.guid.write(w)?;

        // is_online: u8
        w.write_all(&self.is_online.to_le_bytes())?;

        Ok(())
    }

}

impl VariableSized for GroupListMember {
    fn size(&self) -> usize {
        self.name.len() + 1 // name: CString and Null Terminator
        + 8 // guid: Guid
        + 1 // is_online: u8
    }
}

impl MaximumPossibleSized for GroupListMember {
    fn maximum_possible_size() -> usize {
        256 // name: CString
        + 8 // guid: Guid
        + 1 // is_online: u8
    }
}

#[derive(Debug)]
pub enum GroupListMemberError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
}

impl std::error::Error for GroupListMemberError {}
impl std::fmt::Display for GroupListMemberError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for GroupListMemberError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for GroupListMemberError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

