use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_motd.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_motd.wowm#L1):
/// ```text
/// smsg SMSG_MOTD = 0x033D {
///     u32 amount_of_motds;
///     CString[amount_of_motds] motds;
/// }
/// ```
pub struct SMSG_MOTD {
    pub motds: Vec<String>,
}

impl crate::private::Sealed for SMSG_MOTD {}
impl SMSG_MOTD {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(4..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // amount_of_motds: u32
        let amount_of_motds = crate::util::read_u32_le(&mut r)?;

        // motds: CString[amount_of_motds]
        let motds = {
            let mut motds = Vec::with_capacity(amount_of_motds as usize);

            let allocation_size = u64::from(amount_of_motds);
            if allocation_size > crate::errors::MAX_ALLOCATION_SIZE_WRATH {
                return Err(crate::errors::ParseErrorKind::AllocationTooLargeError(allocation_size));
            }

            for _ in 0..amount_of_motds {
                let s = crate::util::read_c_string_to_vec(&mut r)?;
                motds.push(String::from_utf8(s)?);
            }
            motds
        };

        Ok(Self {
            motds,
        })
    }

}

impl crate::Message for SMSG_MOTD {
    const OPCODE: u32 = 0x033d;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_MOTD"
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // amount_of_motds: u32
        w.write_all(&(self.motds.len() as u32).to_le_bytes())?;

        // motds: CString[amount_of_motds]
        for i in self.motds.iter() {
            w.write_all(i.as_bytes())?;
            w.write_all(&[0])?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(829, "SMSG_MOTD", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_MOTD {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_MOTD {}

impl SMSG_MOTD {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_motds: u32
        + self.motds.iter().fold(0, |acc, x| acc + x.len() + 1) // motds: CString[amount_of_motds]
    }
}

#[cfg(all(feature = "tbc", test))]
mod test_tbc {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_MOTD;
    use super::*;
    use super::super::*;
    use crate::tbc::opcodes::ServerOpcodeMessage;
    use crate::tbc::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    fn assert(t: &SMSG_MOTD, expected: &SMSG_MOTD) {
        assert_eq!(t.motds, expected.motds);
    }

    const RAW0: [u8; 118] = [ 0x00, 0x74, 0x3D, 0x03, 0x02, 0x00, 0x00, 0x00, 0x57,
         0x65, 0x6C, 0x63, 0x6F, 0x6D, 0x65, 0x20, 0x74, 0x6F, 0x20, 0x61, 0x6E,
         0x20, 0x41, 0x7A, 0x65, 0x72, 0x6F, 0x74, 0x68, 0x43, 0x6F, 0x72, 0x65,
         0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x2E, 0x00, 0x7C, 0x63, 0x66,
         0x66, 0x46, 0x46, 0x34, 0x41, 0x32, 0x44, 0x54, 0x68, 0x69, 0x73, 0x20,
         0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x72, 0x75, 0x6E, 0x73, 0x20,
         0x6F, 0x6E, 0x20, 0x41, 0x7A, 0x65, 0x72, 0x6F, 0x74, 0x68, 0x43, 0x6F,
         0x72, 0x65, 0x7C, 0x72, 0x20, 0x7C, 0x63, 0x66, 0x66, 0x33, 0x43, 0x45,
         0x37, 0x46, 0x46, 0x77, 0x77, 0x77, 0x2E, 0x61, 0x7A, 0x65, 0x72, 0x6F,
         0x74, 0x68, 0x63, 0x6F, 0x72, 0x65, 0x2E, 0x6F, 0x72, 0x67, 0x7C, 0x72,
         0x00, ];

    pub(crate) fn expected0() -> SMSG_MOTD {
        SMSG_MOTD {
            motds: vec![ "Welcome to an AzerothCore server.".to_string(), "|cffFF4A2DThis server runs on AzerothCore|r |cff3CE7FFwww.azerothcore.org|r".to_string(), ],
        }

    }

    // Generated from `wow_message_parser/wowm/world/chat/smsg_motd.wowm` line 8.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_motd0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MOTD(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MOTD, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/chat/smsg_motd.wowm` line 8.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_motd0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MOTD(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MOTD, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/chat/smsg_motd.wowm` line 8.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_motd0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MOTD(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MOTD, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

#[cfg(all(feature = "wrath", test))]
mod test_wrath {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_MOTD;
    use super::*;
    use super::super::*;
    use crate::wrath::opcodes::ServerOpcodeMessage;
    use crate::wrath::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    fn assert(t: &SMSG_MOTD, expected: &SMSG_MOTD) {
        assert_eq!(t.motds, expected.motds);
    }

    const RAW0: [u8; 118] = [ 0x00, 0x74, 0x3D, 0x03, 0x02, 0x00, 0x00, 0x00, 0x57,
         0x65, 0x6C, 0x63, 0x6F, 0x6D, 0x65, 0x20, 0x74, 0x6F, 0x20, 0x61, 0x6E,
         0x20, 0x41, 0x7A, 0x65, 0x72, 0x6F, 0x74, 0x68, 0x43, 0x6F, 0x72, 0x65,
         0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x2E, 0x00, 0x7C, 0x63, 0x66,
         0x66, 0x46, 0x46, 0x34, 0x41, 0x32, 0x44, 0x54, 0x68, 0x69, 0x73, 0x20,
         0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x72, 0x75, 0x6E, 0x73, 0x20,
         0x6F, 0x6E, 0x20, 0x41, 0x7A, 0x65, 0x72, 0x6F, 0x74, 0x68, 0x43, 0x6F,
         0x72, 0x65, 0x7C, 0x72, 0x20, 0x7C, 0x63, 0x66, 0x66, 0x33, 0x43, 0x45,
         0x37, 0x46, 0x46, 0x77, 0x77, 0x77, 0x2E, 0x61, 0x7A, 0x65, 0x72, 0x6F,
         0x74, 0x68, 0x63, 0x6F, 0x72, 0x65, 0x2E, 0x6F, 0x72, 0x67, 0x7C, 0x72,
         0x00, ];

    pub(crate) fn expected0() -> SMSG_MOTD {
        SMSG_MOTD {
            motds: vec![ "Welcome to an AzerothCore server.".to_string(), "|cffFF4A2DThis server runs on AzerothCore|r |cff3CE7FFwww.azerothcore.org|r".to_string(), ],
        }

    }

    // Generated from `wow_message_parser/wowm/world/chat/smsg_motd.wowm` line 8.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_motd0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MOTD(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MOTD, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/chat/smsg_motd.wowm` line 8.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_motd0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MOTD(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MOTD, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/chat/smsg_motd.wowm` line 8.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_motd0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_MOTD(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_MOTD, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

