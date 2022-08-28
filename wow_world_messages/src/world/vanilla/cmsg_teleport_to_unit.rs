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
///     CString name;
/// }
/// ```
pub struct CMSG_TELEPORT_TO_UNIT {
    pub name: String,
}

impl ClientMessage for CMSG_TELEPORT_TO_UNIT {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // name: CString
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    const OPCODE: u16 = 0x0009;

    fn client_size(&self) -> u16 {
        (self.size() + 6) as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // name: CString
        let name = crate::util::read_c_string_to_vec(r)?;
        let name = String::from_utf8(name)?;

        Ok(Self {
            name,
        })
    }

}

impl CMSG_TELEPORT_TO_UNIT {
    pub(crate) fn size(&self) -> usize {
        self.name.len() + 1 // name: CString
    }
}

#[cfg(test)]
mod test {
    use super::CMSG_TELEPORT_TO_UNIT;
    use super::*;
    use super::super::*;
    use crate::world::vanilla::opcodes::ClientOpcodeMessage;
    use crate::{ClientMessage, ServerMessage};

    const RAW0: [u8; 13] = [ 0x00, 0x0B, 0x09, 0x00, 0x00, 0x00, 0x56, 0x75, 0x72,
         0x74, 0x6E, 0x65, 0x00, ];

    // Generated from `wow_message_parser/wowm/world/movement/cmsg/cmsg_teleport_to_unit.wowm` line 9.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMSG_TELEPORT_TO_UNIT0() {
        let expected = CMSG_TELEPORT_TO_UNIT {
            name: String::from("Vurtne"),
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_TELEPORT_TO_UNIT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_TELEPORT_TO_UNIT, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.name, expected.name);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/cmsg/cmsg_teleport_to_unit.wowm` line 9.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMSG_TELEPORT_TO_UNIT0() {
        let expected = CMSG_TELEPORT_TO_UNIT {
            name: String::from("Vurtne"),
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_TELEPORT_TO_UNIT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_TELEPORT_TO_UNIT, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.name, expected.name);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/cmsg/cmsg_teleport_to_unit.wowm` line 9.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMSG_TELEPORT_TO_UNIT0() {
        let expected = CMSG_TELEPORT_TO_UNIT {
            name: String::from("Vurtne"),
        };

        let header_size = 2 + 4;
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_TELEPORT_TO_UNIT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_TELEPORT_TO_UNIT, got {opcode:#?}", opcode = opcode),
        };

        assert_eq!(t.name, expected.name);

        assert_eq!(t.size() + header_size, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}
