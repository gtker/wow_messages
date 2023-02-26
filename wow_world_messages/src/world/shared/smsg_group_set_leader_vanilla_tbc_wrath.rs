use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_group_set_leader.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_group_set_leader.wowm#L3):
/// ```text
/// smsg SMSG_GROUP_SET_LEADER = 0x0079 {
///     CString name;
/// }
/// ```
pub struct SMSG_GROUP_SET_LEADER {
    pub name: String,
}

impl crate::Message for SMSG_GROUP_SET_LEADER {
    const OPCODE: u32 = 0x0079;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().rev().next(), Some(&0_u8), "String `name` must not be null-terminated.");
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(1..=256).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0079, size: body_size as u32 });
        }

        // name: CString
        let name = {
            let name = crate::util::read_c_string_to_vec(r)?;
            String::from_utf8(name)?
        };

        Ok(Self {
            name,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_GROUP_SET_LEADER {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_GROUP_SET_LEADER {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_GROUP_SET_LEADER {}

impl SMSG_GROUP_SET_LEADER {
    pub(crate) fn size(&self) -> usize {
        self.name.len() + 1 // name: CString
    }
}

