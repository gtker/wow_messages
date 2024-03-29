use crate::base_printer::data::get_fields;
use crate::base_printer::data::items::{
    Array, ArrayField, ArrayInstance, ArrayInstances, Field, Optimizations, Value,
};
use crate::base_printer::read_csv_file;
use crate::base_printer::write::items::GenericThing;
use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct WrathSpell {
    id: u32,
    category: i32,
    dispel: i32,
    mechanic: i32,
    attributes: u32,
    attributes_ex: u32,
    attributes_ex2: u32,
    attributes_ex3: u32,
    attributes_ex4: u32,
    attributes_ex5: u32,
    attributes_ex6: u32,
    attributes_ex7: u32,
    stances: u32,
    stances_not: u32,
    targets: i32,
    target_creature_type: i32,
    requires_spell_focus: i32,
    caster_aura_state: i32,
    target_aura_state: i32,
    casting_time_index: i32,
    recovery_time: i32,
    category_recovery_time: i32,
    interrupt_flags: i32,
    aura_interrupt_flags: u32,
    channel_interrupt_flags: u32,
    proc_flags: i32,
    proc_chance: i32,
    proc_charges: i32,
    max_level: i32,
    base_level: i32,
    spell_level: i32,
    duration_index: i32,
    power_type: u32,
    mana_cost: u32,
    #[serde(rename = "ManaCostPerlevel")]
    mana_cost_per_level: i32,
    mana_per_second: i32,
    mana_per_second_per_level: i32,
    range_index: i32,
    speed: f32,
    modal_next_spell: i32,
    stack_amount: i32,
    totem1: i32,
    totem2: i32,
    reagent1: i32,
    reagent2: i32,
    reagent3: i32,
    reagent4: i32,
    reagent5: i32,
    reagent6: i32,
    reagent7: i32,
    reagent8: i32,
    reagent_count1: u32,
    reagent_count2: u32,
    reagent_count3: u32,
    reagent_count4: u32,
    reagent_count5: u32,
    reagent_count6: u32,
    reagent_count7: u32,
    reagent_count8: u32,
    equipped_item_class: i32,
    equipped_item_sub_class_mask: i32,
    equipped_item_inventory_type_mask: i32,
    effect1: i32,
    effect2: i32,
    effect3: i32,
    effect_die_sides1: i32,
    effect_die_sides2: i32,
    effect_die_sides3: i32,
    effect_real_points_per_level1: f32,
    effect_real_points_per_level2: f32,
    effect_real_points_per_level3: f32,
    effect_base_points1: i32,
    effect_base_points2: i32,
    effect_base_points3: i32,
    effect_mechanic1: i32,
    effect_mechanic2: i32,
    effect_mechanic3: i32,
    effect_implicit_target_a1: i32,
    effect_implicit_target_a2: i32,
    effect_implicit_target_a3: i32,
    effect_implicit_target_b1: i32,
    effect_implicit_target_b2: i32,
    effect_implicit_target_b3: i32,
    effect_radius_index1: i32,
    effect_radius_index2: i32,
    effect_radius_index3: i32,
    effect_apply_aura_name1: i32,
    effect_apply_aura_name2: i32,
    effect_apply_aura_name3: i32,
    effect_amplitude1: i32,
    effect_amplitude2: i32,
    effect_amplitude3: i32,
    effect_multiple_value1: f32,
    effect_multiple_value2: f32,
    effect_multiple_value3: f32,
    effect_chain_target1: i32,
    effect_chain_target2: i32,
    effect_chain_target3: i32,
    effect_item_type1: u32,
    effect_item_type2: u32,
    effect_item_type3: u32,
    effect_misc_value1: i32,
    effect_misc_value2: i32,
    effect_misc_value3: i32,
    effect_trigger_spell1: u32,
    effect_trigger_spell2: u32,
    effect_trigger_spell3: u32,
    effect_points_per_combo_point1: f32,
    effect_points_per_combo_point2: f32,
    effect_points_per_combo_point3: f32,
    spell_visual: i32,
    #[serde(rename = "SpellIconID")]
    spell_icon_id: i32,
    #[serde(rename = "ActiveIconID")]
    active_icon_id: i32,
    spell_priority: i32,

    spell_name: String,
    #[allow(unused)] // Data is always either null or empty string
    spell_name2: Option<String>,
    #[allow(unused)] // Data is always either null or empty string
    spell_name3: Option<String>,
    #[allow(unused)] // Data is always either null or empty string
    spell_name4: Option<String>,
    #[allow(unused)] // Data is always either null or empty string
    spell_name5: Option<String>,
    #[allow(unused)] // Data is always either null or empty string
    spell_name6: Option<String>,
    #[allow(unused)] // Data is always either null or empty string
    spell_name7: Option<String>,
    #[allow(unused)] // Data is always either null or empty string
    spell_name8: Option<String>,

    rank1: Option<String>,
    #[allow(unused)] // Data is always either null or empty string
    rank2: Option<String>,
    #[allow(unused)] // Data is always either null or empty string
    rank3: Option<String>,
    #[allow(unused)] // Data is always either null or empty string
    rank4: Option<String>,
    #[allow(unused)] // Data is always either null or empty string
    rank5: Option<String>,
    #[allow(unused)] // Data is always either null or empty string
    rank6: Option<String>,
    #[allow(unused)] // Data is always either null or empty string
    rank7: Option<String>,
    #[allow(unused)] // Data is always either null or empty string
    rank8: Option<String>,

    mana_cost_percentage: i32,
    start_recovery_category: i32,
    start_recovery_time: i32,
    max_target_level: i32,
    spell_family_name: i32,
    spell_family_flags: i64,
    max_affected_targets: i32,
    dmg_class: i32,
    prevention_type: i32,
    stance_bar_order: i32,
    dmg_multiplier1: f32,
    dmg_multiplier2: f32,
    dmg_multiplier3: f32,
    min_faction_id: i32,
    min_reputation: i32,
    required_aura_vision: i32,
    is_server_side: u32,
    attributes_serverside: i32,
    facing_caster_flags: i32,
    caster_aura_state_not: i32,
    target_aura_state_not: i32,
    effect_misc_value_b1: i32,
    effect_misc_value_b2: i32,
    effect_misc_value_b3: i32,

    #[allow(unused)] // Data is always either null or empty string
    spell_name9: Option<String>,
    #[allow(unused)] // Data is always either null or empty string
    spell_name10: Option<String>,
    #[allow(unused)] // Data is always either null or empty string
    spell_name11: Option<String>,
    #[allow(unused)] // Data is always either null or empty string
    spell_name12: Option<String>,
    #[allow(unused)] // Data is always either null or empty string
    spell_name13: Option<String>,
    #[allow(unused)] // Data is always either null or empty string
    spell_name14: Option<String>,
    #[allow(unused)] // Data is always either null or empty string
    spell_name15: Option<String>,
    #[allow(unused)] // Data is always either null or empty string
    spell_name16: Option<String>,

    #[allow(unused)] // Data is always either null or empty string
    rank9: Option<String>,
    #[allow(unused)] // Data is always either null or empty string
    rank10: Option<String>,
    #[allow(unused)] // Data is always either null or empty string
    rank11: Option<String>,
    #[allow(unused)] // Data is always either null or empty string
    rank12: Option<String>,
    #[allow(unused)] // Data is always either null or empty string
    rank13: Option<String>,
    #[allow(unused)] // Data is always either null or empty string
    rank14: Option<String>,
    #[allow(unused)] // Data is always either null or empty string
    rank15: Option<String>,
    #[allow(unused)] // Data is always either null or empty string
    rank16: Option<String>,

    totem_category1: i32,
    totem_category2: i32,
    area_id: u32,
    school_mask: i32,

    stances2: u32,
    stances_not2: u32,
    caster_aura_spell: i32,
    target_aura_spell: i32,
    exclude_caster_aura_spell: i32,
    exclude_target_aura_spell: i32,
    #[serde(rename = "EffectSpellClassMask1_1")]
    effect_spell_class_mask1_1: u32,
    #[serde(rename = "EffectSpellClassMask1_2")]
    effect_spell_class_mask1_2: u32,
    #[serde(rename = "EffectSpellClassMask1_3")]
    effect_spell_class_mask1_3: u32,
    #[serde(rename = "EffectSpellClassMask2_1")]
    effect_spell_class_mask2_1: u32,
    #[serde(rename = "EffectSpellClassMask2_2")]
    effect_spell_class_mask2_2: u32,
    #[serde(rename = "EffectSpellClassMask2_3")]
    effect_spell_class_mask2_3: u32,
    #[serde(rename = "EffectSpellClassMask3_1")]
    effect_spell_class_mask3_1: u32,
    #[serde(rename = "EffectSpellClassMask3_2")]
    effect_spell_class_mask3_2: u32,
    #[serde(rename = "EffectSpellClassMask3_3")]
    effect_spell_class_mask3_3: u32,
    spell_visual2: i32,
    spell_family_flags2: u32,
    #[serde(rename = "RuneCostID")]
    rune_cost_id: u32,
    #[serde(rename = "SpellMissileID")]
    spell_missile_id: u32,
    power_display_id: u32,
    effect_bonus_coefficient1: f32,
    effect_bonus_coefficient2: f32,
    effect_bonus_coefficient3: f32,
    #[serde(rename = "SpellDescriptionVariableID")]
    spell_description_variable_id: u32,
    spell_difficulty_id: u32,
}

