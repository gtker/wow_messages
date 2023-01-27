pub(crate) mod conversions;

use crate::base_printer::data::items::Items;

pub(crate) struct Stats {
    pub strength: i32,
    pub agility: i32,
    pub stamina: i32,
    pub intellect: i32,
    pub spirit: i32,
    pub health: i32,
    pub mana: i32,
}

use crate::base_printer::data::items::tbc::TbcItem;
use crate::base_printer::data::items::vanilla::VanillaItem;
use crate::base_printer::data::items::wrath::WrathItem;
use crate::base_printer::data::Data;
use crate::base_printer::writer::Writer;
use crate::base_printer::Expansion;
use crate::file_utils::overwrite_autogenerate_if_not_the_same;
use std::path::Path;

pub(crate) fn write_items(path: &Path, data: &Data) {
    let mut s = Writer::new();

    match &data.items {
        Items::Vanilla(v) => vanilla(&mut s, v),
        Items::BurningCrusade(v) => tbc(&mut s, v),
        Items::Wrath(v) => wrath(&mut s, v),
    }

    overwrite_autogenerate_if_not_the_same(&path, s.inner());
}

fn float_format(v: f32) -> String {
    let s = format!("{v}");
    if s.contains('.') {
        format!("{s},")
    } else {
        format!("{s}.0,")
    }
}

fn vanilla_unobtainable(item: &VanillaItem) -> bool {
    unobtainable(item.entry, item.extra_flags, &item.name)
}

fn tbc_unobtainable(item: &TbcItem) -> bool {
    unobtainable(item.entry, item.extra_flags, &item.name)
}

fn wrath_unobtainable(item: &WrathItem) -> bool {
    unobtainable(item.entry, item.extra_flags, &item.name)
}

fn unobtainable(entry: i32, extra_flags: i32, name: &str) -> bool {
    const UNOBTAINABLE_FLAG: i32 = 0x04;
    let unobtainable_flag_is_set = extra_flags & UNOBTAINABLE_FLAG != 0;

    let name_ends_with_deprecated = name.ends_with("DEPRECATED") || name.ends_with("DEP");
    let name_ends_with_test = name.ends_with(" Test") || name.ends_with("(Test)");

    let name_starts_with_old = name.starts_with("OLD") || name.starts_with("(OLD)");
    let name_starts_with_monster = name.starts_with("Monster - ");
    let name_starts_with_test = name.starts_with("TEST ");
    let name_starts_with_deprecated = name.starts_with("Deprecated");

    let name_contains_ph = name.contains("[PH]");

    let martin_thunder_or_martin_fury = entry == 17 || entry == 192;

    let glaive_of_the_defender = entry == 23051;

    let warglaives_of_azzinoth = entry == 18582 || entry == 18583 || entry == 18584;

    unobtainable_flag_is_set
        || name_ends_with_deprecated
        || name_starts_with_old
        || name_ends_with_test
        || name_starts_with_monster
        || name_starts_with_test
        || name_starts_with_deprecated
        || name_contains_ph
        || martin_thunder_or_martin_fury
        || glaive_of_the_defender
        || warglaives_of_azzinoth
}

fn print_unobtainable_cfg(s: &mut Writer) {
    s.wln("#[cfg(feature = \"unobtainable-items\")]");
}

fn string_format(v: &str) -> String {
    format!("\"{}\",", v.replace('"', "\\\""))
}

fn includes(s: &mut Writer, expansion: Expansion) {
    s.wln(format!(
        "use wow_world_base::{}::{{",
        expansion.as_module_string()
    ));
    s.inc_indent();
    for ty in [
        "AllowedClass",
        "AllowedRace",
        "Bonding",
        "InventoryType",
        "Item",
        "ItemClassAndSubClass",
        "ItemQuality",
        "SpellSchool",
        "SpellTriggerType",
    ] {
        s.wln(format!("{},", ty));
    }

    s.dec_indent();
    s.wln("};");
}

