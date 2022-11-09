use crate::position::{positions, RawPosition};
use crate::types::{Class, Race};
use crate::Expansion;
use rusqlite::Connection;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::Path;

pub(crate) struct Data {
    pub exp_per_level: Vec<XpPerLevel>,
    pub base_stats: HashMap<Combination, BTreeMap<u8, BaseStats>>,
    pub skills: HashMap<Combination, BTreeSet<(u32, String)>>,
    pub spells: HashMap<Combination, BTreeSet<(u32, String)>>,
    pub combinations: Vec<Combination>,
    positions: Vec<RawPosition>,
    pub actions: HashMap<Combination, BTreeSet<Action>>,
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

pub(crate) fn get_data_from_sqlite_file(sqlite_file: &Path) -> Data {
    let conn = Connection::open(sqlite_file).unwrap();

    let combinations = get_combinations(&conn);
    let exp_per_level = get_exp_data(&conn);
    let base_stats = get_stat_data(&conn);
    let skills = get_skill_data(&conn);
    let spells = get_spell_data(&conn);
    let actions = get_action_data(&conn);
    let positions = positions();

    Data {
        exp_per_level,
        base_stats,
        skills,
        spells,
        combinations,
        positions,
        actions,
    }
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
