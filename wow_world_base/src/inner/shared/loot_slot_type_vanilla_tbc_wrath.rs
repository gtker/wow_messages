/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/smsg_loot_response.wowm:67`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/smsg_loot_response.wowm#L67):
/// ```text
/// enum LootSlotType : u8 {
///     TYPE_ALLOW_LOOT = 0;
///     TYPE_ROLL_ONGOING = 1;
///     TYPE_MASTER = 2;
///     TYPE_LOCKED = 3;
///     TYPE_OWNER = 4;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum LootSlotType {
    /// player can loot the item.
    ///
    TypeAllowLoot,
    /// roll is ongoing. player cannot loot.
    ///
    TypeRollOngoing,
    /// item can only be distributed by group loot master.
    ///
    TypeMaster,
    /// item is shown in red. player cannot loot.
    ///
    TypeLocked,
    /// ignore binding confirmation and etc, for single player looting
    ///
    TypeOwner,
}

impl LootSlotType {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::TypeAllowLoot => 0x0,
            Self::TypeRollOngoing => 0x1,
            Self::TypeMaster => 0x2,
            Self::TypeLocked => 0x3,
            Self::TypeOwner => 0x4,
        }
    }

}

impl Default for LootSlotType {
    fn default() -> Self {
        Self::TypeAllowLoot
    }
}

impl std::fmt::Display for LootSlotType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TypeAllowLoot => f.write_str("TypeAllowLoot"),
            Self::TypeRollOngoing => f.write_str("TypeRollOngoing"),
            Self::TypeMaster => f.write_str("TypeMaster"),
            Self::TypeLocked => f.write_str("TypeLocked"),
            Self::TypeOwner => f.write_str("TypeOwner"),
        }
    }
}

impl TryFrom<u8> for LootSlotType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::TypeAllowLoot),
            1 => Ok(Self::TypeRollOngoing),
            2 => Ok(Self::TypeMaster),
            3 => Ok(Self::TypeLocked),
            4 => Ok(Self::TypeOwner),
            v => Err(crate::errors::EnumError::new("LootSlotType", v as u64),)
        }
    }
}

