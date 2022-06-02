use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/cmsg_guild_add_rank.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/cmsg_guild_add_rank.wowm#L3):
/// ```text
/// cmsg CMSG_GUILD_ADD_RANK = 0x0232 {
///     CString rank_name;
/// }
/// ```
pub struct CMSG_GUILD_ADD_RANK {
    pub rank_name: String,
}

impl ClientMessage for CMSG_GUILD_ADD_RANK {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // rank_name: CString
        w.write_all(self.rank_name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    const OPCODE: u16 = 0x0232;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        self.size() as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // rank_name: CString
        let rank_name = crate::util::read_c_string_to_vec(r)?;
        let rank_name = String::from_utf8(rank_name)?;

        Ok(Self {
            rank_name,
        })
    }

}

impl CMSG_GUILD_ADD_RANK {
    pub(crate) fn size(&self) -> usize {
        self.rank_name.len() + 1 // rank_name: CString
    }
}

