#![allow(clippy::too_many_arguments)]
use crate::tbc::{
    AllowedClass, AllowedRace, Area, BagFamily, Bonding, Faction, Gold, InventoryType, 
    ItemClassAndSubClass, ItemDamageType, ItemFlag, ItemQuality, ItemSet, ItemSocket, 
    ItemStat, Language, Map, PageTextMaterial, PvpRank, SheatheType, Skill, SpellSchool, 
    SpellTriggerType,
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
    sound_override_sub_class: i8,
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
    item_level: i16,
    required_level: i8,
    required_skill: Skill,
    required_skill_rank: i16,
    required_faction: Faction,
    required_reputation_rank: i8,
    max_count: i32,
    stackable: i16,
    container_slots: i8,
    armor: i16,
    fire_res: i16,
    nature_res: i16,
    frost_res: i16,
    shadow_res: i16,
    arcane_res: i16,
    delay: i16,
    ammo_type: i8,
    ranged_mod_range: f32,
    bonding: Bonding,
    description: &'static str,
    page_text: i16,
    page_text_material: PageTextMaterial,
    start_quest: i16,
    material: i8,
    sheathe_type: SheatheType,
    random_property: i16,
    random_suffix: i16,
    block: i16,
    item_set: ItemSet,
    max_durability: i16,
    bag_family: BagFamily,
    socket_bonus: i16,
    gem_properties: i16,
    required_disenchant_skill: i16,
    armor_damage_modifier: f32,
    disenchant_id: i8,
    food_type: i8,
    duration: i32,
    sockets: &'static [ItemSocket],
    damages: &'static [ItemDamageType],
    stats: &'static [ItemStat],
    spells: &'static [Spells],
}

impl Item {
    #[doc(hidden)]
    pub const fn new(
        entry: u16,
        class_and_sub_class: ItemClassAndSubClass,
        sound_override_sub_class: i8,
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
        item_level: i16,
        required_level: i8,
        required_skill: Skill,
        required_skill_rank: i16,
        required_faction: Faction,
        required_reputation_rank: i8,
        max_count: i32,
        stackable: i16,
        container_slots: i8,
        armor: i16,
        fire_res: i16,
        nature_res: i16,
        frost_res: i16,
        shadow_res: i16,
        arcane_res: i16,
        delay: i16,
        ammo_type: i8,
        ranged_mod_range: f32,
        bonding: Bonding,
        description: &'static str,
        page_text: i16,
        page_text_material: PageTextMaterial,
        start_quest: i16,
        material: i8,
        sheathe_type: SheatheType,
        random_property: i16,
        random_suffix: i16,
        block: i16,
        item_set: ItemSet,
        max_durability: i16,
        bag_family: BagFamily,
        socket_bonus: i16,
        gem_properties: i16,
        required_disenchant_skill: i16,
        armor_damage_modifier: f32,
        disenchant_id: i8,
        food_type: i8,
        duration: i32,
        sockets: &'static [ItemSocket],
        damages: &'static [ItemDamageType],
        stats: &'static [ItemStat],
        spells: &'static [Spells],
    ) -> Self {
        Self {
            entry,
            class_and_sub_class,
            sound_override_sub_class,
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
            required_faction,
            required_reputation_rank,
            max_count,
            stackable,
            container_slots,
            armor,
            fire_res,
            nature_res,
            frost_res,
            shadow_res,
            arcane_res,
            delay,
            ammo_type,
            ranged_mod_range,
            bonding,
            description,
            page_text,
            page_text_material,
            start_quest,
            material,
            sheathe_type,
            random_property,
            random_suffix,
            block,
            item_set,
            max_durability,
            bag_family,
            socket_bonus,
            gem_properties,
            required_disenchant_skill,
            armor_damage_modifier,
            disenchant_id,
            food_type,
            duration,
            sockets,
            damages,
            stats,
            spells,
        }
    }
    pub const fn entry(&self) -> u32 {
        self.entry as u32
    }

    pub const fn class_and_sub_class(&self) -> ItemClassAndSubClass {
        self.class_and_sub_class
    }

