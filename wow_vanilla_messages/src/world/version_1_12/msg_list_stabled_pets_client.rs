use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/pet/msg_list_stabled_pets_client.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/pet/msg_list_stabled_pets_client.wowm#L3):
/// ```text
/// cmsg MSG_LIST_STABLED_PETS_Client = 0x026F {
///     Guid npc;
/// }
/// ```
pub struct MSG_LIST_STABLED_PETS_Client {
    pub npc: Guid,
}

impl ClientMessage for MSG_LIST_STABLED_PETS_Client {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // npc: Guid
        w.write_all(&self.npc.guid().to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x026f;

    fn client_size(&self) -> u16 {
        14
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // npc: Guid
        let npc = Guid::read(r)?;

        Ok(Self {
            npc,
        })
    }

}

