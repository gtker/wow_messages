/// azerothcore: `ItemSpelltriggerType` 5 might have changed on 2.4.3/3.0.3: Such auras will be applied on item pickup and removed on item loss - maybe on the other hand the item is destroyed if the aura is removed ('removed on death' of spell 57348 makes me think so)
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm:54`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm#L54):
/// ```text
/// enum SpellTriggerType : u8 {
///     ON_USE = 0;
///     ON_EQUIP = 1;
///     CHANCE_ON_HIT = 2;
///     SERVER_SIDE_SCRIPT = 3;
///     SOULSTONE = 4;
///     NO_EQUIP_COOLDOWN = 5;
///     LEARN_SPELL_ID = 6;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum SpellTriggerType {
    OnUse,
    OnEquip,
    ChanceOnHit,
    /// cmangos-tbc: Only used by 23442, Glowing Sanctified Crystal which is used for a Hellfire Peninsula quest.
    /// Unknown why exactly it does not use the normal triggers.
    ServerSideScript,
    Soulstone,
    NoEquipCooldown,
    LearnSpellId,
}

impl SpellTriggerType {
    pub const fn as_int(&self) -> u8 {
        match self {
            Self::OnUse => 0x0,
            Self::OnEquip => 0x1,
            Self::ChanceOnHit => 0x2,
            Self::ServerSideScript => 0x3,
            Self::Soulstone => 0x4,
            Self::NoEquipCooldown => 0x5,
            Self::LearnSpellId => 0x6,
        }
    }

}

#[cfg(feature = "print-testcase")]
impl SpellTriggerType {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::OnUse => "ON_USE",
            Self::OnEquip => "ON_EQUIP",
            Self::ChanceOnHit => "CHANCE_ON_HIT",
            Self::ServerSideScript => "SERVER_SIDE_SCRIPT",
            Self::Soulstone => "SOULSTONE",
            Self::NoEquipCooldown => "NO_EQUIP_COOLDOWN",
            Self::LearnSpellId => "LEARN_SPELL_ID",
        }
    }

}

const NAME: &str = "SpellTriggerType";

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
            Self::ServerSideScript => f.write_str("ServerSideScript"),
            Self::Soulstone => f.write_str("Soulstone"),
            Self::NoEquipCooldown => f.write_str("NoEquipCooldown"),
            Self::LearnSpellId => f.write_str("LearnSpellId"),
        }
    }
}

impl TryFrom<u8> for SpellTriggerType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::OnUse),
            1 => Ok(Self::OnEquip),
            2 => Ok(Self::ChanceOnHit),
            3 => Ok(Self::ServerSideScript),
            4 => Ok(Self::Soulstone),
            5 => Ok(Self::NoEquipCooldown),
            6 => Ok(Self::LearnSpellId),
            v => Err(crate::errors::EnumError::new(NAME, v.into()),)
        }
    }
}

impl TryFrom<u16> for SpellTriggerType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u32> for SpellTriggerType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for SpellTriggerType {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for SpellTriggerType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for SpellTriggerType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for SpellTriggerType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for SpellTriggerType {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for SpellTriggerType {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u8>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

