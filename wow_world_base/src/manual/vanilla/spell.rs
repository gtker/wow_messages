#![allow(clippy::too_many_arguments)]
use crate::vanilla::{
    Attributes,
    AttributesEx1,
    AttributesEx2,
    AttributesEx3,
    AttributesEx4,
    AuraMod,
};

/// Struct optimized for containing the original spells most efficiently.
///
/// This type is not supposed to be used by external users of the library for creating custom items.
/// It's only supposed to be used in conjunction with the `wow_spells` crate.
///
/// [`Hash`](core::hash::Hash), [`Ord`], and [`Eq`] all use only the item id without considering other fields.
#[derive(Debug, Copy, Clone, Default)]
pub struct Spell {
    entry: u16,
    school: i8,
    category: i16,
    cast_ui: i8,
    dispel: i8,
    mechanic: i8,
    attributes: Attributes,
    attributes_ex: AttributesEx1,
    attributes_ex2: AttributesEx2,
    attributes_ex3: AttributesEx3,
    stances: u32,
    stances_not: u32,
    targets: i32,
    target_creature_type: i16,
    requires_spell_focus: i16,
    casting_time_index: i16,
    recovery_time: i32,
    category_recovery_time: i32,
    interrupt_flags: i8,
    aura_interrupt_flags: i32,
    channel_interrupt_flags: i32,
    proc_flags: i32,
    proc_chance: i8,
    proc_charges: i8,
    max_level: i16,
    base_level: i8,
    spell_level: i8,
    duration_index: i16,
    power_type: u32,
    mana_cost: i32,
    range_index: i16,
    speed: f32,
    stack_amount: i16,
    equipped_item_class: i8,
    equipped_item_sub_class_mask: i32,
    equipped_item_inventory_type_mask: i32,
    spell_visual: i16,
    spell_icon_id: i16,
    active_icon_id: i16,
    spell_priority: i8,
    spell_name: &'static str,
    rank_text: &'static str,
    mana_cost_percentage: i16,
    start_recovery_category: i16,
    start_recovery_time: i16,
    spell_family_name: i8,
    spell_family_flags: i64,
    dmg_class: i8,
    prevention_type: i8,
    stance_bar_order: i8,
    reagents: &'static [Reagent],
    effects: &'static [SpellEffects],
    totems: &'static [Totem],
}

impl Spell {
    #[doc(hidden)]
    pub const fn new(
        entry: u16,
        school: i8,
        category: i16,
        cast_ui: i8,
        dispel: i8,
        mechanic: i8,
        attributes: Attributes,
        attributes_ex: AttributesEx1,
        attributes_ex2: AttributesEx2,
        attributes_ex3: AttributesEx3,
        stances: u32,
        stances_not: u32,
        targets: i32,
        target_creature_type: i16,
        requires_spell_focus: i16,
        casting_time_index: i16,
        recovery_time: i32,
        category_recovery_time: i32,
        interrupt_flags: i8,
        aura_interrupt_flags: i32,
        channel_interrupt_flags: i32,
        proc_flags: i32,
        proc_chance: i8,
        proc_charges: i8,
        max_level: i16,
        base_level: i8,
        spell_level: i8,
        duration_index: i16,
        power_type: u32,
        mana_cost: i32,
        range_index: i16,
        speed: f32,
        stack_amount: i16,
        equipped_item_class: i8,
        equipped_item_sub_class_mask: i32,
        equipped_item_inventory_type_mask: i32,
        spell_visual: i16,
        spell_icon_id: i16,
        active_icon_id: i16,
        spell_priority: i8,
        spell_name: &'static str,
        rank_text: &'static str,
        mana_cost_percentage: i16,
        start_recovery_category: i16,
        start_recovery_time: i16,
        spell_family_name: i8,
        spell_family_flags: i64,
        dmg_class: i8,
        prevention_type: i8,
        stance_bar_order: i8,
        reagents: &'static [Reagent],
        effects: &'static [SpellEffects],
        totems: &'static [Totem],
    ) -> Self {
        Self {
            entry,
            school,
            category,
            cast_ui,
            dispel,
            mechanic,
            attributes,
            attributes_ex,
            attributes_ex2,
            attributes_ex3,
            stances,
            stances_not,
            targets,
            target_creature_type,
            requires_spell_focus,
            casting_time_index,
            recovery_time,
            category_recovery_time,
            interrupt_flags,
            aura_interrupt_flags,
            channel_interrupt_flags,
            proc_flags,
            proc_chance,
            proc_charges,
            max_level,
            base_level,
            spell_level,
            duration_index,
            power_type,
            mana_cost,
            range_index,
            speed,
            stack_amount,
            equipped_item_class,
            equipped_item_sub_class_mask,
            equipped_item_inventory_type_mask,
            spell_visual,
            spell_icon_id,
            active_icon_id,
            spell_priority,
            spell_name,
            rank_text,
            mana_cost_percentage,
            start_recovery_category,
            start_recovery_time,
            spell_family_name,
            spell_family_flags,
            dmg_class,
            prevention_type,
            stance_bar_order,
            reagents,
            effects,
            totems,
        }
    }
    pub const fn entry(&self) -> u32 {
        self.entry as u32
    }

