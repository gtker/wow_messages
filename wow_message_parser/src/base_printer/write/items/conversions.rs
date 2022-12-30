use crate::base_printer::write::items::Stats;

pub(crate) fn vanilla_stat_types_to_stats(
    stat_type1: i32,
    stat_value1: i32,
    stat_type2: i32,
    stat_value2: i32,
    stat_type3: i32,
    stat_value3: i32,
    stat_type4: i32,
    stat_value4: i32,
    stat_type5: i32,
    stat_value5: i32,
    stat_type6: i32,
    stat_value6: i32,
    stat_type7: i32,
    stat_value7: i32,
    stat_type8: i32,
    stat_value8: i32,
    stat_type9: i32,
    stat_value9: i32,
    stat_type10: i32,
    stat_value10: i32,
) -> Stats {
    let stats = [
        (stat_type1, stat_value1),
        (stat_type2, stat_value2),
        (stat_type3, stat_value3),
        (stat_type4, stat_value4),
        (stat_type5, stat_value5),
        (stat_type6, stat_value6),
        (stat_type7, stat_value7),
        (stat_type8, stat_value8),
        (stat_type9, stat_value9),
        (stat_type10, stat_value10),
    ];

    let mut b = Stats {
        strength: 0,
        agility: 0,
        stamina: 0,
        intellect: 0,
        spirit: 0,
        health: 0,
        mana: 0,
    };

    const ITEM_MOD_MANA: i32 = 0;
    const ITEM_MOD_HEALTH: i32 = 1;
    const ITEM_MOD_AGILITY: i32 = 3;
    const ITEM_MOD_STRENGTH: i32 = 4;
    const ITEM_MOD_INTELLECT: i32 = 5;
    const ITEM_MOD_SPIRIT: i32 = 6;
    const ITEM_MOD_STAMINA: i32 = 7;

    for stat in stats {
        let ty = stat.0;
        let value = stat.1;

        match ty {
            ITEM_MOD_MANA => b.mana += value,
            ITEM_MOD_HEALTH => b.health += value,
            ITEM_MOD_AGILITY => b.agility += value,
            ITEM_MOD_STRENGTH => b.strength += value,
            ITEM_MOD_INTELLECT => b.intellect += value,
            ITEM_MOD_SPIRIT => b.spirit += value,
            ITEM_MOD_STAMINA => b.stamina += value,
            v => panic!("unknown item mod for vanilla: '{}'", v),
        }
    }

    b
}
