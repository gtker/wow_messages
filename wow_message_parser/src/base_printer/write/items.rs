use crate::base_printer::data::items::{Items, VanillaItem};
use crate::base_printer::data::Data;
use crate::base_printer::writer::Writer;
use crate::file_utils::overwrite_autogenerate_if_not_the_same;
use std::path::Path;

pub(crate) fn write_items(directory: &Path, data: &Data) {
    let mut s = Writer::new();

    match &data.items {
        Items::Vanilla(v) => vanilla(&mut s, v),
        Items::BurningCrusade => return,
        Items::Wrath => return,
    }

    let path = directory.join("item.rs");
    overwrite_autogenerate_if_not_the_same(&path, s.inner());
}

fn float_format(v: f32) -> String {
    let s = format!("{}", v);
    if s.contains('.') {
        format!("{},", s)
    } else {
        format!("{}.0,", s)
    }
}

fn vanilla(s: &mut Writer, items: &[VanillaItem]) {
    s.wln("pub const ITEMS: &'static [Item] = &[");
    s.inc_indent();

    for item in items {
        s.w("Item::new(");

        s.w_no_indent(format!("{},", item.entry,));
        s.w_no_indent(format!("{},", item.class,));
        s.w_no_indent(format!("{},", item.sub_class,));
        s.w_no_indent(format!("\"{}\",", item.name.replace('"', "\\\"")));
        s.w_no_indent(format!("{},", item.displayid,));
        s.w_no_indent(format!("{},", item.quality,));
        s.w_no_indent(format!("{},", item.flags,));
        s.w_no_indent(format!("{},", item.buy_count,));
        s.w_no_indent(format!("{},", item.buy_price,));
        s.w_no_indent(format!("{},", item.sell_price,));
        s.w_no_indent(format!("{},", item.inventory_type,));
        s.w_no_indent(format!("{},", item.allowed_class,));
        s.w_no_indent(format!("{},", item.allowed_race,));
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
        s.w_no_indent(format!("{},", item.stat_type1,));
        s.w_no_indent(format!("{},", item.stat_value1,));
        s.w_no_indent(format!("{},", item.stat_type2,));
        s.w_no_indent(format!("{},", item.stat_value2,));
        s.w_no_indent(format!("{},", item.stat_type3,));
        s.w_no_indent(format!("{},", item.stat_value3,));
        s.w_no_indent(format!("{},", item.stat_type4,));
        s.w_no_indent(format!("{},", item.stat_value4,));
        s.w_no_indent(format!("{},", item.stat_type5,));
        s.w_no_indent(format!("{},", item.stat_value5,));
        s.w_no_indent(format!("{},", item.stat_type6,));
        s.w_no_indent(format!("{},", item.stat_value6,));
        s.w_no_indent(format!("{},", item.stat_type7,));
        s.w_no_indent(format!("{},", item.stat_value7,));
        s.w_no_indent(format!("{},", item.stat_type8,));
        s.w_no_indent(format!("{},", item.stat_value8,));
        s.w_no_indent(format!("{},", item.stat_type9,));
        s.w_no_indent(format!("{},", item.stat_value9,));
        s.w_no_indent(format!("{},", item.stat_type10,));
        s.w_no_indent(format!("{},", item.stat_value10,));
        s.w_no_indent(float_format(item.dmg_min1));
        s.w_no_indent(float_format(item.dmg_max1));
        s.w_no_indent(format!("{},", item.dmg_type1,));
        s.w_no_indent(float_format(item.dmg_min2));
        s.w_no_indent(float_format(item.dmg_max2));
        s.w_no_indent(format!("{},", item.dmg_type2,));
        s.w_no_indent(float_format(item.dmg_min3));
        s.w_no_indent(float_format(item.dmg_max3));
        s.w_no_indent(format!("{},", item.dmg_type3,));
        s.w_no_indent(float_format(item.dmg_min4));
        s.w_no_indent(float_format(item.dmg_max4));
        s.w_no_indent(format!("{},", item.dmg_type4,));
        s.w_no_indent(float_format(item.dmg_min5));
        s.w_no_indent(float_format(item.dmg_max5));
        s.w_no_indent(format!("{},", item.dmg_type5,));
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
        s.w_no_indent(format!("{},", item.spell_trigger_1,));
        s.w_no_indent(format!("{},", item.spell_charges_1,));
        s.w_no_indent(float_format(item.spell_ppm_rate_1));
        s.w_no_indent(format!("{},", item.spell_cooldown_1,));
        s.w_no_indent(format!("{},", item.spell_category_1,));
        s.w_no_indent(format!("{},", item.spell_category_cooldown_1,));
        s.w_no_indent(format!("{},", item.spell_id_2,));
        s.w_no_indent(format!("{},", item.spell_trigger_2,));
        s.w_no_indent(format!("{},", item.spell_charges_2,));
        s.w_no_indent(float_format(item.spell_ppm_rate_2));
        s.w_no_indent(format!("{},", item.spell_cooldown_2,));
        s.w_no_indent(format!("{},", item.spell_category_2,));
        s.w_no_indent(format!("{},", item.spell_category_cooldown_2,));
        s.w_no_indent(format!("{},", item.spell_id_3,));
        s.w_no_indent(format!("{},", item.spell_trigger_3,));
        s.w_no_indent(format!("{},", item.spell_charges_3,));
        s.w_no_indent(float_format(item.spell_ppm_rate_3));
        s.w_no_indent(format!("{},", item.spell_cooldown_3,));
        s.w_no_indent(format!("{},", item.spell_category_3,));
        s.w_no_indent(format!("{},", item.spell_category_cooldown_3,));
        s.w_no_indent(format!("{},", item.spell_id_4,));
        s.w_no_indent(format!("{},", item.spell_trigger_4,));
        s.w_no_indent(format!("{},", item.spell_charges_4,));
        s.w_no_indent(float_format(item.spell_ppm_rate_4));
        s.w_no_indent(format!("{},", item.spell_cooldown_4,));
        s.w_no_indent(format!("{},", item.spell_category_4,));
        s.w_no_indent(format!("{},", item.spell_category_cooldown_4,));
        s.w_no_indent(format!("{},", item.spell_id_5,));
        s.w_no_indent(format!("{},", item.spell_trigger_5,));
        s.w_no_indent(format!("{},", item.spell_charges_5,));
        s.w_no_indent(float_format(item.spell_ppm_rate_5));
        s.w_no_indent(format!("{},", item.spell_cooldown_5,));
        s.w_no_indent(format!("{},", item.spell_category_5,));
        s.w_no_indent(format!("{},", item.spell_category_cooldown_5,));
        s.w_no_indent(format!("{},", item.bonding,));
        s.w_no_indent(format!("\"{}\",", item.description.replace('"', "\\\"")));
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
        s.w_no_indent(format!("\"{}\",", item.script_name,));
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