    pub const fn school(&self) -> i32 {
        self.school as i32
    }

    pub const fn category(&self) -> i32 {
        self.category as i32
    }

    pub const fn cast_ui(&self) -> i32 {
        self.cast_ui as i32
    }

    pub const fn dispel(&self) -> i32 {
        self.dispel as i32
    }

    pub const fn mechanic(&self) -> i32 {
        self.mechanic as i32
    }

    pub const fn attributes(&self) -> Attributes {
        self.attributes
    }

    pub const fn attributes_ex(&self) -> AttributesEx1 {
        self.attributes_ex
    }

    pub const fn attributes_ex2(&self) -> AttributesEx2 {
        self.attributes_ex2
    }

    pub const fn attributes_ex3(&self) -> AttributesEx3 {
        self.attributes_ex3
    }

    /// Returns `0` except for specific item entries.
    pub const fn attributes_ex4(&self) -> AttributesEx4 {
        AttributesEx4::new(match self.entry {
            408 | 1943 | 2098 | 6760 | 6761 | 6762 | 8623 | 8624 | 8639 | 8640 | 8643 | 8647 | 8649 | 8650 | 11197 | 11198 | 11273 | 11274 | 11275 | 11299 | 11300 | 31016 => 8,
            1495 | 5308 | 6572 | 6574 | 7379 | 7384 | 7887 | 11584 | 11585 | 11600 | 11601 | 14251 | 14269 | 14270 | 14271 | 19306 | 20658 | 20660 | 20661 | 20662 | 20909 | 20910 | 24239 | 24274 | 24275 | 25288 => 512,
            1714 | 11719 => 2048,
            5171 | 6774 => 16,
            11829 | 11990 | 12468 | 16005 | 16102 | 16419 | 18399 | 19474 | 20296 | 20754 | 20794 | 20813 | 20827 | 22275 => 32,
            14179 | 28814 => 2,
            19752 | 28375 => 256,
            20424 => 128,
            26013 => 4,
            _ => 0,
            })
            }

            pub const fn stances(&self) -> u32 {
                self.stances
            }

            pub const fn stances_not(&self) -> u32 {
                self.stances_not
            }

            pub const fn targets(&self) -> i32 {
                self.targets
            }

            pub const fn target_creature_type(&self) -> i32 {
                self.target_creature_type as i32
            }

            pub const fn requires_spell_focus(&self) -> i32 {
                self.requires_spell_focus as i32
            }

            /// Returns `0` except for specific item entries.
            pub const fn caster_aura_state(&self) -> i32 {
                match self.entry {
                    76 | 1495 | 6186 | 6568 | 6572 | 6574 | 7379 | 10374 | 11600 | 11601 | 12170 | 14251 | 14269 | 14270 | 14271 | 19130 | 25288 => 1,
                    17170 => 2,
                    19306 | 20909 | 20910 => 7,
                    20271 | 27731 => 5,
                    26635 => 3,
                    _ => 0,
                }
            }

            /// Returns `0` except for specific item entries.
            pub const fn target_aura_state(&self) -> i32 {
                match self.entry {
                    5308 | 6174 | 6175 | 7160 | 7938 | 16495 | 20539 | 20647 | 20658 | 20660 | 20661 | 20662 | 24239 | 24274 | 24275 | 31255 => 2,
                    _ => 0,
                }
            }

            pub const fn casting_time_index(&self) -> i32 {
                self.casting_time_index as i32
            }

            pub const fn recovery_time(&self) -> i32 {
                self.recovery_time
            }

            pub const fn category_recovery_time(&self) -> i32 {
                self.category_recovery_time
            }

            pub const fn interrupt_flags(&self) -> i32 {
                self.interrupt_flags as i32
            }