fn vanilla(s: &mut Writer, items: &[VanillaItem]) {
    includes(s, Expansion::Vanilla);

    s.wln("pub const ITEMS: &[Item] = &[");
    s.inc_indent();

    for item in items {
        if vanilla_unobtainable(item) {
            print_unobtainable_cfg(s);
        }
        s.w("Item::new(");

        s.w_no_indent(format!("{},", item.entry,));

        s.w_no_indent(format!(
            "ItemClassAndSubClass::{},",
            item.class_and_sub_class
        ));
        s.w_no_indent(string_format(&item.name));
        s.w_no_indent(format!("{},", item.displayid,));
        s.w_no_indent(format!("ItemQuality::{},", item.quality));
        s.w_no_indent(format!("{},", item.flags,));
        s.w_no_indent(format!("{},", item.buy_count,));
        s.w_no_indent(format!("{},", item.buy_price,));
        s.w_no_indent(format!("{},", item.sell_price,));
        s.w_no_indent(format!("InventoryType::{},", item.inventory_type));
        s.w_no_indent(format!(
            "AllowedClass::new({}),",
            item.allowed_class.as_int(),
        ));
        s.w_no_indent(format!("AllowedRace::new({}),", item.allowed_race.as_int(),));
        s.w_no_indent(format!("{},", item.item_level,));
        s.w_no_indent(format!("{},", item.required_level,));
        s.w_no_indent(format!("{},", item.required_skill,));
        s.w_no_indent(format!("{},", item.required_skill_rank,));
        s.w_no_indent(format!("{},", item.required_spell,));
        s.w_no_indent(format!("{},", item.required_honor_rank,));
        s.w_no_indent(format!("{},", item.required_city_rank,));
        s.w_no_indent(format!("{},", item.required_reputation_faction,));
        s.w_no_indent(format!("{},", item.required_reputation_rank,));
        s.w_no_indent(format!("{},", item.max_count,));
        s.w_no_indent(format!("{},", item.stackable,));
        s.w_no_indent(format!("{},", item.container_slots,));

        s.w_no_indent(format!("{},", item.mana));
        s.w_no_indent(format!("{},", item.health));
        s.w_no_indent(format!("{},", item.agility));
        s.w_no_indent(format!("{},", item.strength));
        s.w_no_indent(format!("{},", item.stamina));
        s.w_no_indent(format!("{},", item.intellect));
        s.w_no_indent(format!("{},", item.spirit));

        s.w_no_indent(float_format(item.dmg_min1));
        s.w_no_indent(float_format(item.dmg_max1));
        s.w_no_indent(format!("SpellSchool::{},", item.dmg_type1));
        s.w_no_indent(float_format(item.dmg_min2));
        s.w_no_indent(float_format(item.dmg_max2));
        s.w_no_indent(format!("SpellSchool::{},", item.dmg_type2));
        s.w_no_indent(float_format(item.dmg_min3));
        s.w_no_indent(float_format(item.dmg_max3));
        s.w_no_indent(format!("SpellSchool::{},", item.dmg_type3));
        s.w_no_indent(float_format(item.dmg_min4));
        s.w_no_indent(float_format(item.dmg_max4));
        s.w_no_indent(format!("SpellSchool::{},", item.dmg_type4));
        s.w_no_indent(float_format(item.dmg_min5));
        s.w_no_indent(float_format(item.dmg_max5));
        s.w_no_indent(format!("SpellSchool::{},", item.dmg_type5));
        s.w_no_indent(format!("{},", item.armor,));
        s.w_no_indent(format!("{},", item.holy_res,));
        s.w_no_indent(format!("{},", item.fire_res,));
        s.w_no_indent(format!("{},", item.nature_res,));
        s.w_no_indent(format!("{},", item.frost_res,));
        s.w_no_indent(format!("{},", item.shadow_res,));
        s.w_no_indent(format!("{},", item.arcane_res,));
        s.w_no_indent(format!("{},", item.delay,));
        s.w_no_indent(format!("{},", item.ammo_type,));
        s.w_no_indent(float_format(item.ranged_mod_range));
        s.w_no_indent(format!("{},", item.spell_id_1,));
        s.w_no_indent(format!("SpellTriggerType::{},", item.spell_trigger_1));
        s.w_no_indent(format!("{},", item.spell_charges_1,));
        s.w_no_indent(float_format(item.spell_ppm_rate_1));
        s.w_no_indent(format!("{},", item.spell_cooldown_1,));
        s.w_no_indent(format!("{},", item.spell_category_1,));
        s.w_no_indent(format!("{},", item.spell_category_cooldown_1,));
        s.w_no_indent(format!("{},", item.spell_id_2,));
        s.w_no_indent(format!("SpellTriggerType::{},", item.spell_trigger_2));
        s.w_no_indent(format!("{},", item.spell_charges_2,));
        s.w_no_indent(float_format(item.spell_ppm_rate_2));
        s.w_no_indent(format!("{},", item.spell_cooldown_2,));
        s.w_no_indent(format!("{},", item.spell_category_2,));
        s.w_no_indent(format!("{},", item.spell_category_cooldown_2,));
        s.w_no_indent(format!("{},", item.spell_id_3,));
        s.w_no_indent(format!("SpellTriggerType::{},", item.spell_trigger_3));
        s.w_no_indent(format!("{},", item.spell_charges_3,));
        s.w_no_indent(float_format(item.spell_ppm_rate_3));
        s.w_no_indent(format!("{},", item.spell_cooldown_3,));
        s.w_no_indent(format!("{},", item.spell_category_3,));
        s.w_no_indent(format!("{},", item.spell_category_cooldown_3,));
        s.w_no_indent(format!("{},", item.spell_id_4,));
        s.w_no_indent(format!("SpellTriggerType::{},", item.spell_trigger_4));
        s.w_no_indent(format!("{},", item.spell_charges_4,));
        s.w_no_indent(float_format(item.spell_ppm_rate_4));
        s.w_no_indent(format!("{},", item.spell_cooldown_4,));
        s.w_no_indent(format!("{},", item.spell_category_4,));
        s.w_no_indent(format!("{},", item.spell_category_cooldown_4,));
        s.w_no_indent(format!("{},", item.spell_id_5,));
        s.w_no_indent(format!("SpellTriggerType::{},", item.spell_trigger_5));
        s.w_no_indent(format!("{},", item.spell_charges_5,));
        s.w_no_indent(float_format(item.spell_ppm_rate_5));
        s.w_no_indent(format!("{},", item.spell_cooldown_5,));
        s.w_no_indent(format!("{},", item.spell_category_5,));
        s.w_no_indent(format!("{},", item.spell_category_cooldown_5,));
        s.w_no_indent(format!("Bonding::{:?},", item.bonding));
        s.w_no_indent(string_format(&item.description));
        s.w_no_indent(format!("{},", item.page_text,));
        s.w_no_indent(format!("{},", item.language_id,));
        s.w_no_indent(format!("{},", item.page_material,));
        s.w_no_indent(format!("{},", item.start_quest,));
        s.w_no_indent(format!("{},", item.lock_id,));
        s.w_no_indent(format!("{},", item.material,));
        s.w_no_indent(format!("{},", item.sheath,));
        s.w_no_indent(format!("{},", item.random_property,));
        s.w_no_indent(format!("{},", item.block,));
        s.w_no_indent(format!("{},", item.itemset,));
        s.w_no_indent(format!("{},", item.max_durability,));
        s.w_no_indent(format!("{},", item.area,));
        s.w_no_indent(format!("{},", item.map,));
        s.w_no_indent(format!("{},", item.bag_family,));
        s.w_no_indent(string_format(&item.script_name));
        s.w_no_indent(format!("{},", item.disenchant_id,));
        s.w_no_indent(format!("{},", item.food_type,));
        s.w_no_indent(format!("{},", item.min_money_loot,));
        s.w_no_indent(format!("{},", item.max_money_loot,));
        s.w_no_indent(format!("{},", item.duration,));
        s.w_no_indent(format!("{},", item.extra_flags,));

        s.wln_no_indent("),");
    }

    s.dec_indent();
    s.wln("];");
}

