pub(crate) mod area_triggers;
pub(crate) mod items;
pub(crate) mod pet_names;
pub(crate) mod spells;

use super::position::{positions, RawPosition};
use super::types::{Class, Race};
use super::{read_csv_file, Expansion};
use crate::base_printer::data::items::{get_items, Field, Optimizations};
use crate::base_printer::data::pet_names::{get_pet_name_data, Pet, PetNames};
use crate::base_printer::data::spells::get_spells;
use crate::base_printer::write::items::GenericThing;
use hashbrown::HashMap;
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

pub(crate) struct Data {
    pub exp_per_level: Vec<XpPerLevel>,
    pub exploration_exp_per_level: Vec<XpPerLevel>,
    pub base_stats: HashMap<Combination, BTreeMap<u8, BaseStats>>,
    pub skills: HashMap<Combination, BTreeSet<(u32, String)>>,
    pub initial_spells: HashMap<Combination, BTreeSet<(u32, String)>>,
    pub combinations: Vec<Combination>,
    positions: Vec<RawPosition>,
    pub actions: HashMap<Combination, BTreeSet<Action>>,
    pub triggers: Vec<Trigger>,
    pub pet_names: BTreeMap<Pet, PetNames>,
    pub items: (Vec<GenericThing>, Optimizations),
    pub spells: (Vec<GenericThing>, Optimizations),
}

impl Data {
    pub fn positions(&self, expansion: Expansion) -> impl Iterator<Item = &RawPosition> {
        self.positions.iter().filter(move |a| match expansion {
            Expansion::Vanilla => a.valid_versions.vanilla(),
            Expansion::BurningCrusade => a.valid_versions.tbc(),
            Expansion::WrathOfTheLichKing => a.valid_versions.wrath(),
        })
    }
}

pub(crate) fn get_fields(things: &[GenericThing]) -> &[Field] {
    &things[0].fields
}

pub(crate) fn get_data_from_csv_files(expansion: Expansion) -> Data {
    let spell_thread =
        std::thread::spawn(move || get_spells(expansion, &expansion.csv_data_directory()));
    let csv_directory = expansion.csv_data_directory();

    let combinations = get_combinations(&csv_directory);
    let exp_per_level = get_exp_data(&csv_directory);
    let exploration_exp_per_level = get_exploration_exp_data(&csv_directory);
    let base_stats = get_stat_data(&csv_directory);
    let skills = get_skill_data(&csv_directory);
    let initial_spells = get_initial_spell_data(&csv_directory);
    let actions = get_action_data(&csv_directory);
    let positions = positions();
    let triggers = get_triggers(&csv_directory);
    let pet_names = get_pet_name_data(&csv_directory);
    let items = get_items(&csv_directory, expansion);

    let spells = spell_thread.join().unwrap();

    Data {
        exp_per_level,
        exploration_exp_per_level,
        base_stats,
        skills,
        initial_spells,
        combinations,
        positions,
        actions,
        triggers,
        pet_names,
        items,
        spells,
    }
}

#[derive(Deserialize)]
pub(crate) struct XpPerLevel {
    pub level: u8,
    pub exp: i32,
}

fn get_exploration_exp_data(dir: &Path) -> Vec<XpPerLevel> {
    read_csv_file(dir, "exploration_exp")
}

