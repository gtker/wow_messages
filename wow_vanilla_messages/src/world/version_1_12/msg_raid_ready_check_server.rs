use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/msg_raid_ready_check_server.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/msg_raid_ready_check_server.wowm#L3):
/// ```text
/// smsg MSG_RAID_READY_CHECK_Server = 0x0322 {
///     optional state_check {
///         Guid guid;
///         u8 state;
///     }
/// }
/// ```
pub struct MSG_RAID_READY_CHECK_Server {
    pub state_check: Option<MSG_RAID_READY_CHECK_Server_state_check>,
}

impl ServerMessage for MSG_RAID_READY_CHECK_Server {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // optional state_check
        if let Some(v) = &self.state_check {
            // guid: Guid
            w.write_all(&v.guid.guid().to_le_bytes())?;

            // state: u8
            w.write_all(&v.state.to_le_bytes())?;

        }

        Ok(())
    }
    const OPCODE: u16 = 0x0322;

    fn server_size(&self) -> u16 {
        (self.size() + 4) as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // optional state_check
        let current_size = {
            0
        };
        let state_check = if current_size < body_size as usize {
            // guid: Guid
            let guid = Guid::read(r)?;

            // state: u8
            let state = crate::util::read_u8_le(r)?;

            Some(MSG_RAID_READY_CHECK_Server_state_check {
                guid,
                state,
            })
        } else {
            None
        };

        Ok(Self {
            state_check,
        })
    }

}

impl MSG_RAID_READY_CHECK_Server {
    pub(crate) fn size(&self) -> usize {
        if let Some(state_check) = &self.state_check {
            8 // guid: Guid
            + 1 // state: u8
        } else {
            0
        }
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MSG_RAID_READY_CHECK_Server_state_check {
    pub guid: Guid,
    pub state: u8,
}

impl MSG_RAID_READY_CHECK_Server_state_check {
    pub(crate) fn size(&self) -> usize {
        8 // guid: Guid
        + 1 // state: u8
    }

}

