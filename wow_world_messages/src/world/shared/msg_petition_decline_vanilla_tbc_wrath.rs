use std::io::{Read, Write};

use crate::Guid;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/msg_petition_decline.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/msg_petition_decline.wowm#L3):
/// ```text
/// msg MSG_PETITION_DECLINE = 0x01C2 {
///     Guid petition;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct MSG_PETITION_DECLINE {
    pub petition: Guid,
}

impl crate::private::Sealed for MSG_PETITION_DECLINE {}
impl MSG_PETITION_DECLINE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 8 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // petition: Guid
        let petition = crate::util::read_guid(&mut r)?;

        Ok(Self {
            petition,
        })
    }

}

impl crate::Message for MSG_PETITION_DECLINE {
    const OPCODE: u32 = 0x01c2;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "MSG_PETITION_DECLINE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        panic!("MSG types not supported");
    }

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // petition: Guid
        w.write_all(&self.petition.guid().to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(450, "MSG_PETITION_DECLINE", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for MSG_PETITION_DECLINE {}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for MSG_PETITION_DECLINE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for MSG_PETITION_DECLINE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for MSG_PETITION_DECLINE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for MSG_PETITION_DECLINE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_PETITION_DECLINE {}

