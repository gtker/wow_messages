use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/msg_talent_wipe_confirm_client.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/msg_talent_wipe_confirm_client.wowm#L3):
/// ```text
/// cmsg MSG_TALENT_WIPE_CONFIRM_Client = 0x02AA {
///     Guid wiping_npc;
/// }
/// ```
pub struct MSG_TALENT_WIPE_CONFIRM_Client {
    pub wiping_npc: Guid,
}

impl ClientMessage for MSG_TALENT_WIPE_CONFIRM_Client {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // wiping_npc: Guid
        w.write_all(&self.wiping_npc.guid().to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x02aa;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        8
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // wiping_npc: Guid
        let wiping_npc = Guid::read(r)?;

        Ok(Self {
            wiping_npc,
        })
    }

}

