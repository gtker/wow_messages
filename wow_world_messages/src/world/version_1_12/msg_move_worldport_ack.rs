use std::convert::{TryFrom, TryInto};
use crate::{ClientMessage, ServerMessage};
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Acknowledge from the client that it has received an [`SMSG_NEW_WORLD`](crate::world::version_1_12::SMSG_NEW_WORLD) and has loaded the new map.
///
/// Despite the name this seems to only be sent by the client.
/// The server should reply with what it normally does to log players into the world.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_worldport_ack.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_worldport_ack.wowm#L3):
/// ```text
/// msg MSG_MOVE_WORLDPORT_ACK = 0x00DC {
/// }
/// ```
pub struct MSG_MOVE_WORLDPORT_ACK {
}

impl ClientMessage for MSG_MOVE_WORLDPORT_ACK {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        Ok(())
    }
    const OPCODE: u16 = 0x00dc;

    fn client_size(&self) -> u16 {
        6
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        Ok(Self {
        })
    }

}

impl ServerMessage for MSG_MOVE_WORLDPORT_ACK {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        Ok(())
    }
    const OPCODE: u16 = 0x00dc;

    fn server_size(&self) -> u16 {
        4
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        Ok(Self {
        })
    }

}

