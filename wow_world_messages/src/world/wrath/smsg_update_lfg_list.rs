use std::io::{Read, Write};

use crate::Guid;
use crate::shared::level_vanilla_tbc_wrath::Level;
use crate::wrath::{
    Area, Class, LfgListGroup, LfgListPlayer, LfgListUpdateType, LfgType, LfgUpdateFlag, 
    Race,
};

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_update_lfg_list.wowm:111`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_update_lfg_list.wowm#L111):
/// ```text
/// smsg SMSG_UPDATE_LFG_LIST = 0x0360 {
///     (u32)LfgType lfg_type;
///     u32 dungeon_id;
///     LfgListUpdateType update_type;
///     if (update_type == PARTIAL) {
///         u32 amount_of_deleted_guids;
///         Guid[amount_of_deleted_guids] deleted_guids;
///     }
///     u32 amount_of_groups;
///     u32 unknown1;
///     LfgListGroup[amount_of_groups] groups;
///     u32 amount_of_players;
///     u32 unknown2;
///     LfgListPlayer[amount_of_players] players;
/// }
/// ```
pub struct SMSG_UPDATE_LFG_LIST {
    pub lfg_type: LfgType,
    pub dungeon_id: u32,
    pub update_type: SMSG_UPDATE_LFG_LIST_LfgListUpdateType,
    /// emus set to 0.
    pub unknown1: u32,
    pub groups: Vec<LfgListGroup>,
    /// emus set to 0.
    pub unknown2: u32,
    pub players: Vec<LfgListPlayer>,
}

impl crate::private::Sealed for SMSG_UPDATE_LFG_LIST {}
impl SMSG_UPDATE_LFG_LIST {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(25..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // lfg_type: LfgType
        let lfg_type = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        // dungeon_id: u32
        let dungeon_id = crate::util::read_u32_le(&mut r)?;

        // update_type: LfgListUpdateType
        let update_type = crate::util::read_u8_le(&mut r)?.try_into()?;

        let update_type_if = match update_type {
            LfgListUpdateType::Partial => {
                // amount_of_deleted_guids: u32
                let amount_of_deleted_guids = crate::util::read_u32_le(&mut r)?;

                // deleted_guids: Guid[amount_of_deleted_guids]
                let deleted_guids = {
                    let mut deleted_guids = Vec::with_capacity(amount_of_deleted_guids as usize);
                    for _ in 0..amount_of_deleted_guids {
                        deleted_guids.push(crate::util::read_guid(&mut r)?);
                    }
                    deleted_guids
                };

                SMSG_UPDATE_LFG_LIST_LfgListUpdateType::Partial {
                    deleted_guids,
                }
            }
            LfgListUpdateType::Full => SMSG_UPDATE_LFG_LIST_LfgListUpdateType::Full,
        };

        // amount_of_groups: u32
        let amount_of_groups = crate::util::read_u32_le(&mut r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(&mut r)?;

        // groups: LfgListGroup[amount_of_groups]
        let groups = {
            let mut groups = Vec::with_capacity(amount_of_groups as usize);
            for _ in 0..amount_of_groups {
                groups.push(LfgListGroup::read(&mut r)?);
            }
            groups
        };

        // amount_of_players: u32
        let amount_of_players = crate::util::read_u32_le(&mut r)?;

        // unknown2: u32
        let unknown2 = crate::util::read_u32_le(&mut r)?;

        // players: LfgListPlayer[amount_of_players]
        let players = {
            let mut players = Vec::with_capacity(amount_of_players as usize);
            for _ in 0..amount_of_players {
                players.push(LfgListPlayer::read(&mut r)?);
            }
            players
        };

        Ok(Self {
            lfg_type,
            dungeon_id,
            update_type: update_type_if,
            unknown1,
            groups,
            unknown2,
            players,
        })
    }

}

impl crate::Message for SMSG_UPDATE_LFG_LIST {
    const OPCODE: u32 = 0x0360;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_UPDATE_LFG_LIST"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_UPDATE_LFG_LIST {{").unwrap();
        // Members
        writeln!(s, "    lfg_type = {};", self.lfg_type.as_test_case_value()).unwrap();
        writeln!(s, "    dungeon_id = {};", self.dungeon_id).unwrap();
        writeln!(s, "    update_type = {};", LfgListUpdateType::try_from(self.update_type.as_int()).unwrap().as_test_case_value()).unwrap();
        match &self.update_type {
            crate::wrath::SMSG_UPDATE_LFG_LIST_LfgListUpdateType::Partial {
                deleted_guids,
            } => {
                writeln!(s, "    amount_of_deleted_guids = {};", deleted_guids.len()).unwrap();
                write!(s, "    deleted_guids = [").unwrap();
                for v in deleted_guids.as_slice() {
                    write!(s, "{v:#08X}, ").unwrap();
                }
                writeln!(s, "];").unwrap();
            }
            _ => {}
        }

