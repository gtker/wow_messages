/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spelllogexecute.wowm:10`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spelllogexecute.wowm#L10):
/// ```text
/// enum SpellEffect : u32 {
///     NONE = 0;
///     INSTAKILL = 1;
///     SCHOOL_DAMAGE = 2;
///     DUMMY = 3;
///     PORTAL_TELEPORT = 4;
///     TELEPORT_UNITS = 5;
///     APPLY_AURA = 6;
///     ENVIRONMENTAL_DAMAGE = 7;
///     POWER_DRAIN = 8;
///     HEALTH_LEECH = 9;
///     HEAL = 10;
///     BIND = 11;
///     PORTAL = 12;
///     RITUAL_BASE = 13;
///     RITUAL_SPECIALIZE = 14;
///     RITUAL_ACTIVATE_PORTAL = 15;
///     QUEST_COMPLETE = 16;
///     WEAPON_DAMAGE_NOSCHOOL = 17;
///     RESURRECT = 18;
///     ADD_EXTRA_ATTACKS = 19;
///     DODGE = 20;
///     EVADE = 21;
///     PARRY = 22;
///     BLOCK = 23;
///     CREATE_ITEM = 24;
///     WEAPON = 25;
///     DEFENSE = 26;
///     PERSISTENT_AREA_AURA = 27;
///     SUMMON = 28;
///     LEAP = 29;
///     ENERGIZE = 30;
///     WEAPON_PERCENT_DAMAGE = 31;
///     TRIGGER_MISSILE = 32;
///     OPEN_LOCK = 33;
///     SUMMON_CHANGE_ITEM = 34;
///     APPLY_AREA_AURA_PARTY = 35;
///     LEARN_SPELL = 36;
///     SPELL_DEFENSE = 37;
///     DISPEL = 38;
///     LANGUAGE = 39;
///     DUAL_WIELD = 40;
///     SUMMON_WILD = 41;
///     SUMMON_GUARDIAN = 42;
///     TELEPORT_UNITS_FACE_CASTER = 43;
///     SKILL_STEP = 44;
///     ADD_HONOR = 45;
///     SPAWN = 46;
///     TRADE_SKILL = 47;
///     STEALTH = 48;
///     DETECT = 49;
///     TRANS_DOOR = 50;
///     FORCE_CRITICAL_HIT = 51;
///     GUARANTEE_HIT = 52;
///     ENCHANT_ITEM = 53;
///     ENCHANT_ITEM_TEMPORARY = 54;
///     TAMECREATURE = 55;
///     SUMMON_PET = 56;
///     LEARN_PET_SPELL = 57;
///     WEAPON_DAMAGE = 58;
///     OPEN_LOCK_ITEM = 59;
///     PROFICIENCY = 60;
///     SEND_EVENT = 61;
///     POWER_BURN = 62;
///     THREAT = 63;
///     TRIGGER_SPELL = 64;
///     HEALTH_FUNNEL = 65;
///     POWER_FUNNEL = 66;
///     HEAL_MAX_HEALTH = 67;
///     INTERRUPT_CAST = 68;
///     DISTRACT = 69;
///     PULL = 70;
///     PICKPOCKET = 71;
///     ADD_FARSIGHT = 72;
///     SUMMON_POSSESSED = 73;
///     SUMMON_TOTEM = 74;
///     HEAL_MECHANICAL = 75;
///     SUMMON_OBJECT_WILD = 76;
///     SCRIPT_EFFECT = 77;
///     ATTACK = 78;
///     SANCTUARY = 79;
///     ADD_COMBO_POINTS = 80;
///     CREATE_HOUSE = 81;
///     BIND_SIGHT = 82;
///     DUEL = 83;
///     STUCK = 84;
///     SUMMON_PLAYER = 85;
///     ACTIVATE_OBJECT = 86;
///     SUMMON_TOTEM_SLOT1 = 87;
///     SUMMON_TOTEM_SLOT2 = 88;
///     SUMMON_TOTEM_SLOT3 = 89;
///     SUMMON_TOTEM_SLOT4 = 90;
///     THREAT_ALL = 91;
///     ENCHANT_HELD_ITEM = 92;
///     SUMMON_PHANTASM = 93;
///     SELF_RESURRECT = 94;
///     SKINNING = 95;
///     CHARGE = 96;
///     SUMMON_CRITTER = 97;
///     KNOCK_BACK = 98;
///     DISENCHANT = 99;
///     INEBRIATE = 100;
///     FEED_PET = 101;
///     DISMISS_PET = 102;
///     REPUTATION = 103;
///     SUMMON_OBJECT_SLOT1 = 104;
///     SUMMON_OBJECT_SLOT2 = 105;
///     SUMMON_OBJECT_SLOT3 = 106;
///     SUMMON_OBJECT_SLOT4 = 107;
///     DISPEL_MECHANIC = 108;
///     SUMMON_DEAD_PET = 109;
///     DESTROY_ALL_TOTEMS = 110;
///     DURABILITY_DAMAGE = 111;
///     SUMMON_DEMON = 112;
///     RESURRECT_NEW = 113;
///     ATTACK_ME = 114;
///     DURABILITY_DAMAGE_PCT = 115;
///     SKIN_PLAYER_CORPSE = 116;
///     SPIRIT_HEAL = 117;
///     SKILL = 118;
///     APPLY_AREA_AURA_PET = 119;
///     TELEPORT_GRAVEYARD = 120;
///     NORMALIZED_WEAPON_DMG = 121;
///     UNKNOWN122 = 122;
///     SEND_TAXI = 123;
///     PLAYER_PULL = 124;
///     MODIFY_THREAT_PERCENT = 125;
///     UNKNOWN126 = 126;
///     UNKNOWN127 = 127;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum SpellEffect {
    None,
    Instakill,
    SchoolDamage,
    Dummy,
    PortalTeleport,
    TeleportUnits,
    ApplyAura,
    EnvironmentalDamage,
    PowerDrain,
    HealthLeech,
    Heal,
    Bind,
    Portal,
    RitualBase,
    RitualSpecialize,
    RitualActivatePortal,
    QuestComplete,
    WeaponDamageNoschool,
    Resurrect,
    AddExtraAttacks,
    Dodge,
    Evade,
    Parry,
    Block,
    CreateItem,
    Weapon,
    Defense,
    PersistentAreaAura,
    Summon,
    Leap,
    Energize,
    WeaponPercentDamage,
    TriggerMissile,
    OpenLock,
    SummonChangeItem,
    ApplyAreaAuraParty,
    LearnSpell,
    SpellDefense,
    Dispel,
    Language,
    DualWield,
    SummonWild,
    SummonGuardian,
    TeleportUnitsFaceCaster,
    SkillStep,
    AddHonor,
    Spawn,
    TradeSkill,
    Stealth,
    Detect,
    TransDoor,
    ForceCriticalHit,
    GuaranteeHit,
    EnchantItem,
    EnchantItemTemporary,
    Tamecreature,
    SummonPet,
    LearnPetSpell,
    WeaponDamage,
    OpenLockItem,
    Proficiency,
    SendEvent,
    PowerBurn,
    Threat,
    TriggerSpell,
    HealthFunnel,
    PowerFunnel,
    HealMaxHealth,
    InterruptCast,
    Distract,
    Pull,
    Pickpocket,
    AddFarsight,
    SummonPossessed,
    SummonTotem,
    HealMechanical,
    SummonObjectWild,
    ScriptEffect,
    Attack,
    Sanctuary,
    AddComboPoints,
    CreateHouse,
    BindSight,
    Duel,
    Stuck,
    SummonPlayer,
    ActivateObject,
    SummonTotemSlot1,
    SummonTotemSlot2,
    SummonTotemSlot3,
    SummonTotemSlot4,
    ThreatAll,
    EnchantHeldItem,
    SummonPhantasm,
    SelfResurrect,
    Skinning,
    Charge,
    SummonCritter,
    KnockBack,
    Disenchant,
    Inebriate,
    FeedPet,
    DismissPet,
    Reputation,
    SummonObjectSlot1,
    SummonObjectSlot2,
    SummonObjectSlot3,
    SummonObjectSlot4,
    DispelMechanic,
    SummonDeadPet,
    DestroyAllTotems,
    DurabilityDamage,
    SummonDemon,
    ResurrectNew,
    AttackMe,
    DurabilityDamagePct,
    SkinPlayerCorpse,
    SpiritHeal,
    Skill,
    ApplyAreaAuraPet,
    TeleportGraveyard,
    NormalizedWeaponDmg,
    Unknown122,
    SendTaxi,
    PlayerPull,
    ModifyThreatPercent,
    Unknown126,
    Unknown127,
}

