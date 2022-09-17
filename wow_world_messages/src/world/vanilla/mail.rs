use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::MailType;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/mail/smsg_mail_list_result.wowm:17`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mail/smsg_mail_list_result.wowm#L17):
/// ```text
/// struct Mail {
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
///     CString subject;
///     u32 item_text_id;
///     u32 unknown1;
///     u32 stationery;
///     u32 item_id;
///     u32 item_enchant_id;
///     u32 item_random_property_id;
///     u32 item_suffix_factor;
///     u8 item_stack_size;
///     u32 item_spell_charges;
///     u32 max_durability;
///     u32 durability;
///     u32 money;
///     u32 cash_on_delivery_amount;
///     u32 checked_timestamp;
///     f32 expiration_time;
///     u32 mail_template_id;
/// }
/// ```
pub struct Mail {
    pub message_id: u32,
    pub message_type: Mail_MailType,
    pub subject: String,
    pub item_text_id: u32,
    /// cmangos/vmangos/mangoszero: set to 0
    ///
    pub unknown1: u32,
    /// cmangos/vmangos/mangoszero: stationery (Stationery.dbc)
    ///
    pub stationery: u32,
    pub item_id: u32,
    pub item_enchant_id: u32,
    pub item_random_property_id: u32,
    pub item_suffix_factor: u32,
    pub item_stack_size: u8,
    pub item_spell_charges: u32,
    pub max_durability: u32,
    pub durability: u32,
    pub money: u32,
    pub cash_on_delivery_amount: u32,
    /// cmangos/vmangos/mangoszero: All have a comment with 'flags' but send the timestamp from the item.
    ///
    pub checked_timestamp: u32,
    pub expiration_time: f32,
    /// cmangos/vmangos/mangoszero: mail template (MailTemplate.dbc)
    ///
    pub mail_template_id: u32,
}

impl Mail {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // message_id: u32
        w.write_all(&self.message_id.to_le_bytes())?;

        // message_type: MailType
        w.write_all(&(self.message_type.as_int() as u8).to_le_bytes())?;

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
            Mail_MailType::Item => {}
        }

        // subject: CString
        // Guard against strings that are already null-terminated
        assert_ne!(self.subject.as_bytes().iter().rev().next(), Some(&0u8), "String subject must not be null-terminated.");
        w.write_all(self.subject.as_bytes())?;
        // Null terminator
        w.write_all(&[0])?;

        // item_text_id: u32
        w.write_all(&self.item_text_id.to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // stationery: u32
        w.write_all(&self.stationery.to_le_bytes())?;

        // item_id: u32
        w.write_all(&self.item_id.to_le_bytes())?;

        // item_enchant_id: u32
        w.write_all(&self.item_enchant_id.to_le_bytes())?;

        // item_random_property_id: u32
        w.write_all(&self.item_random_property_id.to_le_bytes())?;

        // item_suffix_factor: u32
        w.write_all(&self.item_suffix_factor.to_le_bytes())?;

        // item_stack_size: u8
        w.write_all(&self.item_stack_size.to_le_bytes())?;

        // item_spell_charges: u32
        w.write_all(&self.item_spell_charges.to_le_bytes())?;

        // max_durability: u32
        w.write_all(&self.max_durability.to_le_bytes())?;

        // durability: u32
        w.write_all(&self.durability.to_le_bytes())?;

        // money: u32
        w.write_all(&self.money.to_le_bytes())?;

        // cash_on_delivery_amount: u32
        w.write_all(&self.cash_on_delivery_amount.to_le_bytes())?;

        // checked_timestamp: u32
        w.write_all(&self.checked_timestamp.to_le_bytes())?;

        // expiration_time: f32
        w.write_all(&self.expiration_time.to_le_bytes())?;

        // mail_template_id: u32
        w.write_all(&self.mail_template_id.to_le_bytes())?;

        Ok(())
    }
}

impl Mail {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // message_id: u32
        let message_id = crate::util::read_u32_le(r)?;

        // message_type: MailType
        let message_type: MailType = crate::util::read_u8_le(r)?.try_into()?;

