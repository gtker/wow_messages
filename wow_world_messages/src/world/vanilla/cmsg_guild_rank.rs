use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/cmsg_guild_rank.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/cmsg_guild_rank.wowm#L1):
/// ```text
/// cmsg CMSG_GUILD_RANK = 0x0231 {
///     u32 rank_id;
///     u32 rights;
///     CString rank_name;
/// }
/// ```
pub struct CMSG_GUILD_RANK {
    pub rank_id: u32,
    pub rights: u32,
    pub rank_name: String,
}

impl crate::private::Sealed for CMSG_GUILD_RANK {}
impl crate::Message for CMSG_GUILD_RANK {
    const OPCODE: u32 = 0x0231;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // rank_id: u32
        w.write_all(&self.rank_id.to_le_bytes())?;

        // rights: u32
        w.write_all(&self.rights.to_le_bytes())?;

        // rank_name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.rank_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `rank_name` must not be null-terminated.");
        w.write_all(self.rank_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(9..=264).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0231, size: body_size as u32 });
        }

        // rank_id: u32
        let rank_id = crate::util::read_u32_le(&mut r)?;

        // rights: u32
        let rights = crate::util::read_u32_le(&mut r)?;

        // rank_name: CString
        let rank_name = {
            let rank_name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(rank_name)?
        };

        Ok(Self {
            rank_id,
            rights,
            rank_name,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_GUILD_RANK {}

impl CMSG_GUILD_RANK {
    pub(crate) fn size(&self) -> usize {
        4 // rank_id: u32
        + 4 // rights: u32
        + self.rank_name.len() + 1 // rank_name: CString
    }
}

