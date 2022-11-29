pub(crate) mod area_triggers;
pub(crate) mod pet_names;

use super::position::{positions, RawPosition};
use super::types::{Class, Race};
use super::Expansion;
use crate::base_printer::data::pet_names::{get_pet_name_data, Pet, PetNames};
use rusqlite::Connection;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::Path;

pub(crate) struct Data {
    pub exp_per_level: Vec<XpPerLevel>,
    pub exploration_exp_per_level: Vec<XpPerLevel>,
    pub base_stats: HashMap<Combination, BTreeMap<u8, BaseStats>>,
    pub skills: HashMap<Combination, BTreeSet<(u32, String)>>,
    pub spells: HashMap<Combination, BTreeSet<(u32, String)>>,
    pub combinations: Vec<Combination>,
    positions: Vec<RawPosition>,
    pub actions: HashMap<Combination, BTreeSet<Action>>,
    pub triggers: Vec<Trigger>,
    pub pet_names: BTreeMap<Pet, PetNames>,
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

pub(crate) struct XpPerLevel {
    pub level: u8,
    pub exp: i32,
}

pub(crate) fn get_data_from_sqlite_file(sqlite_file: &Path, expansion: Expansion) -> Data {
    let conn = Connection::open(sqlite_file).unwrap();

    let combinations = get_combinations(&conn);
    let exp_per_level = get_exp_data(&conn);
    let exploration_exp_per_level = get_exploration_exp_data(&conn);
    let base_stats = get_stat_data(&conn);
    let skills = get_skill_data(&conn);
    let spells = get_spell_data(&conn);
    let actions = get_action_data(&conn);
    let positions = positions();
    let triggers = get_triggers(&conn, expansion);
    let pet_names = get_pet_name_data(&conn);

    Data {
        exp_per_level,
        exploration_exp_per_level,
        base_stats,
        skills,
        spells,
        combinations,
        positions,
        actions,
        triggers,
        pet_names,
    }
}

fn get_exploration_exp_data(conn: &Connection) -> Vec<XpPerLevel> {
    conn.prepare("SELECT level, basexp FROM exploration_basexp;")
        .unwrap()
        .query_map([], |row| {
            Ok(XpPerLevel {
                level: row.get(0).unwrap(),
                exp: row.get(1).unwrap(),
            })
        })
        .unwrap()
        .filter_map(|a| {
            let a = a.unwrap();
            if a.level == 0 {
                None
            } else {
                Some(a)
            }
        })
        .collect::<Vec<_>>()
}

fn get_exp_data(conn: &Connection) -> Vec<XpPerLevel> {
    let mut s = conn
        .prepare("select lvl, xp_for_next_level from player_xp_for_level;")
        .unwrap();
    let xp = s
        .query_map([], |row| {
            Ok(XpPerLevel {
                level: row.get(0).unwrap(),
                exp: row.get(1).unwrap(),
            })
        })
        .unwrap();

    xp.map(|a| a.unwrap()).collect()
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

fn get_stat_data(conn: &Connection) -> HashMap<Combination, BTreeMap<u8, BaseStats>> {
    let mut s = conn
        .prepare(
            "\
SELECT
    race, l.class, l.level, str, agi, sta, inte, spi, basehp, basemana
FROM player_levelstats l
    LEFT JOIN player_classlevelstats c
        on l.level = c.level and l.class = c.class;",
        )
        .unwrap();

    let stats = s
        .query_map([], |row| {
            Ok(BaseStatLevel {
                race: row.get::<usize, u8>(0).unwrap().try_into().unwrap(),
                class: row.get::<usize, u8>(1).unwrap().try_into().unwrap(),
                level: row.get(2).unwrap(),
                stats: BaseStats {
                    strength: row.get(3).unwrap(),
                    agility: row.get(4).unwrap(),
                    stamina: row.get(5).unwrap(),
                    intellect: row.get(6).unwrap(),
                    spirit: row.get(7).unwrap(),
                    health: row.get(8).unwrap(),
                    mana: row.get(9).unwrap(),
                },
            })
        })
        .unwrap();

    let stats: Vec<BaseStatLevel> = stats.map(|a| a.unwrap()).collect();

    let combinations = get_combinations(conn);

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

struct Skill {
    race_mask: u32,
    class_mask: u32,
    skill: u32,
    note: String,
}

fn get_skill_data(conn: &Connection) -> HashMap<Combination, BTreeSet<(u32, String)>> {
    let mut s = conn
        .prepare(
            "SELECT raceMask, classMask, skill, note FROM playercreateinfo_skills ORDER BY skill;
",
        )
        .unwrap();
    let skills = s
        .query_map([], |row| {
            Ok(Skill {
                race_mask: row.get(0).unwrap(),
                class_mask: row.get(1).unwrap(),
                skill: row.get(2).unwrap(),
                note: row.get(3).unwrap(),
            })
        })
        .unwrap();

    let skills: Vec<Skill> = skills.map(|a| a.unwrap()).collect();

    let combinations = get_combinations(conn);
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

struct Spell {
    race: Race,
    class: Class,
    spell: u32,
    note: String,
}

fn get_spell_data(conn: &Connection) -> HashMap<Combination, BTreeSet<(u32, String)>> {
    let mut s = conn
        .prepare("SELECT race, class, Spell, Note FROM playercreateinfo_spell;")
        .unwrap();
    let spells: Vec<Spell> = s
        .query_map([], |row| {
            Ok(Spell {
                race: row.get::<usize, u8>(0).unwrap().try_into().unwrap(),
                class: row.get::<usize, u8>(1).unwrap().try_into().unwrap(),
                spell: row.get(2).unwrap(),
                note: row.get(3).unwrap(),
            })
        })
        .unwrap()
        .map(|a| a.unwrap())
        .collect();

    let combinations = get_combinations(conn);

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
struct ActionRaceClass {
    race: Race,
    class: Class,
    action: Action,
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct Action {
    pub button: u8,
    pub action: u16,
    pub ty: u8,
}

fn get_action_data(conn: &Connection) -> HashMap<Combination, BTreeSet<Action>> {
    let mut s = conn
        .prepare("SELECT race, class, button, action, type FROM playercreateinfo_action;")
        .unwrap();
    let actions = s
        .query_map([], |row| {
            Ok(ActionRaceClass {
                race: row.get::<usize, u8>(0).unwrap().try_into().unwrap(),
                class: row.get::<usize, u8>(1).unwrap().try_into().unwrap(),
                action: Action {
                    button: row.get(2).unwrap(),
                    action: row.get(3).unwrap(),
                    ty: row.get(4).unwrap(),
                },
            })
        })
        .unwrap()
        .map(|a| a.unwrap())
        .collect::<Vec<_>>();

    let combinations = get_combinations(conn);

    let mut h = HashMap::new();

    for combination in combinations {
        let actions = actions
            .iter()
            .filter(|a| a.class == combination.class && a.race == combination.race);

        let mut s = BTreeSet::new();
        for action in actions {
            s.insert(action.action);
        }
        h.insert(combination, s);
    }

    h
}

fn get_combinations(conn: &Connection) -> Vec<Combination> {
    let mut combinations = conn
        .prepare("SELECT DISTINCT race, class FROM player_levelstats ORDER BY race, class;")
        .unwrap();

    let combinations = combinations
        .query_map([], |row| {
            Ok(Combination {
                race: row.get::<usize, u8>(0).unwrap().try_into().unwrap(),
                class: row.get::<usize, u8>(1).unwrap().try_into().unwrap(),
            })
        })
        .unwrap();
    let mut combinations: Vec<_> = combinations.map(|a| a.unwrap()).collect();
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

fn get_triggers(conn: &Connection, expansion: Expansion) -> Vec<Trigger> {
    let mut taverns = conn
        .prepare("SELECT id, name from areatrigger_tavern;")
        .unwrap();
    let taverns = taverns
        .query_map([], |row| {
            Ok(Trigger::Inn {
                id: row.get(0).unwrap(),
                name: row.get(1).unwrap(),
            })
        })
        .unwrap();

    let mut quests = conn
        .prepare("SELECT id, quest FROM areatrigger_involvedrelation;")
        .unwrap();
    let quests = quests
        .query_map([], |row| {
            Ok(Trigger::Quest {
                id: row.get(0).unwrap(),
                quest: row.get(1).unwrap(),
            })
        })
        .unwrap();

    match expansion {
        Expansion::Vanilla => {
            let mut t = conn
                .prepare(
                    "\
SELECT
    id,
    required_level,
    required_item,
    required_quest_done,
    target_map,
    target_position_x,
    target_position_y,
    target_position_z,
    target_orientation,
    status_failed_text,
    name
FROM
    areatrigger_teleport;",
                )
                .unwrap();
            t.query_map([], |row| {
                Ok(Trigger::Teleport {
                    id: row.get(0).unwrap(),
                    required_level: row.get(1).unwrap(),
                    required_item: row.get(2).unwrap(),
                    required_quest: row.get(3).unwrap(),
                    map: row.get(4).unwrap(),
                    x: row.get(5).unwrap(),
                    y: row.get(6).unwrap(),
                    z: row.get(7).unwrap(),
                    orientation: row.get(8).unwrap(),
                    failed_text: row.get(9).unwrap(),
                    heroic_keys: vec![],
                    heroic_required_quest: 0,
                    name: row.get(10).unwrap(),
                })
            })
            .unwrap()
            .chain(quests)
            .chain(taverns)
            .map(|a| a.unwrap())
            .collect()
        }
        Expansion::BurningCrusade => {
            let mut t = conn
                .prepare(
                    "\
SELECT
    id,
    required_level,
    required_item,
    required_quest_done,
    target_map,
    target_position_x,
    target_position_y,
    target_position_z,
    target_orientation,
    status_failed_text,
    heroic_key,
    heroic_key2,
    required_quest_done_heroic,
    name
FROM
    areatrigger_teleport;",
                )
                .unwrap();
            t.query_map([], |row| {
                let mut heroic_keys = vec![];
                let heroic_key: u32 = row.get(10).unwrap();
                if heroic_key != 0 {
                    heroic_keys.push(heroic_key);
                }
                let heroic_key2 = row.get(11).unwrap();
                if heroic_key2 != 0 {
                    heroic_keys.push(heroic_key2);
                }
                Ok(Trigger::Teleport {
                    id: row.get(0).unwrap(),
                    required_level: row.get(1).unwrap(),
                    required_item: row.get(2).unwrap(),
                    required_quest: row.get(3).unwrap(),
                    map: row.get(4).unwrap(),
                    x: row.get(5).unwrap(),
                    y: row.get(6).unwrap(),
                    z: row.get(7).unwrap(),
                    orientation: row.get(8).unwrap(),
                    failed_text: row.get(9).unwrap(),
                    heroic_keys: vec![],
                    heroic_required_quest: row.get(12).unwrap(),
                    name: row.get(13).unwrap(),
                })
            })
            .unwrap()
            .chain(quests)
            .chain(taverns)
            .map(|a| a.unwrap())
            .collect()
        }
        Expansion::WrathOfTheLichKing => {
            let mut t = conn
                .prepare(
                    "\
SELECT
    id,
    required_level,
    required_item,
    required_quest_done,
    target_map,
    target_position_x,
    target_position_y,
    target_position_z,
    target_orientation,
    heroic_key,
    heroic_key2,
    required_quest_done_heroic,
    name
FROM
    areatrigger_teleport;",
                )
                .unwrap();
            t.query_map([], |row| {
                let mut heroic_keys = vec![];
                let heroic_key = row.get(9).unwrap();
                if heroic_key != 0 {
                    heroic_keys.push(heroic_key);
                }
                let heroic_key2 = row.get(10).unwrap();
                if heroic_key2 != 0 {
                    heroic_keys.push(heroic_key2);
                }

                Ok(Trigger::Teleport {
                    id: row.get(0).unwrap(),
                    required_level: row.get(1).unwrap(),
                    required_item: row.get(2).unwrap(),
                    required_quest: row.get(3).unwrap(),
                    map: row.get(4).unwrap(),
                    x: row.get(5).unwrap(),
                    y: row.get(6).unwrap(),
                    z: row.get(7).unwrap(),
                    orientation: row.get(8).unwrap(),
                    failed_text: None,
                    heroic_keys,
                    heroic_required_quest: row.get(11).unwrap(),
                    name: row.get(12).unwrap(),
                })
            })
            .unwrap()
            .chain(quests)
            .chain(taverns)
            .map(|a| a.unwrap())
            .collect()
        }
    }
}
