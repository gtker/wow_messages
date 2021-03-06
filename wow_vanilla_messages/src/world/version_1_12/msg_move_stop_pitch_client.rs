use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::MovementInfo;
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_stop_pitch.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_stop_pitch.wowm#L3):
/// ```text
/// cmsg MSG_MOVE_STOP_PITCH_Client = 0x00C1 {
///     MovementInfo info;
/// }
/// ```
pub struct MSG_MOVE_STOP_PITCH_Client {
    pub info: MovementInfo,
}

impl ClientMessage for MSG_MOVE_STOP_PITCH_Client {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // info: MovementInfo
        self.info.write_into_vec(w)?;

        Ok(())
    }
    const OPCODE: u16 = 0x00c1;

    fn client_size(&self) -> u16 {
        (self.size() + 6) as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // info: MovementInfo
        let info = MovementInfo::read(r)?;

        Ok(Self {
            info,
        })
    }

}

impl MSG_MOVE_STOP_PITCH_Client {
    pub(crate) fn size(&self) -> usize {
        self.info.size() // info: MovementInfo
    }
}

