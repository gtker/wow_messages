use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// cmangos/vmangos/mangoszero returns guid 0 and unknown 0 when talents can not be reset
/// cmangos/vmangos/mangoszero casts spell 14876 when resetting
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/msg_talent_wipe_confirm_server.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/msg_talent_wipe_confirm_server.wowm#L3):
/// ```text
/// smsg MSG_TALENT_WIPE_CONFIRM_Server = 0x02AA {
///     Guid wiping_npc;
///     u32 cost_in_copper;
/// }
/// ```
pub struct MSG_TALENT_WIPE_CONFIRM_Server {
    pub wiping_npc: Guid,
    pub cost_in_copper: u32,
}

impl crate::private::Sealed for MSG_TALENT_WIPE_CONFIRM_Server {}
impl crate::Message for MSG_TALENT_WIPE_CONFIRM_Server {
    const OPCODE: u32 = 0x02aa;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // wiping_npc: Guid
        w.write_all(&self.wiping_npc.guid().to_le_bytes())?;

        // cost_in_copper: u32
        w.write_all(&self.cost_in_copper.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02AA, size: body_size });
        }

        // wiping_npc: Guid
        let wiping_npc = crate::util::read_guid(&mut r)?;

        // cost_in_copper: u32
        let cost_in_copper = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            wiping_npc,
            cost_in_copper,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for MSG_TALENT_WIPE_CONFIRM_Server {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for MSG_TALENT_WIPE_CONFIRM_Server {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_TALENT_WIPE_CONFIRM_Server {}