            pub const fn aura_interrupt_flags(&self) -> i32 {
                self.aura_interrupt_flags
            }

            pub const fn channel_interrupt_flags(&self) -> i32 {
                self.channel_interrupt_flags
            }

            pub const fn proc_flags(&self) -> i32 {
                self.proc_flags
            }

            pub const fn proc_chance(&self) -> i32 {
                self.proc_chance as i32
            }

            pub const fn proc_charges(&self) -> i32 {
                self.proc_charges as i32
            }

            pub const fn max_level(&self) -> i32 {
                self.max_level as i32
            }

            pub const fn base_level(&self) -> i32 {
                self.base_level as i32
            }

            pub const fn spell_level(&self) -> i32 {
                self.spell_level as i32
            }

            pub const fn duration_index(&self) -> i32 {
                self.duration_index as i32
            }

            pub const fn power_type(&self) -> u32 {
                self.power_type
            }

            pub const fn mana_cost(&self) -> i32 {
                self.mana_cost
            }

            /// Returns `0` except for specific item entries.
            pub const fn mana_cost_per_level(&self) -> i32 {
                match self.entry {
                    271 => 1,
                    4513 | 6742 | 6950 | 7290 | 9373 | 14517 | 14518 | 16170 | 16430 | 17281 | 17633 => 2,
                    5108 | 8362 | 9616 | 11444 | 11974 | 11980 | 12098 | 12493 | 12544 | 12550 | 12741 | 13585 | 15507 | 15784 | 15800 | 15970 | 17139 | 17173 | 17238 | 17620 | 18100 | 18267 | 18557 | 20663 | 20743 | 22168 | 22187 | 22371 | 24618 => 5,
                    5605 | 18545 => 3,
                    7154 => 9,
                    8364 | 12040 | 15783 | 21096 | 22417 | 22822 | 22823 => 8,
                    9591 | 9654 | 9771 | 11021 | 12096 | 13140 | 13326 | 13864 | 15288 | 15653 | 17150 | 19463 | 20657 | 20821 | 25052 => 4,
                    11990 | 20754 => 15,
                    13787 => 20,
                    17243 | 17682 => 6,
                    20798 => 7,
                    _ => 0,
                }
            }

            /// Returns `0` except for specific item entries.
            pub const fn mana_per_second(&self) -> i32 {
                match self.entry {
                    424 | 963 => 2,
                    461 | 3698 => 10,
                    755 => 5,
                    1116 | 3486 => 4,
                    3699 => 17,
                    3700 => 24,
                    10260 => 25,
                    11693 => 33,
                    11694 => 42,
                    11695 => 52,
                    16569 => 75,
                    _ => 0,
                }
            }

            /// Always returns `0`.
            pub const fn mana_per_second_per_level(&self) -> i32 {
                0
            }

            pub const fn range_index(&self) -> i32 {
                self.range_index as i32
            }

            pub const fn speed(&self) -> f32 {
                self.speed
            }

            /// Returns `0` except for specific item entries.
            pub const fn modal_next_spell(&self) -> i32 {
                match self.entry {
                    59 | 60 | 61 => 59,
                    1978 | 2643 | 3034 | 3043 | 3044 | 3674 | 5116 | 13549 | 13550 | 13551 | 13552 | 13553 | 13554 | 13555 | 14274 | 14275 | 14276 | 14277 | 14279 | 14280 | 14281 | 14282 | 14283 | 14284 | 14285 | 14286 | 14287 | 14288 | 14289 | 14290 | 14296 | 15629 | 15630 | 15631 | 15632 | 17171 | 17174 | 18545 | 18649 | 19434 | 19501 | 20736 | 20900 | 20901 | 20902 | 20903 | 20904 | 22914 | 25294 | 25295 | 27632 | 27634 => 75,
                    23675 | 23676 => 23675,
                    _ => 0,
                }
            }

            pub const fn stack_amount(&self) -> i32 {
                self.stack_amount as i32
            }

            pub const fn equipped_item_class(&self) -> i32 {
                self.equipped_item_class as i32
            }

            pub const fn equipped_item_sub_class_mask(&self) -> i32 {
                self.equipped_item_sub_class_mask
            }

            pub const fn equipped_item_inventory_type_mask(&self) -> i32 {
                self.equipped_item_inventory_type_mask
            }

            pub const fn spell_visual(&self) -> i32 {
                self.spell_visual as i32
            }

            pub const fn spell_icon_id(&self) -> i32 {
                self.spell_icon_id as i32
            }

