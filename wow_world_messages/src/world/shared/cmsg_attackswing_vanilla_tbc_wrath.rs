use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Signals that client has right clicked an opponent and is in the attack stance.
/// Server should reply with [`SMSG_ATTACKSTART`](crate::vanilla::SMSG_ATTACKSTART).
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/cmsg_attackswing.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/cmsg_attackswing.wowm#L3):
/// ```text
/// cmsg CMSG_ATTACKSWING = 0x0141 {
///     Guid guid;
/// }
/// ```
pub struct CMSG_ATTACKSWING {
    pub guid: Guid,
}

#[cfg(feature = "print-testcase")]
impl CMSG_ATTACKSWING {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_ATTACKSWING {{").unwrap();
        // Members
        writeln!(s, "    guid = {};", self.guid.guid()).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 14_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 321_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "guid");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1 2 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for CMSG_ATTACKSWING {}
impl crate::Message for CMSG_ATTACKSWING {
    const OPCODE: u32 = 0x0141;

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 8 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0141, size: body_size });
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        Ok(Self {
            guid,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_ATTACKSWING {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for CMSG_ATTACKSWING {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_ATTACKSWING {}

#[cfg(all(feature = "vanilla", test))]
mod test_vanilla {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_ATTACKSWING;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ClientOpcodeMessage;
    use crate::Guid;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    fn assert(t: &CMSG_ATTACKSWING, expected: &CMSG_ATTACKSWING) {
        assert_eq!(t.guid, expected.guid);
    }

    const RAW0: [u8; 14] = [ 0x00, 0x0C, 0x41, 0x01, 0x00, 0x00, 0x64, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> CMSG_ATTACKSWING {
        CMSG_ATTACKSWING {
            guid: Guid::new(0x64),
        }

    }

    // Generated from `wow_message_parser/wowm/world/combat/cmsg_attackswing.wowm` line 10.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_attackswing0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_ATTACKSWING(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_ATTACKSWING, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/combat/cmsg_attackswing.wowm` line 10.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_attackswing0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_ATTACKSWING(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_ATTACKSWING, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/combat/cmsg_attackswing.wowm` line 10.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_attackswing0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_ATTACKSWING(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_ATTACKSWING, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "tbc", test))]
mod test_tbc {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_ATTACKSWING;
    use super::*;
    use super::super::*;
    use crate::tbc::opcodes::ClientOpcodeMessage;
    use crate::Guid;
    use crate::tbc::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    fn assert(t: &CMSG_ATTACKSWING, expected: &CMSG_ATTACKSWING) {
        assert_eq!(t.guid, expected.guid);
    }

    const RAW0: [u8; 14] = [ 0x00, 0x0C, 0x41, 0x01, 0x00, 0x00, 0x64, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> CMSG_ATTACKSWING {
        CMSG_ATTACKSWING {
            guid: Guid::new(0x64),
        }

    }

    // Generated from `wow_message_parser/wowm/world/combat/cmsg_attackswing.wowm` line 10.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_attackswing0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_ATTACKSWING(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_ATTACKSWING, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/combat/cmsg_attackswing.wowm` line 10.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_attackswing0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_ATTACKSWING(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_ATTACKSWING, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/combat/cmsg_attackswing.wowm` line 10.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_attackswing0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_ATTACKSWING(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_ATTACKSWING, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "wrath", test))]
mod test_wrath {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_ATTACKSWING;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ClientOpcodeMessage;
    use crate::Guid;
    use crate::wrath::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    fn assert(t: &CMSG_ATTACKSWING, expected: &CMSG_ATTACKSWING) {
        assert_eq!(t.guid, expected.guid);
    }

    const RAW0: [u8; 14] = [ 0x00, 0x0C, 0x41, 0x01, 0x00, 0x00, 0x64, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> CMSG_ATTACKSWING {
        CMSG_ATTACKSWING {
            guid: Guid::new(0x64),
        }

    }

    // Generated from `wow_message_parser/wowm/world/combat/cmsg_attackswing.wowm` line 10.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_attackswing0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_ATTACKSWING(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_ATTACKSWING, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/combat/cmsg_attackswing.wowm` line 10.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_attackswing0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_ATTACKSWING(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_ATTACKSWING, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/combat/cmsg_attackswing.wowm` line 10.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_attackswing0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_ATTACKSWING(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_ATTACKSWING, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

