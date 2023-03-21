use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Sent when using the `port` console command.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/cmsg/cmsg_teleport_to_unit.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/cmsg/cmsg_teleport_to_unit.wowm#L1):
/// ```text
/// cmsg CMSG_TELEPORT_TO_UNIT = 0x0009 {
///     CString name;
/// }
/// ```
pub struct CMSG_TELEPORT_TO_UNIT {
    pub name: String,
}

impl crate::Message for CMSG_TELEPORT_TO_UNIT {
    const OPCODE: u32 = 0x0009;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // name: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.name.as_bytes().iter().rev().next(), Some(&0_u8), "String `name` must not be null-terminated.");
        w.write_all(self.name.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(1..=256).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0009, size: body_size as u32 });
        }

        // name: CString
        let name = {
            let name = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(name)?
        };

        Ok(Self {
            name,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_TELEPORT_TO_UNIT {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_TELEPORT_TO_UNIT {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_TELEPORT_TO_UNIT {}

impl CMSG_TELEPORT_TO_UNIT {
    pub(crate) fn size(&self) -> usize {
        self.name.len() + 1 // name: CString
    }
}

#[cfg(all(feature = "vanilla", test))]
mod test_vanilla {
    use super::CMSG_TELEPORT_TO_UNIT;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ClientOpcodeMessage;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    fn assert(t: &CMSG_TELEPORT_TO_UNIT, expected: &CMSG_TELEPORT_TO_UNIT) {
        assert_eq!(t.name, expected.name);
    }

    const RAW0: [u8; 13] = [ 0x00, 0x0B, 0x09, 0x00, 0x00, 0x00, 0x56, 0x75, 0x72,
         0x74, 0x6E, 0x65, 0x00, ];

    pub(crate) fn expected0() -> CMSG_TELEPORT_TO_UNIT {
        CMSG_TELEPORT_TO_UNIT {
            name: String::from("Vurtne"),
        }

    }

    // Generated from `wow_message_parser/wowm/world/movement/cmsg/cmsg_teleport_to_unit.wowm` line 9.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMSG_TELEPORT_TO_UNIT0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_TELEPORT_TO_UNIT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_TELEPORT_TO_UNIT, got {opcode:#?}", opcode = opcode),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/cmsg/cmsg_teleport_to_unit.wowm` line 9.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMSG_TELEPORT_TO_UNIT0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_TELEPORT_TO_UNIT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_TELEPORT_TO_UNIT, got {opcode:#?}", opcode = opcode),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/cmsg/cmsg_teleport_to_unit.wowm` line 9.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMSG_TELEPORT_TO_UNIT0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_TELEPORT_TO_UNIT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_TELEPORT_TO_UNIT, got {opcode:#?}", opcode = opcode),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "tbc", test))]
mod test_tbc {
    use super::CMSG_TELEPORT_TO_UNIT;
    use super::*;
    use super::super::*;
    use crate::tbc::opcodes::ClientOpcodeMessage;
    use crate::tbc::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    fn assert(t: &CMSG_TELEPORT_TO_UNIT, expected: &CMSG_TELEPORT_TO_UNIT) {
        assert_eq!(t.name, expected.name);
    }

    const RAW0: [u8; 13] = [ 0x00, 0x0B, 0x09, 0x00, 0x00, 0x00, 0x56, 0x75, 0x72,
         0x74, 0x6E, 0x65, 0x00, ];

    pub(crate) fn expected0() -> CMSG_TELEPORT_TO_UNIT {
        CMSG_TELEPORT_TO_UNIT {
            name: String::from("Vurtne"),
        }

    }

    // Generated from `wow_message_parser/wowm/world/movement/cmsg/cmsg_teleport_to_unit.wowm` line 9.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMSG_TELEPORT_TO_UNIT0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_TELEPORT_TO_UNIT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_TELEPORT_TO_UNIT, got {opcode:#?}", opcode = opcode),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/cmsg/cmsg_teleport_to_unit.wowm` line 9.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMSG_TELEPORT_TO_UNIT0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_TELEPORT_TO_UNIT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_TELEPORT_TO_UNIT, got {opcode:#?}", opcode = opcode),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/cmsg/cmsg_teleport_to_unit.wowm` line 9.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMSG_TELEPORT_TO_UNIT0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_TELEPORT_TO_UNIT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_TELEPORT_TO_UNIT, got {opcode:#?}", opcode = opcode),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "wrath", test))]
mod test_wrath {
    use super::CMSG_TELEPORT_TO_UNIT;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ClientOpcodeMessage;
    use crate::wrath::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    fn assert(t: &CMSG_TELEPORT_TO_UNIT, expected: &CMSG_TELEPORT_TO_UNIT) {
        assert_eq!(t.name, expected.name);
    }

    const RAW0: [u8; 13] = [ 0x00, 0x0B, 0x09, 0x00, 0x00, 0x00, 0x56, 0x75, 0x72,
         0x74, 0x6E, 0x65, 0x00, ];

    pub(crate) fn expected0() -> CMSG_TELEPORT_TO_UNIT {
        CMSG_TELEPORT_TO_UNIT {
            name: String::from("Vurtne"),
        }

    }

    // Generated from `wow_message_parser/wowm/world/movement/cmsg/cmsg_teleport_to_unit.wowm` line 9.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn CMSG_TELEPORT_TO_UNIT0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_TELEPORT_TO_UNIT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_TELEPORT_TO_UNIT, got {opcode:#?}", opcode = opcode),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/cmsg/cmsg_teleport_to_unit.wowm` line 9.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_CMSG_TELEPORT_TO_UNIT0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_TELEPORT_TO_UNIT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_TELEPORT_TO_UNIT, got {opcode:#?}", opcode = opcode),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/movement/cmsg/cmsg_teleport_to_unit.wowm` line 9.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_CMSG_TELEPORT_TO_UNIT0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_TELEPORT_TO_UNIT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_TELEPORT_TO_UNIT, got {opcode:#?}", opcode = opcode),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