impl SpellEffect {
    pub const fn as_int(&self) -> u32 {
        match self {
            Self::None => 0x0,
            Self::Instakill => 0x1,
            Self::SchoolDamage => 0x2,
            Self::Dummy => 0x3,
            Self::PortalTeleport => 0x4,
            Self::TeleportUnits => 0x5,
            Self::ApplyAura => 0x6,
            Self::EnvironmentalDamage => 0x7,
            Self::PowerDrain => 0x8,
            Self::HealthLeech => 0x9,
            Self::Heal => 0xa,
            Self::Bind => 0xb,
            Self::Portal => 0xc,
            Self::RitualBase => 0xd,
            Self::RitualSpecialize => 0xe,
            Self::RitualActivatePortal => 0xf,
            Self::QuestComplete => 0x10,
            Self::WeaponDamageNoschool => 0x11,
            Self::Resurrect => 0x12,
            Self::AddExtraAttacks => 0x13,
            Self::Dodge => 0x14,
            Self::Evade => 0x15,
            Self::Parry => 0x16,
            Self::Block => 0x17,
            Self::CreateItem => 0x18,
            Self::Weapon => 0x19,
            Self::Defense => 0x1a,
            Self::PersistentAreaAura => 0x1b,
            Self::Summon => 0x1c,
            Self::Leap => 0x1d,
            Self::Energize => 0x1e,
            Self::WeaponPercentDamage => 0x1f,
            Self::TriggerMissile => 0x20,
            Self::OpenLock => 0x21,
            Self::SummonChangeItem => 0x22,
            Self::ApplyAreaAuraParty => 0x23,
            Self::LearnSpell => 0x24,
            Self::SpellDefense => 0x25,
            Self::Dispel => 0x26,
            Self::Language => 0x27,
            Self::DualWield => 0x28,
            Self::SummonWild => 0x29,
            Self::SummonGuardian => 0x2a,
            Self::TeleportUnitsFaceCaster => 0x2b,
            Self::SkillStep => 0x2c,
            Self::AddHonor => 0x2d,
            Self::Spawn => 0x2e,
            Self::TradeSkill => 0x2f,
            Self::Stealth => 0x30,
            Self::Detect => 0x31,
            Self::TransDoor => 0x32,
            Self::ForceCriticalHit => 0x33,
            Self::GuaranteeHit => 0x34,
            Self::EnchantItem => 0x35,
            Self::EnchantItemTemporary => 0x36,
            Self::Tamecreature => 0x37,
            Self::SummonPet => 0x38,
            Self::LearnPetSpell => 0x39,
            Self::WeaponDamage => 0x3a,
            Self::OpenLockItem => 0x3b,
            Self::Proficiency => 0x3c,
            Self::SendEvent => 0x3d,
            Self::PowerBurn => 0x3e,
            Self::Threat => 0x3f,
            Self::TriggerSpell => 0x40,
            Self::HealthFunnel => 0x41,
            Self::PowerFunnel => 0x42,
            Self::HealMaxHealth => 0x43,
            Self::InterruptCast => 0x44,
            Self::Distract => 0x45,
            Self::Pull => 0x46,
            Self::Pickpocket => 0x47,
            Self::AddFarsight => 0x48,
            Self::SummonPossessed => 0x49,
            Self::SummonTotem => 0x4a,
            Self::HealMechanical => 0x4b,
            Self::SummonObjectWild => 0x4c,
            Self::ScriptEffect => 0x4d,
            Self::Attack => 0x4e,
            Self::Sanctuary => 0x4f,
            Self::AddComboPoints => 0x50,
            Self::CreateHouse => 0x51,
            Self::BindSight => 0x52,
            Self::Duel => 0x53,
            Self::Stuck => 0x54,
            Self::SummonPlayer => 0x55,
            Self::ActivateObject => 0x56,
            Self::SummonTotemSlot1 => 0x57,
            Self::SummonTotemSlot2 => 0x58,
            Self::SummonTotemSlot3 => 0x59,
            Self::SummonTotemSlot4 => 0x5a,
            Self::ThreatAll => 0x5b,
            Self::EnchantHeldItem => 0x5c,
            Self::SummonPhantasm => 0x5d,
            Self::SelfResurrect => 0x5e,
            Self::Skinning => 0x5f,
            Self::Charge => 0x60,
            Self::SummonCritter => 0x61,
            Self::KnockBack => 0x62,
            Self::Disenchant => 0x63,
            Self::Inebriate => 0x64,
            Self::FeedPet => 0x65,
            Self::DismissPet => 0x66,
            Self::Reputation => 0x67,
            Self::SummonObjectSlot1 => 0x68,
            Self::SummonObjectSlot2 => 0x69,
            Self::SummonObjectSlot3 => 0x6a,
            Self::SummonObjectSlot4 => 0x6b,
            Self::DispelMechanic => 0x6c,
            Self::SummonDeadPet => 0x6d,
            Self::DestroyAllTotems => 0x6e,
            Self::DurabilityDamage => 0x6f,
            Self::SummonDemon => 0x70,
            Self::ResurrectNew => 0x71,
            Self::AttackMe => 0x72,
            Self::DurabilityDamagePct => 0x73,
            Self::SkinPlayerCorpse => 0x74,
            Self::SpiritHeal => 0x75,
            Self::Skill => 0x76,
            Self::ApplyAreaAuraPet => 0x77,
            Self::TeleportGraveyard => 0x78,
            Self::NormalizedWeaponDmg => 0x79,
            Self::Unknown122 => 0x7a,
            Self::SendTaxi => 0x7b,
            Self::PlayerPull => 0x7c,
            Self::ModifyThreatPercent => 0x7d,
            Self::Unknown126 => 0x7e,
            Self::Unknown127 => 0x7f,
        }
    }

