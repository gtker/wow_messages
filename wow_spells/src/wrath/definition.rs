#![allow(clippy::too_many_arguments)]
use crate::wrath::{
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
    entry: u32,
    category: i16,
    dispel: i8,
    mechanic: i8,
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
    target_creature_type: i16,
    requires_spell_focus: i16,
    facing_caster_flags: i8,
    caster_aura_state_not: i8,
    exclude_target_aura_spell: i32,
    casting_time_index: i16,
    recovery_time: i32,
    category_recovery_time: i32,
    interrupt_flags: i8,
    aura_interrupt_flags: u32,
    channel_interrupt_flags: u32,
    proc_flags: i32,
    proc_chance: i8,
    proc_charges: i32,
    max_level: i16,
    base_level: i16,
    spell_level: i16,
    duration_index: i16,
    power_type: u32,
    mana_cost: i32,
    mana_cost_per_level: i8,
    range_index: i16,
    speed: f32,
    modal_next_spell: i16,
    stack_amount: i32,
    equipped_item_class: i8,
    equipped_item_sub_class_mask: i32,
    equipped_item_inventory_type_mask: i32,
    spell_visual: i16,
    spell_visual2: i16,
    spell_icon_id: i16,
    active_icon_id: i16,
    spell_priority: i8,
    spell_name: &'static str,
    rank_text: &'static str,
    mana_cost_percentage: i8,
    start_recovery_category: i16,
    start_recovery_time: i32,
    spell_family_name: i8,
    spell_family_flags: u64,
    spell_family_flags2: u32,
    max_affected_targets: i8,
    dmg_class: i8,
    prevention_type: i8,
    stance_bar_order: i8,
    area_id: u32,
    school_mask: i8,
    rune_cost_id: u16,
    spell_missile_id: u16,
    spell_description_variable_id: u32,
    spell_difficulty_id: u16,
    is_server_side: u32,
    attributes_serverside: i8,
    reagents: &'static [Reagent],
    effects: &'static [SpellEffects],
    totems: &'static [Totem],
    totem_categories: &'static [TotemCategory],
}

impl Spell {
    #[doc(hidden)]
    pub const fn new(
        entry: u32,
        category: i16,
        dispel: i8,
        mechanic: i8,
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
        target_creature_type: i16,
        requires_spell_focus: i16,
        facing_caster_flags: i8,
        caster_aura_state_not: i8,
        exclude_target_aura_spell: i32,
        casting_time_index: i16,
        recovery_time: i32,
        category_recovery_time: i32,
        interrupt_flags: i8,
        aura_interrupt_flags: u32,
        channel_interrupt_flags: u32,
        proc_flags: i32,
        proc_chance: i8,
        proc_charges: i32,
        max_level: i16,
        base_level: i16,
        spell_level: i16,
        duration_index: i16,
        power_type: u32,
        mana_cost: i32,
        mana_cost_per_level: i8,
        range_index: i16,
        speed: f32,
        modal_next_spell: i16,
        stack_amount: i32,
        equipped_item_class: i8,
        equipped_item_sub_class_mask: i32,
        equipped_item_inventory_type_mask: i32,
        spell_visual: i16,
        spell_visual2: i16,
        spell_icon_id: i16,
        active_icon_id: i16,
        spell_priority: i8,
        spell_name: &'static str,
        rank_text: &'static str,
        mana_cost_percentage: i8,
        start_recovery_category: i16,
        start_recovery_time: i32,
        spell_family_name: i8,
        spell_family_flags: u64,
        spell_family_flags2: u32,
        max_affected_targets: i8,
        dmg_class: i8,
        prevention_type: i8,
        stance_bar_order: i8,
        area_id: u32,
        school_mask: i8,
        rune_cost_id: u16,
        spell_missile_id: u16,
        spell_description_variable_id: u32,
        spell_difficulty_id: u16,
        is_server_side: u32,
        attributes_serverside: i8,
        reagents: &'static [Reagent],
        effects: &'static [SpellEffects],
        totems: &'static [Totem],
        totem_categories: &'static [TotemCategory],
    ) -> Self {
        Self {
            entry,
            category,
            dispel,
            mechanic,
            attributes,
            attributes_ex,
            attributes_ex2,
            attributes_ex3,
            attributes_ex4,
            attributes_ex5,
            attributes_ex6,
            attributes_ex7,
            stances,
            stances_not,
            targets,
            target_creature_type,
            requires_spell_focus,
            facing_caster_flags,
            caster_aura_state_not,
            exclude_target_aura_spell,
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
            modal_next_spell,
            stack_amount,
            equipped_item_class,
            equipped_item_sub_class_mask,
            equipped_item_inventory_type_mask,
            spell_visual,
            spell_visual2,
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
            spell_family_flags2,
            max_affected_targets,
            dmg_class,
            prevention_type,
            stance_bar_order,
            area_id,
            school_mask,
            rune_cost_id,
            spell_missile_id,
            spell_description_variable_id,
            spell_difficulty_id,
            is_server_side,
            attributes_serverside,
            reagents,
            effects,
            totems,
            totem_categories,
        }
    }
    pub const fn entry(&self) -> u32 {
        self.entry
    }

