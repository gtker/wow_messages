use std::io::{Read, Write};

use crate::shared::received_mail_tbc_wrath::ReceivedMail;

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

#[cfg(feature = "print-testcase")]
impl MSG_QUERY_NEXT_MAIL_TIME_Server {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test MSG_QUERY_NEXT_MAIL_TIME_Server {{").unwrap();
        // Members
        writeln!(s, "    float = {};", self.float).unwrap();
        writeln!(s, "    amount_of_mails = {};", self.mails.len()).unwrap();
        write!(s, "    mails = [").unwrap();
        for v in self.mails.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "    sender = {};", v.sender.guid()).unwrap();
            writeln!(s, "    auction_house = {};", v.auction_house.as_test_case_value()).unwrap();
            writeln!(s, "    message_type = {};", v.message_type.as_test_case_value()).unwrap();
            writeln!(s, "    stationery = {};", v.stationery).unwrap();
            writeln!(s, "    {}", if v.time.to_string().contains(".") { v.time.to_string() } else { format!("{}.0", v.time) }).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 644_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "float");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"2.4.3 3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for MSG_QUERY_NEXT_MAIL_TIME_Server {}
impl crate::Message for MSG_QUERY_NEXT_MAIL_TIME_Server {
    const OPCODE: u32 = 0x0284;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // float: u32
        w.write_all(&self.float.to_le_bytes())?;

        // amount_of_mails: u32
        w.write_all(&(self.mails.len() as u32).to_le_bytes())?;

        // mails: ReceivedMail[amount_of_mails]
        for i in self.mails.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(8..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0284, size: body_size });
        }

        // float: u32
        let float = crate::util::read_u32_le(&mut r)?;

        // amount_of_mails: u32
        let amount_of_mails = crate::util::read_u32_le(&mut r)?;

        // mails: ReceivedMail[amount_of_mails]
        let mails = {
            let mut mails = Vec::with_capacity(amount_of_mails as usize);
            for _ in 0..amount_of_mails {
                mails.push(ReceivedMail::read(&mut r)?);
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

