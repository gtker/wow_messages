use crate::vanilla::exp::MAX_LEVEL;
use crate::vanilla::{Class, PlayerRace, RaceClass};

/// Calculate base melee attack power.
///
/// For druids this does not take into account the different attack power calculations when
/// the different forms. Use [`non_cat_form_base_attack_power`] or
/// [`cat_form_base_attack_power`] instead.
pub const fn base_melee_attack_power(class: Class, strength: u16, agility: u16, level: u8) -> u32 {
    let level = level as u32;
    let strength = strength as u32;
    let agility = agility as u32;

    match class {
        Class::Paladin | Class::Warrior => (level * 3 + strength * 2).saturating_sub(20),
        Class::Hunter | Class::Rogue => (level * 2 + strength + agility).saturating_sub(20),
        Class::Shaman => (level * 2 + strength * 2).saturating_sub(20),
        Class::Druid => (strength * 2).saturating_sub(20),
        Class::Priest | Class::Mage | Class::Warlock => strength.saturating_sub(10),
    }
}

fn predatory_strikes(predatory_strikes_rank: u8, level: u8) -> f32 {
    let level = level as f32;

    if predatory_strikes_rank == 1 {
        level * 0.5
    } else if predatory_strikes_rank == 2 {
        level
    } else if predatory_strikes_rank == 3 {
        level * 1.5
    } else {
        0.0
    }
}

/// Calculate base attack power for druids in bear, dire bear, and moonkin forms.
///
/// Valid values for `predatory_strikes_rank` is 0, 1, 2, or 3.
/// All other values will be treated as 0.
///
/// Predatory Strikes is a feral druid talent that increases the attack power in
/// cat, bear, and dire bear forms by a percentage of their level.
///
/// Rank 1 (spell id 16972) increases by 50% of level.
///
/// Rank 2 (spell id 16974) increases by 100% of level.
///
/// Rank 3 (spell id 16975) increases by 150% of level.
pub fn non_cat_form_base_attack_power(strength: u16, level: u8, predatory_strikes_rank: u8) -> u32 {
    let strength = strength as f32;

    (predatory_strikes(predatory_strikes_rank, level) * strength * 2.0 - 20.0) as u32
}

/// Calculate base attack power for druids in cat form.
///
/// Valid values for `predatory_strikes_rank` is 0, 1, 2, or 3.
/// All other values will be treated as 0.
///
/// Predatory Strikes is a feral druid talent that increases the attack power in
/// cat, bear, and dire bear forms by a percentage of their level.
///
/// Rank 1 (spell id 16972) increases by 50% of level.
///
/// Rank 2 (spell id 16974) increases by 100% of level.
///
/// Rank 3 (spell id 16975) increases by 150% of level.
pub fn cat_form_base_attack_power(
    strength: u16,
    agility: u16,
    level: u8,
    predatory_strikes_rank: u8,
) -> u32 {
    let strength = strength as f32;
    let agility = agility as f32;

    (predatory_strikes(predatory_strikes_rank, level) * strength * 2.0 + agility - 20.0) as u32
}

/// Calculate base ranged attack power.
///
/// Since only warrior, rogue, and hunter can use weapons that use ranged attack power
/// all other classes return 0.
pub const fn base_ranged_attack_power(class: Class, agility: u16, level: u8) -> u32 {
    let level = level as u32;
    let agility = agility as u32;

    match class {
        Class::Warrior | Class::Rogue => (level + agility).saturating_sub(10),
        Class::Hunter => (level * 2 + agility * 2).saturating_sub(10),

        Class::Paladin
        | Class::Priest
        | Class::Shaman
        | Class::Mage
        | Class::Warlock
        | Class::Druid => 0,
    }
}