        writeln!(s, "    amount_of_groups = {};", self.groups.len()).unwrap();
        writeln!(s, "    unknown1 = {};", self.unknown1).unwrap();
        write!(s, "    groups = [").unwrap();
        for v in self.groups.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        group = {};", v.group.guid()).unwrap();
            writeln!(s, "        flags = {};", LfgUpdateFlag::new(v.flags.as_int()).as_test_case_value()).unwrap();
            if let Some(if_statement) = &v.flags.get_comment() {
                writeln!(s, "        comment = \"{}\";", if_statement.comment).unwrap();
            }

            if let Some(if_statement) = &v.flags.get_roles() {
                write!(s, "        roles = [").unwrap();
                for v in if_statement.roles.as_slice() {
                    write!(s, "{v:#04X}, ").unwrap();
                }
                writeln!(s, "];").unwrap();
            }

            writeln!(s, "        instance = {};", v.instance.guid()).unwrap();
            writeln!(s, "        encounter_mask = {};", v.encounter_mask).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();
        writeln!(s, "    amount_of_players = {};", self.players.len()).unwrap();
        writeln!(s, "    unknown2 = {};", self.unknown2).unwrap();
        write!(s, "    players = [").unwrap();
        for v in self.players.as_slice() {
            writeln!(s, "{{").unwrap();
            // Members
            writeln!(s, "        player = {};", v.player.guid()).unwrap();
            writeln!(s, "        flags = {};", LfgUpdateFlag::new(v.flags.as_int()).as_test_case_value()).unwrap();
            if let Some(if_statement) = &v.flags.get_character_info() {
                writeln!(s, "        level = {};", if_statement.level.as_int()).unwrap();
                writeln!(s, "        class = {};", if_statement.class.as_test_case_value()).unwrap();
                writeln!(s, "        race = {};", if_statement.race.as_test_case_value()).unwrap();
                writeln!(s, "        talents0 = {};", if_statement.talents0).unwrap();
                writeln!(s, "        talents1 = {};", if_statement.talents1).unwrap();
                writeln!(s, "        talents2 = {};", if_statement.talents2).unwrap();
                writeln!(s, "        armor = {};", if_statement.armor).unwrap();
                writeln!(s, "        spell_damage = {};", if_statement.spell_damage).unwrap();
                writeln!(s, "        spell_heal = {};", if_statement.spell_heal).unwrap();
                writeln!(s, "        crit_rating_melee = {};", if_statement.crit_rating_melee).unwrap();
                writeln!(s, "        crit_rating_ranged = {};", if_statement.crit_rating_ranged).unwrap();
                writeln!(s, "        crit_rating_spell = {};", if_statement.crit_rating_spell).unwrap();
                writeln!(s, "        mana_per_5_seconds = {}", if if_statement.mana_per_5_seconds.to_string().contains('.') { if_statement.mana_per_5_seconds.to_string() } else { format!("{}.0", if_statement.mana_per_5_seconds) }).unwrap();
                writeln!(s, "        mana_per_5_seconds_combat = {}", if if_statement.mana_per_5_seconds_combat.to_string().contains('.') { if_statement.mana_per_5_seconds_combat.to_string() } else { format!("{}.0", if_statement.mana_per_5_seconds_combat) }).unwrap();
                writeln!(s, "        attack_power = {};", if_statement.attack_power).unwrap();
                writeln!(s, "        agility = {};", if_statement.agility).unwrap();
                writeln!(s, "        health = {};", if_statement.health).unwrap();
                writeln!(s, "        mana = {};", if_statement.mana).unwrap();
                writeln!(s, "        online = {};", if if_statement.online { "TRUE" } else { "FALSE" }).unwrap();
                writeln!(s, "        average_item_level = {};", if_statement.average_item_level).unwrap();
                writeln!(s, "        defense_skill = {};", if_statement.defense_skill).unwrap();
                writeln!(s, "        dodge_rating = {};", if_statement.dodge_rating).unwrap();
                writeln!(s, "        block_rating = {};", if_statement.block_rating).unwrap();
                writeln!(s, "        parry_rating = {};", if_statement.parry_rating).unwrap();
                writeln!(s, "        haste_rating = {};", if_statement.haste_rating).unwrap();
                writeln!(s, "        expertise_rating = {};", if_statement.expertise_rating).unwrap();
            }

            if let Some(if_statement) = &v.flags.get_comment() {
                writeln!(s, "        comment = \"{}\";", if_statement.comment).unwrap();
            }

            if let Some(if_statement) = &v.flags.get_group_leader() {
                writeln!(s, "        is_looking_for_more = {};", if if_statement.is_looking_for_more { "TRUE" } else { "FALSE" }).unwrap();
            }

            if let Some(if_statement) = &v.flags.get_group_guid() {
                writeln!(s, "        group = {};", if_statement.group.guid()).unwrap();
            }

            if let Some(if_statement) = &v.flags.get_roles() {
                writeln!(s, "        roles = {};", if_statement.roles).unwrap();
            }

            if let Some(if_statement) = &v.flags.get_area() {
                writeln!(s, "        area = {};", if_statement.area.as_test_case_value()).unwrap();
            }

            if let Some(if_statement) = &v.flags.get_status() {
                writeln!(s, "        unknown1 = {};", if_statement.unknown1).unwrap();
            }

            writeln!(s, "        instance = {};", v.instance.guid()).unwrap();
            writeln!(s, "        encounter_mask = {};", v.encounter_mask).unwrap();

            writeln!(s, "    }},").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 864_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "lfg_type", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "dungeon_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 1, "update_type", "    ");
        match &self.update_type {
            crate::wrath::SMSG_UPDATE_LFG_LIST_LfgListUpdateType::Partial {
                deleted_guids,
            } => {
                crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_deleted_guids", "    ");
                if !deleted_guids.is_empty() {
                    writeln!(s, "    /* deleted_guids: Guid[amount_of_deleted_guids] start */").unwrap();
                    for (i, v) in deleted_guids.iter().enumerate() {
                        crate::util::write_bytes(&mut s, &mut bytes, 8, &format!("deleted_guids {i}"), "    ");
                    }
                    writeln!(s, "    /* deleted_guids: Guid[amount_of_deleted_guids] end */").unwrap();
                }
            }
            _ => {}
        }

        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_groups", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown1", "    ");
        if !self.groups.is_empty() {
            writeln!(s, "    /* groups: LfgListGroup[amount_of_groups] start */").unwrap();
            for (i, v) in self.groups.iter().enumerate() {
                writeln!(s, "    /* groups: LfgListGroup[amount_of_groups] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 8, "group", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "flags", "        ");
                if let Some(if_statement) = &v.flags.get_comment() {
                    crate::util::write_bytes(&mut s, &mut bytes, if_statement.comment.len() + 1, "comment", "        ");
                }

                if let Some(if_statement) = &v.flags.get_roles() {
                    crate::util::write_bytes(&mut s, &mut bytes, if_statement.roles.len(), "roles", "        ");
                }

                crate::util::write_bytes(&mut s, &mut bytes, 8, "instance", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "encounter_mask", "        ");
                writeln!(s, "    /* groups: LfgListGroup[amount_of_groups] {i} end */").unwrap();
            }
            writeln!(s, "    /* groups: LfgListGroup[amount_of_groups] end */").unwrap();
        }
        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_players", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "unknown2", "    ");
        if !self.players.is_empty() {
            writeln!(s, "    /* players: LfgListPlayer[amount_of_players] start */").unwrap();
            for (i, v) in self.players.iter().enumerate() {
                writeln!(s, "    /* players: LfgListPlayer[amount_of_players] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 8, "player", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "flags", "        ");
                if let Some(if_statement) = &v.flags.get_character_info() {
                    crate::util::write_bytes(&mut s, &mut bytes, 1, "level", "        ");
                    crate::util::write_bytes(&mut s, &mut bytes, 1, "class", "        ");
                    crate::util::write_bytes(&mut s, &mut bytes, 1, "race", "        ");
                    crate::util::write_bytes(&mut s, &mut bytes, 1, "talents0", "        ");
                    crate::util::write_bytes(&mut s, &mut bytes, 1, "talents1", "        ");
                    crate::util::write_bytes(&mut s, &mut bytes, 1, "talents2", "        ");
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "armor", "        ");
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "spell_damage", "        ");
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "spell_heal", "        ");
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "crit_rating_melee", "        ");
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "crit_rating_ranged", "        ");
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "crit_rating_spell", "        ");
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "mana_per_5_seconds", "        ");
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "mana_per_5_seconds_combat", "        ");
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "attack_power", "        ");
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "agility", "        ");
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "health", "        ");
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "mana", "        ");
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "online", "        ");
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "average_item_level", "        ");
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "defense_skill", "        ");
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "dodge_rating", "        ");
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "block_rating", "        ");
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "parry_rating", "        ");
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "haste_rating", "        ");
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "expertise_rating", "        ");
                }

                if let Some(if_statement) = &v.flags.get_comment() {
                    crate::util::write_bytes(&mut s, &mut bytes, if_statement.comment.len() + 1, "comment", "        ");
                }

                if let Some(if_statement) = &v.flags.get_group_leader() {
                    crate::util::write_bytes(&mut s, &mut bytes, 1, "is_looking_for_more", "        ");
                }

                if let Some(if_statement) = &v.flags.get_group_guid() {
                    crate::util::write_bytes(&mut s, &mut bytes, 8, "group", "        ");
                }

                if let Some(if_statement) = &v.flags.get_roles() {
                    crate::util::write_bytes(&mut s, &mut bytes, 1, "roles", "        ");
                }

                if let Some(if_statement) = &v.flags.get_area() {
                    crate::util::write_bytes(&mut s, &mut bytes, 4, "area", "        ");
                }

                if let Some(if_statement) = &v.flags.get_status() {
                    crate::util::write_bytes(&mut s, &mut bytes, 1, "unknown1", "        ");
                }

                crate::util::write_bytes(&mut s, &mut bytes, 8, "instance", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "encounter_mask", "        ");
                writeln!(s, "    /* players: LfgListPlayer[amount_of_players] {i} end */").unwrap();
            }
            writeln!(s, "    /* players: LfgListPlayer[amount_of_players] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // lfg_type: LfgType
        w.write_all(&u32::from(self.lfg_type.as_int()).to_le_bytes())?;

        // dungeon_id: u32
        w.write_all(&self.dungeon_id.to_le_bytes())?;

        // update_type: LfgListUpdateType
        w.write_all(&(self.update_type.as_int().to_le_bytes()))?;

        match &self.update_type {
            SMSG_UPDATE_LFG_LIST_LfgListUpdateType::Partial {
                deleted_guids,
            } => {
                // amount_of_deleted_guids: u32
                w.write_all(&(deleted_guids.len() as u32).to_le_bytes())?;

                // deleted_guids: Guid[amount_of_deleted_guids]
                for i in deleted_guids.iter() {
                    w.write_all(&i.guid().to_le_bytes())?;
                }

            }
            _ => {}
        }

        // amount_of_groups: u32
        w.write_all(&(self.groups.len() as u32).to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        // groups: LfgListGroup[amount_of_groups]
        for i in self.groups.iter() {
            i.write_into_vec(&mut w)?;
        }

        // amount_of_players: u32
        w.write_all(&(self.players.len() as u32).to_le_bytes())?;

        // unknown2: u32
        w.write_all(&self.unknown2.to_le_bytes())?;

        // players: LfgListPlayer[amount_of_players]
        for i in self.players.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(864, "SMSG_UPDATE_LFG_LIST", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_UPDATE_LFG_LIST {}

impl SMSG_UPDATE_LFG_LIST {
    pub(crate) fn size(&self) -> usize {
        4 // lfg_type: LfgType
        + 4 // dungeon_id: u32
        + self.update_type.size() // update_type: SMSG_UPDATE_LFG_LIST_LfgListUpdateType
        + 4 // amount_of_groups: u32
        + 4 // unknown1: u32
        + self.groups.iter().fold(0, |acc, x| acc + x.size()) // groups: LfgListGroup[amount_of_groups]
        + 4 // amount_of_players: u32
        + 4 // unknown2: u32
        + self.players.iter().fold(0, |acc, x| acc + x.size()) // players: LfgListPlayer[amount_of_players]
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SMSG_UPDATE_LFG_LIST_LfgListUpdateType {
    Partial {
        deleted_guids: Vec<Guid>,
    },
    Full,
}

impl Default for SMSG_UPDATE_LFG_LIST_LfgListUpdateType {
    fn default() -> Self {
        // First enumerator without any fields
        Self::Full
    }
}

impl SMSG_UPDATE_LFG_LIST_LfgListUpdateType {
    pub(crate) const fn as_int(&self) -> u8 {
        match self {
            Self::Partial { .. } => 0,
            Self::Full => 1,
        }
    }

}

impl std::fmt::Display for SMSG_UPDATE_LFG_LIST_LfgListUpdateType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Partial{ .. } => f.write_str("Partial"),
            Self::Full => f.write_str("Full"),
        }
    }
}

impl SMSG_UPDATE_LFG_LIST_LfgListUpdateType {
    pub(crate) fn size(&self) -> usize {
        match self {
            Self::Partial {
                deleted_guids,
            } => {
                1
                + 4 // amount_of_deleted_guids: u32
                + deleted_guids.len() *  8 // deleted_guids: Guid[amount_of_deleted_guids]
            }
            _ => 1,
        }
    }
}