    pub const fn category(&self) -> i32 {
        self.category as i32
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

    pub const fn attributes_ex7(&self) -> u32 {
        self.attributes_ex7
    }

    pub const fn stances(&self) -> u32 {
        self.stances
    }

    /// Always returns `0`.
    pub const fn stances2(&self) -> u32 {
        0
    }

    pub const fn stances_not(&self) -> u32 {
        self.stances_not
    }

    /// Always returns `0`.
    pub const fn stances_not2(&self) -> u32 {
        0
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
            6572 | 6574 | 7379 | 11600 | 11601 | 14251 | 19130 | 25269 | 25288 | 30357 | 34097 | 40019 | 40392 | 57823 | 72556 => 1,
            17170 | 42636 | 45384 | 45433 | 56581 | 59109 | 72559 | 72560 | 72561 => 2,
            19306 | 20909 | 20910 | 27067 | 31819 | 34243 | 48998 | 48999 => 7,
            20271 | 41467 | 53407 | 53408 | 57774 => 5,
            34428 => 10,
            44440 | 44441 | 52234 | 53497 => 13,
            50096 | 50108 | 50109 | 50110 | 50111 => 23,
            50240 => 22,
            55694 => 17,
            60348 => 11,
            _ => 0,
        }
    }

    /// Returns `0` except for specific item entries.
    pub const fn target_aura_state(&self) -> i32 {
        match self.entry {
            5308 | 7160 | 7938 | 16495 | 20539 | 20647 | 20658 | 20660 | 20661 | 20662 | 24239 | 24274 | 24275 | 25234 | 25236 | 27180 | 29364 | 29909 | 31255 | 32772 | 35771 | 36734 | 36779 | 37251 | 37255 | 37259 | 38895 | 38959 | 42325 | 44875 | 47470 | 47471 | 48805 | 48806 | 53351 | 56426 | 56893 | 61005 | 61006 | 61140 | 62125 | 72424 | 72465 => 2,
            14083 | 47679 | 47702 | 47722 | 49024 | 49538 | 50251 | 52862 | 52864 => 13,
            17962 => 14,
            18562 => 15,
            32645 | 32684 | 57992 | 57993 => 16,
            40019 | 72556 => 3,
            42636 | 44572 | 45384 | 45433 | 72559 | 72560 | 72561 => 4,
            51662 => 18,
            _ => 0,
        }
    }

    pub const fn caster_aura_state_not(&self) -> i32 {
        self.caster_aura_state_not as i32
    }

    /// Returns `0` except for specific item entries.
    pub const fn target_aura_state_not(&self) -> i32 {
        match self.entry {
            23452 => 12,
            40019 | 72556 => 4,
            42636 | 45384 | 45433 | 72559 | 72560 | 72561 => 11,
            47299 => 18,
            63884 => 22,
            _ => 0,
        }
    }

    /// Returns `0` except for specific item entries.
    pub const fn caster_aura_spell(&self) -> i32 {
        match self.entry {
            4342 | 59702 => 4341,
            7450 => 51455,
            47480 => 62218,
            48020 => 62388,
            52226 | 56793 => 52255,
            55033 | 55042 => 55032,
            56753 => 56750,
            56815 => 56817,
            60587 => 56706,
            61093 => 61171,
            61484 => 49135,
            61496 => 12345,
            61784 => 61801,
            61785 => 61798,
            61786 => 61802,
            61787 => 61799,
            61788 => 61800,
            62025 => 62028,
            62324 => 62340,
            62473 => 62496,
            62486 => 62480,
            69908 => 69876,
            70360 => 70346,
            72052 | 72800 | 72801 | 72802 => 72059,
            72527 => 72456,
            73288 => 73251,
            73771 => 73770,
            73830 => 73829,
            73832 => 73831,
            73833 => 73834,
            _ => 0,
        }
    }

