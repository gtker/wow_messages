#![allow(clippy::too_many_arguments)]
use crate::vanilla::{
    AllowedClass, AllowedRace, Area, BagFamily, Bonding, Faction, Gold, InventoryType, 
    ItemClassAndSubClass, ItemDamageType, ItemFlag, ItemQuality, ItemSet, Language, 
    Map, PageTextMaterial, PvpRank, SheatheType, Skill, SpellSchool, SpellTriggerType,
};

/// Struct optimized for containing the original items most efficiently.
///
/// This type is not supposed to be used by external users of the library for creating custom items.
/// It's only supposed to be used in conjunction with the `wow_items` crate.
///
/// [`Hash`](core::hash::Hash), [`Ord`], and [`Eq`] all use only the item id without considering other fields.
#[derive(Debug, Copy, Clone, Default)]
pub struct Item {
    entry: u16,
    class_and_sub_class: ItemClassAndSubClass,
    name: &'static str,
    display_id: u16,
    quality: ItemQuality,
    flags: ItemFlag,
    buy_count: i16,
    buy_price: Gold,
    sell_price: Gold,
    inventory_type: InventoryType,
    allowed_class: AllowedClass,
    allowed_race: AllowedRace,
    item_level: i8,
    required_level: i8,
    required_skill: Skill,
    required_skill_rank: i16,
    required_honor_rank: PvpRank,
    required_faction: Faction,
    required_reputation_rank: i8,
    max_count: i16,
    stackable: i16,
    container_slots: i8,
    agility: i8,
    strength: i8,
    stamina: i8,
    intellect: i16,
    spirit: i8,
    armor: i16,
    fire_res: i16,
    nature_res: i16,
    frost_res: i16,
    shadow_res: i16,
    delay: i16,
    ammo_type: i8,
    ranged_mod_range: f32,
    bonding: Bonding,
    description: &'static str,
    page_text: i16,
    language: Language,
    page_text_material: PageTextMaterial,
    start_quest: i16,
    material: i8,
    sheathe_type: SheatheType,
    random_property: i16,
    block: i8,
    item_set: ItemSet,
    max_durability: i16,
    bag_family: BagFamily,
    disenchant_id: i8,
    food_type: i8,
    extra_flags: i8,
    damages: &'static [ItemDamageType],
    spells: &'static [Spells],
}

impl Item {
    #[doc(hidden)]
    pub const fn new(
        entry: u16,
        class_and_sub_class: ItemClassAndSubClass,
        name: &'static str,
        display_id: u16,
        quality: ItemQuality,
        flags: ItemFlag,
        buy_count: i16,
        buy_price: Gold,
        sell_price: Gold,
        inventory_type: InventoryType,
        allowed_class: AllowedClass,
        allowed_race: AllowedRace,
        item_level: i8,
        required_level: i8,
        required_skill: Skill,
        required_skill_rank: i16,
        required_honor_rank: PvpRank,
        required_faction: Faction,
        required_reputation_rank: i8,
        max_count: i16,
        stackable: i16,
        container_slots: i8,
        agility: i8,
        strength: i8,
        stamina: i8,
        intellect: i16,
        spirit: i8,
        armor: i16,
        fire_res: i16,
        nature_res: i16,
        frost_res: i16,
        shadow_res: i16,
        delay: i16,
        ammo_type: i8,
        ranged_mod_range: f32,
        bonding: Bonding,
        description: &'static str,
        page_text: i16,
        language: Language,
        page_text_material: PageTextMaterial,
        start_quest: i16,
        material: i8,
        sheathe_type: SheatheType,
        random_property: i16,
        block: i8,
        item_set: ItemSet,
        max_durability: i16,
        bag_family: BagFamily,
        disenchant_id: i8,
        food_type: i8,
        extra_flags: i8,
        damages: &'static [ItemDamageType],
        spells: &'static [Spells],
    ) -> Self {
        Self {
            entry,
            class_and_sub_class,
            name,
            display_id,
            quality,
            flags,
            buy_count,
            buy_price,
            sell_price,
            inventory_type,
            allowed_class,
            allowed_race,
            item_level,
            required_level,
            required_skill,
            required_skill_rank,
            required_honor_rank,
            required_faction,
            required_reputation_rank,
            max_count,
            stackable,
            container_slots,
            agility,
            strength,
            stamina,
            intellect,
            spirit,
            armor,
            fire_res,
            nature_res,
            frost_res,
            shadow_res,
            delay,
            ammo_type,
            ranged_mod_range,
            bonding,
            description,
            page_text,
            language,
            page_text_material,
            start_quest,
            material,
            sheathe_type,
            random_property,
            block,
            item_set,
            max_durability,
            bag_family,
            disenchant_id,
            food_type,
            extra_flags,
            damages,
            spells,
        }
    }
    pub const fn entry(&self) -> u32 {
        self.entry as u32
    }

