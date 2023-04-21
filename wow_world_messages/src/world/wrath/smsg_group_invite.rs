use std::io::{Read, Write};

use crate::wrath::PlayerInviteStatus;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_group_invite.wowm:14`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_group_invite.wowm#L14):
/// ```text
/// smsg SMSG_GROUP_INVITE = 0x006F {
///     PlayerInviteStatus status;
///     CString name;
///     optional unknown {
///         u32 unknown1;
///         u8 count;
///         u32 unknown2;
///     }
/// }
/// ```
pub struct SMSG_GROUP_INVITE {
    pub status: PlayerInviteStatus,
    pub name: String,
    pub unknown: Option<SMSG_GROUP_INVITE_unknown>,
}

impl crate::private::Sealed for SMSG_GROUP_INVITE {}
impl crate::Message for SMSG_GROUP_INVITE {
    const OPCODE: u32 = 0x006f;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // status: PlayerInviteStatus
        w.write_all(&u8::from(self.status.as_int()).to_le_bytes())?;

        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().rev().next(), Some(&0_u8), "String `name` must not be null-terminated.");
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // optional unknown
        if let Some(v) = &self.unknown {
            // unknown1: u32
            w.write_all(&v.unknown1.to_le_bytes())?;

            // count: u8
            w.write_all(&v.count.to_le_bytes())?;

            // unknown2: u32
            w.write_all(&v.unknown2.to_le_bytes())?;

        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(2..=266).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x006F, size: body_size as u32 });
        }

        // status: PlayerInviteStatus
        let status: PlayerInviteStatus = crate::util::read_u8_le(&mut r)?.try_into()?;

        // name: CString
        let name = {
            let name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(name)?
        };

        // optional unknown
        let current_size = {
            1 // status: PlayerInviteStatus
            + name.len() + 1 // name: CString
        };
        let unknown = if current_size < body_size as usize {
            // unknown1: u32
            let unknown1 = crate::util::read_u32_le(&mut r)?;

            // count: u8
            let count = crate::util::read_u8_le(&mut r)?;

            // unknown2: u32
            let unknown2 = crate::util::read_u32_le(&mut r)?;

            Some(SMSG_GROUP_INVITE_unknown {
                unknown1,
                count,
                unknown2,
            })
        } else {
            None
        };

        Ok(Self {
            status,
            name,
            unknown,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_GROUP_INVITE {}

impl SMSG_GROUP_INVITE {
    pub(crate) fn size(&self) -> usize {
        1 // status: PlayerInviteStatus
        + self.name.len() + 1 // name: CString
        + if let Some(unknown) = &self.unknown {
            4 // unknown1: u32
            + 1 // count: u8
            + 4 // unknown2: u32
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_GROUP_INVITE_unknown {
    pub unknown1: u32,
    pub count: u8,
    pub unknown2: u32,
}

