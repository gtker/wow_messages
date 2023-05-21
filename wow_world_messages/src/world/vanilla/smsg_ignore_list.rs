use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_ignore_list.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_ignore_list.wowm#L3):
/// ```text
/// smsg SMSG_IGNORE_LIST = 0x006B {
///     u8 amount_of_ignored;
///     u64[amount_of_ignored] ignored;
/// }
/// ```
pub struct SMSG_IGNORE_LIST {
    pub ignored: Vec<u64>,
}

impl crate::private::Sealed for SMSG_IGNORE_LIST {}
impl SMSG_IGNORE_LIST {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(1..=2049).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x006B, size: body_size });
        }

        // amount_of_ignored: u8
        let amount_of_ignored = crate::util::read_u8_le(&mut r)?;

        // ignored: u64[amount_of_ignored]
        let ignored = {
            let mut ignored = Vec::with_capacity(amount_of_ignored as usize);
            for _ in 0..amount_of_ignored {
                ignored.push(crate::util::read_u64_le(&mut r)?);
            }
            ignored
        };

        Ok(Self {
            ignored,
        })
    }

}

impl crate::Message for SMSG_IGNORE_LIST {
    const OPCODE: u32 = 0x006b;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // amount_of_ignored: u8
        w.write_all(&(self.ignored.len() as u8).to_le_bytes())?;

        // ignored: u64[amount_of_ignored]
        for i in self.ignored.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size)
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_IGNORE_LIST {}

impl SMSG_IGNORE_LIST {
    pub(crate) fn size(&self) -> usize {
        1 // amount_of_ignored: u8
        + self.ignored.len() * core::mem::size_of::<u64>() // ignored: u64[amount_of_ignored]
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::SMSG_IGNORE_LIST;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ServerOpcodeMessage;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 2;
    fn assert(t: &SMSG_IGNORE_LIST, expected: &SMSG_IGNORE_LIST) {
        assert_eq!(t.ignored, expected.ignored);
    }

    const RAW0: [u8; 13] = [ 0x00, 0x0B, 0x6B, 0x00, 0x01, 0xEF, 0xBE, 0xAD, 0xDE,
         0xFE, 0x0F, 0xDC, 0xBA, ];

    pub(crate) fn expected0() -> SMSG_IGNORE_LIST {
        SMSG_IGNORE_LIST {
            ignored: vec![ 0xBADC0FFEDEADBEEF, ],
        }

    }

    // Generated from `wow_message_parser/wowm/world/social/smsg_ignore_list.wowm` line 8.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_ignore_list0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_IGNORE_LIST(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_IGNORE_LIST, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/social/smsg_ignore_list.wowm` line 8.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_ignore_list0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_IGNORE_LIST(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_IGNORE_LIST, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/social/smsg_ignore_list.wowm` line 8.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_ignore_list0() {
        let expected = expected0();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_IGNORE_LIST(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_IGNORE_LIST, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    const RAW1: [u8; 21] = [ 0x00, 0x13, 0x6B, 0x00, 0x02, 0xEF, 0xBE, 0xAD, 0xDE,
         0xFE, 0x0F, 0xDC, 0xBA, 0xEF, 0xBE, 0xAD, 0xDE, 0x00, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected1() -> SMSG_IGNORE_LIST {
        SMSG_IGNORE_LIST {
            ignored: vec![ 0xBADC0FFEDEADBEEF, 0xDEADBEEF, ],
        }

    }

    // Generated from `wow_message_parser/wowm/world/social/smsg_ignore_list.wowm` line 18.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn smsg_ignore_list1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW1)).unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_IGNORE_LIST(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_IGNORE_LIST, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/social/smsg_ignore_list.wowm` line 18.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_smsg_ignore_list1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_IGNORE_LIST(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_IGNORE_LIST, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.tokio_write_unencrypted_server(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

    // Generated from `wow_message_parser/wowm/world/social/smsg_ignore_list.wowm` line 18.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_smsg_ignore_list1() {
        let expected = expected1();
        let t = ServerOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW1)).await.unwrap();
        let t = match t {
            ServerOpcodeMessage::SMSG_IGNORE_LIST(t) => t,
            opcode => panic!("incorrect opcode. Expected SMSG_IGNORE_LIST, got {opcode:#?}"),
        };

        assert(&t, &expected);
        assert_eq!(t.size() + HEADER_SIZE, RAW1.len());

        let mut dest = Vec::with_capacity(RAW1.len());
        expected.astd_write_unencrypted_server(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW1);
    }

}