    pub const fn class_and_sub_class(&self) -> ItemClassAndSubClass {
        self.class_and_sub_class
    }

    pub const fn name(&self) -> &'static str {
        self.name
    }

    pub const fn display_id(&self) -> u32 {
        self.display_id as u32
    }

    pub const fn quality(&self) -> ItemQuality {
        self.quality
    }

    pub const fn flags(&self) -> ItemFlag {
        self.flags
    }

    pub const fn buy_count(&self) -> i32 {
        self.buy_count as i32
    }

    pub const fn buy_price(&self) -> Gold {
        self.buy_price
    }

    pub const fn sell_price(&self) -> Gold {
        self.sell_price
    }

    pub const fn inventory_type(&self) -> InventoryType {
        self.inventory_type
    }

    pub const fn allowed_class(&self) -> AllowedClass {
        self.allowed_class
    }

    pub const fn allowed_race(&self) -> AllowedRace {
        self.allowed_race
    }

    pub const fn item_level(&self) -> i32 {
        self.item_level as i32
    }

    pub const fn required_level(&self) -> i32 {
        self.required_level as i32
    }

    pub const fn required_skill(&self) -> Skill {
        self.required_skill
    }

    pub const fn required_skill_rank(&self) -> i32 {
        self.required_skill_rank as i32
    }

    /// Returns `0` except for specific item entries.
    pub const fn required_spell(&self) -> i32 {
        match self.entry {
            11610 | 11611 | 12817 | 12831 | 12832 | 12839 | 19208 | 19209 => 9787,
            11612 | 12691 | 12696 | 12699 | 12703 | 12711 | 12716 | 12717 | 12720 | 12725 | 12726 | 12727 | 12728 | 17049 | 17051 | 17052 | 17053 | 19206 | 19207 | 20040 | 22388 => 9788,
            12821 | 12835 | 12838 | 17060 | 19212 => 17041,
            12824 | 12827 | 12833 | 12837 | 19210 | 22390 => 17040,
            12825 | 12830 | 12834 | 12836 | 17059 | 19211 | 22389 => 17039,
            15726 | 15730 | 15733 | 15751 | 15759 | 15763 | 15770 | 15780 | 15781 | 17025 | 18517 | 19331 | 20382 => 10656,
            15729 | 15735 | 15737 | 15740 | 15742 | 15746 | 15747 | 15754 | 15755 | 15758 | 15760 | 15761 | 15772 | 15779 | 17022 | 18518 | 19332 | 20253 | 20254 => 10660,
            15732 | 15734 | 15741 | 15749 | 15752 | 15753 | 15764 | 15771 | 15775 | 17023 | 18519 | 19333 | 21548 => 10658,
            18653 | 18984 => 20222,
            18654 | 18660 | 18661 | 18986 => 20219,
            _ => 0,
        }
    }

    pub const fn required_honor_rank(&self) -> PvpRank {
        self.required_honor_rank
    }

    /// Returns `0` except for specific item entries.
    pub const fn required_city_rank(&self) -> i32 {
        match self.entry {
            12585 => 1,
            _ => 0,
        }
    }

    pub const fn required_faction(&self) -> Faction {
        self.required_faction
    }

    pub const fn required_reputation_rank(&self) -> i32 {
        self.required_reputation_rank as i32
    }

    pub const fn max_count(&self) -> i32 {
        self.max_count as i32
    }

    pub const fn stackable(&self) -> i32 {
        self.stackable as i32
    }

    pub const fn container_slots(&self) -> i32 {
        self.container_slots as i32
    }

    /// Returns `0` except for specific item entries.
    pub const fn mana(&self) -> i32 {
        match self.entry {
            6674 => 2500,
            _ => 0,
        }
    }

    /// Returns `0` except for specific item entries.
    pub const fn health(&self) -> i32 {
        match self.entry {
            6673 => 2500,
            _ => 0,
        }
    }

    pub const fn agility(&self) -> i32 {
        self.agility as i32
    }

    pub const fn strength(&self) -> i32 {
        self.strength as i32
    }

    pub const fn stamina(&self) -> i32 {
        self.stamina as i32
    }

    pub const fn intellect(&self) -> i32 {
        self.intellect as i32
    }

    pub const fn spirit(&self) -> i32 {
        self.spirit as i32
    }

    pub const fn armor(&self) -> i32 {
        self.armor as i32
    }

    /// Returns `0` except for specific item entries.
    pub const fn holy_res(&self) -> i32 {
        match self.entry {
            3359 | 3398 => 100,
            _ => 0,
        }
    }

    pub const fn fire_res(&self) -> i32 {
        self.fire_res as i32
    }

    pub const fn nature_res(&self) -> i32 {
        self.nature_res as i32
    }

    pub const fn frost_res(&self) -> i32 {
        self.frost_res as i32
    }

    pub const fn shadow_res(&self) -> i32 {
        self.shadow_res as i32
    }

    /// Returns `0` except for specific item entries.
    pub const fn arcane_res(&self) -> i32 {
        match self.entry {
            1604 | 12409 | 12967 | 20141 | 20267 => 7,
            5828 => 255,
            9434 | 9455 | 9461 | 9508 | 12408 | 12903 | 12945 | 17107 | 17113 | 17717 | 18800 | 20269 | 21467 => 5,
            11785 | 12410 | 12414 | 13009 | 13961 | 13966 | 15857 | 16901 | 16909 | 16915 | 16922 | 16930 | 16938 | 16946 | 16954 | 16962 | 17000 | 17737 | 18545 | 18813 | 20136 | 20139 | 21128 => 10,
            11934 | 12252 | 12406 | 13030 | 15049 | 17078 | 17110 | 20142 => 6,
            12065 | 12405 | 15048 => 8,
            12413 | 14543 | 15815 => 15,
            12423 | 14130 => 18,
            12609 | 13538 => 20,
            13065 => 9,
            13080 | 13535 | 13539 | 23042 => 13,
            14128 => 17,
            14132 | 15072 | 15075 => 16,
            15073 | 18341 | 20295 => 12,
            15074 | 18342 => 11,
            18582 => 100,
            18583 => 60,
            18584 => 50,
            20146 => 2,
            20275 | 20276 | 20615 => 4,
            20278 | 20279 => 3,
            _ => 0,
        }
    }

    pub const fn delay(&self) -> i32 {
        self.delay as i32
    }

    pub const fn ammo_type(&self) -> i32 {
        self.ammo_type as i32
    }

    pub const fn ranged_mod_range(&self) -> f32 {
        self.ranged_mod_range
    }

    pub const fn bonding(&self) -> Bonding {
        self.bonding
    }

    pub const fn description(&self) -> &'static str {
        self.description
    }

    pub const fn page_text(&self) -> i32 {
        self.page_text as i32
    }

    pub const fn language(&self) -> Language {
        self.language
    }

    pub const fn page_text_material(&self) -> PageTextMaterial {
        self.page_text_material
    }

    pub const fn start_quest(&self) -> i32 {
        self.start_quest as i32
    }

    /// Returns `0` except for specific item entries.
    pub const fn lock_id(&self) -> i32 {
        match self.entry {
            3746 => 2,
            4632 | 6354 | 6712 | 16882 => 5,
            4633 => 23,
            4634 | 5046 | 6355 | 7869 | 16883 => 24,
            4636 => 60,
            4637 | 13875 | 16884 => 61,
            4638 | 5758 | 5759 | 5760 => 62,
            7209 => 319,
            7868 => 57,
            12033 => 600,
            13918 | 16885 => 599,
            _ => 0,
        }
    }

    pub const fn material(&self) -> i32 {
        self.material as i32
    }

    pub const fn sheathe_type(&self) -> SheatheType {
        self.sheathe_type
    }

    pub const fn random_property(&self) -> i32 {
        self.random_property as i32
    }

    pub const fn block(&self) -> i32 {
        self.block as i32
    }

    pub const fn item_set(&self) -> ItemSet {
        self.item_set
    }

    pub const fn max_durability(&self) -> i32 {
        self.max_durability as i32
    }

    /// Returns `Area::None` except for specific item entries.
    pub const fn area(&self) -> Area {
        match self.entry {
            17306 | 17323 | 17324 | 17325 | 17326 | 17327 | 17328 | 17353 | 17422 | 17423 | 17442 | 17502 | 17503 | 17504 | 17505 | 17506 | 17507 | 17522 | 17542 | 17626 | 17642 | 17643 | 17689 | 21038 => Area::AlteracValley0,
            18266 | 18268 => Area::DireMaul0,
            _ => Area::None,
        }
    }

    /// Returns `Map::EasternKingdoms` except for specific item entries.
    pub const fn map(&self) -> Map {
        match self.entry {
            18266 | 18268 => Map::DireMaul,
            _ => Map::EasternKingdoms,
        }
    }

    pub const fn bag_family(&self) -> BagFamily {
        self.bag_family
    }

    pub const fn disenchant_id(&self) -> i32 {
        self.disenchant_id as i32
    }

    pub const fn food_type(&self) -> i32 {
        self.food_type as i32
    }

    /// Returns `0` except for specific item entries.
    pub const fn min_money_loot(&self) -> i32 {
        match self.entry {
            5335 => 750,
            6827 => 200,
            10456 => 2037,
            11937 => 6235,
            11966 => 425,
            16882 => 25,
            20602 => 600000,
            20708 => 50,
            20766 | 21113 | 21150 | 21228 => 100,
            20767 => 500,
            20768 => 1000,
            23022 => 50000,
            _ => 0,
        }
    }

    /// Returns `0` except for specific item entries.
    pub const fn max_money_loot(&self) -> i32 {
        match self.entry {
            5335 => 2250,
            6827 => 300,
            10456 => 6110,
            11937 => 18704,
            11966 => 1275,
            16882 => 125,
            20602 => 1000000,
            20708 => 100,
            20766 => 1000,
            20767 => 1500,
            20768 => 2500,
            21113 | 21150 | 21228 => 200,
            23022 => 50000,
            _ => 0,
        }
    }

    /// Returns `0` except for specific item entries.
    pub const fn duration(&self) -> i32 {
        match self.entry {
            1164 | 10642 | 11885 | 21038 => 300,
            4986 | 5810 | 9322 | 10338 | 13320 => 1800,
            5768 | 5769 | 15447 | 22736 => 600,
            6091 | 8708 | 12586 => 3600,
            9365 | 9437 | 9438 | 9439 | 9440 | 9441 | 9442 | 18904 => 7200,
            10684 => 1200,
            10790 | 10791 | 20557 | 23211 | 23246 | 23247 | 23326 | 23327 | 23379 | 23435 => 1209600,
            11804 => 1500,
            17354 => 900,
            19807 => 14400,
            20391 | 20392 | 20561 | 20562 | 20563 | 20564 | 20565 | 20566 | 20567 | 20568 | 20569 | 20570 | 20571 | 20572 | 20573 | 20574 | 21212 | 23164 | 23176 => 604800,
            21171 | 21174 | 21711 | 21739 => 86400,
            21325 | 21328 => 864000,
            23215 => 20160,
            _ => 0,
        }
    }

    pub const fn extra_flags(&self) -> i32 {
        self.extra_flags as i32
    }

    pub const fn damages_array(&self) -> [ItemDamageType; 5] {
        const D: ItemDamageType=ItemDamageType{
            damage_minimum:0.0,
            damage_maximum:0.0,
            school:SpellSchool::Normal,
        };
        let l = self.damages.len();
        [
            if l >= 1 { self.damages()[0] } else { D },
            if l >= 2 { self.damages()[1] } else { D },
            if l >= 3 { self.damages()[2] } else { D },
            if l >= 4 { self.damages()[3] } else { D },
            if l >= 5 { self.damages()[4] } else { D },
        ]
    }

    pub const fn damages(&self) -> &[ItemDamageType] {
        self.damages
    }

    pub const fn spells_array(&self) -> [Spells; 5] {
        const D: Spells=Spells{
            spell:0,
            spell_trigger:SpellTriggerType::OnUse,
            spell_charges:0,
            spell_ppm_rate:0.0,
            spell_cooldown:0,
            spell_category:0,
            spell_category_cooldown:0,
        };
        let l = self.spells.len();
        [
            if l >= 1 { self.spells()[0] } else { D },
            if l >= 2 { self.spells()[1] } else { D },
            if l >= 3 { self.spells()[2] } else { D },
            if l >= 4 { self.spells()[3] } else { D },
            if l >= 5 { self.spells()[4] } else { D },
        ]
    }

    pub const fn spells(&self) -> &[Spells] {
        self.spells
    }

}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.entry.cmp(&other.entry)
    }
}

impl PartialEq<Self> for Item {
    fn eq(&self, other: &Self) -> bool {
        self.entry.eq(&other.entry)
    }
}

impl Eq for Item {}
impl core::hash::Hash for Item {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.entry.hash(state)
    }
}

#[derive(Debug, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Spells {
    pub spell: i32,
    pub spell_trigger: SpellTriggerType,
    pub spell_charges: i32,
    pub spell_ppm_rate: f32,
    pub spell_cooldown: i32,
    pub spell_category: i32,
    pub spell_category_cooldown: i32,
}

impl Spells {
    pub const fn new(
        spell: i32,
        spell_trigger: SpellTriggerType,
        spell_charges: i32,
        spell_ppm_rate: f32,
        spell_cooldown: i32,
        spell_category: i32,
        spell_category_cooldown: i32,
    ) -> Self {
        Self {
            spell,
            spell_trigger,
            spell_charges,
            spell_ppm_rate,
            spell_cooldown,
            spell_category,
            spell_category_cooldown,
        }
    }
}