    pub const fn variants() -> [Self; 128] {
        [
            Self::None,
            Self::Instakill,
            Self::SchoolDamage,
            Self::Dummy,
            Self::PortalTeleport,
            Self::TeleportUnits,
            Self::ApplyAura,
            Self::EnvironmentalDamage,
            Self::PowerDrain,
            Self::HealthLeech,
            Self::Heal,
            Self::Bind,
            Self::Portal,
            Self::RitualBase,
            Self::RitualSpecialize,
            Self::RitualActivatePortal,
            Self::QuestComplete,
            Self::WeaponDamageNoschool,
            Self::Resurrect,
            Self::AddExtraAttacks,
            Self::Dodge,
            Self::Evade,
            Self::Parry,
            Self::Block,
            Self::CreateItem,
            Self::Weapon,
            Self::Defense,
            Self::PersistentAreaAura,
            Self::Summon,
            Self::Leap,
            Self::Energize,
            Self::WeaponPercentDamage,
            Self::TriggerMissile,
            Self::OpenLock,
            Self::SummonChangeItem,
            Self::ApplyAreaAuraParty,
            Self::LearnSpell,
            Self::SpellDefense,
            Self::Dispel,
            Self::Language,
            Self::DualWield,
            Self::SummonWild,
            Self::SummonGuardian,
            Self::TeleportUnitsFaceCaster,
            Self::SkillStep,
            Self::AddHonor,
            Self::Spawn,
            Self::TradeSkill,
            Self::Stealth,
            Self::Detect,
            Self::TransDoor,
            Self::ForceCriticalHit,
            Self::GuaranteeHit,
            Self::EnchantItem,
            Self::EnchantItemTemporary,
            Self::Tamecreature,
            Self::SummonPet,
            Self::LearnPetSpell,
            Self::WeaponDamage,
            Self::OpenLockItem,
            Self::Proficiency,
            Self::SendEvent,
            Self::PowerBurn,
            Self::Threat,
            Self::TriggerSpell,
            Self::HealthFunnel,
            Self::PowerFunnel,
            Self::HealMaxHealth,
            Self::InterruptCast,
            Self::Distract,
            Self::Pull,
            Self::Pickpocket,
            Self::AddFarsight,
            Self::SummonPossessed,
            Self::SummonTotem,
            Self::HealMechanical,
            Self::SummonObjectWild,
            Self::ScriptEffect,
            Self::Attack,
            Self::Sanctuary,
            Self::AddComboPoints,
            Self::CreateHouse,
            Self::BindSight,
            Self::Duel,
            Self::Stuck,
            Self::SummonPlayer,
            Self::ActivateObject,
            Self::SummonTotemSlot1,
            Self::SummonTotemSlot2,
            Self::SummonTotemSlot3,
            Self::SummonTotemSlot4,
            Self::ThreatAll,
            Self::EnchantHeldItem,
            Self::SummonPhantasm,
            Self::SelfResurrect,
            Self::Skinning,
            Self::Charge,
            Self::SummonCritter,
            Self::KnockBack,
            Self::Disenchant,
            Self::Inebriate,
            Self::FeedPet,
            Self::DismissPet,
            Self::Reputation,
            Self::SummonObjectSlot1,
            Self::SummonObjectSlot2,
            Self::SummonObjectSlot3,
            Self::SummonObjectSlot4,
            Self::DispelMechanic,
            Self::SummonDeadPet,
            Self::DestroyAllTotems,
            Self::DurabilityDamage,
            Self::SummonDemon,
            Self::ResurrectNew,
            Self::AttackMe,
            Self::DurabilityDamagePct,
            Self::SkinPlayerCorpse,
            Self::SpiritHeal,
            Self::Skill,
            Self::ApplyAreaAuraPet,
            Self::TeleportGraveyard,
            Self::NormalizedWeaponDmg,
            Self::Unknown122,
            Self::SendTaxi,
            Self::PlayerPull,
            Self::ModifyThreatPercent,
            Self::Unknown126,
            Self::Unknown127,
        ]
    }