fn get_exp_data(dir: &Path) -> Vec<XpPerLevel> {
    read_csv_file::<XpPerLevel>(dir, "level_exp")
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct BaseStats {
    pub strength: u8,
    pub agility: u8,
    pub stamina: u8,
    pub intellect: u8,
    pub spirit: u8,
    pub health: u32,
    pub mana: u32,
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct BaseStatLevel {
    race: Race,
    class: Class,
    level: u8,
    stats: BaseStats,
}

#[derive(Deserialize)]
struct StatData {
    pub race: u8,
    pub class: u8,
    pub level: u8,
    pub str: u8,
    pub agi: u8,
    pub sta: u8,
    pub inte: u8,
    pub spi: u8,
    pub basehp: u32,
    pub basemana: u32,
}

fn get_stat_data(dir: &Path) -> HashMap<Combination, BTreeMap<u8, BaseStats>> {
    let stats: Vec<_> = read_csv_file::<StatData>(dir, "stat_data")
        .into_iter()
        .map(|row| BaseStatLevel {
            race: row.race.try_into().unwrap(),
            class: row.class.try_into().unwrap(),
            level: row.level,
            stats: BaseStats {
                strength: row.str,
                agility: row.agi,
                stamina: row.sta,
                intellect: row.inte,
                spirit: row.spi,
                health: row.basehp,
                mana: row.basemana,
            },
        })
        .collect();

    let combinations = get_combinations(dir);

    let mut h = HashMap::new();

    for combination in combinations {
        let stats = stats
            .iter()
            .filter(|a| a.race == combination.race && a.class == combination.class);

        let mut s = BTreeMap::new();

        for stat in stats {
            s.insert(stat.level, stat.stats);
        }

        h.insert(combination, s);
    }

    h
}

#[derive(Deserialize)]
struct Skill {
    #[serde(rename = "raceMask")]
    race_mask: u32,
    #[serde(rename = "classMask")]
    class_mask: u32,
    skill: u32,
    note: String,
}

fn get_skill_data(dir: &Path) -> HashMap<Combination, BTreeSet<(u32, String)>> {
    let skills = read_csv_file::<Skill>(dir, "skills");

    let combinations = get_combinations(dir);
    let mut h = HashMap::new();

    for combination in combinations {
        let skills = skills.iter().filter(|a| {
            let race_mask = 1 << (combination.race.as_int() - 1);
            let class_mask = 1 << (combination.class.as_int() - 1);

            let race = a.race_mask & race_mask != 0 || a.race_mask == 0;
            let class = a.class_mask & class_mask != 0 || a.class_mask == 0;

            race && class
        });

        let mut s = BTreeSet::new();

        for skill in skills {
            s.insert((skill.skill, skill.note.clone()));
        }

        h.insert(combination, s);
    }

    h
}

#[derive(Deserialize)]
struct CsvSpell {
    race: u8,
    class: u8,
    #[serde(rename = "Spell")]
    spell: u32,
    #[serde(rename = "Note")]
    note: String,
}

struct Spell {
    race: Race,
    class: Class,
    spell: u32,
    note: String,
}

fn get_initial_spell_data(dir: &Path) -> HashMap<Combination, BTreeSet<(u32, String)>> {
    let spells = read_csv_file::<CsvSpell>(dir, "initial_spells")
        .into_iter()
        .map(|a| Spell {
            race: a.race.try_into().unwrap(),
            class: a.class.try_into().unwrap(),
            spell: a.spell,
            note: a.note,
        })
        .collect::<Vec<_>>();

    let combinations = get_combinations(dir);

    let mut h = HashMap::new();

    for combination in combinations {
        let spells = spells
            .iter()
            .filter(|a| a.class == combination.class && a.race == combination.race);

        let mut s = BTreeSet::new();
        for spell in spells {
            s.insert((spell.spell, spell.note.clone()));
        }

        h.insert(combination, s);
    }

    h
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct Combination {
    pub race: Race,
    pub class: Class,
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct Action {
    pub button: u8,
    pub action: u16,
    pub ty: u8,
}

#[derive(Deserialize)]
pub(crate) struct CsvCombination {
    pub race: u8,
    pub class: u8,
}

#[derive(Deserialize)]
struct ActionRaceClass {
    pub race: u8,
    pub class: u8,
    pub button: u8,
    pub action: u16,
    #[serde(rename = "type")]
    pub ty: u8,
}

fn get_action_data(dir: &Path) -> HashMap<Combination, BTreeSet<Action>> {
    let actions = read_csv_file::<ActionRaceClass>(dir, "actions");

    let combinations = get_combinations(dir);

    let mut h = HashMap::new();

    for combination in combinations {
        let actions = actions.iter().filter(|a| {
            a.class == combination.class.as_int() && a.race == combination.race.as_int()
        });

        let mut s = BTreeSet::new();
        for action in actions {
            s.insert(Action {
                button: action.button,
                action: action.action,
                ty: action.ty,
            });
        }
        h.insert(combination, s);
    }

    h
}

fn get_combinations(dir: &Path) -> Vec<Combination> {
    let mut combinations: Vec<_> = read_csv_file::<CsvCombination>(dir, "distinct_actions")
        .iter()
        .map(|a| Combination {
            race: a.race.try_into().unwrap(),
            class: a.class.try_into().unwrap(),
        })
        .collect();
    combinations.sort_by(|a, b| a.race.cmp(&b.race).then(a.class.cmp(&b.class)));
    combinations
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Trigger {
    Quest {
        id: u32,
        quest: u32,
    },
    Inn {
        id: u32,
        name: String,
    },
    Teleport {
        id: u32,
        map: u32,
        x: f32,
        y: f32,
        z: f32,
        orientation: f32,
        required_level: u8,
        required_item: u32,
        required_quest: u32,
        failed_text: Option<String>,
        // Below not required for vanilla
        heroic_keys: Vec<u32>,
        heroic_required_quest: u32,
        name: String,
    },
}

impl Trigger {
    pub const fn id(&self) -> u32 {
        match *self {
            Trigger::Inn { id, .. } => id,
            Trigger::Teleport { id, .. } => id,
            Trigger::Quest { id, .. } => id,
        }
    }
}

#[derive(Deserialize, Clone)]
struct AreaTriggerTavern {
    id: u32,
    name: String,
}

#[derive(Deserialize, Clone)]
struct AreaTriggerQuest {
    id: u32,
    quest: u32,
}

#[derive(Deserialize, Clone)]
struct AreaTriggerTeleport {
    name: String,
    id: u32,
    #[serde(rename = "target_map")]
    map: u32,
    #[serde(rename = "target_position_x")]
    x: f32,
    #[serde(rename = "target_position_y")]
    y: f32,
    #[serde(rename = "target_position_z")]
    z: f32,
    #[serde(rename = "target_orientation")]
    orientation: f32,
    required_level: u8,
    required_item: u32,
    #[serde(rename = "required_quest_done")]
    required_quest: u32,
    #[serde(rename = "status_failed_text")]
    failed_text: Option<String>,
    // Below not required for vanilla
    #[serde(default)]
    heroic_key: u32,
    #[serde(default)]
    heroic_key2: u32,
    #[serde(rename = "required_quest_done_heroic")]
    #[serde(default)]
    heroic_required_quest: u32,
}

fn get_triggers(dir: &Path) -> Vec<Trigger> {
    let taverns = read_csv_file::<AreaTriggerTavern>(&dir, "tavern_triggers")
        .into_iter()
        .map(|row| Trigger::Inn {
            id: row.id,
            name: row.name,
        });

    let quests = read_csv_file::<AreaTriggerQuest>(&dir, "quest_triggers")
        .into_iter()
        .map(|row| Trigger::Quest {
            id: row.id,
            quest: row.quest,
        });

    let teleports = read_csv_file::<AreaTriggerTeleport>(&dir, "teleport_triggers")
        .into_iter()
        .map(|row| {
            let mut heroic_keys = vec![];
            let heroic_key = row.heroic_key;
            if heroic_key != 0 {
                heroic_keys.push(heroic_key);
            }
            let heroic_key2 = row.heroic_key2;
            if heroic_key2 != 0 {
                heroic_keys.push(heroic_key2);
            }

            Trigger::Teleport {
                id: row.id,
                map: row.map,
                x: row.x,
                y: row.y,
                z: row.z,
                orientation: row.orientation,
                required_level: row.required_level,
                required_item: row.required_item,
                required_quest: row.required_quest,
                failed_text: row.failed_text,
                heroic_keys,
                heroic_required_quest: row.heroic_required_quest,
                name: row.name,
            }
        });

    teleports.chain(quests).chain(taverns).collect()
}
