/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/item/smsg_item_refund_result.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/item/smsg_item_refund_result.wowm#L1):
/// ```text
/// enum ItemRefundResult : u8 {
///     SUCCESS = 0;
///     FAILURE = 10;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub(crate) enum ItemRefundResult {
    Success,
    Failure,
}

impl ItemRefundResult {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Success => 0x0,
            Self::Failure => 0xa,
        }
    }

}

impl Default for ItemRefundResult {
    fn default() -> Self {
        Self::Success
    }
}

impl std::fmt::Display for ItemRefundResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success => f.write_str("Success"),
            Self::Failure => f.write_str("Failure"),
        }
    }
}

impl TryFrom<u8> for ItemRefundResult {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Success),
            10 => Ok(Self::Failure),
            v => Err(crate::errors::EnumError::new("ItemRefundResult", v as u64),)
        }
    }
}