fn tbc(s: &mut Writer, items: &[TbcItem]) {
    includes(s, Expansion::BurningCrusade);

    s.wln("pub const ITEMS: &[Item] = &[");
    s.inc_indent();

    for item in items {
        if tbc_unobtainable(item) {
            print_unobtainable_cfg(s);
        }
        s.w("Item::new(");
        s.w_no_indent(format!("{},", item.entry));
        s.w_no_indent(format!(
            "ItemClassAndSubClass::{},",
            item.class_and_sub_class,
        ));
        s.w_no_indent(format!("{},", item.unk0));
        s.w_no_indent(string_format(&item.name));
        s.w_no_indent(format!("{},", item.displayid));
        s.w_no_indent(format!("ItemQuality::{},", item.quality));
        s.w_no_indent(format!("{},", item.flags));
        s.w_no_indent(format!("{},", item.buy_count));
        s.w_no_indent(format!("{},", item.buy_price));
        s.w_no_indent(format!("{},", item.sell_price));
        s.w_no_indent(format!("InventoryType::{},", item.inventory_type));
        s.w_no_indent(format!(
            "AllowedClass::new({}),",
            item.allowed_class.as_int()
        ));
        s.w_no_indent(format!("AllowedRace::new({}),", item.allowed_race.as_int()));
        s.w_no_indent(format!("{},", item.item_level));
        s.w_no_indent(format!("{},", item.required_level));
        s.w_no_indent(format!("{},", item.required_skill));
        s.w_no_indent(format!("{},", item.required_skill_rank));
        s.w_no_indent(format!("{},", item.required_spell));
        s.w_no_indent(format!("{},", item.required_honor_rank));
        s.w_no_indent(format!("{},", item.required_city_rank));
        s.w_no_indent(format!("{},", item.required_reputation_faction));
        s.w_no_indent(format!("{},", item.required_reputation_rank));
        s.w_no_indent(format!("{},", item.max_count));
        s.w_no_indent(format!("{},", item.stackable));
        s.w_no_indent(format!("{},", item.container_slots));
        s.w_no_indent(format!("{},", item.stat_type1));
        s.w_no_indent(format!("{},", item.stat_value1));
        s.w_no_indent(format!("{},", item.stat_type2));
        s.w_no_indent(format!("{},", item.stat_value2));
        s.w_no_indent(format!("{},", item.stat_type3));
        s.w_no_indent(format!("{},", item.stat_value3));
        s.w_no_indent(format!("{},", item.stat_type4));
        s.w_no_indent(format!("{},", item.stat_value4));
        s.w_no_indent(format!("{},", item.stat_type5));
        s.w_no_indent(format!("{},", item.stat_value5));
        s.w_no_indent(format!("{},", item.stat_type6));
        s.w_no_indent(format!("{},", item.stat_value6));
        s.w_no_indent(format!("{},", item.stat_type7));
        s.w_no_indent(format!("{},", item.stat_value7));

        assert_eq!(item.stat_type8, 0);
        assert_eq!(item.stat_value8, 0);
        assert_eq!(item.stat_type9, 0);
        assert_eq!(item.stat_value9, 0);
        assert_eq!(item.stat_type10, 0);
        assert_eq!(item.stat_value10, 0);

        s.w_no_indent(float_format(item.dmg_min1));
        s.w_no_indent(float_format(item.dmg_max1));
        s.w_no_indent(format!("SpellSchool::{},", item.dmg_type1));
        s.w_no_indent(float_format(item.dmg_min2));
        s.w_no_indent(float_format(item.dmg_max2));
        s.w_no_indent(format!("SpellSchool::{},", item.dmg_type2));
        s.w_no_indent(float_format(item.dmg_min3));
        s.w_no_indent(float_format(item.dmg_max3));
        s.w_no_indent(format!("SpellSchool::{},", item.dmg_type3));
        s.w_no_indent(float_format(item.dmg_min4));
        s.w_no_indent(float_format(item.dmg_max4));
        s.w_no_indent(format!("SpellSchool::{},", item.dmg_type4));
        s.w_no_indent(float_format(item.dmg_min5));
        s.w_no_indent(float_format(item.dmg_max5));
        s.w_no_indent(format!("SpellSchool::{},", item.dmg_type5));
        s.w_no_indent(format!("{},", item.armor));
        s.w_no_indent(format!("{},", item.holy_res));
        s.w_no_indent(format!("{},", item.fire_res));
        s.w_no_indent(format!("{},", item.nature_res));
        s.w_no_indent(format!("{},", item.frost_res));
        s.w_no_indent(format!("{},", item.shadow_res));
        s.w_no_indent(format!("{},", item.arcane_res));
        s.w_no_indent(format!("{},", item.delay));
        s.w_no_indent(format!("{},", item.ammo_type));
        s.w_no_indent(float_format(item.ranged_mod_range));
        s.w_no_indent(format!("{},", item.spell_id_1));
        s.w_no_indent(format!("SpellTriggerType::{},", item.spell_trigger_1));
        s.w_no_indent(format!("{},", item.spell_charges_1));
        s.w_no_indent(float_format(item.spell_ppm_rate_1));
        s.w_no_indent(format!("{},", item.spell_cooldown_1));
        s.w_no_indent(format!("{},", item.spell_category_1));
        s.w_no_indent(format!("{},", item.spell_category_cooldown_1));
        s.w_no_indent(format!("{},", item.spell_id_2));
        s.w_no_indent(format!("SpellTriggerType::{},", item.spell_trigger_2));
        s.w_no_indent(format!("{},", item.spell_charges_2));
        s.w_no_indent(float_format(item.spell_ppm_rate_2));
        s.w_no_indent(format!("{},", item.spell_cooldown_2));
        s.w_no_indent(format!("{},", item.spell_category_2));
        s.w_no_indent(format!("{},", item.spell_category_cooldown_2));
        s.w_no_indent(format!("{},", item.spell_id_3));
        s.w_no_indent(format!("SpellTriggerType::{},", item.spell_trigger_3));
        s.w_no_indent(format!("{},", item.spell_charges_3));
        s.w_no_indent(float_format(item.spell_ppm_rate_3));
        s.w_no_indent(format!("{},", item.spell_cooldown_3));
        s.w_no_indent(format!("{},", item.spell_category_3));
        s.w_no_indent(format!("{},", item.spell_category_cooldown_3));
        s.w_no_indent(format!("{},", item.spell_id_4));
        s.w_no_indent(format!("SpellTriggerType::{},", item.spell_trigger_4));
        s.w_no_indent(format!("{},", item.spell_charges_4));
        s.w_no_indent(float_format(item.spell_ppm_rate_4));
        s.w_no_indent(format!("{},", item.spell_cooldown_4));
        s.w_no_indent(format!("{},", item.spell_category_4));
        s.w_no_indent(format!("{},", item.spell_category_cooldown_4));
        s.w_no_indent(format!("{},", item.spell_id_5));
        s.w_no_indent(format!("SpellTriggerType::{},", item.spell_trigger_5));
        s.w_no_indent(format!("{},", item.spell_charges_5));
        s.w_no_indent(float_format(item.spell_ppm_rate_5));
        s.w_no_indent(format!("{},", item.spell_cooldown_5));
        s.w_no_indent(format!("{},", item.spell_category_5));
        s.w_no_indent(format!("{},", item.spell_category_cooldown_5));
        s.w_no_indent(format!("Bonding::{:?},", item.bonding));
        s.w_no_indent(string_format(&item.description));
        s.w_no_indent(format!("{},", item.page_text));
        s.w_no_indent(format!("{},", item.language_id));
        s.w_no_indent(format!("{},", item.page_material));
        s.w_no_indent(format!("{},", item.start_quest));
        s.w_no_indent(format!("{},", item.lock_id));
        s.w_no_indent(format!("{},", item.material));
        s.w_no_indent(format!("{},", item.sheath));
        s.w_no_indent(format!("{},", item.random_property));
        s.w_no_indent(format!("{},", item.random_suffix));
        s.w_no_indent(format!("{},", item.block));
        s.w_no_indent(format!("{},", item.itemset));
        s.w_no_indent(format!("{},", item.max_durability));
        s.w_no_indent(format!("{},", item.area));
        s.w_no_indent(format!("{},", item.map));
        s.w_no_indent(format!("{},", item.bag_family));
        s.w_no_indent(format!("{},", item.totem_category));
        s.w_no_indent(format!("{},", item.socket_color_1));
        s.w_no_indent(format!("{},", item.socket_content_1));
        s.w_no_indent(format!("{},", item.socket_color_2));
        s.w_no_indent(format!("{},", item.socket_content_2));
        s.w_no_indent(format!("{},", item.socket_color_3));
        s.w_no_indent(format!("{},", item.socket_content_3));
        s.w_no_indent(format!("{},", item.socket_bonus));
        s.w_no_indent(format!("{},", item.gem_properties));
        s.w_no_indent(format!("{},", item.required_disenchant_skill));
        s.w_no_indent(float_format(item.armor_damage_modifier));
        s.w_no_indent(string_format(&item.script_name));
        s.w_no_indent(format!("{},", item.disenchant_id));
        s.w_no_indent(format!("{},", item.food_type));
        s.w_no_indent(format!("{},", item.min_money_loot));
        s.w_no_indent(format!("{},", item.max_money_loot));
        s.w_no_indent(format!("{},", item.duration));
        s.w_no_indent(format!("{},", item.extra_flags));

        s.wln_no_indent("),");
    }

    s.dec_indent();
    s.wln("];");
}