    pub const fn from_int(value: u32) -> Result<Self, crate::errors::EnumError> {
        match value {
            0 => Ok(Self::None),
            1 => Ok(Self::Instakill),
            2 => Ok(Self::SchoolDamage),
            3 => Ok(Self::Dummy),
            4 => Ok(Self::PortalTeleport),
            5 => Ok(Self::TeleportUnits),
            6 => Ok(Self::ApplyAura),
            7 => Ok(Self::EnvironmentalDamage),
            8 => Ok(Self::PowerDrain),
            9 => Ok(Self::HealthLeech),
            10 => Ok(Self::Heal),
            11 => Ok(Self::Bind),
            12 => Ok(Self::Portal),
            13 => Ok(Self::RitualBase),
            14 => Ok(Self::RitualSpecialize),
            15 => Ok(Self::RitualActivatePortal),
            16 => Ok(Self::QuestComplete),
            17 => Ok(Self::WeaponDamageNoschool),
            18 => Ok(Self::Resurrect),
            19 => Ok(Self::AddExtraAttacks),
            20 => Ok(Self::Dodge),
            21 => Ok(Self::Evade),
            22 => Ok(Self::Parry),
            23 => Ok(Self::Block),
            24 => Ok(Self::CreateItem),
            25 => Ok(Self::Weapon),
            26 => Ok(Self::Defense),
            27 => Ok(Self::PersistentAreaAura),
            28 => Ok(Self::Summon),
            29 => Ok(Self::Leap),
            30 => Ok(Self::Energize),
            31 => Ok(Self::WeaponPercentDamage),
            32 => Ok(Self::TriggerMissile),
            33 => Ok(Self::OpenLock),
            34 => Ok(Self::SummonChangeItem),
            35 => Ok(Self::ApplyAreaAuraParty),
            36 => Ok(Self::LearnSpell),
            37 => Ok(Self::SpellDefense),
            38 => Ok(Self::Dispel),
            39 => Ok(Self::Language),
            40 => Ok(Self::DualWield),
            41 => Ok(Self::SummonWild),
            42 => Ok(Self::SummonGuardian),
            43 => Ok(Self::TeleportUnitsFaceCaster),
            44 => Ok(Self::SkillStep),
            45 => Ok(Self::AddHonor),
            46 => Ok(Self::Spawn),
            47 => Ok(Self::TradeSkill),
            48 => Ok(Self::Stealth),
            49 => Ok(Self::Detect),
            50 => Ok(Self::TransDoor),
            51 => Ok(Self::ForceCriticalHit),
            52 => Ok(Self::GuaranteeHit),
            53 => Ok(Self::EnchantItem),
            54 => Ok(Self::EnchantItemTemporary),
            55 => Ok(Self::Tamecreature),
            56 => Ok(Self::SummonPet),
            57 => Ok(Self::LearnPetSpell),
            58 => Ok(Self::WeaponDamage),
            59 => Ok(Self::OpenLockItem),
            60 => Ok(Self::Proficiency),
            61 => Ok(Self::SendEvent),
            62 => Ok(Self::PowerBurn),
            63 => Ok(Self::Threat),
            64 => Ok(Self::TriggerSpell),
            65 => Ok(Self::HealthFunnel),
            66 => Ok(Self::PowerFunnel),
            67 => Ok(Self::HealMaxHealth),
            68 => Ok(Self::InterruptCast),
            69 => Ok(Self::Distract),
            70 => Ok(Self::Pull),
            71 => Ok(Self::Pickpocket),
            72 => Ok(Self::AddFarsight),
            73 => Ok(Self::SummonPossessed),
            74 => Ok(Self::SummonTotem),
            75 => Ok(Self::HealMechanical),
            76 => Ok(Self::SummonObjectWild),
            77 => Ok(Self::ScriptEffect),
            78 => Ok(Self::Attack),
            79 => Ok(Self::Sanctuary),
            80 => Ok(Self::AddComboPoints),
            81 => Ok(Self::CreateHouse),
            82 => Ok(Self::BindSight),
            83 => Ok(Self::Duel),
            84 => Ok(Self::Stuck),
            85 => Ok(Self::SummonPlayer),
            86 => Ok(Self::ActivateObject),
            87 => Ok(Self::SummonTotemSlot1),
            88 => Ok(Self::SummonTotemSlot2),
            89 => Ok(Self::SummonTotemSlot3),
            90 => Ok(Self::SummonTotemSlot4),
            91 => Ok(Self::ThreatAll),
            92 => Ok(Self::EnchantHeldItem),
            93 => Ok(Self::SummonPhantasm),
            94 => Ok(Self::SelfResurrect),
            95 => Ok(Self::Skinning),
            96 => Ok(Self::Charge),
            97 => Ok(Self::SummonCritter),
            98 => Ok(Self::KnockBack),
            99 => Ok(Self::Disenchant),
            100 => Ok(Self::Inebriate),
            101 => Ok(Self::FeedPet),
            102 => Ok(Self::DismissPet),
            103 => Ok(Self::Reputation),
            104 => Ok(Self::SummonObjectSlot1),
            105 => Ok(Self::SummonObjectSlot2),
            106 => Ok(Self::SummonObjectSlot3),
            107 => Ok(Self::SummonObjectSlot4),
            108 => Ok(Self::DispelMechanic),
            109 => Ok(Self::SummonDeadPet),
            110 => Ok(Self::DestroyAllTotems),
            111 => Ok(Self::DurabilityDamage),
            112 => Ok(Self::SummonDemon),
            113 => Ok(Self::ResurrectNew),
            114 => Ok(Self::AttackMe),
            115 => Ok(Self::DurabilityDamagePct),
            116 => Ok(Self::SkinPlayerCorpse),
            117 => Ok(Self::SpiritHeal),
            118 => Ok(Self::Skill),
            119 => Ok(Self::ApplyAreaAuraPet),
            120 => Ok(Self::TeleportGraveyard),
            121 => Ok(Self::NormalizedWeaponDmg),
            122 => Ok(Self::Unknown122),
            123 => Ok(Self::SendTaxi),
            124 => Ok(Self::PlayerPull),
            125 => Ok(Self::ModifyThreatPercent),
            126 => Ok(Self::Unknown126),
            127 => Ok(Self::Unknown127),
            v => Err(crate::errors::EnumError::new(NAME, v as i128),)
        }
    }
}

