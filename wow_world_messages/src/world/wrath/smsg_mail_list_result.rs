use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::{
    Mail, MailListItem, MailListItemEnchant, MailType,
};
use wow_world_base::shared::gold_vanilla_tbc_wrath::Gold;

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
    pub real_mail_amount: u32,
    pub mails: Vec<Mail>,
}

impl crate::private::Sealed for SMSG_MAIL_LIST_RESULT {}
impl SMSG_MAIL_LIST_RESULT {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(5..=7678981).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize { opcode: 0x023B, size: body_size });
        }

        // real_mail_amount: u32
        let real_mail_amount = crate::util::read_u32_le(&mut r)?;

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
            real_mail_amount,
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
        writeln!(s, "    real_mail_amount = {};", self.real_mail_amount).unwrap();
        writeln!(s, "    amount_of_mails = {};", self.mails.len()).unwrap();
        write!(s, "    mails = [").unwrap();
        for v in self.mails.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        message_id = {};", v.message_id).unwrap();
            writeln!(s, "        message_type = {};", MailType::try_from(v.message_type.as_int()).unwrap().as_test_case_value()).unwrap();
            match &v.message_type {
                crate::wrath::Mail_MailType::Normal {
                    sender,
                } => {
                    writeln!(s, "        sender = {};", sender.guid()).unwrap();
                }
                crate::wrath::Mail_MailType::Auction {
                    auction_id,
                } => {
                    writeln!(s, "        auction_id = {};", auction_id).unwrap();
                }
                crate::wrath::Mail_MailType::Creature {
                    sender_id,
                } => {
                    writeln!(s, "        sender_id = {};", sender_id).unwrap();
                }
                crate::wrath::Mail_MailType::Gameobject {
                    sender_id,
                } => {
                    writeln!(s, "        sender_id = {};", sender_id).unwrap();
                }
                crate::wrath::Mail_MailType::Item {
                    item,
                } => {
                    writeln!(s, "        item = {};", item).unwrap();
                }
            }

            writeln!(s, "        cash_on_delivery = {};", v.cash_on_delivery.as_int()).unwrap();
            writeln!(s, "        unknown = {};", v.unknown).unwrap();
            writeln!(s, "        stationery = {};", v.stationery).unwrap();
            writeln!(s, "        money = {};", v.money.as_int()).unwrap();
            writeln!(s, "        flags = {};", v.flags).unwrap();
            writeln!(s, "    {}", if v.expiration_time.to_string().contains('.') { v.expiration_time.to_string() } else { format!("{}.0", v.expiration_time) }).unwrap();
            writeln!(s, "        mail_template_id = {};", v.mail_template_id).unwrap();
            writeln!(s, "        subject = \"{}\";", v.subject).unwrap();
            writeln!(s, "        message = \"{}\";", v.message).unwrap();
            writeln!(s, "        amount_of_items = {};", v.items.len()).unwrap();
            write!(s, "        items = [").unwrap();
            for v in v.items.as_slice() {
                writeln!(s, "{{").unwrap();
                // Members
                writeln!(s, "            item_index = {};", v.item_index).unwrap();
                writeln!(s, "            low_guid = {};", v.low_guid).unwrap();
                writeln!(s, "            item = {};", v.item).unwrap();
                write!(s, "            enchants = [").unwrap();
                for v in v.enchants.as_slice() {
                    writeln!(s, "{{").unwrap();
                    // Members
                    writeln!(s, "                charges = {};", v.charges).unwrap();
                    writeln!(s, "                duration = {};", v.duration).unwrap();
                    writeln!(s, "                enchant_id = {};", v.enchant_id).unwrap();

                    writeln!(s, "    }},").unwrap();
                }
                writeln!(s, "];").unwrap();
                writeln!(s, "            item_random_property_id = {};", v.item_random_property_id).unwrap();
                writeln!(s, "            item_suffix_factor = {};", v.item_suffix_factor).unwrap();
                writeln!(s, "            item_amount = {};", v.item_amount).unwrap();
                writeln!(s, "            charges = {};", v.charges).unwrap();
                writeln!(s, "            max_durability = {};", v.max_durability).unwrap();
                writeln!(s, "            durability = {};", v.durability).unwrap();
                writeln!(s, "            unknown = {};", v.unknown).unwrap();

                writeln!(s, "    }},").unwrap();
            }
            writeln!(s, "];").unwrap();

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

        crate::util::write_bytes(&mut s, &mut bytes, 4, "real_mail_amount", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_mails", "    ");
        if !self.mails.is_empty() {
            writeln!(s, "    /* mails: Mail[amount_of_mails] start */").unwrap();
            for (i, v) in self.mails.iter().enumerate() {
                writeln!(s, "    /* mails: Mail[amount_of_mails] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 2, "size", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "message_id", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "message_type", "        ");
                match &v.message_type {
                    crate::wrath::Mail_MailType::Normal {
                        sender,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, "sender", "        ");
                    }
                    crate::wrath::Mail_MailType::Auction {
                        auction_id,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "auction_id", "        ");
                    }
                    crate::wrath::Mail_MailType::Creature {
                        sender_id,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "sender_id", "        ");
                    }
                    crate::wrath::Mail_MailType::Gameobject {
                        sender_id,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "sender_id", "        ");
                    }
                    crate::wrath::Mail_MailType::Item {
                        item,
                    } => {
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "        ");
                    }
                }

                crate::util::write_bytes(&mut s, &mut bytes, 4, "cash_on_delivery", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "stationery", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "money", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "flags", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "expiration_time", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "mail_template_id", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, v.subject.len() + 1, "subject", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, v.message.len() + 1, "message", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 1, "amount_of_items", "        ");
                if !v.items.is_empty() {
                    writeln!(s, "    /* items: MailListItem[amount_of_items] start */").unwrap();
                    for (i, v) in v.items.iter().enumerate() {
                        writeln!(s, "    /* items: MailListItem[amount_of_items] {i} start */").unwrap();
                        crate::util::write_bytes(&mut s, &mut bytes, 1, "item_index", "            ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "low_guid", "            ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "item", "            ");
                        writeln!(s, "    /* enchants: MailListItemEnchant[7] start */").unwrap();
                        for (i, v) in v.enchants.iter().enumerate() {
                            writeln!(s, "    /* enchants: MailListItemEnchant[7] {i} start */").unwrap();
                            crate::util::write_bytes(&mut s, &mut bytes, 4, "charges", "                ");
                            crate::util::write_bytes(&mut s, &mut bytes, 4, "duration", "                ");
                            crate::util::write_bytes(&mut s, &mut bytes, 4, "enchant_id", "                ");
                            writeln!(s, "    /* enchants: MailListItemEnchant[7] {i} end */").unwrap();
                        }
                        writeln!(s, "    /* enchants: MailListItemEnchant[7] end */").unwrap();
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "item_random_property_id", "            ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "item_suffix_factor", "            ");
                        crate::util::write_bytes(&mut s, &mut bytes, 1, "item_amount", "            ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "charges", "            ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "max_durability", "            ");
                        crate::util::write_bytes(&mut s, &mut bytes, 4, "durability", "            ");
                        crate::util::write_bytes(&mut s, &mut bytes, 1, "unknown", "            ");
                        writeln!(s, "    /* items: MailListItem[amount_of_items] {i} end */").unwrap();
                    }
                    writeln!(s, "    /* items: MailListItem[amount_of_items] end */").unwrap();
                }
                writeln!(s, "    /* mails: Mail[amount_of_mails] {i} end */").unwrap();
            }
            writeln!(s, "    /* mails: Mail[amount_of_mails] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
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

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        Self::read_inner(r, body_size)
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

