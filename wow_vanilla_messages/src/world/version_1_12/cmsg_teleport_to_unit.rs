use std::convert::{TryFrom, TryInto};
use crate::ClientMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Sent when using the `port` console command.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_teleport_to_unit.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_teleport_to_unit.wowm#L3):
/// ```text
/// cmsg CMSG_TELEPORT_TO_UNIT = 0x0009 {
///     CString location;
/// }
/// ```
pub struct CMSG_TELEPORT_TO_UNIT {
    pub location: String,
}

impl ClientMessage for CMSG_TELEPORT_TO_UNIT {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // location: CString
        w.write_all(self.location.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    const OPCODE: u16 = 0x0009;

    fn client_size(&self) -> u16 {
        (self.size() + 6) as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // location: CString
        let location = crate::util::read_c_string_to_vec(r)?;
        let location = String::from_utf8(location)?;

        Ok(Self {
            location,
        })
    }

}

impl CMSG_TELEPORT_TO_UNIT {
    pub(crate) fn size(&self) -> usize {
        self.location.len() + 1 // location: CString
    }
}

