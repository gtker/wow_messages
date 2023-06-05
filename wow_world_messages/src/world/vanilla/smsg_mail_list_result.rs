use std::io::{Read, Write};

use crate::vanilla::Mail;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/mail/smsg_mail_list_result.wowm:118`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mail/smsg_mail_list_result.wowm#L118):
/// ```text
/// smsg SMSG_MAIL_LIST_RESULT = 0x023B {
///     u8 amount_of_mails;
///     Mail[amount_of_mails] mails;
/// }
/// ```
pub struct SMSG_MAIL_LIST_RESULT {
    pub mails: Vec<Mail>,
}

#[cfg(feature = "print-testcase")]
impl SMSG_MAIL_LIST_RESULT {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_MAIL_LIST_RESULT {{").unwrap();
        // Members
        writeln!(s, "    amount_of_mails = {};", self.mails.len()).unwrap();
        write!(s, "    mails = [").unwrap();
        for v in self.mails.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "    message_id = {};", v.message_id).unwrap();
            writeln!(s, "    message_type = {};", crate::vanilla::MailType::try_from(v.message_type.as_int()).unwrap().as_test_case_value()).unwrap();
            match &v.message_type {
                crate::vanilla::Mail_MailType::Normal {
                    sender,
                } => {
                    writeln!(s, "    sender = {};", sender.guid()).unwrap();
                }
                crate::vanilla::Mail_MailType::Auction {
                    auction_id,
                } => {
                    writeln!(s, "    auction_id = {};", auction_id).unwrap();
                }
                crate::vanilla::Mail_MailType::Creature {
                    sender_id,
                } => {
                    writeln!(s, "    sender_id = {};", sender_id).unwrap();
                }
                crate::vanilla::Mail_MailType::Gameobject {
                    sender_id,
                } => {
                    writeln!(s, "    sender_id = {};", sender_id).unwrap();
                }
                _ => {}
            }

            writeln!(s, "    subject = \"{}\";", v.subject).unwrap();
            writeln!(s, "    item_text_id = {};", v.item_text_id).unwrap();
            writeln!(s, "    unknown1 = {};", v.unknown1).unwrap();
            writeln!(s, "    stationery = {};", v.stationery).unwrap();
            writeln!(s, "    item = {};", v.item).unwrap();
            writeln!(s, "    item_enchant_id = {};", v.item_enchant_id).unwrap();
            writeln!(s, "    item_random_property_id = {};", v.item_random_property_id).unwrap();
            writeln!(s, "    item_suffix_factor = {};", v.item_suffix_factor).unwrap();
            writeln!(s, "    item_stack_size = {};", v.item_stack_size).unwrap();
            writeln!(s, "    item_spell_charges = {};", v.item_spell_charges).unwrap();
            writeln!(s, "    max_durability = {};", v.max_durability).unwrap();
            writeln!(s, "    durability = {};", v.durability).unwrap();
            writeln!(s, "    money = {};", v.money.as_int()).unwrap();
            writeln!(s, "    cash_on_delivery_amount = {};", v.cash_on_delivery_amount).unwrap();
            writeln!(s, "    checked_timestamp = {};", v.checked_timestamp).unwrap();
            writeln!(s, "    {}", if v.expiration_time.to_string().contains(".") { v.expiration_time.to_string() } else { format!("{}.0", v.expiration_time) }).unwrap();
            writeln!(s, "    mail_template_id = {};", v.mail_template_id).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 571_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_mails");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1.12\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_MAIL_LIST_RESULT {}
impl crate::Message for SMSG_MAIL_LIST_RESULT {
    const OPCODE: u32 = 0x023b;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // amount_of_mails: u8
        w.write_all(&(self.mails.len() as u8).to_le_bytes())?;

        // mails: Mail[amount_of_mails]
        for i in self.mails.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(1..=84481).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x023B, size: body_size });
        }

        // amount_of_mails: u8
        let amount_of_mails = crate::util::read_u8_le(&mut r)?;

        // mails: Mail[amount_of_mails]
        let mails = {
            let mut mails = Vec::with_capacity(amount_of_mails as usize);
            for _ in 0..amount_of_mails {
                mails.push(Mail::read(&mut r)?);
            }
            mails
        };

        Ok(Self {
            mails,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_MAIL_LIST_RESULT {}

impl SMSG_MAIL_LIST_RESULT {
    pub(crate) fn size(&self) -> usize {
        1 // amount_of_mails: u8
        + self.mails.iter().fold(0, |acc, x| acc + x.size()) // mails: Mail[amount_of_mails]
    }
}

