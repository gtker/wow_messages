use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::{
    Mail, MailType,
};
use wow_world_base::shared::gold_vanilla_tbc_wrath::Gold;

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

impl crate::private::Sealed for SMSG_MAIL_LIST_RESULT {}
impl SMSG_MAIL_LIST_RESULT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(1..=84481).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x023B, size: body_size });
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

impl crate::Message for SMSG_MAIL_LIST_RESULT {
    const OPCODE: u32 = 0x023b;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
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
            writeln!(s, "        message_id = {};", v.message_id).unwrap();
            writeln!(s, "        message_type = {};", MailType::try_from(v.message_type.as_int()).unwrap().as_test_case_value()).unwrap();
            match &v.message_type {
                crate::vanilla::Mail_MailType::Normal {
                    sender,
                } => {
                    writeln!(s, "        sender = {};", sender.guid()).unwrap();
                }
                crate::vanilla::Mail_MailType::Auction {
                    auction_id,
                } => {
                    writeln!(s, "        auction_id = {};", auction_id).unwrap();
                }
                crate::vanilla::Mail_MailType::Creature {
                    sender_id,
                } => {
                    writeln!(s, "        sender_id = {};", sender_id).unwrap();
                }
                crate::vanilla::Mail_MailType::Gameobject {
                    sender_id,
                } => {
                    writeln!(s, "        sender_id = {};", sender_id).unwrap();
                }
                _ => {}
            }

            writeln!(s, "        subject = \"{}\";", v.subject).unwrap();
            writeln!(s, "        item_text_id = {};", v.item_text_id).unwrap();
            writeln!(s, "        unknown1 = {};", v.unknown1).unwrap();
            writeln!(s, "        stationery = {};", v.stationery).unwrap();
            writeln!(s, "        item = {};", v.item).unwrap();
            writeln!(s, "        item_enchant_id = {};", v.item_enchant_id).unwrap();
            writeln!(s, "        item_random_property_id = {};", v.item_random_property_id).unwrap();
            writeln!(s, "        item_suffix_factor = {};", v.item_suffix_factor).unwrap();
            writeln!(s, "        item_stack_size = {};", v.item_stack_size).unwrap();
            writeln!(s, "        item_spell_charges = {};", v.item_spell_charges).unwrap();
            writeln!(s, "        max_durability = {};", v.max_durability).unwrap();
            writeln!(s, "        durability = {};", v.durability).unwrap();
            writeln!(s, "        money = {};", v.money.as_int()).unwrap();
            writeln!(s, "        cash_on_delivery_amount = {};", v.cash_on_delivery_amount).unwrap();
            writeln!(s, "        checked_timestamp = {};", v.checked_timestamp).unwrap();
            writeln!(s, "    {}", if v.expiration_time.to_string().contains('.') { v.expiration_time.to_string() } else { format!("{}.0", v.expiration_time) }).unwrap();
            writeln!(s, "        mail_template_id = {};", v.mail_template_id).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 571_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_mails", "    ");
        if !self.mails.is_empty() {
            writeln!(s, "    /* mails: Mail[amount_of_mails] start */").unwrap();
            for (i, v) in self.mails.iter().enumerate() {
                writeln!(s, "    /* mails: Mail[amount_of_mails] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "message_id", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "message_type", "        ");
                match &v.message_type {
                    crate::vanilla::Mail_MailType::Normal {
                        sender,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "sender", "        ");
                    }
                    crate::vanilla::Mail_MailType::Auction {
                        auction_id,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "auction_id", "        ");
                    }
                    crate::vanilla::Mail_MailType::Creature {
                        sender_id,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "sender_id", "        ");
                    }
                    crate::vanilla::Mail_MailType::Gameobject {
                        sender_id,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "sender_id", "        ");
                    }
                    _ => {}
                }

                crate::util::write_bytes(&mut s, &mut bytes, v.subject.len() + 1, "subject", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item_text_id", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown1", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "stationery", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item_enchant_id", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item_random_property_id", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item_suffix_factor", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "item_stack_size", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "item_spell_charges", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "max_durability", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "durability", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "money", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "cash_on_delivery_amount", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "checked_timestamp", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "expiration_time", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "mail_template_id", "        ");
                writeln!(s, "    /* mails: Mail[amount_of_mails] {i} end */").unwrap();
            }
            writeln!(s, "    /* mails: Mail[amount_of_mails] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

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

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
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

