use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Respond with [`SMSG_CALENDAR_SEND_NUM_PENDING`](crate::wrath::SMSG_CALENDAR_SEND_NUM_PENDING)
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/cmsg_calendar_get_num_pending.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/cmsg_calendar_get_num_pending.wowm#L3):
/// ```text
/// msg CMSG_CALENDAR_GET_NUM_PENDING = 0x0447 {
/// }
/// ```
pub struct CMSG_CALENDAR_GET_NUM_PENDING {
}

impl crate::private::Sealed for CMSG_CALENDAR_GET_NUM_PENDING {}
impl CMSG_CALENDAR_GET_NUM_PENDING {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 0 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        Ok(Self {
        })
    }

}

impl crate::Message for CMSG_CALENDAR_GET_NUM_PENDING {
    const OPCODE: u32 = 0x0447;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_CALENDAR_GET_NUM_PENDING"
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
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(1095, "CMSG_CALENDAR_GET_NUM_PENDING", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_CALENDAR_GET_NUM_PENDING {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for CMSG_CALENDAR_GET_NUM_PENDING {}