        let message_type_if = match message_type {
            MailType::Normal => {
                // sender: Guid
                let sender = Guid::read(r)?;

                Mail_MailType::Normal {
                    sender,
                }
            }
            MailType::Auction => {
                // auction_id: u32
                let auction_id = crate::util::read_u32_le(r)?;

                Mail_MailType::Auction {
                    auction_id,
                }
            }
            MailType::Creature => {
                // sender_id: u32
                let sender_id = crate::util::read_u32_le(r)?;

                Mail_MailType::Creature {
                    sender_id,
                }
            }
            MailType::Gameobject => {
                // sender_id: u32
                let sender_id = crate::util::read_u32_le(r)?;

                Mail_MailType::Gameobject {
                    sender_id,
                }
            }
            MailType::Item => Mail_MailType::Item,
        };

        // subject: CString
        let subject = crate::util::read_c_string_to_vec(r)?;
        let subject = String::from_utf8(subject)?;

        // item_text_id: u32
        let item_text_id = crate::util::read_u32_le(r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(r)?;

        // stationery: u32
        let stationery = crate::util::read_u32_le(r)?;

        // item_id: u32
        let item_id = crate::util::read_u32_le(r)?;

        // item_enchant_id: u32
        let item_enchant_id = crate::util::read_u32_le(r)?;

        // item_random_property_id: u32
        let item_random_property_id = crate::util::read_u32_le(r)?;

        // item_suffix_factor: u32
        let item_suffix_factor = crate::util::read_u32_le(r)?;

        // item_stack_size: u8
        let item_stack_size = crate::util::read_u8_le(r)?;

        // item_spell_charges: u32
        let item_spell_charges = crate::util::read_u32_le(r)?;

        // max_durability: u32
        let max_durability = crate::util::read_u32_le(r)?;

        // durability: u32
        let durability = crate::util::read_u32_le(r)?;

        // money: u32
        let money = crate::util::read_u32_le(r)?;

        // cash_on_delivery_amount: u32
        let cash_on_delivery_amount = crate::util::read_u32_le(r)?;

        // checked_timestamp: u32
        let checked_timestamp = crate::util::read_u32_le(r)?;

        // expiration_time: f32
        let expiration_time = crate::util::read_f32_le(r)?;
        // mail_template_id: u32
        let mail_template_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            message_id,
            message_type: message_type_if,
            subject,
            item_text_id,
            unknown1,
            stationery,
            item_id,
            item_enchant_id,
            item_random_property_id,
            item_suffix_factor,
            item_stack_size,
            item_spell_charges,
            max_durability,
            durability,
            money,
            cash_on_delivery_amount,
            checked_timestamp,
            expiration_time,
            mail_template_id,
        })
    }

}

impl Mail {
    pub(crate) fn size(&self) -> usize {
        4 // message_id: u32
        + self.message_type.size() // message_type: Mail_MailType
        + self.subject.len() + 1 // subject: CString
        + 4 // item_text_id: u32
        + 4 // unknown1: u32
        + 4 // stationery: u32
        + 4 // item_id: u32
        + 4 // item_enchant_id: u32
        + 4 // item_random_property_id: u32
        + 4 // item_suffix_factor: u32
        + 1 // item_stack_size: u8
        + 4 // item_spell_charges: u32
        + 4 // max_durability: u32
        + 4 // durability: u32
        + 4 // money: u32
        + 4 // cash_on_delivery_amount: u32
        + 4 // checked_timestamp: u32
        + 4 // expiration_time: f32
        + 4 // mail_template_id: u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    Item,
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
            Self::Item => 5,
        }
    }

}

impl Mail_MailType {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Normal {
                sender,
            } => {
                1
                + 8 // sender: Guid
            }
            Self::Auction {
                auction_id,
            } => {
                1
                + 4 // auction_id: u32
            }
            Self::Creature {
                sender_id,
            } => {
                1
                + 4 // sender_id: u32
            }
            Self::Gameobject {
                sender_id,
            } => {
                1
                + 4 // sender_id: u32
            }
            Self::Item => {
                1
            }
        }
    }
}

