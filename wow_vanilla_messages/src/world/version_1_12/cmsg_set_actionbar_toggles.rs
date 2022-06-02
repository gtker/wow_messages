use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
#[cfg(feature = "tokio")]
use tokio::io::AsyncReadExt;
#[cfg(feature = "async-std")]
use async_std::io::ReadExt;
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/login_logout/cmsg_set_actionbar_toggles.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/login_logout/cmsg_set_actionbar_toggles.wowm#L3):
/// ```text
/// cmsg CMSG_SET_ACTIONBAR_TOGGLES = 0x02BF {
///     u8 action_bar;
/// }
/// ```
pub struct CMSG_SET_ACTIONBAR_TOGGLES {
    pub action_bar: u8,
}

impl ClientMessage for CMSG_SET_ACTIONBAR_TOGGLES {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // action_bar: u8
        w.write_all(&self.action_bar.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x02bf;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        1
    }

    fn read_body<R: std::io::Read>(r: &mut R, body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // action_bar: u8
        let action_bar = crate::util::read_u8_le(r)?;

        Ok(Self {
            action_bar,
        })
    }

}