    /// Returns `0` except for specific item entries.
    pub const fn target_aura_spell(&self) -> i32 {
        match self.entry {
            52053 | 57922 => 49777,
            62482 | 67387 => 62495,
            63361..=63363 => 63034,
            64176 => 64175,
            67820 => 67823,
            68470 => 68479,
            68471 => 68478,
            69511 => 69510,
            69678 | 70222 => 70120,
            70162 | 70295 | 70609 | 72566 | 72567 | 72568 => 70121,
            70192 => 71507,
            70459 => 70447,
            71466 => 70203,
            71518 => 71516,
            72202 => 72178,
            72255 | 72444 | 72445 | 72446 => 72293,
            72289 => 72290,
            72409 | 72447 | 72448 | 72449 => 72410,
            72934 => 72154,
            75127 => 74276,
            _ => 0,
        }
    }

    /// Returns `0` except for specific item entries.
    pub const fn exclude_caster_aura_spell(&self) -> i32 {
        match self.entry {
            498 | 642 | 66010 | 71550 => 61988,
            883 | 62757 => 61431,
            5217 | 6793 | 9845 | 9846 | 50212 | 50213 => 50334,
            12042 => 12043,
            12043 => 12042,
            31884 | 66011 => 61987,
            44029 | 44083 => 44021,
            45438 | 65802 => 41425,
            59193 => 61498,
            59194 => 61499,
            59196 => 61497,
            62306 | 62490 => 62340,
            62412 | 62416 | 62419 | 62425 | 62426 | 62427 | 64414 => 62433,
            62673 => 62719,
            62719 => 62665,
            62846 | 64454 => 62798,
            63003 => 62799,
            63018 | 63024 | 64234 | 65121 | 65598 => 62776,
            63031 => 62978,
            64104 => 64100,
            65403 => 65400,
            68470..=68471 => 68472,
            69057 | 69138 | 70826 | 71580 | 73142 | 73143 => 69076,
            69471 => 69487,
            69871 => 69871,
            69900 | 73046 => 69876,
            70498 | 72762 => 72679,
            73536 => 73535,
            74203..=74205 => 75572,
            76069 | 76071 | 76096 | 76098 | 76114 | 76116 => 61551,
            _ => 0,
        }
    }