    pub const fn sound_override_sub_class(&self) -> i32 {
        self.sound_override_sub_class as i32
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
            15780 | 29515 | 29516 | 29517 | 29519 | 29520 | 29521 | 29971 | 29975 => 10656,
            18653 | 18984 | 23836 | 23838 | 23839 | 30542 | 35485 => 20222,
            18654 | 18660 | 18661 | 18986 | 23828 | 23829 | 23835 | 30544 => 20219,
            21846 | 21847 | 21848 | 21908 | 21909 | 21910 => 26797,
            21869 | 21870 | 21871 | 21912 | 21913 | 21914 => 26801,
            21873 | 21874 | 21875 | 21916 | 21917 | 21918 => 26798,
            23563 | 23564 | 23565 | 28483 | 28484 | 28485 | 30069 | 30070 | 30074 | 30076 => 9788,
            28425 | 28426 | 28427 | 28428 | 28429 | 28430 | 30077 | 30086 => 17039,
            28431 | 28432 | 28433 | 28434 | 28435 | 28436 | 30087 | 30088 => 17041,
            28437 | 28438 | 28439 | 28440 | 28441 | 28442 | 30089 | 30093 => 17040,
            29522 | 29523 | 29524 | 29970 | 29974 => 10660,
            29525 | 29526 | 29527 | 29964 | 29973 => 10658,
            30071 | 30072 | 30073 => 9787,
            _ => 0,
        }
    }

    /// Returns `PvpRank::NoRank` except for specific item entries.
    pub const fn required_honor_rank(&self) -> PvpRank {
        match self.entry {
            31667 => PvpRank::Pariah,
            _ => PvpRank::NoRank,
        }
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
        self.max_count
    }

    pub const fn stackable(&self) -> i32 {
        self.stackable as i32
    }

    pub const fn container_slots(&self) -> i32 {
        self.container_slots as i32
    }

    pub const fn armor(&self) -> i32 {
        self.armor as i32
    }

    /// Always returns `0`.
    pub const fn holy_res(&self) -> i32 {
        0
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

    pub const fn arcane_res(&self) -> i32 {
        self.arcane_res as i32
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

    /// Returns `Language::Universal` except for specific item entries.
    pub const fn language(&self) -> Language {
        match self.entry {
            25 | 4992 | 4995 | 5917 | 6167 | 20010 | 21783 | 23798 => Language::Orcish,
            745 | 957 | 1208 | 1252 | 1283 | 1284 | 1307 | 1319 | 1637 | 1656 | 1971 | 1972 | 2161 | 2223 | 2548 | 2560 | 2637 | 2666 | 2696 | 2720 | 2722 | 2724 | 2760 | 2794 | 2795 | 3085 | 3086 | 3087 | 3248 | 3250 | 3518 | 4130 | 4429 | 4432 | 4514 | 5351 | 5354 | 5455 | 5520 | 5622 | 5790 | 5882 | 5947 | 5998 | 6280 | 6332 | 6678 | 6743 | 6843 | 9244 | 9245 | 9250 | 9329 | 9330 | 9370 | 11108 | 11125 | 12771 | 12813 | 12820 | 12985 | 20009 | 20828 | 22979 | 23777 | 23780 | 23797 | 28303 | 30420 | 30973 => Language::Common,
            5383 => Language::Demonic,
            5594 => Language::Taurahe,
            5839 | 9331 => Language::Darnassian,
            20949 => Language::Draconic,
            24237 => Language::Draenei,
            _ => Language::Universal,
        }
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
            2503 | 3746 => 2,
            4632 | 6354 | 6712 | 16882 => 5,
            4633 => 23,
            4634 | 6355 | 7869 | 16883 => 24,
            4636 => 60,
            4637 | 13875 | 16884 => 61,
            4638 | 5758 | 5759 | 5760 => 62,
            7209 => 319,
            7868 => 57,
            12033 => 600,
            13918 | 16885 => 599,
            29569 => 1665,
            31952 => 1666,
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

    pub const fn random_suffix(&self) -> i32 {
        self.random_suffix as i32
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
            17306 | 17323 | 17324 | 17325 | 17326 | 17327 | 17328 | 17353 | 17422 | 17423 | 17442 | 17502 | 17503 | 17504 | 17505 | 17506 | 17507 | 17522 | 17542 | 17626 | 17642 | 17643 | 17689 | 21038 => Area::AlteracValley,
            18266 | 18268 => Area::DireMaul,
            22736 => Area::Stratholme,
            24289 => Area::TheBlackMorass,
            31279 => Area::ShadowmoonValley,
            31366 | 32372 => Area::BladesEdgeMountains,
            36748 => Area::BlackrockDepths,
            _ => Area::None,
        }
    }

    /// Returns `Map::EasternKingdoms` except for specific item entries.
    pub const fn map(&self) -> Map {
        match self.entry {
            18266 | 18268 => Map::DireMaul,
            23857 | 23862 | 23864 | 23865 => Map::Karazhan,
            24494 => Map::TheBattleForMountHyjal,
            25853 => Map::TheEscapeFromDurnholde,
            30311 | 30312 | 30313 | 30314 | 30316 | 30317 | 30318 | 30319 | 30320 => Map::TempestKeep,
            31088 => Map::CoilfangSerpentshrineCavern,
            32408 => Map::BlackTemple,
            33865 | 33930 | 33931 | 33932 | 33933 => Map::ZulAman,
            _ => Map::EasternKingdoms,
        }
    }

    pub const fn bag_family(&self) -> BagFamily {
        self.bag_family
    }

    /// Returns `0` except for specific item entries.
    pub const fn totem_category(&self) -> i32 {
        match self.entry {
            756 | 778 | 1819 | 1893 | 1959 | 2901 | 9465 | 20723 | 30855 => 11,
            1500 => 21,
            5175 => 2,
            5176 | 30845 => 4,
            5177 => 5,
            5178 => 3,
            5956 => 13,
            6218 => 6,
            6219 => 14,
            6339 => 7,
            7005 | 12709 | 19901 => 1,
            9149 | 13503 | 31080 | 35748 | 35749 | 35750 | 35751 => 12,
            10498 => 15,
            11130 => 8,
            11145 => 9,
            16207 => 10,
            22461 => 41,
            22462 => 62,
            22463 => 63,
            _ => 0,
        }
    }

    pub const fn socket_bonus(&self) -> i32 {
        self.socket_bonus as i32
    }

    pub const fn gem_properties(&self) -> i32 {
        self.gem_properties as i32
    }

    pub const fn required_disenchant_skill(&self) -> i32 {
        self.required_disenchant_skill as i32
    }

    pub const fn armor_damage_modifier(&self) -> f32 {
        self.armor_damage_modifier
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
            16883 | 16884 => 5,
            16885 | 29569 => 150,
            20602 => 600000,
            20708 => 50,
            20766 | 21113 | 21150 | 21228 => 100,
            20767 => 500,
            20768 => 1000,
            23022 | 34592 | 34593 => 50000,
            23921 => 1930,
            34583 | 34584 => 20000,
            34585 | 34587 => 30000,
            34594 | 34595 => 90000,
            34863 | 35348 => 60000,
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
            16883 | 16884 => 15,
            16885 | 29569 => 600,
            20602 => 1000000,
            20708 => 100,
            20766 => 1000,
            20767 => 1500,
            20768 => 2500,
            21113 | 21150 | 21228 => 200,
            23022 => 50000,
            23921 => 5790,
            34583 | 34584 => 30000,
            34585 | 34587 => 40000,
            34592 | 34593 => 70000,
            34594 | 34595 => 110000,
            34863 | 35348 => 80000,
            _ => 0,
        }
    }

    pub const fn duration(&self) -> i32 {
        self.duration
    }

    /// Returns `0` except for specific item entries.
    pub const fn extra_flags(&self) -> i32 {
        match self.entry {
            5810 | 9365 | 9437 | 9438 | 9439 | 9440 | 9441 | 9442 | 10338 | 10684 | 10790 | 10791 | 11885 | 12586 | 19807 | 20391 | 20392 | 20557 | 20561 | 20562 | 20563 | 20564 | 20565 | 20566 | 20567 | 20568 | 20569 | 20570 | 20571 | 20572 | 20573 | 20574 | 21038 | 21171 | 21174 | 21212 | 21328 | 22736 | 23247 | 23379 | 30311 | 30312 | 30313 | 30314 | 30316 | 30317 | 30318 | 30320 | 30850 | 33096 | 33176 | 33182 | 33183 | 33184 | 33189 | 33226 | 34191 | 35313 | 37582 | 37583 | 37584 | 37585 => 1,
            7666 | 18706 => 2,
            _ => 0,
        }
    }

    pub const fn sockets_array(&self) -> [ItemSocket; 3] {
        const D: ItemSocket=ItemSocket{
            color:0,
            content:0,
        };
        let l = self.sockets.len();
        [
            if l >= 1 { self.sockets()[0] } else { D },
            if l >= 2 { self.sockets()[1] } else { D },
            if l >= 3 { self.sockets()[2] } else { D },
        ]
    }

    pub const fn sockets(&self) -> &[ItemSocket] {
        self.sockets
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

    pub const fn stats_array(&self) -> [ItemStat; 10] {
        const D: ItemStat=ItemStat{
            stat_type:0,
            value:0,
        };
        let l = self.stats.len();
        [
            if l >= 1 { self.stats()[0] } else { D },
            if l >= 2 { self.stats()[1] } else { D },
            if l >= 3 { self.stats()[2] } else { D },
            if l >= 4 { self.stats()[3] } else { D },
            if l >= 5 { self.stats()[4] } else { D },
            if l >= 6 { self.stats()[5] } else { D },
            if l >= 7 { self.stats()[6] } else { D },
            if l >= 8 { self.stats()[7] } else { D },
            if l >= 9 { self.stats()[8] } else { D },
            if l >= 10 { self.stats()[9] } else { D },
        ]
    }

    pub const fn stats(&self) -> &[ItemStat] {
        self.stats
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
