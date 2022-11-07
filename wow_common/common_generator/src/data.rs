use crate::types::{Class, Race};
use rusqlite::Connection;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::Path;

pub(crate) struct Data {
    pub exp_per_level: Vec<XpPerLevel>,
    pub base_stats: HashMap<(Race, Class), BTreeMap<u8, BaseStats>>,
    pub skills: HashMap<(Race, Class), BTreeSet<(u32, String)>>,
    pub spells: HashMap<(Race, Class), BTreeSet<(u32, String)>>,
    pub combinations: Vec<(Race, Class)>,
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

    Data {
        exp_per_level,
        base_stats,
        skills,
        spells,
        combinations,
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

fn get_stat_data(conn: &Connection) -> HashMap<(Race, Class), BTreeMap<u8, BaseStats>> {
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
        let race = combination.0;
        let class = combination.1;

        let stats = stats.iter().filter(|a| a.race == race && a.class == class);

        let mut s = BTreeMap::new();

        for stat in stats {
            s.insert(stat.level, stat.stats);
        }

        h.insert((race, class), s);
    }

    h
}

struct Skill {
    race_mask: u32,
    class_mask: u32,
    skill: u32,
    note: String,
}

fn get_skill_data(conn: &Connection) -> HashMap<(Race, Class), BTreeSet<(u32, String)>> {
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
        let race = combination.0;
        let class = combination.1;

        let skills = skills.iter().filter(|a| {
            let race_mask = 1 << race.as_int() - 1;
            let class_mask = 1 << class.as_int() - 1;

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

fn get_spell_data(conn: &Connection) -> HashMap<(Race, Class), BTreeSet<(u32, String)>> {
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
        let race = combination.0;
        let class = combination.1;

        let spells = spells.iter().filter(|a| a.class == class && a.race == race);

        let mut s = BTreeSet::new();
        for spell in spells {
            s.insert((spell.spell, spell.note.clone()));
        }

        h.insert(combination, s);
    }

    h
}

fn get_combinations(conn: &Connection) -> Vec<(Race, Class)> {
    let mut combinations = conn
        .prepare("SELECT DISTINCT race, class FROM player_levelstats ORDER BY race, class;")
        .unwrap();

    let combinations = combinations
        .query_map([], |row| {
            Ok((
                row.get::<usize, u8>(0).unwrap().try_into().unwrap(),
                row.get::<usize, u8>(1).unwrap().try_into().unwrap(),
            ))
        })
        .unwrap();
    let mut combinations: Vec<(Race, Class)> = combinations.map(|a| a.unwrap()).collect();
    combinations.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

    combinations
}
