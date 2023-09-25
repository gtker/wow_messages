/// mangoszero: in `SMSG_SEND_MAIL_RESULT`, 7-13 and 16+: 'Mail database error'
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/mail/smsg_send_mail_result.wowm:21`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/mail/smsg_send_mail_result.wowm#L21):
/// ```text
/// enum MailResult : u32 {
///     OK = 0;
///     ERR_EQUIP_ERROR = 1;
///     ERR_CANNOT_SEND_TO_SELF = 2;
///     ERR_NOT_ENOUGH_MONEY = 3;
///     ERR_RECIPIENT_NOT_FOUND = 4;
///     ERR_NOT_YOUR_TEAM = 5;
///     ERR_INTERNAL_ERROR = 6;
///     ERR_DISABLED_FOR_TRIAL_ACC = 14;
///     ERR_RECIPIENT_CAP_REACHED = 15;
///     ERR_CANT_SEND_WRAPPED_COD = 16;
///     ERR_MAIL_AND_CHAT_SUSPENDED = 17;
///     ERR_TOO_MANY_ATTACHMENTS = 18;
///     ERR_MAIL_ATTACHMENT_INVALID = 19;
///     ERR_ITEM_HAS_EXPIRED = 21;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum MailResult {
    Ok,
    ErrEquipError,
    ErrCannotSendToSelf,
    ErrNotEnoughMoney,
    ErrRecipientNotFound,
    ErrNotYourTeam,
    ErrInternalError,
    ErrDisabledForTrialAcc,
    ErrRecipientCapReached,
    ErrCantSendWrappedCod,
    ErrMailAndChatSuspended,
    ErrTooManyAttachments,
    ErrMailAttachmentInvalid,
    ErrItemHasExpired,
}

impl MailResult {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::Ok => 0x0,
            Self::ErrEquipError => 0x1,
            Self::ErrCannotSendToSelf => 0x2,
            Self::ErrNotEnoughMoney => 0x3,
            Self::ErrRecipientNotFound => 0x4,
            Self::ErrNotYourTeam => 0x5,
            Self::ErrInternalError => 0x6,
            Self::ErrDisabledForTrialAcc => 0xe,
            Self::ErrRecipientCapReached => 0xf,
            Self::ErrCantSendWrappedCod => 0x10,
            Self::ErrMailAndChatSuspended => 0x11,
            Self::ErrTooManyAttachments => 0x12,
            Self::ErrMailAttachmentInvalid => 0x13,
            Self::ErrItemHasExpired => 0x15,
        }
    }

    pub const fn variants() -> [Self; 14] {
        [
            Self::Ok,
            Self::ErrEquipError,
            Self::ErrCannotSendToSelf,
            Self::ErrNotEnoughMoney,
            Self::ErrRecipientNotFound,
            Self::ErrNotYourTeam,
            Self::ErrInternalError,
            Self::ErrDisabledForTrialAcc,
            Self::ErrRecipientCapReached,
            Self::ErrCantSendWrappedCod,
            Self::ErrMailAndChatSuspended,
            Self::ErrTooManyAttachments,
            Self::ErrMailAttachmentInvalid,
            Self::ErrItemHasExpired,
        ]
    }

    pub const fn from_int(value: u32) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::Ok),
            1 => Ok(Self::ErrEquipError),
            2 => Ok(Self::ErrCannotSendToSelf),
            3 => Ok(Self::ErrNotEnoughMoney),
            4 => Ok(Self::ErrRecipientNotFound),
            5 => Ok(Self::ErrNotYourTeam),
            6 => Ok(Self::ErrInternalError),
            14 => Ok(Self::ErrDisabledForTrialAcc),
            15 => Ok(Self::ErrRecipientCapReached),
            16 => Ok(Self::ErrCantSendWrappedCod),
            17 => Ok(Self::ErrMailAndChatSuspended),
            18 => Ok(Self::ErrTooManyAttachments),
            19 => Ok(Self::ErrMailAttachmentInvalid),
            21 => Ok(Self::ErrItemHasExpired),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl MailResult {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::Ok => "OK",
            Self::ErrEquipError => "ERR_EQUIP_ERROR",
            Self::ErrCannotSendToSelf => "ERR_CANNOT_SEND_TO_SELF",
            Self::ErrNotEnoughMoney => "ERR_NOT_ENOUGH_MONEY",
            Self::ErrRecipientNotFound => "ERR_RECIPIENT_NOT_FOUND",
            Self::ErrNotYourTeam => "ERR_NOT_YOUR_TEAM",
            Self::ErrInternalError => "ERR_INTERNAL_ERROR",
            Self::ErrDisabledForTrialAcc => "ERR_DISABLED_FOR_TRIAL_ACC",
            Self::ErrRecipientCapReached => "ERR_RECIPIENT_CAP_REACHED",
            Self::ErrCantSendWrappedCod => "ERR_CANT_SEND_WRAPPED_COD",
            Self::ErrMailAndChatSuspended => "ERR_MAIL_AND_CHAT_SUSPENDED",
            Self::ErrTooManyAttachments => "ERR_TOO_MANY_ATTACHMENTS",
            Self::ErrMailAttachmentInvalid => "ERR_MAIL_ATTACHMENT_INVALID",
            Self::ErrItemHasExpired => "ERR_ITEM_HAS_EXPIRED",
        }
    }

}

const NAME: &str = "MailResult";

impl Default for MailResult {
    fn default() -> Self {
        Self::Ok
    }
}

impl std::fmt::Display for MailResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok => f.write_str("Ok"),
            Self::ErrEquipError => f.write_str("ErrEquipError"),
            Self::ErrCannotSendToSelf => f.write_str("ErrCannotSendToSelf"),
            Self::ErrNotEnoughMoney => f.write_str("ErrNotEnoughMoney"),
            Self::ErrRecipientNotFound => f.write_str("ErrRecipientNotFound"),
            Self::ErrNotYourTeam => f.write_str("ErrNotYourTeam"),
            Self::ErrInternalError => f.write_str("ErrInternalError"),
            Self::ErrDisabledForTrialAcc => f.write_str("ErrDisabledForTrialAcc"),
            Self::ErrRecipientCapReached => f.write_str("ErrRecipientCapReached"),
            Self::ErrCantSendWrappedCod => f.write_str("ErrCantSendWrappedCod"),
            Self::ErrMailAndChatSuspended => f.write_str("ErrMailAndChatSuspended"),
            Self::ErrTooManyAttachments => f.write_str("ErrTooManyAttachments"),
            Self::ErrMailAttachmentInvalid => f.write_str("ErrMailAttachmentInvalid"),
            Self::ErrItemHasExpired => f.write_str("ErrItemHasExpired"),
        }
    }
}

impl TryFrom<u32> for MailResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u8> for MailResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u16> for MailResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for MailResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for MailResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for MailResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for MailResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for MailResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for MailResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

