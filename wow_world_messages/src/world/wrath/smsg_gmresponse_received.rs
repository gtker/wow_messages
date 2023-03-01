use crate:: {
};
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gamemaster/smsg_gmresponse_received.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gamemaster/smsg_gmresponse_received.wowm#L1):
/// ```text
/// smsg SMSG_GMRESPONSE_RECEIVED = 0x04EF {
///     u32 response_id;
///     u32 ticket_id;
///     CString message;
///     CString[4] response;
/// }
/// ```
pub struct SMSG_GMRESPONSE_RECEIVED {
    pub response_id: u32,
    pub ticket_id: u32,
    pub message: String,
    pub response: [String; 4],
}

impl crate::Message for SMSG_GMRESPONSE_RECEIVED {
    const OPCODE: u32 = 0x04ef;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // response_id: u32
        w.write_all(&self.response_id.to_le_bytes())?;

        // ticket_id: u32
        w.write_all(&self.ticket_id.to_le_bytes())?;

        // message: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.message.as_bytes().iter().rev().next(), Some(&0_u8), "String `message` must not be null-terminated.");
        w.write_all(self.message.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // response: CString[4]
        for i in self.response.iter() {
            w.write_all(i.as_bytes())?;
            w.write_all(&[0])?;
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(13..=1288).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04EF, size: body_size as u32 });
        }

        // response_id: u32
        let response_id = crate::util::read_u32_le(&mut r)?;

        // ticket_id: u32
        let ticket_id = crate::util::read_u32_le(&mut r)?;

        // message: CString
        let message = {
            let message = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(message)?
        };

        // response: CString[4]
        let response = {
            let mut response = [(); 4].map(|_| String::default());
            for i in response.iter_mut() {
                let s = crate::util::read_c_string_to_vec(&mut r)?;
                *i = String::from_utf8(s)?;
            }
            response
        };

        Ok(Self {
            response_id,
            ticket_id,
            message,
            response,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_GMRESPONSE_RECEIVED {}

impl SMSG_GMRESPONSE_RECEIVED {
    pub(crate) fn size(&self) -> usize {
        4 // response_id: u32
        + 4 // ticket_id: u32
        + self.message.len() + 1 // message: CString
        + self.response.iter().fold(0, |acc, x| acc + x.len() + 1) // response: CString[4]
    }
}