            pub const fn active_icon_id(&self) -> i32 {
                self.active_icon_id as i32
            }

            pub const fn spell_priority(&self) -> i32 {
                self.spell_priority as i32
            }

            pub const fn spell_name(&self) -> &'static str {
                self.spell_name
            }

            pub const fn rank_text(&self) -> &'static str {
                self.rank_text
            }

            pub const fn mana_cost_percentage(&self) -> i32 {
                self.mana_cost_percentage as i32
            }

            pub const fn start_recovery_category(&self) -> i32 {
                self.start_recovery_category as i32
            }

            pub const fn start_recovery_time(&self) -> i32 {
                self.start_recovery_time as i32
            }

            /// Returns `0` except for specific item entries.
            pub const fn max_target_level(&self) -> i32 {
                match self.entry {
                    453 | 2908 | 26195 => 40,
                    8192 | 8955 => 55,
                    9439 | 9976 | 9991 | 9999 | 10723 | 12332 | 17085 | 21544 | 21565 => 60,
                    9736 => 100,
                    9773 | 9779 => 255,
                    9901 | 10953 | 26198 => 70,
                    10267 | 26197 => 50,
                    10268 => 80,
                    26258 => 30,
                    26259 => 20,
                    _ => 0,
                }
            }

            pub const fn spell_family_name(&self) -> i32 {
                self.spell_family_name as i32
            }

            pub const fn spell_family_flags(&self) -> i64 {
                self.spell_family_flags
            }

            /// Returns `0` except for specific item entries.
            pub const fn max_affected_targets(&self) -> i32 {
                match self.entry {
                    802 | 804 | 15398 | 23138 | 23173 | 23603 | 24019 | 24150 | 24330 | 24781 | 24933 | 25024 | 25026 | 25027 | 25030 | 25031 | 25032 | 26080 | 26219 | 26220 | 26221 | 26237 | 26398 | 26476 | 26479 | 26524 | 28236 | 28415 | 28416 | 28417 | 28455 | 28560 => 1,
                    1680 | 6343 | 8198 | 8204 | 8205 | 10888 | 11580 | 11581 | 13532 | 23023 | 24083 => 4,
                    5246 | 5484 | 10890 | 13704 | 17928 | 19482 | 20549 | 22884 | 24054 | 25260 | 25815 | 26042 | 27610 | 28318 | 28405 | 29232 => 5,
                    8122 | 10258 | 28542 => 2,
                    8124 | 11790 | 27885 | 28781 | 29874 => 3,
                    13910 | 22012 | 24415 | 26180 | 28137 | 28796 | 29213 => 10,
                    25676 | 25754 => 6,
                    26052 | 28293 => 15,
                    26457 | 26559 => 12,
                    _ => 0,
                }
            }

            pub const fn dmg_class(&self) -> i32 {
                self.dmg_class as i32
            }

            pub const fn prevention_type(&self) -> i32 {
                self.prevention_type as i32
            }

            pub const fn stance_bar_order(&self) -> i32 {
                self.stance_bar_order as i32
            }

            /// Returns `0` except for specific item entries.
            pub const fn min_faction_id(&self) -> i32 {
                match self.entry {
                    6994 => 369,
                    _ => 0,
                }
            }

            /// Returns `0` except for specific item entries.
            pub const fn min_reputation(&self) -> i32 {
                match self.entry {
                    6994 => 4,
                    _ => 0,
                }
            }

            /// Returns `0` except for specific item entries.
            pub const fn required_aura_vision(&self) -> i32 {
                match self.entry {
                    26869 => 1,
                    _ => 0,
                }
            }

            /// Always returns `0`.
            pub const fn is_server_side(&self) -> i32 {
                0
            }

            /// Returns `0` except for specific item entries.
            pub const fn attributes_serverside(&self) -> i32 {
                match self.entry {
                    4044 | 4133 | 11816 | 18115 | 21789 | 27791 | 28330 => 4,
                    _ => 0,
                }
            }

            pub const fn reagents_array(&self) -> [Reagent; 8] {
                const D: Reagent=Reagent{
                    reagent:0,
                    amount:0,
                };
                let l = self.reagents.len();
                [
                    if l >= 1 { self.reagents()[0] } else { D },
                    if l >= 2 { self.reagents()[1] } else { D },
                    if l >= 3 { self.reagents()[2] } else { D },
                    if l >= 4 { self.reagents()[3] } else { D },
                    if l >= 5 { self.reagents()[4] } else { D },
                    if l >= 6 { self.reagents()[5] } else { D },
                    if l >= 7 { self.reagents()[6] } else { D },
                    if l >= 8 { self.reagents()[7] } else { D },
                ]
            }

            pub const fn reagents(&self) -> &[Reagent] {
                self.reagents
            }

            pub const fn effects_array(&self) -> [SpellEffects; 3] {
                const D: SpellEffects=SpellEffects{
                    effect:0,
                    die_sides:0,
                    base_dice:0,
                    dice_per_level:0.0,
                    real_points_per_level:0.0,
                    base_points:0,
                    mechanic:0,
                    implicit_target_a:0,
                    implicit_target_b:0,
                    radius_index:0,
                    apply_aura_name:AuraMod::None,
                    amplitude:0,
                    multiple_value:0.0,
                    chain_target:0,
                    item_type:0,
                    misc_value:0,
                    trigger_spell:0,
                    effect_points_per_combo_point:0.0,
                    damage_multiplier:0.0,
                };
                let l = self.effects.len();
                [
                    if l >= 1 { self.effects()[0] } else { D },
                    if l >= 2 { self.effects()[1] } else { D },
                    if l >= 3 { self.effects()[2] } else { D },
                ]
            }

            pub const fn effects(&self) -> &[SpellEffects] {
                self.effects
            }

            pub const fn totems_array(&self) -> [Totem; 2] {
                const D: Totem=Totem{
                    totem:0,
                };
                let l = self.totems.len();
                [
                    if l >= 1 { self.totems()[0] } else { D },
                    if l >= 2 { self.totems()[1] } else { D },
                ]
            }

            pub const fn totems(&self) -> &[Totem] {
                self.totems
            }

        }

        impl PartialOrd for Spell {
            fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for Spell {
            fn cmp(&self, other: &Self) -> core::cmp::Ordering {
                self.entry.cmp(&other.entry)
            }
        }

        impl PartialEq<Self> for Spell {
            fn eq(&self, other: &Self) -> bool {
                self.entry.eq(&other.entry)
            }
        }

        impl Eq for Spell {}
        impl core::hash::Hash for Spell {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.entry.hash(state)
            }
        }

        #[derive(Debug, Copy, Clone)]
        pub struct Reagent {
            pub reagent: i32,
            pub amount: i32,
        }

        impl Reagent {
            pub const fn new(
                reagent: i32,
                amount: i32,
            ) -> Self {
                Self {
                    reagent,
                    amount,
                }
            }
        }
        #[derive(Debug, Copy, Clone)]
        pub struct SpellEffects {
            pub effect: i32,
            pub die_sides: i32,
            pub base_dice: i32,
            pub dice_per_level: f32,
            pub real_points_per_level: f32,
            pub base_points: i32,
            pub mechanic: i32,
            pub implicit_target_a: i32,
            pub implicit_target_b: i32,
            pub radius_index: i32,
            pub apply_aura_name: AuraMod,
            pub amplitude: i32,
            pub multiple_value: f32,
            pub chain_target: i32,
            pub item_type: u32,
            pub misc_value: i32,
            pub trigger_spell: i32,
            pub effect_points_per_combo_point: f32,
            pub damage_multiplier: f32,
        }

        impl SpellEffects {
            pub const fn new(
                effect: i32,
                die_sides: i32,
                base_dice: i32,
                dice_per_level: f32,
                real_points_per_level: f32,
                base_points: i32,
                mechanic: i32,
                implicit_target_a: i32,
                implicit_target_b: i32,
                radius_index: i32,
                apply_aura_name: AuraMod,
                amplitude: i32,
                multiple_value: f32,
                chain_target: i32,
                item_type: u32,
                misc_value: i32,
                trigger_spell: i32,
                effect_points_per_combo_point: f32,
                damage_multiplier: f32,
            ) -> Self {
                Self {
                    effect,
                    die_sides,
                    base_dice,
                    dice_per_level,
                    real_points_per_level,
                    base_points,
                    mechanic,
                    implicit_target_a,
                    implicit_target_b,
                    radius_index,
                    apply_aura_name,
                    amplitude,
                    multiple_value,
                    chain_target,
                    item_type,
                    misc_value,
                    trigger_spell,
                    effect_points_per_combo_point,
                    damage_multiplier,
                }
            }
        }
        #[derive(Debug, Copy, Clone)]
        pub struct Totem {
            pub totem: i32,
        }

        impl Totem {
            pub const fn new(
                totem: i32,
            ) -> Self {
                Self {
                    totem,
                }
            }
        }
