use std::io::{Read, Write};

use crate::Guid;
use crate::tbc::{
    MailListItem, MailType,
};
use wow_world_base::shared::gold_vanilla_tbc_wrath::Gold;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/mail/smsg_mail_list_result.wowm:60`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mail/smsg_mail_list_result.wowm#L60):
/// ```text
/// struct Mail {
///     u16 size = self.size;
///     u32 message_id;
///     MailType message_type;
///     if (message_type == NORMAL) {
///         Guid sender;
///     }
///     else if (message_type == CREATURE
///         || message_type == GAMEOBJECT) {
///         u32 sender_id;
///     }
///     else if (message_type == AUCTION) {
///         u32 auction_id;
///     }
///     else if (message_type == ITEM) {
///         u32 item;
///     }
///     Gold cash_on_delivery;
///     u32 item_text_id;
///     u32 unknown;
///     u32 stationery;
///     Gold money;
///     u32 flags;
///     f32 expiration_time;
///     u32 mail_template_id;
///     CString subject;
///     u8 amount_of_items;
///     MailListItem[amount_of_items] items;
/// }
/// ```
pub struct Mail {
    pub message_id: u32,
    pub message_type: Mail_MailType,
    pub cash_on_delivery: Gold,
    pub item_text_id: u32,
    pub unknown: u32,
    pub stationery: u32,
    pub money: Gold,
    pub flags: u32,
    pub expiration_time: f32,
    /// cmangos/vmangos/mangoszero: mail template (MailTemplate.dbc)
    ///
    pub mail_template_id: u32,
    pub subject: String,
    pub items: Vec<MailListItem>,
}

impl Mail {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // size: u16
        w.write_all(&((self.size() - 2) as u16).to_le_bytes())?;

        // message_id: u32
        w.write_all(&self.message_id.to_le_bytes())?;

        // message_type: MailType
        w.write_all(&(self.message_type.as_int().to_le_bytes()))?;

        match &self.message_type {
            Mail_MailType::Normal {
                sender,
            } => {
                // sender: Guid
                w.write_all(&sender.guid().to_le_bytes())?;

            }
            Mail_MailType::Auction {
                auction_id,
            } => {
                // auction_id: u32
                w.write_all(&auction_id.to_le_bytes())?;

            }
            Mail_MailType::Creature {
                sender_id,
            } => {
                // sender_id: u32
                w.write_all(&sender_id.to_le_bytes())?;

            }
            Mail_MailType::Gameobject {
                sender_id,
            } => {
                // sender_id: u32
                w.write_all(&sender_id.to_le_bytes())?;

            }
            Mail_MailType::Item {
                item,
            } => {
                // item: u32
                w.write_all(&item.to_le_bytes())?;

            }
        }

        // cash_on_delivery: Gold
        w.write_all((self.cash_on_delivery.as_int()).to_le_bytes().as_slice())?;

        // item_text_id: u32
        w.write_all(&self.item_text_id.to_le_bytes())?;

        // unknown: u32
        w.write_all(&self.unknown.to_le_bytes())?;

        // stationery: u32
        w.write_all(&self.stationery.to_le_bytes())?;

        // money: Gold
        w.write_all((self.money.as_int()).to_le_bytes().as_slice())?;

        // flags: u32
        w.write_all(&self.flags.to_le_bytes())?;

        // expiration_time: f32
        w.write_all(&self.expiration_time.to_le_bytes())?;

        // mail_template_id: u32
        w.write_all(&self.mail_template_id.to_le_bytes())?;

