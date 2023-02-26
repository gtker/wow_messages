use crate::shared::received_mail_tbc_wrath::ReceivedMail;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/msg_query_next_mail_time_server.wowm:42`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/msg_query_next_mail_time_server.wowm#L42):
/// ```text
/// smsg MSG_QUERY_NEXT_MAIL_TIME_Server = 0x0284 {
///     u32 float;
///     u32 amount_of_mails;
///     ReceivedMail[amount_of_mails] mails;
/// }
/// ```
pub struct MSG_QUERY_NEXT_MAIL_TIME_Server {
    pub float: u32,
    pub mails: Vec<ReceivedMail>,
}

impl crate::Message for MSG_QUERY_NEXT_MAIL_TIME_Server {
    const OPCODE: u32 = 0x0284;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut impl std::io::Write) -> Result<(), std::io::Error> {
        // float: u32
        w.write_all(&self.float.to_le_bytes())?;

        // amount_of_mails: u32
        w.write_all(&(self.mails.len() as u32).to_le_bytes())?;

        // mails: ReceivedMail[amount_of_mails]
        for i in self.mails.iter() {
            i.write_into_vec(w)?;
        }

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(8..=4294967294).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0284, size: body_size as u32 });
        }

        // float: u32
        let float = crate::util::read_u32_le(r)?;

        // amount_of_mails: u32
        let amount_of_mails = crate::util::read_u32_le(r)?;

        // mails: ReceivedMail[amount_of_mails]
        let mails = {
            let mut mails = Vec::with_capacity(amount_of_mails as usize);
            for i in 0..amount_of_mails {
                mails.push(ReceivedMail::read(r)?);
            }
            mails
        };

        Ok(Self {
            float,
            mails,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for MSG_QUERY_NEXT_MAIL_TIME_Server {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for MSG_QUERY_NEXT_MAIL_TIME_Server {}

impl MSG_QUERY_NEXT_MAIL_TIME_Server {
    pub(crate) fn size(&self) -> usize {
        4 // float: u32
        + 4 // amount_of_mails: u32
        + self.mails.len() * 24 // mails: ReceivedMail[amount_of_mails]
    }
}