fn wrath(s: &mut Writer, items: &[WrathItem]) {
    includes(s, Expansion::WrathOfTheLichKing);

    s.wln("pub const ITEMS: &[Item] = &[");
    s.inc_indent();

    for item in items {
        if wrath_unobtainable(item) {
            print_unobtainable_cfg(s);
        }
        s.w("Item::new(");

        s.w_no_indent(format!("{},", item.entry));
        s.w_no_indent(format!(
            "ItemClassAndSubClass::{},",
            item.class_and_subclass,
        ));
        s.w_no_indent(format!("{},", item.unk0));
        s.w_no_indent(string_format(&item.name));
        s.w_no_indent(format!("{},", item.displayid));
        s.w_no_indent(format!("ItemQuality::{},", item.quality));
        s.w_no_indent(format!("{},", item.flags));
        s.w_no_indent(format!("{},", item.flags2));
        s.w_no_indent(format!("{},", item.buy_count));
        s.w_no_indent(format!("{},", item.buy_price));
        s.w_no_indent(format!("{},", item.sell_price));
        s.w_no_indent(format!("InventoryType::{},", item.inventory_type));
        s.w_no_indent(format!(
            "AllowedClass::new({}),",
            item.allowed_class.as_int()
        ));
        s.w_no_indent(format!("AllowedRace::new({}),", item.allowed_race.as_int()));
        s.w_no_indent(format!("{},", item.item_level));
        s.w_no_indent(format!("{},", item.required_level));
        s.w_no_indent(format!("{},", item.required_skill));
        s.w_no_indent(format!("{},", item.required_skill_rank));
        s.w_no_indent(format!("{},", item.required_spell));
        s.w_no_indent(format!("{},", item.required_honor_rank));
        s.w_no_indent(format!("{},", item.required_city_rank));
        s.w_no_indent(format!("{},", item.required_reputation_faction));
        s.w_no_indent(format!("{},", item.required_reputation_rank));
        s.w_no_indent(format!("{},", item.max_count));
        s.w_no_indent(format!("{},", item.stackable));
        s.w_no_indent(format!("{},", item.container_slots));
        s.w_no_indent(format!("{},", item.stats_count));
        s.w_no_indent(format!("{},", item.stat_type1));
        s.w_no_indent(format!("{},", item.stat_value1));
        s.w_no_indent(format!("{},", item.stat_type2));
        s.w_no_indent(format!("{},", item.stat_value2));
        s.w_no_indent(format!("{},", item.stat_type3));
        s.w_no_indent(format!("{},", item.stat_value3));
        s.w_no_indent(format!("{},", item.stat_type4));
        s.w_no_indent(format!("{},", item.stat_value4));
        s.w_no_indent(format!("{},", item.stat_type5));
        s.w_no_indent(format!("{},", item.stat_value5));
        s.w_no_indent(format!("{},", item.stat_type6));
        s.w_no_indent(format!("{},", item.stat_value6));
        s.w_no_indent(format!("{},", item.stat_type7));
        s.w_no_indent(format!("{},", item.stat_value7));

        assert_eq!(item.stat_type8, 0);
        assert_eq!(item.stat_value8, 0);
        assert_eq!(item.stat_type9, 0);
        assert_eq!(item.stat_value9, 0);
        assert_eq!(item.stat_type10, 0);
        assert_eq!(item.stat_value10, 0);

        s.w_no_indent(format!("{},", item.scaling_stat_distribution));
        s.w_no_indent(format!("{},", item.scaling_stat_value));
        s.w_no_indent(float_format(item.dmg_min1));
        s.w_no_indent(float_format(item.dmg_max1));
        s.w_no_indent(format!("SpellSchool::{},", item.dmg_type1));
        s.w_no_indent(float_format(item.dmg_min2));
        s.w_no_indent(float_format(item.dmg_max2));
        s.w_no_indent(format!("SpellSchool::{},", item.dmg_type2));
        s.w_no_indent(format!("{},", item.armor));
        s.w_no_indent(format!("{},", item.holy_res));
        s.w_no_indent(format!("{},", item.fire_res));
        s.w_no_indent(format!("{},", item.nature_res));
        s.w_no_indent(format!("{},", item.frost_res));
        s.w_no_indent(format!("{},", item.shadow_res));
        s.w_no_indent(format!("{},", item.arcane_res));
        s.w_no_indent(format!("{},", item.delay));
        s.w_no_indent(format!("{},", item.ammo_type));
        s.w_no_indent(float_format(item.ranged_mod_range));
        s.w_no_indent(format!("{},", item.spell_id_1));
        s.w_no_indent(format!("SpellTriggerType::{},", item.spell_trigger_1));
        s.w_no_indent(format!("{},", item.spell_charges_1));
        s.w_no_indent(float_format(item.spell_ppm_rate_1));
        s.w_no_indent(format!("{},", item.spell_cooldown_1));
        s.w_no_indent(format!("{},", item.spell_category_1));
        s.w_no_indent(format!("{},", item.spell_category_cooldown_1));
        s.w_no_indent(format!("{},", item.spell_id_2));
        s.w_no_indent(format!("SpellTriggerType::{},", item.spell_trigger_2));
        s.w_no_indent(format!("{},", item.spell_charges_2));
        s.w_no_indent(float_format(item.spell_ppm_rate_2));
        s.w_no_indent(format!("{},", item.spell_cooldown_2));
        s.w_no_indent(format!("{},", item.spell_category_2));
        s.w_no_indent(format!("{},", item.spell_category_cooldown_2));
        s.w_no_indent(format!("{},", item.spell_id_3));
        s.w_no_indent(format!("SpellTriggerType::{},", item.spell_trigger_3));
        s.w_no_indent(format!("{},", item.spell_charges_3));
        s.w_no_indent(float_format(item.spell_ppm_rate_3));
        s.w_no_indent(format!("{},", item.spell_cooldown_3));
        s.w_no_indent(format!("{},", item.spell_category_3));
        s.w_no_indent(format!("{},", item.spell_category_cooldown_3));
        s.w_no_indent(format!("{},", item.spell_id_4));
        s.w_no_indent(format!("SpellTriggerType::{},", item.spell_trigger_4));
        s.w_no_indent(format!("{},", item.spell_charges_4));
        s.w_no_indent(float_format(item.spell_ppm_rate_4));
        s.w_no_indent(format!("{},", item.spell_cooldown_4));
        s.w_no_indent(format!("{},", item.spell_category_4));
        s.w_no_indent(format!("{},", item.spell_category_cooldown_4));
        s.w_no_indent(format!("{},", item.spell_id_5));
        s.w_no_indent(format!("SpellTriggerType::{},", item.spell_trigger_5));
        s.w_no_indent(format!("{},", item.spell_charges_5));
        s.w_no_indent(float_format(item.spell_ppm_rate_5));
        s.w_no_indent(format!("{},", item.spell_cooldown_5));
        s.w_no_indent(format!("{},", item.spell_category_5));
        s.w_no_indent(format!("{},", item.spell_category_cooldown_5));
        s.w_no_indent(format!("Bonding::{:?},", item.bonding));
        s.w_no_indent(string_format(&item.description));
        s.w_no_indent(format!("{},", item.page_text));
        s.w_no_indent(format!("{},", item.language_id));
        s.w_no_indent(format!("{},", item.page_material));
        s.w_no_indent(format!("{},", item.start_quest));
        s.w_no_indent(format!("{},", item.lock_id));
        s.w_no_indent(format!("{},", item.material));
        s.w_no_indent(format!("{},", item.sheath));
        s.w_no_indent(format!("{},", item.random_property));
        s.w_no_indent(format!("{},", item.random_suffix));
        s.w_no_indent(format!("{},", item.block));
        s.w_no_indent(format!("{},", item.itemset));
        s.w_no_indent(format!("{},", item.max_durability));
        s.w_no_indent(format!("{},", item.area));
        s.w_no_indent(format!("{},", item.map));
        s.w_no_indent(format!("{},", item.bag_family));
        s.w_no_indent(format!("{},", item.totem_category));
        s.w_no_indent(format!("{},", item.socket_color_1));
        s.w_no_indent(format!("{},", item.socket_content_1));
        s.w_no_indent(format!("{},", item.socket_color_2));
        s.w_no_indent(format!("{},", item.socket_content_2));
        s.w_no_indent(format!("{},", item.socket_color_3));
        s.w_no_indent(format!("{},", item.socket_content_3));
        s.w_no_indent(format!("{},", item.socket_bonus));
        s.w_no_indent(format!("{},", item.gem_properties));
        s.w_no_indent(format!("{},", item.required_disenchant_skill));
        s.w_no_indent(float_format(item.armor_damage_modifier));
        s.w_no_indent(format!("{},", item.duration));
        s.w_no_indent(format!("{},", item.item_limit_category));
        s.w_no_indent(format!("{},", item.holiday_id));
        s.w_no_indent(string_format(&item.script_name));
        s.w_no_indent(format!("{},", item.disenchant_id));
        s.w_no_indent(format!("{},", item.food_type));
        s.w_no_indent(format!("{},", item.min_money_loot));
        s.w_no_indent(format!("{},", item.max_money_loot));
        s.w_no_indent(format!("{},", item.extra_flags));

        s.wln_no_indent("),");
    }

    s.dec_indent();
    s.wln("];");
}