pub(crate) fn wrath(dir: &Path) -> (Vec<GenericThing>, Optimizations) {
    let spells: Vec<_> = read_csv_file::<WrathSpell>(dir, "spells")
        .into_iter()
        .map(|row| {
            let fields = vec![
                Field::new("entry", Value::Uint(row.id)),
                Field::new("category", Value::Int(row.category)),
                Field::new("dispel", Value::Int(row.dispel)),
                Field::new("mechanic", Value::Int(row.mechanic)),
                Field::new("attributes", Value::Uint(row.attributes)),
                Field::new("attributes_ex", Value::Uint(row.attributes_ex)),
                Field::new("attributes_ex2", Value::Uint(row.attributes_ex2)),
                Field::new("attributes_ex3", Value::Uint(row.attributes_ex3)),
                Field::new("attributes_ex4", Value::Uint(row.attributes_ex4)),
                Field::new("attributes_ex5", Value::Uint(row.attributes_ex5)),
                Field::new("attributes_ex6", Value::Uint(row.attributes_ex6)),
                Field::new("attributes_ex7", Value::Uint(row.attributes_ex7)),
                Field::new("stances", Value::Uint(row.stances)),
                Field::new("stances2", Value::Uint(row.stances2)),
                Field::new("stances_not", Value::Uint(row.stances_not)),
                Field::new("stances_not2", Value::Uint(row.stances_not2)),
                Field::new("targets", Value::Int(row.targets)),
                Field::new("target_creature_type", Value::Int(row.target_creature_type)),
                Field::new("requires_spell_focus", Value::Int(row.requires_spell_focus)),
                Field::new("facing_caster_flags", Value::Int(row.facing_caster_flags)),
                Field::new("caster_aura_state", Value::Int(row.caster_aura_state)),
                Field::new("target_aura_state", Value::Int(row.target_aura_state)),
                Field::new(
                    "caster_aura_state_not",
                    Value::Int(row.caster_aura_state_not),
                ),
                Field::new(
                    "target_aura_state_not",
                    Value::Int(row.target_aura_state_not),
                ),
                Field::new("caster_aura_spell", Value::Int(row.caster_aura_spell)),
                Field::new("target_aura_spell", Value::Int(row.target_aura_spell)),
                Field::new(
                    "exclude_caster_aura_spell",
                    Value::Int(row.exclude_caster_aura_spell),
                ),
                Field::new(
                    "exclude_target_aura_spell",
                    Value::Int(row.exclude_target_aura_spell),
                ),
                Field::new("casting_time_index", Value::Int(row.casting_time_index)),
                Field::new("recovery_time", Value::Int(row.recovery_time)),
                Field::new(
                    "category_recovery_time",
                    Value::Int(row.category_recovery_time),
                ),
                Field::new("interrupt_flags", Value::Int(row.interrupt_flags)),
                Field::new(
                    "aura_interrupt_flags",
                    Value::Uint(row.aura_interrupt_flags),
                ),
                Field::new(
                    "channel_interrupt_flags",
                    Value::Uint(row.channel_interrupt_flags),
                ),
                Field::new("proc_flags", Value::Int(row.proc_flags)),
                Field::new("proc_chance", Value::Int(row.proc_chance)),
                Field::new("proc_charges", Value::Int(row.proc_charges)),
                Field::new("max_level", Value::Int(row.max_level)),
                Field::new("base_level", Value::Int(row.base_level)),
                Field::new("spell_level", Value::Int(row.spell_level)),
                Field::new("duration_index", Value::Int(row.duration_index)),
                Field::new("power_type", Value::Uint(row.power_type)),
                Field::new(
                    "mana_cost",
                    Value::Int(i32::from_le_bytes(row.mana_cost.to_le_bytes())),
                ),
                Field::new("mana_cost_per_level", Value::Int(row.mana_cost_per_level)),
                Field::new("mana_per_second", Value::Int(row.mana_per_second)),
                Field::new(
                    "mana_per_second_per_level",
                    Value::Int(row.mana_per_second_per_level),
                ),
                Field::new("range_index", Value::Int(row.range_index)),
                Field::new("speed", Value::float(row.speed)),
                Field::new("modal_next_spell", Value::Int(row.modal_next_spell)),
                Field::new("stack_amount", Value::Int(row.stack_amount)),
                Field::new("equipped_item_class", Value::Int(row.equipped_item_class)),
                Field::new(
                    "equipped_item_sub_class_mask",
                    Value::Int(row.equipped_item_sub_class_mask),
                ),
                Field::new(
                    "equipped_item_inventory_type_mask",
                    Value::Int(row.equipped_item_inventory_type_mask),
                ),
                Field::new("spell_visual", Value::Int(row.spell_visual)),
                Field::new("spell_visual2", Value::Int(row.spell_visual2)),
                Field::new("spell_icon_id", Value::Int(row.spell_icon_id)),
                Field::new("active_icon_id", Value::Int(row.active_icon_id)),
                Field::new("spell_priority", Value::Int(row.spell_priority)),
                Field::new("spell_name", Value::String(row.spell_name.clone())),
                Field::new("rank_text", Value::String(row.rank1.unwrap_or_default())),
                Field::new("mana_cost_percentage", Value::Int(row.mana_cost_percentage)),
                Field::new(
                    "start_recovery_category",
                    Value::Int(row.start_recovery_category),
                ),
                Field::new("start_recovery_time", Value::Int(row.start_recovery_time)),
                Field::new("max_target_level", Value::Int(row.max_target_level)),
                Field::new("spell_family_name", Value::Int(row.spell_family_name)),
                Field::new(
                    "spell_family_flags",
                    Value::Uint64(u64::from_le_bytes(row.spell_family_flags.to_le_bytes())),
                ),
                Field::new("spell_family_flags2", Value::Uint(row.spell_family_flags2)),
                Field::new("max_affected_targets", Value::Int(row.max_affected_targets)),
                Field::new("dmg_class", Value::Int(row.dmg_class)),
                Field::new("prevention_type", Value::Int(row.prevention_type)),
                Field::new("stance_bar_order", Value::Int(row.stance_bar_order)),
                Field::new("min_faction_id", Value::Int(row.min_faction_id)),
                Field::new("min_reputation", Value::Int(row.min_reputation)),
                Field::new("required_aura_vision", Value::Int(row.required_aura_vision)),
                Field::new("area_id", Value::Uint(row.area_id.try_into().unwrap())),
                Field::new("school_mask", Value::Int(row.school_mask)),
                Field::new("rune_cost_id", Value::Uint(row.rune_cost_id)),
                Field::new("spell_missile_id", Value::Uint(row.spell_missile_id)),
                Field::new("power_display_id", Value::Uint(row.power_display_id)),
                Field::new(
                    "spell_description_variable_id",
                    Value::Uint(row.spell_description_variable_id),
                ),
                Field::new("spell_difficulty_id", Value::Uint(row.spell_difficulty_id)),
                Field::new(
                    "is_server_side",
                    Value::Uint(row.is_server_side.try_into().unwrap()),
                ),
                Field::new(
                    "attributes_serverside",
                    Value::Int(row.attributes_serverside),
                ),
            ];

            let arrays = vec![
                Array::new(
                    "reagents",
                    "Reagent",
                    false,
                    ArrayInstances::new(vec![
                        ArrayInstance::new(
                            row.reagent_count1 == 0,
                            vec![
                                ArrayField::new("reagent", "reagent1", Value::Int(row.reagent1)),
                                ArrayField::new(
                                    "amount",
                                    "reagent_count1",
                                    Value::Uint(row.reagent_count1),
                                ),
                            ],
                        ),
                        ArrayInstance::new(
                            row.reagent_count2 == 0,
                            vec![
                                ArrayField::new("reagent", "reagent2", Value::Int(row.reagent2)),
                                ArrayField::new(
                                    "amount",
                                    "reagent_count2",
                                    Value::Uint(row.reagent_count2),
                                ),
                            ],
                        ),
                        ArrayInstance::new(
                            row.reagent_count3 == 0,
                            vec![
                                ArrayField::new("reagent", "reagent3", Value::Int(row.reagent3)),
                                ArrayField::new(
                                    "amount",
                                    "reagent_count3",
                                    Value::Uint(row.reagent_count3),
                                ),
                            ],
                        ),
                        ArrayInstance::new(
                            row.reagent_count4 == 0,
                            vec![
                                ArrayField::new("reagent", "reagent4", Value::Int(row.reagent4)),
                                ArrayField::new(
                                    "amount",
                                    "reagent_count4",
                                    Value::Uint(row.reagent_count4),
                                ),
                            ],
                        ),
                        ArrayInstance::new(
                            row.reagent_count5 == 0,
                            vec![
                                ArrayField::new("reagent", "reagent5", Value::Int(row.reagent5)),
                                ArrayField::new(
                                    "amount",
                                    "reagent_count5",
                                    Value::Uint(row.reagent_count5),
                                ),
                            ],
                        ),
                        ArrayInstance::new(
                            row.reagent_count6 == 0,
                            vec![
                                ArrayField::new("reagent", "reagent6", Value::Int(row.reagent6)),
                                ArrayField::new(
                                    "amount",
                                    "reagent_count6",
                                    Value::Uint(row.reagent_count6),
                                ),
                            ],
                        ),
                        ArrayInstance::new(
                            row.reagent_count7 == 0,
                            vec![
                                ArrayField::new("reagent", "reagent7", Value::Int(row.reagent7)),
                                ArrayField::new(
                                    "amount",
                                    "reagent_count7",
                                    Value::Uint(row.reagent_count7),
                                ),
                            ],
                        ),
                        ArrayInstance::new(
                            row.reagent_count8 == 0,
                            vec![
                                ArrayField::new("reagent", "reagent8", Value::Int(row.reagent8)),
                                ArrayField::new(
                                    "amount",
                                    "reagent_count8",
                                    Value::Uint(row.reagent_count8),
                                ),
                            ],
                        ),
                    ]),
                ),
                Array::new(
                    "effects",
                    "SpellEffects",
                    false,
                    ArrayInstances::new(vec![
                        ArrayInstance::new(
                            row.effect1 == 0,
                            vec![
                                ArrayField::new("effect", "effect1", Value::Int(row.effect1)),
                                ArrayField::new(
                                    "die_sides",
                                    "effect_die_sides1",
                                    Value::Int(row.effect_die_sides1),
                                ),
                                ArrayField::new(
                                    "real_points_per_level",
                                    "effect_real_points_per_level1",
                                    Value::float(row.effect_real_points_per_level1),
                                ),
                                ArrayField::new(
                                    "base_points",
                                    "effect_base_points1",
                                    Value::Int(row.effect_base_points1),
                                ),
                                ArrayField::new(
                                    "mechanics",
                                    "effect_mechanic1",
                                    Value::Int(row.effect_mechanic1),
                                ),
                                ArrayField::new(
                                    "implicit_target_a",
                                    "effect_implicit_target_a1",
                                    Value::Int(row.effect_implicit_target_a1),
                                ),
                                ArrayField::new(
                                    "implicit_target_b",
                                    "effect_implicit_target_b1",
                                    Value::Int(row.effect_implicit_target_b1),
                                ),
                                ArrayField::new(
                                    "radius_index",
                                    "effect_radius_index1",
                                    Value::Int(row.effect_radius_index1),
                                ),
                                ArrayField::new(
                                    "apply_aura_name",
                                    "effect_apply_aura_name1",
                                    Value::WrathAuraMod(
                                        row.effect_apply_aura_name1.try_into().unwrap(),
                                    ),
                                ),
                                ArrayField::new(
                                    "amplitude",
                                    "effect_amplitude1",
                                    Value::Int(row.effect_amplitude1),
                                ),
                                ArrayField::new(
                                    "multiple_value",
                                    "effect_multiple_value1",
                                    Value::float(row.effect_multiple_value1),
                                ),
                                ArrayField::new(
                                    "chain_target",
                                    "effect_chain_target1",
                                    Value::Int(row.effect_chain_target1),
                                ),
                                ArrayField::new(
                                    "item_type",
                                    "effect_item_type1",
                                    Value::Uint(row.effect_item_type1),
                                ),
                                ArrayField::new(
                                    "misc_value",
                                    "effect_misc_value1",
                                    Value::Int(row.effect_misc_value1),
                                ),
                                ArrayField::new(
                                    "misc_value_b",
                                    "effect_misc_value_b1",
                                    Value::Int(row.effect_misc_value_b1),
                                ),
                                ArrayField::new(
                                    "trigger_spell",
                                    "effect_trigger_spell1",
                                    Value::Uint(row.effect_trigger_spell1),
                                ),
                                ArrayField::new(
                                    "points_per_combo_point",
                                    "effect_points_per_combo_point1",
                                    Value::float(row.effect_points_per_combo_point1),
                                ),
                                ArrayField::new(
                                    "spell_class_mask1",
                                    "effect_spell_class_mask1_1",
                                    Value::Uint(row.effect_spell_class_mask1_1),
                                ),
                                ArrayField::new(
                                    "spell_class_mask2",
                                    "effect_spell_class_mask1_2",
                                    Value::Uint(row.effect_spell_class_mask1_2),
                                ),
                                ArrayField::new(
                                    "spell_class_mask3",
                                    "effect_spell_class_mask1_3",
                                    Value::Uint(row.effect_spell_class_mask1_3),
                                ),
                                ArrayField::new(
                                    "damage_multiplier",
                                    "dmg_multiplier1",
                                    Value::float(row.dmg_multiplier1),
                                ),
                                ArrayField::new(
                                    "bonus_coefficient",
                                    "effect_bonus_coefficient1",
                                    Value::float(row.effect_bonus_coefficient1),
                                ),
                            ],
                        ),
                        ArrayInstance::new(
                            row.effect2 == 0,
                            vec![
                                ArrayField::new("effect", "effect2", Value::Int(row.effect2)),
                                ArrayField::new(
                                    "die_sides",
                                    "effect_die_sides2",
                                    Value::Int(row.effect_die_sides2),
                                ),
                                ArrayField::new(
                                    "real_points_per_level",
                                    "effect_real_points_per_level2",
                                    Value::float(row.effect_real_points_per_level2),
                                ),
                                ArrayField::new(
                                    "base_points",
                                    "effect_base_points2",
                                    Value::Int(row.effect_base_points2),
                                ),
                                ArrayField::new(
                                    "mechanics",
                                    "effect_mechanic2",
                                    Value::Int(row.effect_mechanic2),
                                ),
                                ArrayField::new(
                                    "implicit_target_a",
                                    "effect_implicit_target_a2",
                                    Value::Int(row.effect_implicit_target_a2),
                                ),
                                ArrayField::new(
                                    "implicit_target_b",
                                    "effect_implicit_target_b2",
                                    Value::Int(row.effect_implicit_target_b2),
                                ),
                                ArrayField::new(
                                    "radius_index",
                                    "effect_radius_index2",
                                    Value::Int(row.effect_radius_index2),
                                ),
                                ArrayField::new(
                                    "apply_aura_name",
                                    "effect_apply_aura_name2",
                                    Value::WrathAuraMod(
                                        row.effect_apply_aura_name2.try_into().unwrap(),
                                    ),
                                ),
                                ArrayField::new(
                                    "amplitude",
                                    "effect_amplitude2",
                                    Value::Int(row.effect_amplitude2),
                                ),
                                ArrayField::new(
                                    "multiple_value",
                                    "effect_multiple_value2",
                                    Value::float(row.effect_multiple_value2),
                                ),
                                ArrayField::new(
                                    "chain_target",
                                    "effect_chain_target2",
                                    Value::Int(row.effect_chain_target2),
                                ),
                                ArrayField::new(
                                    "item_type",
                                    "effect_item_type2",
                                    Value::Uint(row.effect_item_type2),
                                ),
                                ArrayField::new(
                                    "misc_value",
                                    "effect_misc_value2",
                                    Value::Int(row.effect_misc_value2),
                                ),
                                ArrayField::new(
                                    "misc_value_b",
                                    "effect_misc_value_b2",
                                    Value::Int(row.effect_misc_value_b2),
                                ),
                                ArrayField::new(
                                    "trigger_spell",
                                    "effect_trigger_spell2",
                                    Value::Uint(row.effect_trigger_spell2),
                                ),
                                ArrayField::new(
                                    "points_per_combo_point",
                                    "effect_points_per_combo_point2",
                                    Value::float(row.effect_points_per_combo_point2),
                                ),
                                ArrayField::new(
                                    "spell_class_mask1",
                                    "effect_spell_class_mask2_1",
                                    Value::Uint(row.effect_spell_class_mask2_1),
                                ),
                                ArrayField::new(
                                    "spell_class_mask2",
                                    "effect_spell_class_mask2_2",
                                    Value::Uint(row.effect_spell_class_mask2_2),
                                ),
                                ArrayField::new(
                                    "spell_class_mask3",
                                    "effect_spell_class_mask2_3",
                                    Value::Uint(row.effect_spell_class_mask2_3),
                                ),
                                ArrayField::new(
                                    "damage_multiplier",
                                    "dmg_multiplier2",
                                    Value::float(row.dmg_multiplier2),
                                ),
                                ArrayField::new(
                                    "bonus_coefficient",
                                    "effect_bonus_coefficient2",
                                    Value::float(row.effect_bonus_coefficient2),
                                ),
                            ],
                        ),
                        ArrayInstance::new(
                            row.effect3 == 0,
                            vec![
                                ArrayField::new("effect", "effect3", Value::Int(row.effect3)),
                                ArrayField::new(
                                    "die_sides",
                                    "effect_die_sides3",
                                    Value::Int(row.effect_die_sides3),
                                ),
                                ArrayField::new(
                                    "real_points_per_level",
                                    "effect_real_points_per_level3",
                                    Value::float(row.effect_real_points_per_level3),
                                ),
                                ArrayField::new(
                                    "base_points",
                                    "effect_base_points3",
                                    Value::Int(row.effect_base_points3),
                                ),
                                ArrayField::new(
                                    "mechanics",
                                    "effect_mechanic3",
                                    Value::Int(row.effect_mechanic3),
                                ),
                                ArrayField::new(
                                    "implicit_target_a",
                                    "effect_implicit_target_a3",
                                    Value::Int(row.effect_implicit_target_a3),
                                ),
                                ArrayField::new(
                                    "implicit_target_b",
                                    "effect_implicit_target_b3",
                                    Value::Int(row.effect_implicit_target_b3),
                                ),
                                ArrayField::new(
                                    "radius_index",
                                    "effect_radius_index3",
                                    Value::Int(row.effect_radius_index3),
                                ),
                                ArrayField::new(
                                    "apply_aura_name",
                                    "effect_apply_aura_name3",
                                    Value::WrathAuraMod(
                                        row.effect_apply_aura_name3.try_into().unwrap(),
                                    ),
                                ),
                                ArrayField::new(
                                    "amplitude",
                                    "effect_amplitude3",
                                    Value::Int(row.effect_amplitude3),
                                ),
                                ArrayField::new(
                                    "multiple_value",
                                    "effect_multiple_value3",
                                    Value::float(row.effect_multiple_value3),
                                ),
                                ArrayField::new(
                                    "chain_target",
                                    "effect_chain_target3",
                                    Value::Int(row.effect_chain_target3),
                                ),
                                ArrayField::new(
                                    "item_type",
                                    "effect_item_type3",
                                    Value::Uint(row.effect_item_type3),
                                ),
                                ArrayField::new(
                                    "misc_value",
                                    "effect_misc_value3",
                                    Value::Int(row.effect_misc_value3),
                                ),
                                ArrayField::new(
                                    "misc_value_b",
                                    "effect_misc_value_b3",
                                    Value::Int(row.effect_misc_value_b3),
                                ),
                                ArrayField::new(
                                    "trigger_spell",
                                    "effect_trigger_spell3",
                                    Value::Uint(row.effect_trigger_spell3),
                                ),
                                ArrayField::new(
                                    "points_per_combo_point",
                                    "effect_points_per_combo_point3",
                                    Value::float(row.effect_points_per_combo_point3),
                                ),
                                ArrayField::new(
                                    "spell_class_mask1",
                                    "effect_spell_class_mask3_1",
                                    Value::Uint(row.effect_spell_class_mask3_1),
                                ),
                                ArrayField::new(
                                    "spell_class_mask2",
                                    "effect_spell_class_mask3_2",
                                    Value::Uint(row.effect_spell_class_mask3_2),
                                ),
                                ArrayField::new(
                                    "spell_class_mask3",
                                    "effect_spell_class_mask3_3",
                                    Value::Uint(row.effect_spell_class_mask3_3),
                                ),
                                ArrayField::new(
                                    "damage_multiplier",
                                    "dmg_multiplier3",
                                    Value::float(row.dmg_multiplier3),
                                ),
                                ArrayField::new(
                                    "bonus_coefficient",
                                    "effect_bonus_coefficient3",
                                    Value::float(row.effect_bonus_coefficient3),
                                ),
                            ],
                        ),
                    ]),
                ),
                Array::new(
                    "totems",
                    "Totem",
                    false,
                    ArrayInstances::new(vec![
                        ArrayInstance::default_values(vec![ArrayField::new(
                            "totem",
                            "totem1",
                            Value::Int(row.totem1),
                        )]),
                        ArrayInstance::default_values(vec![ArrayField::new(
                            "totem",
                            "totem2",
                            Value::Int(row.totem2),
                        )]),
                    ]),
                ),
                Array::new(
                    "totem_categories",
                    "TotemCategory",
                    false,
                    ArrayInstances::new(vec![
                        ArrayInstance::default_values(vec![ArrayField::new(
                            "category",
                            "totem_category1",
                            Value::Int(row.totem_category1),
                        )]),
                        ArrayInstance::default_values(vec![ArrayField::new(
                            "category",
                            "totem_category2",
                            Value::Int(row.totem_category2),
                        )]),
                    ]),
                ),
            ];

            GenericThing {
                entry: row.id,
                extra_flags: 0,
                name: row.spell_name,
                fields,
                arrays,
            }
        })
        .collect();

    let optimizations = Optimizations::new(&spells, get_fields(&spells));
    (spells, optimizations)
}
