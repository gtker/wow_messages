/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spelllogexecute.wowm:444`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spelllogexecute.wowm#L444):
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
///     JUMP = 41;
///     JUMP2 = 42;
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
///     CREATE_RANDOM_ITEM = 59;
///     PROFICIENCY = 60;
///     SEND_EVENT = 61;
///     POWER_BURN = 62;
///     THREAT = 63;
///     TRIGGER_SPELL = 64;
///     APPLY_AREA_AURA_RAID = 65;
///     RESTORE_ITEM_CHARGES = 66;
///     HEAL_MAX_HEALTH = 67;
///     INTERRUPT_CAST = 68;
///     DISTRACT = 69;
///     PULL = 70;
///     PICKPOCKET = 71;
///     ADD_FARSIGHT = 72;
///     UNTRAIN_TALENTS = 73;
///     APPLY_GLYPH = 74;
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
///     WMO_DAMAGE = 87;
///     WMO_REPAIR = 88;
///     WMO_CHANGE = 89;
///     KILL_CREDIT_PERSONAL = 90;
///     THREAT_ALL = 91;
///     ENCHANT_HELD_ITEM = 92;
///     BREAK_PLAYER_TARGETING = 93;
///     SELF_RESURRECT = 94;
///     SKINNING = 95;
///     CHARGE = 96;
///     SUMMON_ALL_TOTEMS = 97;
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
///     UNKNOWN112 = 112;
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
///     STEAL_BENEFICIAL_BUFF = 126;
///     PROSPECTING = 127;
///     APPLY_AREA_AURA_FRIEND = 128;
///     APPLY_AREA_AURA_ENEMY = 129;
///     REDIRECT_THREAT = 130;
///     PLAY_SOUND = 131;
///     PLAY_MUSIC = 132;
///     UNLEARN_SPECIALIZATION = 133;
///     KILL_CREDIT_GROUP = 134;
///     CALL_PET = 135;
///     HEAL_PCT = 136;
///     ENERGIZE_PCT = 137;
///     LEAP_BACK = 138;
///     CLEAR_QUEST = 139;
///     FORCE_CAST = 140;
///     FORCE_CAST_WITH_VALUE = 141;
///     TRIGGER_SPELL_WITH_VALUE = 142;
///     APPLY_AREA_AURA_OWNER = 143;
///     KNOCKBACK_FROM_POSITION = 144;
///     GRAVITY_PULL = 145;
///     ACTIVATE_RUNE = 146;
///     QUEST_FAIL = 147;
///     UNKNOWN148 = 148;
///     CHARGE2 = 149;
///     QUEST_OFFER = 150;
///     TRIGGER_SPELL_2 = 151;
///     UNKNOWN152 = 152;
///     CREATE_PET = 153;
///     TEACH_TAXI_NODE = 154;
///     TITAN_GRIP = 155;
///     ENCHANT_ITEM_PRISMATIC = 156;
///     CREATE_ITEM_2 = 157;
///     MILLING = 158;
///     ALLOW_RENAME_PET = 159;
///     UNKNOWN160 = 160;
///     TALENT_SPEC_COUNT = 161;
///     TALENT_SPEC_SELECT = 162;
///     UNKNOWN163 = 163;
///     CANCEL_AURA = 164;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone)]
pub(crate) enum SpellEffect {
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
    Jump,
    Jump2,
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
    CreateRandomItem,
    Proficiency,
    SendEvent,
    PowerBurn,
    Threat,
    TriggerSpell,
    ApplyAreaAuraRaid,
    RestoreItemCharges,
    HealMaxHealth,
    InterruptCast,
    Distract,
    Pull,
    Pickpocket,
    AddFarsight,
    UntrainTalents,
    ApplyGlyph,
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
    WmoDamage,
    WmoRepair,
    WmoChange,
    KillCreditPersonal,
    ThreatAll,
    EnchantHeldItem,
    BreakPlayerTargeting,
    SelfResurrect,
    Skinning,
    Charge,
    SummonAllTotems,
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
    /// old SPELL_EFFECT_SUMMON_DEMON
    ///
    Unknown112,
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
    StealBeneficialBuff,
    Prospecting,
    ApplyAreaAuraFriend,
    ApplyAreaAuraEnemy,
    RedirectThreat,
    PlaySound,
    PlayMusic,
    UnlearnSpecialization,
    KillCreditGroup,
    CallPet,
    HealPct,
    EnergizePct,
    LeapBack,
    ClearQuest,
    ForceCast,
    ForceCastWithValue,
    TriggerSpellWithValue,
    ApplyAreaAuraOwner,
    KnockbackFromPosition,
    GravityPull,
    ActivateRune,
    QuestFail,
    Unknown148,
    Charge2,
    QuestOffer,
    TriggerSpell2,
    Unknown152,
    CreatePet,
    TeachTaxiNode,
    TitanGrip,
    EnchantItemPrismatic,
    CreateItem2,
    Milling,
    AllowRenamePet,
    Unknown160,
    TalentSpecCount,
    TalentSpecSelect,
    Unknown163,
    CancelAura,
}

impl SpellEffect {
    pub(crate) const fn as_int(&self) -> u32 {
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
            Self::Jump => 0x29,
            Self::Jump2 => 0x2a,
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
            Self::CreateRandomItem => 0x3b,
            Self::Proficiency => 0x3c,
            Self::SendEvent => 0x3d,
            Self::PowerBurn => 0x3e,
            Self::Threat => 0x3f,
            Self::TriggerSpell => 0x40,
            Self::ApplyAreaAuraRaid => 0x41,
            Self::RestoreItemCharges => 0x42,
            Self::HealMaxHealth => 0x43,
            Self::InterruptCast => 0x44,
            Self::Distract => 0x45,
            Self::Pull => 0x46,
            Self::Pickpocket => 0x47,
            Self::AddFarsight => 0x48,
            Self::UntrainTalents => 0x49,
            Self::ApplyGlyph => 0x4a,
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
            Self::WmoDamage => 0x57,
            Self::WmoRepair => 0x58,
            Self::WmoChange => 0x59,
            Self::KillCreditPersonal => 0x5a,
            Self::ThreatAll => 0x5b,
            Self::EnchantHeldItem => 0x5c,
            Self::BreakPlayerTargeting => 0x5d,
            Self::SelfResurrect => 0x5e,
            Self::Skinning => 0x5f,
            Self::Charge => 0x60,
            Self::SummonAllTotems => 0x61,
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
            Self::Unknown112 => 0x70,
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
            Self::StealBeneficialBuff => 0x7e,
            Self::Prospecting => 0x7f,
            Self::ApplyAreaAuraFriend => 0x80,
            Self::ApplyAreaAuraEnemy => 0x81,
            Self::RedirectThreat => 0x82,
            Self::PlaySound => 0x83,
            Self::PlayMusic => 0x84,
            Self::UnlearnSpecialization => 0x85,
            Self::KillCreditGroup => 0x86,
            Self::CallPet => 0x87,
            Self::HealPct => 0x88,
            Self::EnergizePct => 0x89,
            Self::LeapBack => 0x8a,
            Self::ClearQuest => 0x8b,
            Self::ForceCast => 0x8c,
            Self::ForceCastWithValue => 0x8d,
            Self::TriggerSpellWithValue => 0x8e,
            Self::ApplyAreaAuraOwner => 0x8f,
            Self::KnockbackFromPosition => 0x90,
            Self::GravityPull => 0x91,
            Self::ActivateRune => 0x92,
            Self::QuestFail => 0x93,
            Self::Unknown148 => 0x94,
            Self::Charge2 => 0x95,
            Self::QuestOffer => 0x96,
            Self::TriggerSpell2 => 0x97,
            Self::Unknown152 => 0x98,
            Self::CreatePet => 0x99,
            Self::TeachTaxiNode => 0x9a,
            Self::TitanGrip => 0x9b,
            Self::EnchantItemPrismatic => 0x9c,
            Self::CreateItem2 => 0x9d,
            Self::Milling => 0x9e,
            Self::AllowRenamePet => 0x9f,
            Self::Unknown160 => 0xa0,
            Self::TalentSpecCount => 0xa1,
            Self::TalentSpecSelect => 0xa2,
            Self::Unknown163 => 0xa3,
            Self::CancelAura => 0xa4,
        }
    }

}

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
            Self::Jump => f.write_str("Jump"),
            Self::Jump2 => f.write_str("Jump2"),
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
            Self::CreateRandomItem => f.write_str("CreateRandomItem"),
            Self::Proficiency => f.write_str("Proficiency"),
            Self::SendEvent => f.write_str("SendEvent"),
            Self::PowerBurn => f.write_str("PowerBurn"),
            Self::Threat => f.write_str("Threat"),
            Self::TriggerSpell => f.write_str("TriggerSpell"),
            Self::ApplyAreaAuraRaid => f.write_str("ApplyAreaAuraRaid"),
            Self::RestoreItemCharges => f.write_str("RestoreItemCharges"),
            Self::HealMaxHealth => f.write_str("HealMaxHealth"),
            Self::InterruptCast => f.write_str("InterruptCast"),
            Self::Distract => f.write_str("Distract"),
            Self::Pull => f.write_str("Pull"),
            Self::Pickpocket => f.write_str("Pickpocket"),
            Self::AddFarsight => f.write_str("AddFarsight"),
            Self::UntrainTalents => f.write_str("UntrainTalents"),
            Self::ApplyGlyph => f.write_str("ApplyGlyph"),
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
            Self::WmoDamage => f.write_str("WmoDamage"),
            Self::WmoRepair => f.write_str("WmoRepair"),
            Self::WmoChange => f.write_str("WmoChange"),
            Self::KillCreditPersonal => f.write_str("KillCreditPersonal"),
            Self::ThreatAll => f.write_str("ThreatAll"),
            Self::EnchantHeldItem => f.write_str("EnchantHeldItem"),
            Self::BreakPlayerTargeting => f.write_str("BreakPlayerTargeting"),
            Self::SelfResurrect => f.write_str("SelfResurrect"),
            Self::Skinning => f.write_str("Skinning"),
            Self::Charge => f.write_str("Charge"),
            Self::SummonAllTotems => f.write_str("SummonAllTotems"),
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
            Self::Unknown112 => f.write_str("Unknown112"),
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
            Self::StealBeneficialBuff => f.write_str("StealBeneficialBuff"),
            Self::Prospecting => f.write_str("Prospecting"),
            Self::ApplyAreaAuraFriend => f.write_str("ApplyAreaAuraFriend"),
            Self::ApplyAreaAuraEnemy => f.write_str("ApplyAreaAuraEnemy"),
            Self::RedirectThreat => f.write_str("RedirectThreat"),
            Self::PlaySound => f.write_str("PlaySound"),
            Self::PlayMusic => f.write_str("PlayMusic"),
            Self::UnlearnSpecialization => f.write_str("UnlearnSpecialization"),
            Self::KillCreditGroup => f.write_str("KillCreditGroup"),
            Self::CallPet => f.write_str("CallPet"),
            Self::HealPct => f.write_str("HealPct"),
            Self::EnergizePct => f.write_str("EnergizePct"),
            Self::LeapBack => f.write_str("LeapBack"),
            Self::ClearQuest => f.write_str("ClearQuest"),
            Self::ForceCast => f.write_str("ForceCast"),
            Self::ForceCastWithValue => f.write_str("ForceCastWithValue"),
            Self::TriggerSpellWithValue => f.write_str("TriggerSpellWithValue"),
            Self::ApplyAreaAuraOwner => f.write_str("ApplyAreaAuraOwner"),
            Self::KnockbackFromPosition => f.write_str("KnockbackFromPosition"),
            Self::GravityPull => f.write_str("GravityPull"),
            Self::ActivateRune => f.write_str("ActivateRune"),
            Self::QuestFail => f.write_str("QuestFail"),
            Self::Unknown148 => f.write_str("Unknown148"),
            Self::Charge2 => f.write_str("Charge2"),
            Self::QuestOffer => f.write_str("QuestOffer"),
            Self::TriggerSpell2 => f.write_str("TriggerSpell2"),
            Self::Unknown152 => f.write_str("Unknown152"),
            Self::CreatePet => f.write_str("CreatePet"),
            Self::TeachTaxiNode => f.write_str("TeachTaxiNode"),
            Self::TitanGrip => f.write_str("TitanGrip"),
            Self::EnchantItemPrismatic => f.write_str("EnchantItemPrismatic"),
            Self::CreateItem2 => f.write_str("CreateItem2"),
            Self::Milling => f.write_str("Milling"),
            Self::AllowRenamePet => f.write_str("AllowRenamePet"),
            Self::Unknown160 => f.write_str("Unknown160"),
            Self::TalentSpecCount => f.write_str("TalentSpecCount"),
            Self::TalentSpecSelect => f.write_str("TalentSpecSelect"),
            Self::Unknown163 => f.write_str("Unknown163"),
            Self::CancelAura => f.write_str("CancelAura"),
        }
    }
}

impl TryFrom<u32> for SpellEffect {
    type Error = crate::errors::EnumError;
    fn try_from(value: u32) -> std::result::Result<Self, Self::Error> {
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
            41 => Ok(Self::Jump),
            42 => Ok(Self::Jump2),
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
            59 => Ok(Self::CreateRandomItem),
            60 => Ok(Self::Proficiency),
            61 => Ok(Self::SendEvent),
            62 => Ok(Self::PowerBurn),
            63 => Ok(Self::Threat),
            64 => Ok(Self::TriggerSpell),
            65 => Ok(Self::ApplyAreaAuraRaid),
            66 => Ok(Self::RestoreItemCharges),
            67 => Ok(Self::HealMaxHealth),
            68 => Ok(Self::InterruptCast),
            69 => Ok(Self::Distract),
            70 => Ok(Self::Pull),
            71 => Ok(Self::Pickpocket),
            72 => Ok(Self::AddFarsight),
            73 => Ok(Self::UntrainTalents),
            74 => Ok(Self::ApplyGlyph),
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
            87 => Ok(Self::WmoDamage),
            88 => Ok(Self::WmoRepair),
            89 => Ok(Self::WmoChange),
            90 => Ok(Self::KillCreditPersonal),
            91 => Ok(Self::ThreatAll),
            92 => Ok(Self::EnchantHeldItem),
            93 => Ok(Self::BreakPlayerTargeting),
            94 => Ok(Self::SelfResurrect),
            95 => Ok(Self::Skinning),
            96 => Ok(Self::Charge),
            97 => Ok(Self::SummonAllTotems),
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
            112 => Ok(Self::Unknown112),
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
            126 => Ok(Self::StealBeneficialBuff),
            127 => Ok(Self::Prospecting),
            128 => Ok(Self::ApplyAreaAuraFriend),
            129 => Ok(Self::ApplyAreaAuraEnemy),
            130 => Ok(Self::RedirectThreat),
            131 => Ok(Self::PlaySound),
            132 => Ok(Self::PlayMusic),
            133 => Ok(Self::UnlearnSpecialization),
            134 => Ok(Self::KillCreditGroup),
            135 => Ok(Self::CallPet),
            136 => Ok(Self::HealPct),
            137 => Ok(Self::EnergizePct),
            138 => Ok(Self::LeapBack),
            139 => Ok(Self::ClearQuest),
            140 => Ok(Self::ForceCast),
            141 => Ok(Self::ForceCastWithValue),
            142 => Ok(Self::TriggerSpellWithValue),
            143 => Ok(Self::ApplyAreaAuraOwner),
            144 => Ok(Self::KnockbackFromPosition),
            145 => Ok(Self::GravityPull),
            146 => Ok(Self::ActivateRune),
            147 => Ok(Self::QuestFail),
            148 => Ok(Self::Unknown148),
            149 => Ok(Self::Charge2),
            150 => Ok(Self::QuestOffer),
            151 => Ok(Self::TriggerSpell2),
            152 => Ok(Self::Unknown152),
            153 => Ok(Self::CreatePet),
            154 => Ok(Self::TeachTaxiNode),
            155 => Ok(Self::TitanGrip),
            156 => Ok(Self::EnchantItemPrismatic),
            157 => Ok(Self::CreateItem2),
            158 => Ok(Self::Milling),
            159 => Ok(Self::AllowRenamePet),
            160 => Ok(Self::Unknown160),
            161 => Ok(Self::TalentSpecCount),
            162 => Ok(Self::TalentSpecSelect),
            163 => Ok(Self::Unknown163),
            164 => Ok(Self::CancelAura),
            v => Err(crate::errors::EnumError::new("SpellEffect", v as u64),)
        }
    }
}

