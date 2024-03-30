use std::io::{Read, Write};

/// Acknowledge from the client that it has received an [`SMSG_NEW_WORLD`](crate::vanilla::SMSG_NEW_WORLD) and has loaded the new map.
/// Despite the name this seems to only be sent by the client.
/// The server should reply with what it normally does to log players into the world.
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/msg/msg_move_worldport_ack.wowm:6`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/msg/msg_move_worldport_ack.wowm#L6):
/// ```text
/// msg MSG_MOVE_WORLDPORT_ACK = 0x00DC {
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct MSG_MOVE_WORLDPORT_ACK {
}

impl crate::private::Sealed for MSG_MOVE_WORLDPORT_ACK {}
impl MSG_MOVE_WORLDPORT_ACK {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 0 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        Ok(Self {
        })
    }

}

impl crate::Message for MSG_MOVE_WORLDPORT_ACK {
    const OPCODE: u32 = 0x00dc;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "MSG_MOVE_WORLDPORT_ACK"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        panic!("MSG types not supported");
    }

    fn size_without_header(&self) -> u32 {
        0
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(220, "MSG_MOVE_WORLDPORT_ACK", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for MSG_MOVE_WORLDPORT_ACK {}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for MSG_MOVE_WORLDPORT_ACK {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for MSG_MOVE_WORLDPORT_ACK {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for MSG_MOVE_WORLDPORT_ACK {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for MSG_MOVE_WORLDPORT_ACK {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_MOVE_WORLDPORT_ACK {}

