use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/cmsg_guild_add_rank.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/cmsg_guild_add_rank.wowm#L3):
/// ```text
/// cmsg CMSG_GUILD_ADD_RANK = 0x0232 {
///     CString rank_name;
/// }
/// ```
pub struct CMSG_GUILD_ADD_RANK {
    pub rank_name: String,
}

impl crate::Message for CMSG_GUILD_ADD_RANK {
    const OPCODE: u32 = 0x0232;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // rank_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.rank_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `rank_name` must not be null-terminated.");
        w.write_all(self.rank_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(1..=256).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0232, size: body_size as u32 });
        }

        // rank_name: CString
        let rank_name = {
            let rank_name = crate::util::read_c_string_to_vec(r)?;
            String::from_utf8(rank_name)?
        };

        Ok(Self {
            rank_name,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_GUILD_ADD_RANK {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_GUILD_ADD_RANK {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_GUILD_ADD_RANK {}

impl CMSG_GUILD_ADD_RANK {
    pub(crate) fn size(&self) -> usize {
        self.rank_name.len() + 1 // rank_name: CString
    }
}

