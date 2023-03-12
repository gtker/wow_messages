use std::io::{Read, Write};
use crate::wrath::Mail;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/mail/smsg_mail_list_result.wowm:176`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mail/smsg_mail_list_result.wowm#L176):
/// ```text
/// smsg SMSG_MAIL_LIST_RESULT = 0x023B {
///     u32 real_mail_amount;
///     u8 amount_of_mails;
///     Mail[amount_of_mails] mails;
/// }
/// ```
pub struct SMSG_MAIL_LIST_RESULT {
    /// azerothcore: this will display warning about undelivered mail to player if realCount > mailsCount
    ///
    pub real_mail_amount: u32,
    pub mails: Vec<Mail>,
}

impl crate::Message for SMSG_MAIL_LIST_RESULT {
    const OPCODE: u32 = 0x023b;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // real_mail_amount: u32
        w.write_all(&self.real_mail_amount.to_le_bytes())?;

        // amount_of_mails: u8
        w.write_all(&(self.mails.len() as u8).to_le_bytes())?;

        // mails: Mail[amount_of_mails]
        for i in self.mails.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(5..=7678981).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x023B, size: body_size as u32 });
        }

        // real_mail_amount: u32
        let real_mail_amount = crate::util::read_u32_le(&mut r)?;

        // amount_of_mails: u8
        let amount_of_mails = crate::util::read_u8_le(&mut r)?;

        // mails: Mail[amount_of_mails]
        let mails = {
            let mut mails = Vec::with_capacity(amount_of_mails as usize);
            for i in 0..amount_of_mails {
                mails.push(Mail::read(&mut r)?);
            }
            mails
        };

        Ok(Self {
            real_mail_amount,
            mails,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_MAIL_LIST_RESULT {}

impl SMSG_MAIL_LIST_RESULT {
    pub(crate) fn size(&self) -> usize {
        4 // real_mail_amount: u32
        + 1 // amount_of_mails: u8
        + self.mails.iter().fold(0, |acc, x| acc + x.size()) // mails: Mail[amount_of_mails]
    }
}

