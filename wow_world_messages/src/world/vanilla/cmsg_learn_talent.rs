use std::io::{Read, Write};

use crate::vanilla::Talent;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/cmsg_learn_talent.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/cmsg_learn_talent.wowm#L1):
/// ```text
/// cmsg CMSG_LEARN_TALENT = 0x0251 {
///     Talent talent;
///     u32 requested_rank;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct CMSG_LEARN_TALENT {
    pub talent: Talent,
    pub requested_rank: u32,
}

impl crate::private::Sealed for CMSG_LEARN_TALENT {}
impl CMSG_LEARN_TALENT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 8 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // talent: Talent
        let talent = crate::util::read_u32_le(&mut r)?.try_into()?;

        // requested_rank: u32
        let requested_rank = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            talent,
            requested_rank,
        })
    }

}

impl crate::Message for CMSG_LEARN_TALENT {
    const OPCODE: u32 = 0x0251;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_LEARN_TALENT"
    }

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // talent: Talent
        w.write_all(&(self.talent.as_int().to_le_bytes()))?;

        // requested_rank: u32
        w.write_all(&self.requested_rank.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(593, "CMSG_LEARN_TALENT", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for CMSG_LEARN_TALENT {}

#[cfg(test)]
mod test {
    #![allow(clippy::missing_const_for_fn)]
    use super::CMSG_LEARN_TALENT;
    use super::*;
    use super::super::*;
    use crate::vanilla::opcodes::ClientOpcodeMessage;
    use crate::vanilla::{ClientMessage, ServerMessage};

    const HEADER_SIZE: usize = 2 + 4;
    const RAW0: [u8; 14] = [ 0x00, 0x0C, 0x51, 0x02, 0x00, 0x00, 0x9E, 0x00, 0x00,
         0x00, 0x00, 0x00, 0x00, 0x00, ];

    pub(crate) fn expected0() -> CMSG_LEARN_TALENT {
        CMSG_LEARN_TALENT {
            talent: Talent::BoomingVoice,
            requested_rank: 0x0,
        }

    }

    // Generated from `wow_message_parser/wowm/world/spell/cmsg_learn_talent.wowm` line 2040.
    #[cfg(feature = "sync")]
    #[cfg_attr(feature = "sync", test)]
    fn cmsg_learn_talent0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::read_unencrypted(&mut std::io::Cursor::new(&RAW0)).unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_LEARN_TALENT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_LEARN_TALENT, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/spell/cmsg_learn_talent.wowm` line 2040.
    #[cfg(feature = "tokio")]
    #[cfg_attr(feature = "tokio", tokio::test)]
    async fn tokio_cmsg_learn_talent0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::tokio_read_unencrypted(&mut std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_LEARN_TALENT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_LEARN_TALENT, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.tokio_write_unencrypted_client(&mut std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

    // Generated from `wow_message_parser/wowm/world/spell/cmsg_learn_talent.wowm` line 2040.
    #[cfg(feature = "async-std")]
    #[cfg_attr(feature = "async-std", async_std::test)]
    async fn astd_cmsg_learn_talent0() {
        let expected = expected0();
        let t = ClientOpcodeMessage::astd_read_unencrypted(&mut async_std::io::Cursor::new(&RAW0)).await.unwrap();
        let t = match t {
            ClientOpcodeMessage::CMSG_LEARN_TALENT(t) => t,
            opcode => panic!("incorrect opcode. Expected CMSG_LEARN_TALENT, got {opcode:#?}"),
        };

        assert_eq!(&t, &expected);
        assert_eq!(8 + HEADER_SIZE, RAW0.len());

        let mut dest = Vec::with_capacity(RAW0.len());
        expected.astd_write_unencrypted_client(&mut async_std::io::Cursor::new(&mut dest)).await.unwrap();

        assert_eq!(dest, RAW0);
    }

}