/// Calculate base melee crit from agility.
///
/// Does not return the chance as a percentage, but as a whole number.
/// So a 4% chance to crit would return 4.0.
pub fn base_melee_crit(class: Class, agility: u16, level: u8) -> f32 {
    let level_1: f32;
    let level_60: f32;

    match class {
        Class::Paladin | Class::Shaman | Class::Druid => {
            level_1 = 4.6;
            level_60 = 20.0;
        }
        Class::Mage => {
            level_1 = 12.9;
            level_60 = 20.0;
        }
        Class::Rogue => {
            level_1 = 2.2;
            level_60 = 29.0;
        }
        Class::Hunter => {
            level_1 = 3.5;
            level_60 = 53.0;
        }
        Class::Priest => {
            level_1 = 11.0;
            level_60 = 20.0;
        }
        Class::Warlock => {
            level_1 = 8.4;
            level_60 = 20.0;
        }
        Class::Warrior => {
            level_1 = 3.9;
            level_60 = 20.0;
        }
    }

    let extrapolated = {
        let level = level as f32;
        let max_level = MAX_LEVEL as f32;
        let max_level_minus_one = (MAX_LEVEL - 1) as f32;
        level_1 * (max_level - level) / max_level_minus_one
            + level_60 * (level - 1.0) / max_level_minus_one
    };

    agility as f32 / extrapolated
}

/// Calculate base dodge chance from agility.
///
/// *Does* include the 1% from the night elf Quickness racial (skill id 20582)
/// and the base class dodge chances.
pub fn base_dodge_chance(class: Class, race: PlayerRace, agility: u16, level: u8) -> f32 {
    let class_base: f32 = match class {
        Class::Druid => 0.9,
        Class::Mage => 3.2,
        Class::Paladin => 0.7,
        Class::Priest => 3.0,
        Class::Shaman => 1.7,
        Class::Warlock => 2.0,

        Class::Rogue | Class::Hunter | Class::Warrior => 0.0,
    };

    let level_1: f32;
    let level_60: f32;
    match class {
        Class::Warrior => {
            level_1 = 3.9;
            level_60 = 20.0;
        }
        Class::Hunter => {
            level_1 = 1.8;
            level_60 = 26.5;
        }
        Class::Rogue => {
            level_1 = 1.1;
            level_60 = 14.5;
        }
        Class::Priest => {
            level_1 = 11.0;
            level_60 = 20.0;
        }
        Class::Mage => {
            level_1 = 12.9;
            level_60 = 20.0;
        }
        Class::Warlock => {
            level_1 = 8.4;
            level_60 = 20.0;
        }
        Class::Paladin | Class::Shaman | Class::Druid => {
            level_1 = 4.6;
            level_60 = 20.0;
        }
    }

    let extrapolated = {
        let level = level as f32;
        let max_level = MAX_LEVEL as f32;
        let max_level_minus_one = (MAX_LEVEL - 1) as f32;
        level_1 * (max_level - level) / max_level_minus_one
            + level_60 * (level - 1.0) / max_level_minus_one
    };
    let from_agility = agility as f32 / extrapolated;

    let racial_bonus: f32 = if matches!(race, PlayerRace::NightElf) {
        1.0
    } else {
        0.0
    };

    class_base + racial_bonus + from_agility
}

impl RaceClass {
    /// Calculate base melee attack power.
    ///
    /// For druids this does not take into account the different attack power calculations when
    /// the different forms. Use [`non_cat_form_base_attack_power`] or
    /// [`cat_form_base_attack_power`] instead.
    pub const fn base_melee_attack_power(&self, strength: u16, agility: u16, level: u8) -> u32 {
        base_melee_attack_power(self.class(), strength, agility, level)
    }

    /// Calculate base ranged attack power.
    ///
    /// Since only warrior, rogue, and hunter can use weapons that use ranged attack power
    /// all other classes return 0.
    pub const fn base_ranged_attack_power(&self, agility: u16, level: u8) -> u32 {
        base_ranged_attack_power(self.class(), agility, level)
    }

    /// Calculate base melee crit from agility.
    ///
    /// Does not return the chance as a percentage, but as a whole number.
    /// So a 4% chance to crit would return 4.0.
    pub fn base_melee_crit(&self, agility: u16, level: u8) -> f32 {
        base_melee_crit(self.class(), agility, level)
    }

    /// Calculate base dodge chance from agility.
    ///
    /// *Does* include the 1% from the night elf Quickness racial (skill id 20582)
    /// and the base class dodge chances.
    pub fn base_dodge_chance(&self, agility: u16, level: u8) -> f32 {
        base_dodge_chance(self.class(), self.race(), agility, level)
    }
}