#[cfg(feature = "print-testcase")]
impl SpellEffect {
    pub const fn as_test_case_value(&self) -> &'static str {
        match self {
            Self::None => "NONE",
            Self::Instakill => "INSTAKILL",
            Self::SchoolDamage => "SCHOOL_DAMAGE",
            Self::Dummy => "DUMMY",
            Self::PortalTeleport => "PORTAL_TELEPORT",
            Self::TeleportUnits => "TELEPORT_UNITS",
            Self::ApplyAura => "APPLY_AURA",
            Self::EnvironmentalDamage => "ENVIRONMENTAL_DAMAGE",
            Self::PowerDrain => "POWER_DRAIN",
            Self::HealthLeech => "HEALTH_LEECH",
            Self::Heal => "HEAL",
            Self::Bind => "BIND",
            Self::Portal => "PORTAL",
            Self::RitualBase => "RITUAL_BASE",
            Self::RitualSpecialize => "RITUAL_SPECIALIZE",
            Self::RitualActivatePortal => "RITUAL_ACTIVATE_PORTAL",
            Self::QuestComplete => "QUEST_COMPLETE",
            Self::WeaponDamageNoschool => "WEAPON_DAMAGE_NOSCHOOL",
            Self::Resurrect => "RESURRECT",
            Self::AddExtraAttacks => "ADD_EXTRA_ATTACKS",
            Self::Dodge => "DODGE",
            Self::Evade => "EVADE",
            Self::Parry => "PARRY",
            Self::Block => "BLOCK",
            Self::CreateItem => "CREATE_ITEM",
            Self::Weapon => "WEAPON",
            Self::Defense => "DEFENSE",
            Self::PersistentAreaAura => "PERSISTENT_AREA_AURA",
            Self::Summon => "SUMMON",
            Self::Leap => "LEAP",
            Self::Energize => "ENERGIZE",
            Self::WeaponPercentDamage => "WEAPON_PERCENT_DAMAGE",
            Self::TriggerMissile => "TRIGGER_MISSILE",
            Self::OpenLock => "OPEN_LOCK",
            Self::SummonChangeItem => "SUMMON_CHANGE_ITEM",
            Self::ApplyAreaAuraParty => "APPLY_AREA_AURA_PARTY",
            Self::LearnSpell => "LEARN_SPELL",
            Self::SpellDefense => "SPELL_DEFENSE",
            Self::Dispel => "DISPEL",
            Self::Language => "LANGUAGE",
            Self::DualWield => "DUAL_WIELD",
            Self::SummonWild => "SUMMON_WILD",
            Self::SummonGuardian => "SUMMON_GUARDIAN",
            Self::TeleportUnitsFaceCaster => "TELEPORT_UNITS_FACE_CASTER",
            Self::SkillStep => "SKILL_STEP",
            Self::AddHonor => "ADD_HONOR",
            Self::Spawn => "SPAWN",
            Self::TradeSkill => "TRADE_SKILL",
            Self::Stealth => "STEALTH",
            Self::Detect => "DETECT",
            Self::TransDoor => "TRANS_DOOR",
            Self::ForceCriticalHit => "FORCE_CRITICAL_HIT",
            Self::GuaranteeHit => "GUARANTEE_HIT",
            Self::EnchantItem => "ENCHANT_ITEM",
            Self::EnchantItemTemporary => "ENCHANT_ITEM_TEMPORARY",
            Self::Tamecreature => "TAMECREATURE",
            Self::SummonPet => "SUMMON_PET",
            Self::LearnPetSpell => "LEARN_PET_SPELL",
            Self::WeaponDamage => "WEAPON_DAMAGE",
            Self::OpenLockItem => "OPEN_LOCK_ITEM",
            Self::Proficiency => "PROFICIENCY",
            Self::SendEvent => "SEND_EVENT",
            Self::PowerBurn => "POWER_BURN",
            Self::Threat => "THREAT",
            Self::TriggerSpell => "TRIGGER_SPELL",
            Self::HealthFunnel => "HEALTH_FUNNEL",
            Self::PowerFunnel => "POWER_FUNNEL",
            Self::HealMaxHealth => "HEAL_MAX_HEALTH",
            Self::InterruptCast => "INTERRUPT_CAST",
            Self::Distract => "DISTRACT",
            Self::Pull => "PULL",
            Self::Pickpocket => "PICKPOCKET",
            Self::AddFarsight => "ADD_FARSIGHT",
            Self::SummonPossessed => "SUMMON_POSSESSED",
            Self::SummonTotem => "SUMMON_TOTEM",
            Self::HealMechanical => "HEAL_MECHANICAL",
            Self::SummonObjectWild => "SUMMON_OBJECT_WILD",
            Self::ScriptEffect => "SCRIPT_EFFECT",
            Self::Attack => "ATTACK",
            Self::Sanctuary => "SANCTUARY",
            Self::AddComboPoints => "ADD_COMBO_POINTS",
            Self::CreateHouse => "CREATE_HOUSE",
            Self::BindSight => "BIND_SIGHT",
            Self::Duel => "DUEL",
            Self::Stuck => "STUCK",
            Self::SummonPlayer => "SUMMON_PLAYER",
            Self::ActivateObject => "ACTIVATE_OBJECT",
            Self::SummonTotemSlot1 => "SUMMON_TOTEM_SLOT1",
            Self::SummonTotemSlot2 => "SUMMON_TOTEM_SLOT2",
            Self::SummonTotemSlot3 => "SUMMON_TOTEM_SLOT3",
            Self::SummonTotemSlot4 => "SUMMON_TOTEM_SLOT4",
            Self::ThreatAll => "THREAT_ALL",
            Self::EnchantHeldItem => "ENCHANT_HELD_ITEM",
            Self::SummonPhantasm => "SUMMON_PHANTASM",
            Self::SelfResurrect => "SELF_RESURRECT",
            Self::Skinning => "SKINNING",
            Self::Charge => "CHARGE",
            Self::SummonCritter => "SUMMON_CRITTER",
            Self::KnockBack => "KNOCK_BACK",
            Self::Disenchant => "DISENCHANT",
            Self::Inebriate => "INEBRIATE",
            Self::FeedPet => "FEED_PET",
            Self::DismissPet => "DISMISS_PET",
            Self::Reputation => "REPUTATION",
            Self::SummonObjectSlot1 => "SUMMON_OBJECT_SLOT1",
            Self::SummonObjectSlot2 => "SUMMON_OBJECT_SLOT2",
            Self::SummonObjectSlot3 => "SUMMON_OBJECT_SLOT3",
            Self::SummonObjectSlot4 => "SUMMON_OBJECT_SLOT4",
            Self::DispelMechanic => "DISPEL_MECHANIC",
            Self::SummonDeadPet => "SUMMON_DEAD_PET",
            Self::DestroyAllTotems => "DESTROY_ALL_TOTEMS",
            Self::DurabilityDamage => "DURABILITY_DAMAGE",
            Self::SummonDemon => "SUMMON_DEMON",
            Self::ResurrectNew => "RESURRECT_NEW",
            Self::AttackMe => "ATTACK_ME",
            Self::DurabilityDamagePct => "DURABILITY_DAMAGE_PCT",
            Self::SkinPlayerCorpse => "SKIN_PLAYER_CORPSE",
            Self::SpiritHeal => "SPIRIT_HEAL",
            Self::Skill => "SKILL",
            Self::ApplyAreaAuraPet => "APPLY_AREA_AURA_PET",
            Self::TeleportGraveyard => "TELEPORT_GRAVEYARD",
            Self::NormalizedWeaponDmg => "NORMALIZED_WEAPON_DMG",
            Self::Unknown122 => "UNKNOWN122",
            Self::SendTaxi => "SEND_TAXI",
            Self::PlayerPull => "PLAYER_PULL",
            Self::ModifyThreatPercent => "MODIFY_THREAT_PERCENT",
            Self::Unknown126 => "UNKNOWN126",
            Self::Unknown127 => "UNKNOWN127",
        }
    }

}

