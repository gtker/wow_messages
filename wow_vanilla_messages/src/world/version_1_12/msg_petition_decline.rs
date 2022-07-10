use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::{ClientMessage, ServerMessage};
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/msg_petition_decline.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/msg_petition_decline.wowm#L3):
/// ```text
/// msg MSG_PETITION_DECLINE = 0x01C2 {
///     Guid petition;
/// }
/// ```
pub struct MSG_PETITION_DECLINE {
    pub petition: Guid,
}

impl ClientMessage for MSG_PETITION_DECLINE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // petition: Guid
        w.write_all(&self.petition.guid().to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01c2;

    fn client_size(&self) -> u16 {
        14
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // petition: Guid
        let petition = Guid::read(r)?;

        Ok(Self {
            petition,
        })
    }

}

impl ServerMessage for MSG_PETITION_DECLINE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // petition: Guid
        w.write_all(&self.petition.guid().to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01c2;

    fn server_size(&self) -> u16 {
        12
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // petition: Guid
        let petition = Guid::read(r)?;

        Ok(Self {
            petition,
        })
    }

}

