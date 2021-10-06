use std::convert::{TryFrom, TryInto};
use crate::world::helper::Guid;
use crate::world::v1::v12::{MailType, MailTypeError};
use crate::{ConstantSized, MaximumPossibleSized, ReadableAndWritable, VariableSized};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/needs_else_if_else.wowm:117`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/needs_else_if_else.wowm):
/// ```text
/// struct Mail {
///     u32 message_id;
///     MailType message_type;
///     if (message_type == NORMAL) {
///         Guid sender;
///         ELSE-IF-STATEMENT-DOCC: unimplemented
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
    pub message_type: MailMailType,
    pub subject: String,
    pub item_text_id: u32,
    pub unknown1: u32,
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
    pub checked_timestamp: u32,
    pub expiration_time: f32,
    pub mail_template_id: u32,
}

impl ReadableAndWritable for Mail {
    type Error = MailError;

    fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, Self::Error> {
        // message_id: u32
        let message_id = crate::util::read_u32_le(r)?;

        // message_type: MailType
        let message_type = MailType::read(r)?;

        let message_type_if = match message_type {
            MailType::NORMAL => {
                // sender: Guid
                let sender = Guid::read(r)?;

                MailMailType::NORMAL {
                    sender,
                }
            }
            MailType::AUCTION => {
                // auction_id: u32
                let auction_id = crate::util::read_u32_le(r)?;

                MailMailType::AUCTION {
                    auction_id,
                }
            }
            MailType::CREATURE => {
                // sender_id: u32
                let sender_id = crate::util::read_u32_le(r)?;

                MailMailType::CREATURE {
                    sender_id,
                }
            }
            MailType::GAMEOBJECT => {
                // sender_id: u32
                let sender_id = crate::util::read_u32_le(r)?;

                MailMailType::GAMEOBJECT {
                    sender_id,
                }
            }
            MailType::ITEM => MailMailType::ITEM,
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

    fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        // message_id: u32
        w.write_all(&self.message_id.to_le_bytes())?;

        // message_type: MailType
        self.message_type.write(w)?;

        match &self.message_type {
            MailMailType::NORMAL {
                sender,
            } => {
                // sender: Guid
                sender.write(w)?;

            }
            MailMailType::AUCTION {
                auction_id,
            } => {
                // auction_id: u32
                w.write_all(&auction_id.to_le_bytes())?;

            }
            MailMailType::CREATURE {
                sender_id,
            } => {
                // sender_id: u32
                w.write_all(&sender_id.to_le_bytes())?;

            }
            MailMailType::GAMEOBJECT {
                sender_id,
            } => {
                // sender_id: u32
                w.write_all(&sender_id.to_le_bytes())?;

            }
            MailMailType::ITEM => {}
        }

        // subject: CString
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

impl VariableSized for Mail {
    fn size(&self) -> usize {
        4 // message_id: u32
        + self.message_type.size() // message_type: MailType and subfields
        + self.subject.len() + 1 // subject: CString and Null Terminator
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

impl MaximumPossibleSized for Mail {
    fn maximum_possible_size() -> usize {
        4 // message_id: u32
        + MailType::maximum_possible_size() // message_type: MailType
        + 256 // subject: CString
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

#[derive(Debug)]
pub enum MailError {
    Io(std::io::Error),
    String(std::string::FromUtf8Error),
    MailType(MailTypeError),
}

impl std::error::Error for MailError {}
impl std::fmt::Display for MailError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(i) => i.fmt(f),
            Self::String(i) => i.fmt(f),
            Self::MailType(i) => i.fmt(f),
        }
    }
}

impl From<std::io::Error> for MailError {
    fn from(e : std::io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<std::string::FromUtf8Error> for MailError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        Self::String(e)
    }
}

impl From<MailTypeError> for MailError {
    fn from(e: MailTypeError) -> Self {
        Self::MailType(e)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum MailMailType {
    NORMAL {
        sender: Guid,
    },
    AUCTION {
        auction_id: u32,
    },
    CREATURE {
        sender_id: u32,
    },
    GAMEOBJECT {
        sender_id: u32,
    },
    ITEM,
}

impl From<&MailType> for MailMailType {
    fn from(e: &MailType) -> Self {
        match &e {
            MailType::NORMAL => Self::NORMAL {
                sender: Default::default(),
            },
            MailType::AUCTION => Self::AUCTION {
                auction_id: Default::default(),
            },
            MailType::CREATURE => Self::CREATURE {
                sender_id: Default::default(),
            },
            MailType::GAMEOBJECT => Self::GAMEOBJECT {
                sender_id: Default::default(),
            },
            MailType::ITEM => Self::ITEM,
        }
    }
}

impl From<&MailMailType> for MailType {
    fn from(v: &MailMailType) -> Self {
        match &v {
            MailMailType::NORMAL { .. } => Self::NORMAL,
            MailMailType::AUCTION { .. } => Self::AUCTION,
            MailMailType::CREATURE { .. } => Self::CREATURE,
            MailMailType::GAMEOBJECT { .. } => Self::GAMEOBJECT,
            MailMailType::ITEM => Self::ITEM,
        }
    }
}

impl Default for MailMailType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::NORMAL {
            sender: Default::default(),
        }
    }
}

impl MailMailType {
    pub fn write<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: MailType = self.into();
        a.write(w)?;
        Ok(())
    }

    pub fn write_u16_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: MailType = self.into();
        a.write_u16_le(w)
    }

    pub fn write_u16_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: MailType = self.into();
        a.write_u16_be(w)
    }

    pub fn write_u32_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: MailType = self.into();
        a.write_u32_le(w)
    }

    pub fn write_u32_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: MailType = self.into();
        a.write_u32_be(w)
    }

    pub fn write_u64_le<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: MailType = self.into();
        a.write_u64_le(w)
    }

    pub fn write_u64_be<W: std::io::Write>(&self, w: &mut W) -> std::result::Result<(), std::io::Error> {
        let a: MailType = self.into();
        a.write_u64_be(w)
    }

}

impl VariableSized for MailMailType {
    fn size(&self) -> usize {
        match self {
            Self::NORMAL  {
                sender,
            } => {
                1
                + 8 // sender: Guid
            }
            Self::AUCTION  {
                auction_id,
            } => {
                1
                + 4 // auction_id: u32
            }
            Self::CREATURE  {
                sender_id,
            } => {
                1
                + 4 // sender_id: u32
            }
            Self::GAMEOBJECT  {
                sender_id,
            } => {
                1
                + 4 // sender_id: u32
            }
            Self::ITEM =>  {
                1
            }
        }
    }
}

impl MaximumPossibleSized for MailMailType {
    fn maximum_possible_size() -> usize {
        65536 // maximum possible u16 size. TODO value.
    }
}

