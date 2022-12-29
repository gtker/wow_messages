use crate::vanilla::{Class, PlayerRace, RaceClass};

/// Calculate base melee attack power.
///
/// For druids this does not take into account the different attack power calculations when
/// the different forms. Use [`non_cat_form_base_attack_power`] or
/// [`cat_form_base_attack_power`] instead.
pub fn base_melee_attack_power(class: Class, strength: u16, agility: u16, level: u8) -> u32 {
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
pub fn base_ranged_attack_power(class: Class, agility: u16, level: u8) -> u32 {
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

impl RaceClass {
    /// Calculate base melee attack power.
    ///
    /// For druids this does not take into account the different attack power calculations when
    /// the different forms. Use [`non_cat_form_base_attack_power`] or
    /// [`cat_form_base_attack_power`] instead.
    pub fn base_melee_attack_power(&self, strength: u16, agility: u16, level: u8) -> u32 {
        base_melee_attack_power(self.class(), strength, agility, level)
    }

    /// Calculate base ranged attack power.
    ///
    /// Since only warrior, rogue, and hunter can use weapons that use ranged attack power
    /// all other classes return 0.
    pub fn base_ranged_attack_power(&self, agility: u16, level: u8) -> u32 {
        base_ranged_attack_power(self.class(), agility, level)
    }
}
