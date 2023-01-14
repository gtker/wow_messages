use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/_need_sorting/smsg_battlefield_mgr_ejected.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/_need_sorting/smsg_battlefield_mgr_ejected.wowm#L1):
/// ```text
/// smsg SMSG_BATTLEFIELD_MGR_EJECTED = 0x04E6 {
///     u32 battle_id;
///     u8 reason;
///     u8 battle_status;
///     u8 relocated;
/// }
/// ```
pub struct SMSG_BATTLEFIELD_MGR_EJECTED {
    pub battle_id: u32,
    pub reason: u8,
    pub battle_status: u8,
    pub relocated: u8,
}

impl crate::Message for SMSG_BATTLEFIELD_MGR_EJECTED {
    const OPCODE: u32 = 0x04e6;

    fn size_without_header(&self) -> u32 {
        7
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // battle_id: u32
        w.write_all(&self.battle_id.to_le_bytes())?;

        // reason: u8
        w.write_all(&self.reason.to_le_bytes())?;

        // battle_status: u8
        w.write_all(&self.battle_status.to_le_bytes())?;

        // relocated: u8
        w.write_all(&self.relocated.to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 7 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04E6, size: body_size as u32 });
        }

        // battle_id: u32
        let battle_id = crate::util::read_u32_le(r)?;

        // reason: u8
        let reason = crate::util::read_u8_le(r)?;

        // battle_status: u8
        let battle_status = crate::util::read_u8_le(r)?;

        // relocated: u8
        let relocated = crate::util::read_u8_le(r)?;

        Ok(Self {
            battle_id,
            reason,
            battle_status,
            relocated,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::world::wrath::ServerMessage for SMSG_BATTLEFIELD_MGR_EJECTED {}

