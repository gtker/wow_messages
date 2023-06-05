#![allow(clippy::too_many_arguments)]
use crate::tbc::{
    AuraMod,
};

/// Struct optimized for containing the original spells most efficiently.
///
/// This type is not supposed to be used by external users of the library for creating custom spells.
/// It's only supposed to be used in conjunction with the `wow_spells` crate.
///
/// [`Hash`](core::hash::Hash), [`Ord`], and [`Eq`] all use only the spell id without considering other fields.
#[derive(Debug, Copy, Clone, Default)]
pub struct Spell {
    entry: u16,
    category: i16,
    cast_ui: i8,
    dispel: i8,
    mechanic: i8,
    attributes: u32,
    attributes_ex: u32,
    attributes_ex2: u32,
    attributes_ex3: u32,
    attributes_ex4: u32,
    attributes_ex5: u32,
    attributes_ex6: u32,
    stances: u32,
    stances_not: u32,
    targets: i32,
    target_creature_type: i16,
    requires_spell_focus: i16,
    facing_caster_flags: i8,
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
    base_level: i16,
    spell_level: i16,
    duration_index: i16,
    power_type: u32,
    mana_cost: i32,
    mana_cost_per_level: i8,
    range_index: i16,
    speed: f32,
    stack_amount: i32,
    equipped_item_class: i8,
    equipped_item_sub_class_mask: i32,
    equipped_item_inventory_type_mask: i32,
    spell_visual: i16,
    spell_icon_id: i16,
    active_icon_id: i16,
    spell_priority: i8,
    spell_name: &'static str,
    rank_text: &'static str,
    mana_cost_percentage: i8,
    start_recovery_category: i16,
    start_recovery_time: i16,
    spell_family_name: i8,
    spell_family_flags: i64,
    max_affected_targets: i8,
    dmg_class: i8,
    prevention_type: i8,
    stance_bar_order: i8,
    area_id: i16,
    school_mask: i8,
    is_server_side: i8,
    attributes_serverside: i8,
    reagents: &'static [Reagent],
    effects: &'static [SpellEffects],
    totems: &'static [Totem],
    totem_categories: &'static [TotemCategory],
}