        // subject: CString
        // TODO: Guard against strings that are already null-terminated
        assert_ne!(self.subject.as_bytes().iter().rev().next(), Some(&0_u8), "String `subject` must not be null-terminated.");
        w.write_all(self.subject.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // amount_of_items: u8
        w.write_all(&(self.items.len() as u8).to_le_bytes())?;

        // items: MailListItem[amount_of_items]
        for i in self.items.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
}

impl Mail {
    pub(crate) fn read<R: Read>(mut r: R) -> Result<Self, crate::errors::ParseError> {
        // size: u16
        let _size = crate::util::read_u16_le(&mut r)?;
        // size is expected to always be self.size (0)

        // message_id: u32
        let message_id = crate::util::read_u32_le(&mut r)?;

        // message_type: MailType
        let message_type = crate::util::read_u8_le(&mut r)?.try_into()?;

        let message_type_if = match message_type {
            MailType::Normal => {
                // sender: Guid
                let sender = crate::util::read_guid(&mut r)?;

                Mail_MailType::Normal {
                    sender,
                }
            }
            MailType::Auction => {
                // auction_id: u32
                let auction_id = crate::util::read_u32_le(&mut r)?;

                Mail_MailType::Auction {
                    auction_id,
                }
            }
            MailType::Creature => {
                // sender_id: u32
                let sender_id = crate::util::read_u32_le(&mut r)?;

                Mail_MailType::Creature {
                    sender_id,
                }
            }
            MailType::Gameobject => {
                // sender_id: u32
                let sender_id = crate::util::read_u32_le(&mut r)?;

                Mail_MailType::Gameobject {
                    sender_id,
                }
            }
            MailType::Item => {
                // item: u32
                let item = crate::util::read_u32_le(&mut r)?;

                Mail_MailType::Item {
                    item,
                }
            }
        };

        // cash_on_delivery: Gold
        let cash_on_delivery = Gold::new(crate::util::read_u32_le(&mut r)?);

        // item_text_id: u32
        let item_text_id = crate::util::read_u32_le(&mut r)?;

        // unknown: u32
        let unknown = crate::util::read_u32_le(&mut r)?;

        // stationery: u32
        let stationery = crate::util::read_u32_le(&mut r)?;

        // money: Gold
        let money = Gold::new(crate::util::read_u32_le(&mut r)?);

        // flags: u32
        let flags = crate::util::read_u32_le(&mut r)?;

        // expiration_time: f32
        let expiration_time = crate::util::read_f32_le(&mut r)?;

        // mail_template_id: u32
        let mail_template_id = crate::util::read_u32_le(&mut r)?;

        // subject: CString
        let subject = {
            let subject = crate::util::read_c_string_to_vec(&mut r)?;
            String::from_utf8(subject)?
        };

        // amount_of_items: u8
        let amount_of_items = crate::util::read_u8_le(&mut r)?;

        // items: MailListItem[amount_of_items]
        let items = {
            let mut items = Vec::with_capacity(amount_of_items as usize);
            for _ in 0..amount_of_items {
                items.push(MailListItem::read(&mut r)?);
            }
            items
        };

        Ok(Self {
            message_id,
            message_type: message_type_if,
            cash_on_delivery,
            item_text_id,
            unknown,
            stationery,
            money,
            flags,
            expiration_time,
            mail_template_id,
            subject,
            items,
        })
    }

}

impl Mail {
    pub(crate) fn size(&self) -> usize {
        2 // size: u16
        + 4 // message_id: u32
        + self.message_type.size() // message_type: Mail_MailType
        + 4 // cash_on_delivery: Gold
        + 4 // item_text_id: u32
        + 4 // unknown: u32
        + 4 // stationery: u32
        + 4 // money: Gold
        + 4 // flags: u32
        + 4 // expiration_time: f32
        + 4 // mail_template_id: u32
        + self.subject.len() + 1 // subject: CString
        + 1 // amount_of_items: u8
        + self.items.len() * 102 // items: MailListItem[amount_of_items]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Mail_MailType {
    Normal {
        sender: Guid,
    },
    Auction {
        auction_id: u32,
    },
    Creature {
        sender_id: u32,
    },
    Gameobject {
        sender_id: u32,
    },
    Item {
        item: u32,
    },
}

impl Default for Mail_MailType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Normal {
            sender: Default::default(),
        }
    }
}

impl Mail_MailType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Normal { .. } => 0,
            Self::Auction { .. } => 2,
            Self::Creature { .. } => 3,
            Self::Gameobject { .. } => 4,
            Self::Item { .. } => 5,
        }
    }

}

impl std::fmt::Display for Mail_MailType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal{ .. } => f.write_str("Normal"),
            Self::Auction{ .. } => f.write_str("Auction"),
            Self::Creature{ .. } => f.write_str("Creature"),
            Self::Gameobject{ .. } => f.write_str("Gameobject"),
            Self::Item{ .. } => f.write_str("Item"),
        }
    }
}

impl Mail_MailType {
    pub(crate) const fn size(&self) -> usize {
        match self {
            Self::Normal {
                ..
            } => {
                1
                + 8 // sender: Guid
            }
            Self::Auction {
                ..
            } => {
                1
                + 4 // auction_id: u32
            }
            Self::Creature {
                ..
            } => {
                1
                + 4 // sender_id: u32
            }
            Self::Gameobject {
                ..
            } => {
                1
                + 4 // sender_id: u32
            }
            Self::Item {
                ..
            } => {
                1
                + 4 // item: u32
            }
        }
    }
}

