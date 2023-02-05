use std::convert::{TryFrom, TryInto};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm:31`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm#L31):
/// ```text
/// enum SpellTriggerType : u8 {
///     ON_USE = 0;
///     ON_EQUIP = 1;
///     CHANCE_ON_HIT = 2;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum SpellTriggerType {
    OnUse,
    OnEquip,
    ChanceOnHit,
}

impl SpellTriggerType {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::OnUse => 0x0,
            Self::OnEquip => 0x1,
            Self::ChanceOnHit => 0x2,
        }
    }

}

impl Default for SpellTriggerType {
    fn default() -> Self {
        Self::OnUse
    }
}

impl std::fmt::Display for SpellTriggerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OnUse => f.write_str("OnUse"),
            Self::OnEquip => f.write_str("OnEquip"),
            Self::ChanceOnHit => f.write_str("ChanceOnHit"),
        }
    }
}

impl TryFrom<u8> for SpellTriggerType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::OnUse),
            1 => Ok(Self::OnEquip),
            2 => Ok(Self::ChanceOnHit),
            v => Err(crate::errors::EnumError::new("SpellTriggerType", v as u64),)
        }
    }
}

