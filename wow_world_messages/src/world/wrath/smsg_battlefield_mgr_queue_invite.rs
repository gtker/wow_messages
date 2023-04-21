use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/smsg_battlefield_mgr_queue_invite.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/smsg_battlefield_mgr_queue_invite.wowm#L1):
/// ```text
/// smsg SMSG_BATTLEFIELD_MGR_QUEUE_INVITE = 0x04E1 {
///     u32 battle_id;
///     u8 warmup;
/// }
/// ```
pub struct SMSG_BATTLEFIELD_MGR_QUEUE_INVITE {
    pub battle_id: u32,
    /// Possibly not used.
    ///
    pub warmup: u8,
}

impl crate::private::Sealed for SMSG_BATTLEFIELD_MGR_QUEUE_INVITE {}
impl crate::Message for SMSG_BATTLEFIELD_MGR_QUEUE_INVITE {
    const OPCODE: u32 = 0x04e1;

    fn size_without_header(&self) -> u32 {
        5
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // battle_id: u32
        w.write_all(&self.battle_id.to_le_bytes())?;

        // warmup: u8
        w.write_all(&self.warmup.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 5 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04E1, size: body_size as u32 });
        }

        // battle_id: u32
        let battle_id = crate::util::read_u32_le(&mut r)?;

        // warmup: u8
        let warmup = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            battle_id,
            warmup,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_BATTLEFIELD_MGR_QUEUE_INVITE {}

