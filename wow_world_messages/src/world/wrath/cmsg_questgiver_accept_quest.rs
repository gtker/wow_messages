use std::io::{Read, Write};

use crate::Guid;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/cmsg_questgiver_accept_quest.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/cmsg_questgiver_accept_quest.wowm#L8):
/// ```text
/// cmsg CMSG_QUESTGIVER_ACCEPT_QUEST = 0x0189 {
///     Guid guid;
///     u32 quest_id;
///     u32 unk1;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_QUESTGIVER_ACCEPT_QUEST {
    pub guid: Guid,
    pub quest_id: u32,
    pub unk1: u32,
}

impl crate::private::Sealed for CMSG_QUESTGIVER_ACCEPT_QUEST {}
impl CMSG_QUESTGIVER_ACCEPT_QUEST {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 16 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        // quest_id: u32
        let quest_id = crate::util::read_u32_le(&mut r)?;

        // unk1: u32
        let unk1 = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            guid,
            quest_id,
            unk1,
        })
    }

}

impl crate::Message for CMSG_QUESTGIVER_ACCEPT_QUEST {
    const OPCODE: u32 = 0x0189;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_QUESTGIVER_ACCEPT_QUEST"
    }

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        // quest_id: u32
        w.write_all(&self.quest_id.to_le_bytes())?;

        // unk1: u32
        w.write_all(&self.unk1.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(393, "CMSG_QUESTGIVER_ACCEPT_QUEST", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_QUESTGIVER_ACCEPT_QUEST {}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_QUESTGIVER_ACCEPT_QUEST;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ClientOpcodeMessage;
    use crate::Guid;
    use crate::wrath::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    const RAW0: [u8; 22] = [ 0x00, 0x14, 0x89, 0x01, 0x00, 0x00, 0x88, 0x77, 0x66,
         0x55, 0x44, 0x33, 0x22, 0x11, 0x78, 0x56, 0x34, 0x12, 0xBE, 0xBA, 0xFE,
         0xCA, ];

    pub(crate) fn expected0() -> CMSG_QUESTGIVER_ACCEPT_QUEST {
        CMSG_QUESTGIVER_ACCEPT_QUEST {
            guid: Guid::new(0x1122334455667788),
            quest_id: 0x12345678,
            unk1: 0xCAFEBABE,
        }

    }

    // Generated from `wow_message_parser/wowm/world/quest/cmsg_questgiver_accept_quest.wowm` line 16.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_questgiver_accept_quest0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_QUESTGIVER_ACCEPT_QUEST(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_QUESTGIVER_ACCEPT_QUEST, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(16 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/quest/cmsg_questgiver_accept_quest.wowm` line 16.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_questgiver_accept_quest0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_QUESTGIVER_ACCEPT_QUEST(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_QUESTGIVER_ACCEPT_QUEST, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(16 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/quest/cmsg_questgiver_accept_quest.wowm` line 16.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_questgiver_accept_quest0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_QUESTGIVER_ACCEPT_QUEST(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_QUESTGIVER_ACCEPT_QUEST, got {opcode:#?}"),
        };

        assert_eq!(t.as_ref(), &expected);
        assert_eq!(16 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