impl Spell {
    #[doc(hidden)]
    pub const fn new(
        entry: u16,
        category: i16,
        cast_ui: i8,
        dispel: i8,
        mechanic: i8,
        attributes: u32,
        attributes_ex: u32,
        attributes_ex2: u32,
        attributes_ex3: u32,
        attributes_ex4: u32,
        attributes_ex5: u32,
        attributes_ex6: u32,
        stances: u32,
        stances_not: u32,
        targets: i32,
        target_creature_type: i16,
        requires_spell_focus: i16,
        facing_caster_flags: i8,
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
        base_level: i16,
        spell_level: i16,
        duration_index: i16,
        power_type: u32,
        mana_cost: i32,
        mana_cost_per_level: i8,
        range_index: i16,
        speed: f32,
        stack_amount: i32,
        equipped_item_class: i8,
        equipped_item_sub_class_mask: i32,
        equipped_item_inventory_type_mask: i32,
        spell_visual: i16,
        spell_icon_id: i16,
        active_icon_id: i16,
        spell_priority: i8,
        spell_name: &'static str,
        rank_text: &'static str,
        mana_cost_percentage: i8,
        start_recovery_category: i16,
        start_recovery_time: i16,
        spell_family_name: i8,
        spell_family_flags: i64,
        max_affected_targets: i8,
        dmg_class: i8,
        prevention_type: i8,
        stance_bar_order: i8,
        area_id: i16,
        school_mask: i8,
        is_server_side: i8,
        attributes_serverside: i8,
        reagents: &'static [Reagent],
        effects: &'static [SpellEffects],
        totems: &'static [Totem],
        totem_categories: &'static [TotemCategory],
    ) -> Self {
        Self {
            entry,
            category,
            cast_ui,
            dispel,
            mechanic,
            attributes,
            attributes_ex,
            attributes_ex2,
            attributes_ex3,
            attributes_ex4,
            attributes_ex5,
            attributes_ex6,
            stances,
            stances_not,
            targets,
            target_creature_type,
            requires_spell_focus,
            facing_caster_flags,
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
            mana_cost_per_level,
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
            max_affected_targets,
            dmg_class,
            prevention_type,
            stance_bar_order,
            area_id,
            school_mask,
            is_server_side,
            attributes_serverside,
            reagents,
            effects,
            totems,
            totem_categories,
        }
    }
    pub const fn entry(&self) -> u32 {
        self.entry as u32
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

    pub const fn attributes(&self) -> u32 {
        self.attributes
    }

    pub const fn attributes_ex(&self) -> u32 {
        self.attributes_ex
    }

    pub const fn attributes_ex2(&self) -> u32 {
        self.attributes_ex2
    }

    pub const fn attributes_ex3(&self) -> u32 {
        self.attributes_ex3
    }

    pub const fn attributes_ex4(&self) -> u32 {
        self.attributes_ex4
    }

    pub const fn attributes_ex5(&self) -> u32 {
        self.attributes_ex5
    }

    pub const fn attributes_ex6(&self) -> u32 {
        self.attributes_ex6
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

    pub const fn facing_caster_flags(&self) -> i32 {
        self.facing_caster_flags as i32
    }

    /// Returns `0` except for specific item entries.
    pub const fn caster_aura_state(&self) -> i32 {
        match self.entry {
            1495 | 6572 | 6574 | 7379 | 11600 | 11601 | 12170 | 14251 | 14269 | 14270 | 14271 | 19130 | 25269 | 25288 | 30357 | 34097 | 36916 | 40019 | 40392 | 46873 => 1,
            17170 => 2,
            19306 | 20909 | 20910 | 27067 | 31819 | 34243 => 7,
            20271 | 41467 => 5,
            26635 => 3,
            29801 | 30030 | 30033 => 11,
            31850 | 31851 | 31852 | 31853 | 31854 => 13,
            34026 | 34428 => 10,
            _ => 0,
        }
    }

    /// Returns `0` except for specific item entries.
    pub const fn target_aura_state(&self) -> i32 {
        match self.entry {
            5308 | 7160 | 7938 | 16495 | 20539 | 20647 | 20658 | 20660 | 20661 | 20662 | 24239 | 24274 | 24275 | 25234 | 25236 | 27180 | 29364 | 29909 | 31255 | 32772 | 34104 | 35771 | 36734 | 36779 | 37251 | 37255 | 37259 | 38895 | 38959 | 42325 => 2,
            14083 => 13,
            17962 | 18930 | 18931 | 18932 | 27266 | 30912 => 14,
            18562 => 15,
            32645 | 32684 => 16,
            40019 | 46873 => 3,
            _ => 0,
        }
    }

    /// Returns `0` except for specific item entries.
    pub const fn caster_aura_state_not(&self) -> i32 {
        match self.entry {
            66 | 1784 | 1785 | 1786 | 1787 | 1856 | 1857 | 2425 | 3680 | 4079 | 4986 | 5215 | 5543 | 5858 | 5916 | 6408 | 6538 | 6634 | 6783 | 6920 | 7870 | 8149 | 8152 | 8218 | 9093 | 9587 | 9738 | 9913 | 11327 | 11329 | 11392 | 12845 | 16122 | 16380 | 20540 | 20580 | 20672 | 22766 | 24235 | 24450 | 24452 | 24453 | 26888 | 26889 | 28500 | 29627 | 29921 | 30628 | 30831 | 30991 | 32199 | 32612 | 32615 | 32648 | 32811 | 32943 | 34189 | 34840 | 34858 | 35205 | 36535 | 36544 | 37821 | 39596 | 41253 | 42347 | 42969 | 44035 | 44036 | 44505 | 46808 => 12,
            498 | 642 | 1020 | 5573 | 31884 => 17,
            22911 | 35062 | 35570 | 35754 | 36038 | 36058 | 37511 | 41272 | 50874 | 50876 => 4,
            36414 | 36458 | 38985 => 7,
            40019 | 46873 => 2,
            45438 => 19,
            _ => 0,
        }
    }

    /// Returns `0` except for specific item entries.
    pub const fn target_aura_state_not(&self) -> i32 {
        match self.entry {
            17 | 592 | 600 | 3747 | 6065 | 6066 | 10898 | 10899 | 10900 | 10901 | 25217 | 25218 => 18,
            1022 | 5599 | 10278 => 17,
            23452 => 12,
            40019 | 46873 => 4,
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

    pub const fn mana_cost_per_level(&self) -> i32 {
        self.mana_cost_per_level as i32
    }

    /// Returns `0` except for specific item entries.
    pub const fn mana_per_second(&self) -> i32 {
        match self.entry {
            461 | 3698 => 10,
            755 => 5,
            3699 => 17,
            3700 => 24,
            10260 => 25,
            11693 => 33,
            11694 => 42,
            11695 => 52,
            16569 | 40884 => 75,
            27259 => 65,
            40671 => 1000,
            46467 => 4500,
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
            1978 | 2643 | 3034 | 3043 | 3044 | 3674 | 5116 | 13549 | 13550 | 13551 | 13552 | 13553 | 13554 | 13555 | 14274 | 14279 | 14280 | 14281 | 14282 | 14283 | 14284 | 14285 | 14286 | 14287 | 14288 | 14289 | 14290 | 14296 | 15629 | 15630 | 15631 | 15632 | 17171 | 17174 | 18545 | 18649 | 19434 | 20736 | 20900 | 20901 | 20902 | 20903 | 20904 | 22914 | 25294 | 25295 | 27016 | 27018 | 27019 | 27020 | 27021 | 27065 | 27632 | 27634 | 30614 | 31407 | 31975 | 34104 | 34120 | 34490 | 34829 | 35401 | 35511 | 36609 | 36623 | 36984 | 37551 | 38370 | 38807 | 38859 | 38861 | 38914 | 39182 | 39413 | 40411 | 41084 | 44271 => 75,
            23675 | 23676 => 23675,
            _ => 0,
        }
    }

    pub const fn stack_amount(&self) -> i32 {
        self.stack_amount
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
            9439 | 9976 | 9991 | 9999 | 10723 | 12332 | 21544 | 21565 | 34943 | 35037 => 60,
            9736 | 32926 | 33639 | 41233 => 100,
            9773 | 9779 => 255,
            9901 | 10953 | 26198 => 70,
            10267 | 26197 => 50,
            10268 => 80,
            15366 | 22888 => 63,
            25596 | 26995 => 85,
            26258 => 30,
            26259 => 20,
            39560 | 39758 => 200,
            42630 | 43594 => 73,
            _ => 0,
        }
    }

    pub const fn spell_family_name(&self) -> i32 {
        self.spell_family_name as i32
    }

    pub const fn spell_family_flags(&self) -> i64 {
        self.spell_family_flags
    }

    pub const fn max_affected_targets(&self) -> i32 {
        self.max_affected_targets as i32
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

    /// Always returns `0`.
    pub const fn min_faction_id(&self) -> i32 {
        0
    }

    /// Always returns `0`.
    pub const fn min_reputation(&self) -> i32 {
        0
    }

    /// Returns `0` except for specific item entries.
    pub const fn required_aura_vision(&self) -> i32 {
        match self.entry {
            26869 | 44068 | 45723 | 46901 => 1,
            44067 => 2,
            _ => 0,
        }
    }

    pub const fn area_id(&self) -> i32 {
        self.area_id as i32
    }

    pub const fn school_mask(&self) -> i32 {
        self.school_mask as i32
    }

    pub const fn is_server_side(&self) -> i32 {
        self.is_server_side as i32
    }

    pub const fn attributes_serverside(&self) -> i32 {
        self.attributes_serverside as i32
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
            realm_points_per_level:0.0,
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
            misc_value_b:0,
            trigger_spell:0,
            points_per_combo_point:0.0,
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

    pub const fn totem_categories_array(&self) -> [TotemCategory; 2] {
        const D: TotemCategory=TotemCategory{
            category:0,
        };
        let l = self.totem_categories.len();
        [
            if l >= 1 { self.totem_categories()[0] } else { D },
            if l >= 2 { self.totem_categories()[1] } else { D },
        ]
    }

    pub const fn totem_categories(&self) -> &[TotemCategory] {
        self.totem_categories
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

#[derive(Debug, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Reagent {
    pub reagent: i32,
    pub amount: u32,
}

impl Reagent {
    pub const fn new(
        reagent: i32,
        amount: u32,
    ) -> Self {
        Self {
            reagent,
            amount,
        }
    }
}
#[derive(Debug, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct SpellEffects {
    pub effect: i32,
    pub die_sides: i32,
    pub base_dice: i32,
    pub dice_per_level: f32,
    pub realm_points_per_level: f32,
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
    pub misc_value_b: i32,
    pub trigger_spell: i32,
    pub points_per_combo_point: f32,
    pub damage_multiplier: f32,
}

impl SpellEffects {
    pub const fn new(
        effect: i32,
        die_sides: i32,
        base_dice: i32,
        dice_per_level: f32,
        realm_points_per_level: f32,
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
        misc_value_b: i32,
        trigger_spell: i32,
        points_per_combo_point: f32,
        damage_multiplier: f32,
    ) -> Self {
        Self {
            effect,
            die_sides,
            base_dice,
            dice_per_level,
            realm_points_per_level,
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
            misc_value_b,
            trigger_spell,
            points_per_combo_point,
            damage_multiplier,
        }
    }
}
#[derive(Debug, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
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
#[derive(Debug, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct TotemCategory {
    pub category: i32,
}

impl TotemCategory {
    pub const fn new(
        category: i32,
    ) -> Self {
        Self {
            category,
        }
    }
}
