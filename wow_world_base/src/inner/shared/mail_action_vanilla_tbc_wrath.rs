/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/mail/smsg_send_mail_result.wowm:40`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mail/smsg_send_mail_result.wowm#L40):
/// ```text
/// enum MailAction : u32 {
///     SEND = 0;
///     MONEY_TAKEN = 1;
///     ITEM_TAKEN = 2;
///     RETURNED_TO_SENDER = 3;
///     DELETED = 4;
///     MADE_PERMANENT = 5;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum MailAction {
    Send,
    MoneyTaken,
    ItemTaken,
    ReturnedToSender,
    Deleted,
    MadePermanent,
}

impl MailAction {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::Send => 0x0,
            Self::MoneyTaken => 0x1,
            Self::ItemTaken => 0x2,
            Self::ReturnedToSender => 0x3,
            Self::Deleted => 0x4,
            Self::MadePermanent => 0x5,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl MailAction {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Send => "SEND",
            Self::MoneyTaken => "MONEY_TAKEN",
            Self::ItemTaken => "ITEM_TAKEN",
            Self::ReturnedToSender => "RETURNED_TO_SENDER",
            Self::Deleted => "DELETED",
            Self::MadePermanent => "MADE_PERMANENT",
        }
    }

}

impl Default for MailAction {
    fn default() -> Self {
        Self::Send
    }
}

impl std::fmt::Display for MailAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Send => f.write_str("Send"),
            Self::MoneyTaken => f.write_str("MoneyTaken"),
            Self::ItemTaken => f.write_str("ItemTaken"),
            Self::ReturnedToSender => f.write_str("ReturnedToSender"),
            Self::Deleted => f.write_str("Deleted"),
            Self::MadePermanent => f.write_str("MadePermanent"),
        }
    }
}

impl TryFrom<u32> for MailAction {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Send),
            1 => Ok(Self::MoneyTaken),
            2 => Ok(Self::ItemTaken),
            3 => Ok(Self::ReturnedToSender),
            4 => Ok(Self::Deleted),
            5 => Ok(Self::MadePermanent),
            v => Err(crate::errors::EnumError::new("MailAction", v as u64),)
        }
    }
}