const NAME: &str = "SpellEffect";

impl Default for SpellEffect {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Display for SpellEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("None"),
            Self::Instakill => f.write_str("Instakill"),
            Self::SchoolDamage => f.write_str("SchoolDamage"),
            Self::Dummy => f.write_str("Dummy"),
            Self::PortalTeleport => f.write_str("PortalTeleport"),
            Self::TeleportUnits => f.write_str("TeleportUnits"),
            Self::ApplyAura => f.write_str("ApplyAura"),
            Self::EnvironmentalDamage => f.write_str("EnvironmentalDamage"),
            Self::PowerDrain => f.write_str("PowerDrain"),
            Self::HealthLeech => f.write_str("HealthLeech"),
            Self::Heal => f.write_str("Heal"),
            Self::Bind => f.write_str("Bind"),
            Self::Portal => f.write_str("Portal"),
            Self::RitualBase => f.write_str("RitualBase"),
            Self::RitualSpecialize => f.write_str("RitualSpecialize"),
            Self::RitualActivatePortal => f.write_str("RitualActivatePortal"),
            Self::QuestComplete => f.write_str("QuestComplete"),
            Self::WeaponDamageNoschool => f.write_str("WeaponDamageNoschool"),
            Self::Resurrect => f.write_str("Resurrect"),
            Self::AddExtraAttacks => f.write_str("AddExtraAttacks"),
            Self::Dodge => f.write_str("Dodge"),
            Self::Evade => f.write_str("Evade"),
            Self::Parry => f.write_str("Parry"),
            Self::Block => f.write_str("Block"),
            Self::CreateItem => f.write_str("CreateItem"),
            Self::Weapon => f.write_str("Weapon"),
            Self::Defense => f.write_str("Defense"),
            Self::PersistentAreaAura => f.write_str("PersistentAreaAura"),
            Self::Summon => f.write_str("Summon"),
            Self::Leap => f.write_str("Leap"),
            Self::Energize => f.write_str("Energize"),
            Self::WeaponPercentDamage => f.write_str("WeaponPercentDamage"),
            Self::TriggerMissile => f.write_str("TriggerMissile"),
            Self::OpenLock => f.write_str("OpenLock"),
            Self::SummonChangeItem => f.write_str("SummonChangeItem"),
            Self::ApplyAreaAuraParty => f.write_str("ApplyAreaAuraParty"),
            Self::LearnSpell => f.write_str("LearnSpell"),
            Self::SpellDefense => f.write_str("SpellDefense"),
            Self::Dispel => f.write_str("Dispel"),
            Self::Language => f.write_str("Language"),
            Self::DualWield => f.write_str("DualWield"),
            Self::SummonWild => f.write_str("SummonWild"),
            Self::SummonGuardian => f.write_str("SummonGuardian"),
            Self::TeleportUnitsFaceCaster => f.write_str("TeleportUnitsFaceCaster"),
            Self::SkillStep => f.write_str("SkillStep"),
            Self::AddHonor => f.write_str("AddHonor"),
            Self::Spawn => f.write_str("Spawn"),
            Self::TradeSkill => f.write_str("TradeSkill"),
            Self::Stealth => f.write_str("Stealth"),
            Self::Detect => f.write_str("Detect"),
            Self::TransDoor => f.write_str("TransDoor"),
            Self::ForceCriticalHit => f.write_str("ForceCriticalHit"),
            Self::GuaranteeHit => f.write_str("GuaranteeHit"),
            Self::EnchantItem => f.write_str("EnchantItem"),
            Self::EnchantItemTemporary => f.write_str("EnchantItemTemporary"),
            Self::Tamecreature => f.write_str("Tamecreature"),
            Self::SummonPet => f.write_str("SummonPet"),
            Self::LearnPetSpell => f.write_str("LearnPetSpell"),
            Self::WeaponDamage => f.write_str("WeaponDamage"),
            Self::OpenLockItem => f.write_str("OpenLockItem"),
            Self::Proficiency => f.write_str("Proficiency"),
            Self::SendEvent => f.write_str("SendEvent"),
            Self::PowerBurn => f.write_str("PowerBurn"),
            Self::Threat => f.write_str("Threat"),
            Self::TriggerSpell => f.write_str("TriggerSpell"),
            Self::HealthFunnel => f.write_str("HealthFunnel"),
            Self::PowerFunnel => f.write_str("PowerFunnel"),
            Self::HealMaxHealth => f.write_str("HealMaxHealth"),
            Self::InterruptCast => f.write_str("InterruptCast"),
            Self::Distract => f.write_str("Distract"),
            Self::Pull => f.write_str("Pull"),
            Self::Pickpocket => f.write_str("Pickpocket"),
            Self::AddFarsight => f.write_str("AddFarsight"),
            Self::SummonPossessed => f.write_str("SummonPossessed"),
            Self::SummonTotem => f.write_str("SummonTotem"),
            Self::HealMechanical => f.write_str("HealMechanical"),
            Self::SummonObjectWild => f.write_str("SummonObjectWild"),
            Self::ScriptEffect => f.write_str("ScriptEffect"),
            Self::Attack => f.write_str("Attack"),
            Self::Sanctuary => f.write_str("Sanctuary"),
            Self::AddComboPoints => f.write_str("AddComboPoints"),
            Self::CreateHouse => f.write_str("CreateHouse"),
            Self::BindSight => f.write_str("BindSight"),
            Self::Duel => f.write_str("Duel"),
            Self::Stuck => f.write_str("Stuck"),
            Self::SummonPlayer => f.write_str("SummonPlayer"),
            Self::ActivateObject => f.write_str("ActivateObject"),
            Self::SummonTotemSlot1 => f.write_str("SummonTotemSlot1"),
            Self::SummonTotemSlot2 => f.write_str("SummonTotemSlot2"),
            Self::SummonTotemSlot3 => f.write_str("SummonTotemSlot3"),
            Self::SummonTotemSlot4 => f.write_str("SummonTotemSlot4"),
            Self::ThreatAll => f.write_str("ThreatAll"),
            Self::EnchantHeldItem => f.write_str("EnchantHeldItem"),
            Self::SummonPhantasm => f.write_str("SummonPhantasm"),
            Self::SelfResurrect => f.write_str("SelfResurrect"),
            Self::Skinning => f.write_str("Skinning"),
            Self::Charge => f.write_str("Charge"),
            Self::SummonCritter => f.write_str("SummonCritter"),
            Self::KnockBack => f.write_str("KnockBack"),
            Self::Disenchant => f.write_str("Disenchant"),
            Self::Inebriate => f.write_str("Inebriate"),
            Self::FeedPet => f.write_str("FeedPet"),
            Self::DismissPet => f.write_str("DismissPet"),
            Self::Reputation => f.write_str("Reputation"),
            Self::SummonObjectSlot1 => f.write_str("SummonObjectSlot1"),
            Self::SummonObjectSlot2 => f.write_str("SummonObjectSlot2"),
            Self::SummonObjectSlot3 => f.write_str("SummonObjectSlot3"),
            Self::SummonObjectSlot4 => f.write_str("SummonObjectSlot4"),
            Self::DispelMechanic => f.write_str("DispelMechanic"),
            Self::SummonDeadPet => f.write_str("SummonDeadPet"),
            Self::DestroyAllTotems => f.write_str("DestroyAllTotems"),
            Self::DurabilityDamage => f.write_str("DurabilityDamage"),
            Self::SummonDemon => f.write_str("SummonDemon"),
            Self::ResurrectNew => f.write_str("ResurrectNew"),
            Self::AttackMe => f.write_str("AttackMe"),
            Self::DurabilityDamagePct => f.write_str("DurabilityDamagePct"),
            Self::SkinPlayerCorpse => f.write_str("SkinPlayerCorpse"),
            Self::SpiritHeal => f.write_str("SpiritHeal"),
            Self::Skill => f.write_str("Skill"),
            Self::ApplyAreaAuraPet => f.write_str("ApplyAreaAuraPet"),
            Self::TeleportGraveyard => f.write_str("TeleportGraveyard"),
            Self::NormalizedWeaponDmg => f.write_str("NormalizedWeaponDmg"),
            Self::Unknown122 => f.write_str("Unknown122"),
            Self::SendTaxi => f.write_str("SendTaxi"),
            Self::PlayerPull => f.write_str("PlayerPull"),
            Self::ModifyThreatPercent => f.write_str("ModifyThreatPercent"),
            Self::Unknown126 => f.write_str("Unknown126"),
            Self::Unknown127 => f.write_str("Unknown127"),
        }
    }
}

impl TryFrom<u32> for SpellEffect {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Self::from_int(value)
    }
}

impl TryFrom<u8> for SpellEffect {
    type Error = crate::errors::EnumError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u16> for SpellEffect {
    type Error = crate::errors::EnumError;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<u64> for SpellEffect {
    type Error = crate::errors::EnumError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i8> for SpellEffect {
    type Error = crate::errors::EnumError;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i16> for SpellEffect {
    type Error = crate::errors::EnumError;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i32> for SpellEffect {
    type Error = crate::errors::EnumError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<i64> for SpellEffect {
    type Error = crate::errors::EnumError;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value.into()))?
            .try_into()
    }
}

impl TryFrom<usize> for SpellEffect {
    type Error = crate::errors::EnumError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        TryInto::<u32>::try_into(value)
            .map_err(|_| crate::errors::EnumError::new(NAME, value as i128))?
            .try_into()
    }
}

