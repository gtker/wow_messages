use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::TextEmote;

/// Sent to notify the server that the client wants to perform an emote like /dance or /cry.
/// Server responds with [`SMSG_TEXT_EMOTE`](crate::vanilla::SMSG_TEXT_EMOTE) and [`SMSG_EMOTE`](crate::vanilla::SMSG_EMOTE).
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/cmsg_text_emote.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/cmsg_text_emote.wowm#L3):
/// ```text
/// cmsg CMSG_TEXT_EMOTE = 0x0104 {
///     TextEmote text_emote;
///     u32 emote;
///     Guid target;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_TEXT_EMOTE {
    pub text_emote: TextEmote,
    pub emote: u32,
    /// Guid targeted by the client.
    pub target: Guid,
}

impl crate::private::Sealed for CMSG_TEXT_EMOTE {}
impl CMSG_TEXT_EMOTE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 16 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // text_emote: TextEmote
        let text_emote = crate::util::read_u32_le(&mut r)?.try_into()?;

        // emote: u32
        let emote = crate::util::read_u32_le(&mut r)?;

        // target: Guid
        let target = crate::util::read_guid(&mut r)?;

        Ok(Self {
            text_emote,
            emote,
            target,
        })
    }

}

impl crate::Message for CMSG_TEXT_EMOTE {
    const OPCODE: u32 = 0x0104;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_TEXT_EMOTE"
    }

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // text_emote: TextEmote
        w.write_all(&(self.text_emote.as_int().to_le_bytes()))?;

        // emote: u32
        w.write_all(&self.emote.to_le_bytes())?;

        // target: Guid
        w.write_all(&self.target.guid().to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(260, "CMSG_TEXT_EMOTE", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_TEXT_EMOTE {}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_TEXT_EMOTE;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ClientOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    fn assert(t: &CMSG_TEXT_EMOTE, expected: &CMSG_TEXT_EMOTE) {
        assert_eq!(t.text_emote, expected.text_emote);
        assert_eq!(t.emote, expected.emote);
        assert_eq!(t.target, expected.target);
    }

    const RAW0: [u8; 22] = [ 0x00, 0x14, 0x04, 0x01, 0x00, 0x00, 0x22, 0x00, 0x00,
         0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
         0x00, ];

    pub(crate) fn expected0() -> CMSG_TEXT_EMOTE {
        CMSG_TEXT_EMOTE {
            text_emote: TextEmote::Dance,
            emote: 0xFFFFFFFF,
            target: Guid::new(0x0),
        }

    }

    // Generated from `wow_message_parser/wowm/world/chat/cmsg_text_emote.wowm` line 12.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_text_emote0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_TEXT_EMOTE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_TEXT_EMOTE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(16 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/chat/cmsg_text_emote.wowm` line 12.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_text_emote0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_TEXT_EMOTE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_TEXT_EMOTE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(16 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/chat/cmsg_text_emote.wowm` line 12.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_text_emote0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_TEXT_EMOTE(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_TEXT_EMOTE, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(16 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

