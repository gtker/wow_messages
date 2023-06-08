use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Respond with [`SMSG_CALENDAR_SEND_NUM_PENDING`](crate::wrath::SMSG_CALENDAR_SEND_NUM_PENDING)
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/calendar/cmsg_calendar_get_num_pending.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/calendar/cmsg_calendar_get_num_pending.wowm#L3):
/// ```text
/// msg CMSG_CALENDAR_GET_NUM_PENDING = 0x0447 {
/// }
/// ```
pub struct CMSG_CALENDAR_GET_NUM_PENDING {
}

impl crate::private::Sealed for CMSG_CALENDAR_GET_NUM_PENDING {}
impl crate::Message for CMSG_CALENDAR_GET_NUM_PENDING {
    const OPCODE: u32 = 0x0447;

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

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 0 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0447, size: body_size });
        }

        Ok(Self {
        })
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_CALENDAR_GET_NUM_PENDING {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for CMSG_CALENDAR_GET_NUM_PENDING {}

