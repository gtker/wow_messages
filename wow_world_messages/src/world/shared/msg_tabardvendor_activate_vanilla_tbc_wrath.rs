use std::io::{Read, Write};

use crate::Guid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/msg_tabardvendor_activate.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/msg_tabardvendor_activate.wowm#L3):
/// ```text
/// msg MSG_TABARDVENDOR_ACTIVATE = 0x01F2 {
///     Guid guid;
/// }
/// ```
pub struct MSG_TABARDVENDOR_ACTIVATE {
    pub guid: Guid,
}

impl crate::private::Sealed for MSG_TABARDVENDOR_ACTIVATE {}
impl MSG_TABARDVENDOR_ACTIVATE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 8 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // guid: Guid
        let guid = crate::util::read_guid(&mut r)?;

        Ok(Self {
            guid,
        })
    }

}

impl crate::Message for MSG_TABARDVENDOR_ACTIVATE {
    const OPCODE: u32 = 0x01f2;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "MSG_TABARDVENDOR_ACTIVATE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        panic!("MSG types not supported");
    }

    fn size_without_header(&self) -> u32 {
        8
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(498, "MSG_TABARDVENDOR_ACTIVATE", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ClientMessage for MSG_TABARDVENDOR_ACTIVATE {}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for MSG_TABARDVENDOR_ACTIVATE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ClientMessage for MSG_TABARDVENDOR_ACTIVATE {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for MSG_TABARDVENDOR_ACTIVATE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for MSG_TABARDVENDOR_ACTIVATE {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_TABARDVENDOR_ACTIVATE {}