    pub const fn exclude_target_aura_spell(&self) -> i32 {
        self.exclude_target_aura_spell
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

    pub const fn aura_interrupt_flags(&self) -> u32 {
        self.aura_interrupt_flags
    }

    pub const fn channel_interrupt_flags(&self) -> u32 {
        self.channel_interrupt_flags
    }

    pub const fn proc_flags(&self) -> i32 {
        self.proc_flags
    }

    pub const fn proc_chance(&self) -> i32 {
        self.proc_chance as i32
    }

    pub const fn proc_charges(&self) -> i32 {
        self.proc_charges
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
            461 | 3698 | 54900 => 10,
            755 => 5,
            2378 => 2,
            3699 => 17,
            3700 => 24,
            10260 => 25,
            11389 => 8,
            11693 => 33,
            11694 => 42,
            11695 => 52,
            16569 | 40884 | 60829 => 75,
            27259 => 65,
            40671 => 1000,
            42636 | 45384 | 45433 | 72559 | 72560 | 72561 => 1,
            46467 => 4500,
            47856 => 182,
            50514 => 30,
            66183 | 68831 => 20,
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

    pub const fn modal_next_spell(&self) -> i32 {
        self.modal_next_spell as i32
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

    pub const fn spell_visual2(&self) -> i32 {
        self.spell_visual2 as i32
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
        self.start_recovery_time
    }

    /// Returns `0` except for specific item entries.
    pub const fn max_target_level(&self) -> i32 {
        match self.entry {
            2908 | 26195 => 40,
            8955 => 55,
            9439 | 9976 | 9991 | 9999 | 10723 | 12332 | 21544 | 21565 | 34943 | 35037 => 60,
            9736 | 32926 | 33639 | 41233 => 100,
            9773 | 9779 => 255,
            9901 | 26198 => 70,
            10267 | 26197 => 50,
            10268 => 80,
            15366 | 22888 => 63,
            26258 => 30,
            26259 => 20,
            26995 => 85,
            39560 | 39758 => 200,
            42630 | 43594 => 73,
            42636 | 45384 | 45433 | 72559 | 72560 | 72561 => 1,
            _ => 0,
        }
    }

    pub const fn spell_family_name(&self) -> i32 {
        self.spell_family_name as i32
    }

    pub const fn spell_family_flags(&self) -> u64 {
        self.spell_family_flags
    }

    pub const fn spell_family_flags2(&self) -> u32 {
        self.spell_family_flags2
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

    /// Returns `0` except for specific item entries.
    pub const fn min_faction_id(&self) -> i32 {
        match self.entry {
            42636 | 45384 | 45433 | 72559 | 72560 | 72561 => 570,
            51186 | 51188 | 51189 => 1104,
            51190..=51192 => 1105,
            _ => 0,
        }
    }

    /// Returns `0` except for specific item entries.
    pub const fn min_reputation(&self) -> i32 {
        match self.entry {
            42636 | 45384 | 45433 | 72559 | 72560 | 72561 => 2,
            51186 | 51188 | 51189 | 51190 | 51191 | 51192 => 3,
            _ => 0,
        }
    }

    /// Returns `0` except for specific item entries.
    pub const fn required_aura_vision(&self) -> i32 {
        match self.entry {
            26869 | 42636 | 44068 | 45384 | 45433 | 45723 | 46901 | 72559 | 72560 | 72561 => 1,
            44067 => 2,
            46077 | 49561 | 54273 | 54284 | 71949 => 3,
            _ => 0,
        }
    }

    pub const fn area_id(&self) -> u32 {
        self.area_id
    }

    pub const fn school_mask(&self) -> i32 {
        self.school_mask as i32
    }

    pub const fn rune_cost_id(&self) -> u32 {
        self.rune_cost_id as u32
    }

    pub const fn spell_missile_id(&self) -> u32 {
        self.spell_missile_id as u32
    }

    /// Returns `0` except for specific item entries.
    pub const fn power_display_id(&self) -> u32 {
        match self.entry {
            40152 => 1,
            51678 | 54109 | 57609 | 62345 | 62346 | 62358 | 62359 | 62522 | 64677 | 64872 | 64979 | 66183 | 66186 | 66203 | 66223 | 66224 | 66518 | 66529 | 67461 | 67462 | 67796 | 67797 | 67816 | 68825 | 68831 | 68832 | 69495 | 69502 | 69505 => 61,
            52358 | 53032 => 142,
            62471 | 62490 => 41,
            66227 | 66251 | 72202 | 72255 | 72256 | 72379 | 72444 | 72445 | 72446 => 4294967295,
            69399 | 69401 | 69402 | 69470 | 69471 | 69487 | 69488 | 70172 | 70174 | 70175 | 70385 => 101,
            70539 | 71516 | 72457 | 72875 | 72876 => 121,
            72195 | 72242 | 72254 | 72260 | 72278 | 72279 | 72280 | 72293 | 72370 | 72371 | 72378 | 72380 | 72385 | 72409 | 72410 | 72438 | 72439 | 72440 | 72441 | 72442 | 72443 | 72447 | 72448 | 72449 | 73058 => 141,
            _ => 0,
        }
    }

    pub const fn spell_description_variable_id(&self) -> u32 {
        self.spell_description_variable_id
    }

    pub const fn spell_difficulty_id(&self) -> u32 {
        self.spell_difficulty_id as u32
    }

    pub const fn is_server_side(&self) -> u32 {
        self.is_server_side
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
            real_points_per_level:0.0,
            base_points:0,
            mechanics:0,
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
            spell_class_mask1:0,
            spell_class_mask2:0,
            spell_class_mask3:0,
            damage_multiplier:0.0,
            bonus_coefficient:0.0,
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
    pub real_points_per_level: f32,
    pub base_points: i32,
    pub mechanics: i32,
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
    pub trigger_spell: u32,
    pub points_per_combo_point: f32,
    pub spell_class_mask1: u32,
    pub spell_class_mask2: u32,
    pub spell_class_mask3: u32,
    pub damage_multiplier: f32,
    pub bonus_coefficient: f32,
}

impl SpellEffects {
    pub const fn new(
        effect: i32,
        die_sides: i32,
        real_points_per_level: f32,
        base_points: i32,
        mechanics: i32,
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
        trigger_spell: u32,
        points_per_combo_point: f32,
        spell_class_mask1: u32,
        spell_class_mask2: u32,
        spell_class_mask3: u32,
        damage_multiplier: f32,
        bonus_coefficient: f32,
    ) -> Self {
        Self {
            effect,
            die_sides,
            real_points_per_level,
            base_points,
            mechanics,
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
            spell_class_mask1,
            spell_class_mask2,
            spell_class_mask3,
            damage_multiplier,
            bonus_coefficient,
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
