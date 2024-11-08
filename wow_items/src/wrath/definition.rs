#![allow(clippy::too_many_arguments)]
use crate::wrath::{
    AllowedClass, AllowedRace, Area, BagFamily, Bonding, Faction, Gold, InventoryType, 
    ItemClassAndSubClass, ItemDamageType, ItemFlag, ItemFlag2, ItemQuality, ItemSet, 
    ItemSocket, ItemStat, Language, Map, PageTextMaterial, PvpRank, SheatheType, 
    Skill, SpellSchool, SpellTriggerType,
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
    display_id: u32,
    quality: ItemQuality,
    flags: ItemFlag,
    flags2: ItemFlag2,
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
    required_spell: i32,
    required_faction: Faction,
    required_reputation_rank: i8,
    max_count: i16,
    stackable: i16,
    container_slots: i8,
    stats_count: i8,
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
    totem_category: i16,
    socket_bonus: i16,
    gem_properties: i16,
    required_disenchant_skill: i16,
    armor_damage_modifier: f32,
    duration: i32,
    item_limit_category: i8,
    disenchant_id: i8,
    food_type: i8,
    extra_flags: i8,
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
        display_id: u32,
        quality: ItemQuality,
        flags: ItemFlag,
        flags2: ItemFlag2,
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
        required_spell: i32,
        required_faction: Faction,
        required_reputation_rank: i8,
        max_count: i16,
        stackable: i16,
        container_slots: i8,
        stats_count: i8,
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
        totem_category: i16,
        socket_bonus: i16,
        gem_properties: i16,
        required_disenchant_skill: i16,
        armor_damage_modifier: f32,
        duration: i32,
        item_limit_category: i8,
        disenchant_id: i8,
        food_type: i8,
        extra_flags: i8,
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
            flags2,
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
            required_spell,
            required_faction,
            required_reputation_rank,
            max_count,
            stackable,
            container_slots,
            stats_count,
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
            totem_category,
            socket_bonus,
            gem_properties,
            required_disenchant_skill,
            armor_damage_modifier,
            duration,
            item_limit_category,
            disenchant_id,
            food_type,
            extra_flags,
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
        self.display_id
    }

    pub const fn quality(&self) -> ItemQuality {
        self.quality
    }

    pub const fn flags(&self) -> ItemFlag {
        self.flags
    }

    pub const fn flags2(&self) -> ItemFlag2 {
        self.flags2
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

    pub const fn required_spell(&self) -> i32 {
        self.required_spell
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
            12585 | 44119 => 1,
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

    pub const fn stats_count(&self) -> i32 {
        self.stats_count as i32
    }

    /// Returns `0` except for specific item entries.
    pub const fn scaling_stat_distribution(&self) -> i32 {
        match self.entry {
            42943 => 1,
            42944 => 2,
            42945 => 3,
            42946 => 4,
            42947 => 5,
            42948 => 6,
            42949 => 7,
            42950 => 8,
            42951 => 9,
            42952 => 10,
            42984 => 11,
            42985 => 16,
            42991 => 251,
            42992 => 271,
            44091 => 292,
            44092 => 293,
            44093 => 294,
            44094 => 295,
            44095 => 296,
            44096 => 297,
            44097 => 298,
            44098 => 299,
            44099 => 300,
            44100 => 301,
            44101 => 302,
            44102 => 303,
            44103 => 304,
            44105 => 305,
            44107 => 306,
            48677 => 331,
            48683 => 332,
            48685 => 333,
            48687 => 334,
            48689 => 335,
            48691 => 336,
            48716 => 351,
            48718 => 352,
            50255 => 371,
            _ => 0,
        }
    }

    /// Returns `0` except for specific item entries.
    pub const fn scaling_stat_value(&self) -> i32 {
        match self.entry {
            42943 | 44092 | 48718 => 1032,
            42944 | 42945 | 44091 | 44096 | 48716 => 516,
            42946 | 44093 => 8208,
            42947 | 44095 => 36872,
            42948 | 44094 => 34820,
            42949 | 44099 | 44100 => 257,
            42950 | 42951 | 44101 | 44102 => 129,
            42952 | 42984 | 44103 | 44105 => 65,
            42985 | 44107 => 33,
            42991 | 42992 | 44097 | 44098 => 2,
            48677 | 48683 => 4194312,
            48685 => 8388616,
            48687 | 48689 => 2097160,
            48691 => 1048584,
            50255 => 262144,
            _ => 0,
        }
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

    /// Returns `0` except for specific item entries.
    pub const fn arcane_res(&self) -> i32 {
        match self.entry {
            1604 | 20141 | 20267 => 7,
            5828 => 255,
            9434 | 9455 | 9461 | 9508 | 12903 | 12945 | 17107 | 17113 | 18800 | 20269 | 21467 => 5,
            11785 | 13009 | 13961 | 13966 | 15857 | 16901 | 16909 | 16915 | 16922 | 16930 | 16938 | 16946 | 16954 | 16962 | 17000 | 18545 | 18813 | 20136 | 20139 | 21128 | 23363 | 27895 => 10,
            12065 => 8,
            12252 | 13030 | 17078 | 17110 | 20142 | 49307 | 49491 => 6,
            12609 | 13538 | 27449 => 20,
            13065 => 9,
            13535 | 13539 | 23042 => 13,
            14128 => 17,
            14130 | 31929 | 31930 | 31938 => 18,
            14132 | 15072 | 15075 => 16,
            14543 | 15815 => 15,
            15073 | 18341 | 28301 => 12,
            15074 | 18342 => 11,
            18582 => 100,
            18583 => 60,
            18584 | 21868 => 50,
            20146 => 2,
            20275 | 20276 | 20615 => 4,
            20278 | 20279 | 21792 => 3,
            21863 | 21867 => 35,
            21864 | 23510 | 23511 | 24098 | 29490 | 29491 | 29496 | 29497 | 30825 | 49489 => 30,
            21865 | 31113 => 45,
            21866 | 49309 => 25,
            23509 | 23512 | 29489 | 29495 => 40,
            30831 => 32,
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

    /// Returns `Language::Universal` except for specific item entries.
    pub const fn language(&self) -> Language {
        match self.entry {
            25 | 4992 | 4995 | 5917 | 6167 | 20010 | 21783 | 23798 => Language::Orcish,
            745 | 957 | 1208 | 1252 | 1283 | 1284 | 1307 | 1319 | 1637 | 1656 | 1971 | 1972 | 2161 | 2223 | 2548 | 2560 | 2637 | 2666 | 2696 | 2720 | 2722 | 2724 | 2760 | 2794 | 2795 | 3085 | 3086 | 3087 | 3248 | 3250 | 3518 | 4130 | 4429 | 4432 | 4514 | 5351 | 5354 | 5455 | 5520 | 5622 | 5790 | 5882 | 5947 | 5998 | 6280 | 6332 | 6678 | 6743 | 6843 | 9244 | 9245 | 9250 | 9329 | 9330 | 9370 | 11108 | 11125 | 12771 | 12813 | 12820 | 12985 | 20009 | 20828 | 22979 | 23777 | 23780 | 23797 | 28303 | 30420 | 30973 | 35122 => Language::Common,
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
            39014 => 1780,
            42953 => 1807,
            43575 => 1667,
            43622 => 1812,
            43624 | 45986 => 1813,
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
            37664 => Area::GrizzlyHills,
            38335 | 38336 | 38337 | 38338 | 38339 | 38340 | 38341 | 38342 | 38343 | 38344 | 38345 | 38346 | 38369 | 38370 | 38379 | 38381 | 38384 | 38386 | 38393 | 38396 | 38397 | 38398 | 39668 | 39669 | 39670 => Area::ZulDrak,
            39213 => Area::StrandOfTheAncients,
            39737 => Area::SholazarBasin,
            42986 | 43002 => Area::Wintergrasp,
            45765 | 45918 | 45925 => Area::Icecrown,
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
            37372 => Map::UtgardePinnacle,
            37815 | 37859 | 37860 => Map::TheOculus,
            37888 => Map::TheCullingOfStratholme,
            38555 => Map::DrakTharonKeep,
            43494 => Map::AhnKahetTheOldKingdom,
            46847 | 47030 => Map::IsleOfConquest,
            50307 => Map::IcecrownCitadel,
            _ => Map::EasternKingdoms,
        }
    }

    pub const fn bag_family(&self) -> BagFamily {
        self.bag_family
    }

    pub const fn totem_category(&self) -> i32 {
        self.totem_category as i32
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

    pub const fn duration(&self) -> i32 {
        self.duration
    }

    pub const fn item_limit_category(&self) -> i32 {
        self.item_limit_category as i32
    }

    /// Returns `0` except for specific item entries.
    pub const fn holiday_id(&self) -> i32 {
        match self.entry {
            21212 => 141,
            21816 | 21819 | 21822 | 22236 | 49351 | 49352 | 49631 | 49669 | 49670 | 49867 | 49909 | 49915 | 49937 | 49939 | 49941 | 54537 => 423,
            33023 | 33024 | 33025 | 33026 | 33028 | 33029 | 33030 | 33031 | 33032 | 33033 | 33034 | 33035 | 33036 | 33043 | 34017 | 34018 | 34019 | 34020 | 34021 | 34022 | 34063 | 34064 | 34065 | 37750 | 37829 | 39476 | 39477 | 46399 | 46400 | 46401 | 46402 | 46403 | 46735 => 372,
            37011 | 54516 => 324,
            44791 | 44802 | 44806 | 44818 | 45072 => 181,
            46690 | 46691 | 46711 | 46718 | 46831 | 46860 => 409,
            46793 | 46796 => 404,
            _ => 0,
        }
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
            6827 | 16884 => 200,
            9276 => 47,
            10456 => 2037,
            11937 => 6235,
            11966 => 425,
            16882 => 25,
            16883 | 20766 | 21113 | 21150 | 21228 => 100,
            16885 => 300,
            20602 => 600000,
            20708 => 50,
            20767 => 500,
            20768 => 1000,
            23022 | 34592 | 34593 => 50000,
            23921 => 1930,
            29569 => 150,
            34583..=34584 => 20000,
            34585 | 34587 => 30000,
            34594..=34595 => 90000,
            34863 | 35348 => 60000,
            43346 => 500000,
            43347 => 300000,
            44663 => 150000,
            45724 => 100000,
            _ => 0,
        }
    }

    /// Returns `0` except for specific item entries.
    pub const fn max_money_loot(&self) -> i32 {
        match self.entry {
            5335 => 2250,
            6827 | 16884 => 300,
            9276 => 302,
            10456 => 6110,
            11937 => 18704,
            11966 => 1275,
            16882 => 125,
            16883 | 21113 | 21150 | 21228 => 200,
            16885 | 29569 => 600,
            20602 => 1000000,
            20708 => 100,
            20766 => 1000,
            20767 => 1500,
            20768 => 2500,
            23022 => 50000,
            23921 => 5790,
            34583..=34584 => 30000,
            34585 | 34587 => 40000,
            34592..=34593 => 70000,
            34594..=34595 => 110000,
            34863 | 35348 => 80000,
            43346 => 750000,
            43347 => 500000,
            44663 => 250000,
            45724 => 100000,
            _ => 0,
        }
    }

    pub const fn extra_flags(&self) -> i32 {
        self.extra_flags as i32
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

    pub const fn damages_array(&self) -> [ItemDamageType; 2] {
        const D: ItemDamageType=ItemDamageType{
            damage_minimum:0.0,
            damage_maximum:0.0,
            school:SpellSchool::Normal,
        };
        let l = self.damages.len();
        [
            if l >= 1 { self.damages()[0] } else { D },
            if l >= 2 { self.damages()[1] } else { D },
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
