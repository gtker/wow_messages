use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/cmsg_petition_show_signatures.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/cmsg_petition_show_signatures.wowm#L3):
/// ```text
/// cmsg CMSG_PETITION_SHOW_SIGNATURES = 0x01BE {
///     Guid item_guid;
/// }
/// ```
pub struct CMSG_PETITION_SHOW_SIGNATURES {
    pub item_guid: Guid,
}

impl ClientMessage for CMSG_PETITION_SHOW_SIGNATURES {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // item_guid: Guid
        w.write_all(&self.item_guid.guid().to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01be;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        8
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // item_guid: Guid
        let item_guid = Guid::read(r)?;

        Ok(Self {
            item_guid,
        })
    }

}

