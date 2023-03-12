use std::io::{Read, Write};

use crate::Guid;
use crate::wrath::{
    Area, Class, LfgUpdateFlag, Race,
};
use wow_world_base::shared::level_vanilla_tbc_wrath::Level;

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/smsg_update_lfg_list.wowm:50`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/smsg_update_lfg_list.wowm#L50):
/// ```text
/// struct LfgListPlayer {
///     Guid player;
///     LfgUpdateFlag flags;
///     if (flags & CHARACTER_INFO) {
///         Level level;
///         Class class;
///         Race race;
///         u8 talents0;
///         u8 talents1;
///         u8 talents2;
///         u32 armor;
///         u32 spell_damage;
///         u32 spell_heal;
///         u32 crit_rating_melee;
///         u32 crit_rating_ranged;
///         u32 crit_rating_spell;
///         f32 mana_per_5_seconds;
///         f32 mana_per_5_seconds_combat;
///         u32 attack_power;
///         u32 agility;
///         u32 health;
///         u32 mana;
///         Bool32 online;
///         u32 average_item_level;
///         u32 defense_skill;
///         u32 dodge_rating;
///         u32 block_rating;
///         u32 parry_rating;
///         u32 haste_rating;
///         u32 expertise_rating;
///     }
///     if (flags & COMMENT) {
///         CString comment;
///     }
///     if (flags & GROUP_LEADER) {
///         Bool is_looking_for_more;
///     }
///     if (flags & GROUP_GUID) {
///         Guid group;
///     }
///     if (flags & ROLES) {
///         u8 roles;
///     }
///     if (flags & AREA) {
///         Area area;
///     }
///     if (flags & STATUS) {
///         u8 unknown1;
///     }
///     Guid instance;
///     u32 encounter_mask;
/// }
/// ```
pub struct LfgListPlayer {
    pub player: Guid,
    pub flags: LfgListPlayer_LfgUpdateFlag,
    pub instance: Guid,
    pub encounter_mask: u32,
}

impl LfgListPlayer {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // player: Guid
        w.write_all(&self.player.guid().to_le_bytes())?;

        // flags: LfgUpdateFlag
        w.write_all(&u32::from(self.flags.as_int()).to_le_bytes())?;

        if let Some(if_statement) = &self.flags.character_info {
            // level: Level
            w.write_all(&if_statement.level.as_int().to_le_bytes())?;

            // class: Class
            w.write_all(&u8::from(if_statement.class.as_int()).to_le_bytes())?;

            // race: Race
            w.write_all(&u8::from(if_statement.race.as_int()).to_le_bytes())?;

            // talents0: u8
            w.write_all(&if_statement.talents0.to_le_bytes())?;

            // talents1: u8
            w.write_all(&if_statement.talents1.to_le_bytes())?;

            // talents2: u8
            w.write_all(&if_statement.talents2.to_le_bytes())?;

            // armor: u32
            w.write_all(&if_statement.armor.to_le_bytes())?;

            // spell_damage: u32
            w.write_all(&if_statement.spell_damage.to_le_bytes())?;

            // spell_heal: u32
            w.write_all(&if_statement.spell_heal.to_le_bytes())?;

            // crit_rating_melee: u32
            w.write_all(&if_statement.crit_rating_melee.to_le_bytes())?;

            // crit_rating_ranged: u32
            w.write_all(&if_statement.crit_rating_ranged.to_le_bytes())?;

            // crit_rating_spell: u32
            w.write_all(&if_statement.crit_rating_spell.to_le_bytes())?;

            // mana_per_5_seconds: f32
            w.write_all(&if_statement.mana_per_5_seconds.to_le_bytes())?;

            // mana_per_5_seconds_combat: f32
            w.write_all(&if_statement.mana_per_5_seconds_combat.to_le_bytes())?;

            // attack_power: u32
            w.write_all(&if_statement.attack_power.to_le_bytes())?;

            // agility: u32
            w.write_all(&if_statement.agility.to_le_bytes())?;

            // health: u32
            w.write_all(&if_statement.health.to_le_bytes())?;

            // mana: u32
            w.write_all(&if_statement.mana.to_le_bytes())?;

            // online: Bool32
            w.write_all(u32::from(if_statement.online).to_le_bytes().as_slice())?;

            // average_item_level: u32
            w.write_all(&if_statement.average_item_level.to_le_bytes())?;

            // defense_skill: u32
            w.write_all(&if_statement.defense_skill.to_le_bytes())?;

            // dodge_rating: u32
            w.write_all(&if_statement.dodge_rating.to_le_bytes())?;

            // block_rating: u32
            w.write_all(&if_statement.block_rating.to_le_bytes())?;

            // parry_rating: u32
            w.write_all(&if_statement.parry_rating.to_le_bytes())?;

            // haste_rating: u32
            w.write_all(&if_statement.haste_rating.to_le_bytes())?;

            // expertise_rating: u32
            w.write_all(&if_statement.expertise_rating.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.flags.comment {
            // comment: CString
            // TODO: Guard against strings that are already null-terminated
            assert_ne!(if_statement.comment.as_bytes().iter().rev().next(), Some(&0_u8), "String `comment` must not be null-terminated.");
            w.write_all(if_statement.comment.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

        }

        if let Some(if_statement) = &self.flags.group_leader {
            // is_looking_for_more: Bool
            w.write_all(u8::from(if_statement.is_looking_for_more).to_le_bytes().as_slice())?;

        }

        if let Some(if_statement) = &self.flags.group_guid {
            // group: Guid
            w.write_all(&if_statement.group.guid().to_le_bytes())?;

        }

        if let Some(if_statement) = &self.flags.roles {
            // roles: u8
            w.write_all(&if_statement.roles.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.flags.area {
            // area: Area
            w.write_all(&u32::from(if_statement.area.as_int()).to_le_bytes())?;

        }

        if let Some(if_statement) = &self.flags.status {
            // unknown1: u8
            w.write_all(&if_statement.unknown1.to_le_bytes())?;

        }

        // instance: Guid
        w.write_all(&self.instance.guid().to_le_bytes())?;

        // encounter_mask: u32
        w.write_all(&self.encounter_mask.to_le_bytes())?;

        Ok(())
    }
}

impl LfgListPlayer {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> std::result::Result<Self, crate::errors::ParseError> {
        // player: Guid
        let player = Guid::read(&mut r)?;

        // flags: LfgUpdateFlag
        let flags = LfgUpdateFlag::new(crate::util::read_u32_le(&mut r)?);

        let flags_CHARACTER_INFO = if flags.is_CHARACTER_INFO() {
            // level: Level
            let level = Level::new(crate::util::read_u8_le(&mut r)?);

            // class: Class
            let class: Class = crate::util::read_u8_le(&mut r)?.try_into()?;

            // race: Race
            let race: Race = crate::util::read_u8_le(&mut r)?.try_into()?;

            // talents0: u8
            let talents0 = crate::util::read_u8_le(&mut r)?;

            // talents1: u8
            let talents1 = crate::util::read_u8_le(&mut r)?;

            // talents2: u8
            let talents2 = crate::util::read_u8_le(&mut r)?;

            // armor: u32
            let armor = crate::util::read_u32_le(&mut r)?;

            // spell_damage: u32
            let spell_damage = crate::util::read_u32_le(&mut r)?;

            // spell_heal: u32
            let spell_heal = crate::util::read_u32_le(&mut r)?;

            // crit_rating_melee: u32
            let crit_rating_melee = crate::util::read_u32_le(&mut r)?;

            // crit_rating_ranged: u32
            let crit_rating_ranged = crate::util::read_u32_le(&mut r)?;

            // crit_rating_spell: u32
            let crit_rating_spell = crate::util::read_u32_le(&mut r)?;

            // mana_per_5_seconds: f32
            let mana_per_5_seconds = crate::util::read_f32_le(&mut r)?;

            // mana_per_5_seconds_combat: f32
            let mana_per_5_seconds_combat = crate::util::read_f32_le(&mut r)?;

            // attack_power: u32
            let attack_power = crate::util::read_u32_le(&mut r)?;

            // agility: u32
            let agility = crate::util::read_u32_le(&mut r)?;

            // health: u32
            let health = crate::util::read_u32_le(&mut r)?;

            // mana: u32
            let mana = crate::util::read_u32_le(&mut r)?;

            // online: Bool32
            let online = crate::util::read_u32_le(&mut r)? != 0;

            // average_item_level: u32
            let average_item_level = crate::util::read_u32_le(&mut r)?;

            // defense_skill: u32
            let defense_skill = crate::util::read_u32_le(&mut r)?;

            // dodge_rating: u32
            let dodge_rating = crate::util::read_u32_le(&mut r)?;

            // block_rating: u32
            let block_rating = crate::util::read_u32_le(&mut r)?;

            // parry_rating: u32
            let parry_rating = crate::util::read_u32_le(&mut r)?;

            // haste_rating: u32
            let haste_rating = crate::util::read_u32_le(&mut r)?;

            // expertise_rating: u32
            let expertise_rating = crate::util::read_u32_le(&mut r)?;

            Some(LfgListPlayer_LfgUpdateFlag_CharacterInfo {
                agility,
                armor,
                attack_power,
                average_item_level,
                block_rating,
                class,
                crit_rating_melee,
                crit_rating_ranged,
                crit_rating_spell,
                defense_skill,
                dodge_rating,
                expertise_rating,
                haste_rating,
                health,
                level,
                mana,
                mana_per_5_seconds,
                mana_per_5_seconds_combat,
                online,
                parry_rating,
                race,
                spell_damage,
                spell_heal,
                talents0,
                talents1,
                talents2,
            })
        }
        else {
            None
        };

        let flags_COMMENT = if flags.is_COMMENT() {
            // comment: CString
            let comment = {
                let comment = crate::util::read_c_string_to_vec(&mut r)?;
                String::from_utf8(comment)?
            };

            Some(LfgListPlayer_LfgUpdateFlag_Comment {
                comment,
            })
        }
        else {
            None
        };

        let flags_GROUP_LEADER = if flags.is_GROUP_LEADER() {
            // is_looking_for_more: Bool
            let is_looking_for_more = crate::util::read_u8_le(&mut r)? != 0;

            Some(LfgListPlayer_LfgUpdateFlag_GroupLeader {
                is_looking_for_more,
            })
        }
        else {
            None
        };

        let flags_GROUP_GUID = if flags.is_GROUP_GUID() {
            // group: Guid
            let group = Guid::read(&mut r)?;

            Some(LfgListPlayer_LfgUpdateFlag_GroupGuid {
                group,
            })
        }
        else {
            None
        };

        let flags_ROLES = if flags.is_ROLES() {
            // roles: u8
            let roles = crate::util::read_u8_le(&mut r)?;

            Some(LfgListPlayer_LfgUpdateFlag_Roles {
                roles,
            })
        }
        else {
            None
        };

        let flags_AREA = if flags.is_AREA() {
            // area: Area
            let area: Area = crate::util::read_u32_le(&mut r)?.try_into()?;

            Some(LfgListPlayer_LfgUpdateFlag_Area {
                area,
            })
        }
        else {
            None
        };

        let flags_STATUS = if flags.is_STATUS() {
            // unknown1: u8
            let unknown1 = crate::util::read_u8_le(&mut r)?;

            Some(LfgListPlayer_LfgUpdateFlag_Status {
                unknown1,
            })
        }
        else {
            None
        };

        // instance: Guid
        let instance = Guid::read(&mut r)?;

        // encounter_mask: u32
        let encounter_mask = crate::util::read_u32_le(&mut r)?;

        let flags = LfgListPlayer_LfgUpdateFlag {
            inner: flags.as_int(),
            character_info: flags_CHARACTER_INFO,
            comment: flags_COMMENT,
            group_leader: flags_GROUP_LEADER,
            group_guid: flags_GROUP_GUID,
            roles: flags_ROLES,
            area: flags_AREA,
            status: flags_STATUS,
        };

        Ok(Self {
            player,
            flags,
            instance,
            encounter_mask,
        })
    }

}

impl LfgListPlayer {
    pub(crate) fn size(&self) -> usize {
        8 // player: Guid
        + self.flags.size() // flags: LfgListPlayer_LfgUpdateFlag
        + 8 // instance: Guid
        + 4 // encounter_mask: u32
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct LfgListPlayer_LfgUpdateFlag {
    inner: u32,
    character_info: Option<LfgListPlayer_LfgUpdateFlag_CharacterInfo>,
    comment: Option<LfgListPlayer_LfgUpdateFlag_Comment>,
    group_leader: Option<LfgListPlayer_LfgUpdateFlag_GroupLeader>,
    group_guid: Option<LfgListPlayer_LfgUpdateFlag_GroupGuid>,
    roles: Option<LfgListPlayer_LfgUpdateFlag_Roles>,
    area: Option<LfgListPlayer_LfgUpdateFlag_Area>,
    status: Option<LfgListPlayer_LfgUpdateFlag_Status>,
}

impl LfgListPlayer_LfgUpdateFlag {
    pub const fn new(inner: u32, character_info: Option<LfgListPlayer_LfgUpdateFlag_CharacterInfo>,comment: Option<LfgListPlayer_LfgUpdateFlag_Comment>,group_leader: Option<LfgListPlayer_LfgUpdateFlag_GroupLeader>,group_guid: Option<LfgListPlayer_LfgUpdateFlag_GroupGuid>,roles: Option<LfgListPlayer_LfgUpdateFlag_Roles>,area: Option<LfgListPlayer_LfgUpdateFlag_Area>,status: Option<LfgListPlayer_LfgUpdateFlag_Status>,) -> Self {
        Self {
            inner,
            character_info, 
            comment, 
            group_leader, 
            group_guid, 
            roles, 
            area, 
            status, 
        }
    }

    pub const fn empty() -> Self {
        Self {
            inner: 0,
            character_info: None,
            comment: None,
            group_leader: None,
            group_guid: None,
            roles: None,
            area: None,
            status: None,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
        && self.character_info.is_none()
        && self.comment.is_none()
        && self.group_leader.is_none()
        && self.group_guid.is_none()
        && self.roles.is_none()
        && self.area.is_none()
        && self.status.is_none()
    }

    pub const fn new_CHARACTER_INFO(character_info: LfgListPlayer_LfgUpdateFlag_CharacterInfo) -> Self {
        Self {
            inner: LfgUpdateFlag::CHARACTER_INFO,
            character_info: Some(character_info),
            comment: None,
            group_leader: None,
            group_guid: None,
            roles: None,
            area: None,
            status: None,
        }
    }

    pub fn set_CHARACTER_INFO(mut self, character_info: LfgListPlayer_LfgUpdateFlag_CharacterInfo) -> Self {
        self.inner |= LfgUpdateFlag::CHARACTER_INFO;
        self.character_info = Some(character_info);
        self
    }

    pub const fn get_CHARACTER_INFO(&self) -> Option<&LfgListPlayer_LfgUpdateFlag_CharacterInfo> {
        self.character_info.as_ref()
    }

    pub fn clear_CHARACTER_INFO(mut self) -> Self {
        self.inner &= LfgUpdateFlag::CHARACTER_INFO.reverse_bits();
        self.character_info = None;
        self
    }

    pub const fn new_COMMENT(comment: LfgListPlayer_LfgUpdateFlag_Comment) -> Self {
        Self {
            inner: LfgUpdateFlag::COMMENT,
            character_info: None,
            comment: Some(comment),
            group_leader: None,
            group_guid: None,
            roles: None,
            area: None,
            status: None,
        }
    }

    pub fn set_COMMENT(mut self, comment: LfgListPlayer_LfgUpdateFlag_Comment) -> Self {
        self.inner |= LfgUpdateFlag::COMMENT;
        self.comment = Some(comment);
        self
    }

    pub const fn get_COMMENT(&self) -> Option<&LfgListPlayer_LfgUpdateFlag_Comment> {
        self.comment.as_ref()
    }

    pub fn clear_COMMENT(mut self) -> Self {
        self.inner &= LfgUpdateFlag::COMMENT.reverse_bits();
        self.comment = None;
        self
    }

    pub const fn new_GROUP_LEADER(group_leader: LfgListPlayer_LfgUpdateFlag_GroupLeader) -> Self {
        Self {
            inner: LfgUpdateFlag::GROUP_LEADER,
            character_info: None,
            comment: None,
            group_leader: Some(group_leader),
            group_guid: None,
            roles: None,
            area: None,
            status: None,
        }
    }

    pub fn set_GROUP_LEADER(mut self, group_leader: LfgListPlayer_LfgUpdateFlag_GroupLeader) -> Self {
        self.inner |= LfgUpdateFlag::GROUP_LEADER;
        self.group_leader = Some(group_leader);
        self
    }

    pub const fn get_GROUP_LEADER(&self) -> Option<&LfgListPlayer_LfgUpdateFlag_GroupLeader> {
        self.group_leader.as_ref()
    }

    pub fn clear_GROUP_LEADER(mut self) -> Self {
        self.inner &= LfgUpdateFlag::GROUP_LEADER.reverse_bits();
        self.group_leader = None;
        self
    }

    pub const fn new_GROUP_GUID(group_guid: LfgListPlayer_LfgUpdateFlag_GroupGuid) -> Self {
        Self {
            inner: LfgUpdateFlag::GROUP_GUID,
            character_info: None,
            comment: None,
            group_leader: None,
            group_guid: Some(group_guid),
            roles: None,
            area: None,
            status: None,
        }
    }

    pub fn set_GROUP_GUID(mut self, group_guid: LfgListPlayer_LfgUpdateFlag_GroupGuid) -> Self {
        self.inner |= LfgUpdateFlag::GROUP_GUID;
        self.group_guid = Some(group_guid);
        self
    }

    pub const fn get_GROUP_GUID(&self) -> Option<&LfgListPlayer_LfgUpdateFlag_GroupGuid> {
        self.group_guid.as_ref()
    }

    pub fn clear_GROUP_GUID(mut self) -> Self {
        self.inner &= LfgUpdateFlag::GROUP_GUID.reverse_bits();
        self.group_guid = None;
        self
    }

    pub const fn new_ROLES(roles: LfgListPlayer_LfgUpdateFlag_Roles) -> Self {
        Self {
            inner: LfgUpdateFlag::ROLES,
            character_info: None,
            comment: None,
            group_leader: None,
            group_guid: None,
            roles: Some(roles),
            area: None,
            status: None,
        }
    }

    pub fn set_ROLES(mut self, roles: LfgListPlayer_LfgUpdateFlag_Roles) -> Self {
        self.inner |= LfgUpdateFlag::ROLES;
        self.roles = Some(roles);
        self
    }

    pub const fn get_ROLES(&self) -> Option<&LfgListPlayer_LfgUpdateFlag_Roles> {
        self.roles.as_ref()
    }

    pub fn clear_ROLES(mut self) -> Self {
        self.inner &= LfgUpdateFlag::ROLES.reverse_bits();
        self.roles = None;
        self
    }

    pub const fn new_AREA(area: LfgListPlayer_LfgUpdateFlag_Area) -> Self {
        Self {
            inner: LfgUpdateFlag::AREA,
            character_info: None,
            comment: None,
            group_leader: None,
            group_guid: None,
            roles: None,
            area: Some(area),
            status: None,
        }
    }

    pub fn set_AREA(mut self, area: LfgListPlayer_LfgUpdateFlag_Area) -> Self {
        self.inner |= LfgUpdateFlag::AREA;
        self.area = Some(area);
        self
    }

    pub const fn get_AREA(&self) -> Option<&LfgListPlayer_LfgUpdateFlag_Area> {
        self.area.as_ref()
    }

    pub fn clear_AREA(mut self) -> Self {
        self.inner &= LfgUpdateFlag::AREA.reverse_bits();
        self.area = None;
        self
    }

    pub const fn new_STATUS(status: LfgListPlayer_LfgUpdateFlag_Status) -> Self {
        Self {
            inner: LfgUpdateFlag::STATUS,
            character_info: None,
            comment: None,
            group_leader: None,
            group_guid: None,
            roles: None,
            area: None,
            status: Some(status),
        }
    }

    pub fn set_STATUS(mut self, status: LfgListPlayer_LfgUpdateFlag_Status) -> Self {
        self.inner |= LfgUpdateFlag::STATUS;
        self.status = Some(status);
        self
    }

    pub const fn get_STATUS(&self) -> Option<&LfgListPlayer_LfgUpdateFlag_Status> {
        self.status.as_ref()
    }

    pub fn clear_STATUS(mut self) -> Self {
        self.inner &= LfgUpdateFlag::STATUS.reverse_bits();
        self.status = None;
        self
    }

    pub const fn new_BOUND() -> Self {
        Self {
            inner: LfgUpdateFlag::BOUND,
            character_info: None,
            comment: None,
            group_leader: None,
            group_guid: None,
            roles: None,
            area: None,
            status: None,
        }
    }

    pub fn set_BOUND(mut self) -> Self {
        self.inner |= LfgUpdateFlag::BOUND;
        self
    }

    pub const fn get_BOUND(&self) -> bool {
        (self.inner & LfgUpdateFlag::BOUND) != 0
    }

    pub fn clear_BOUND(mut self) -> Self {
        self.inner &= LfgUpdateFlag::BOUND.reverse_bits();
        self
    }

    pub(crate) const fn as_int(&self) -> u32 {
        self.inner
    }

}
impl LfgListPlayer_LfgUpdateFlag {
    pub(crate) fn size(&self) -> usize {
        4 // inner
        + {
            if let Some(s) = &self.character_info {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.comment {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.group_leader {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.group_guid {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.roles {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.area {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.status {
                s.size()
            } else {
                0
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct LfgListPlayer_LfgUpdateFlag_CharacterInfo {
    pub agility: u32,
    pub armor: u32,
    pub attack_power: u32,
    pub average_item_level: u32,
    pub block_rating: u32,
    pub class: Class,
    pub crit_rating_melee: u32,
    pub crit_rating_ranged: u32,
    pub crit_rating_spell: u32,
    pub defense_skill: u32,
    pub dodge_rating: u32,
    pub expertise_rating: u32,
    pub haste_rating: u32,
    pub health: u32,
    pub level: Level,
    pub mana: u32,
    pub mana_per_5_seconds: f32,
    pub mana_per_5_seconds_combat: f32,
    pub online: bool,
    pub parry_rating: u32,
    pub race: Race,
    pub spell_damage: u32,
    pub spell_heal: u32,
    pub talents0: u8,
    pub talents1: u8,
    pub talents2: u8,
}

impl LfgListPlayer_LfgUpdateFlag_CharacterInfo {
    pub(crate) fn size(&self) -> usize {
        4 // agility: u32
        + 4 // armor: u32
        + 4 // attack_power: u32
        + 4 // average_item_level: u32
        + 4 // block_rating: u32
        + 1 // class: Class
        + 4 // crit_rating_melee: u32
        + 4 // crit_rating_ranged: u32
        + 4 // crit_rating_spell: u32
        + 4 // defense_skill: u32
        + 4 // dodge_rating: u32
        + 4 // expertise_rating: u32
        + 4 // haste_rating: u32
        + 4 // health: u32
        + 1 // level: Level
        + 4 // mana: u32
        + 4 // mana_per_5_seconds: f32
        + 4 // mana_per_5_seconds_combat: f32
        + 4 // online: Bool32
        + 4 // parry_rating: u32
        + 1 // race: Race
        + 4 // spell_damage: u32
        + 4 // spell_heal: u32
        + 1 // talents0: u8
        + 1 // talents1: u8
        + 1 // talents2: u8
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct LfgListPlayer_LfgUpdateFlag_Comment {
    pub comment: String,
}

impl LfgListPlayer_LfgUpdateFlag_Comment {
    pub(crate) fn size(&self) -> usize {
        self.comment.len() + 1 // comment: CString
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct LfgListPlayer_LfgUpdateFlag_GroupLeader {
    pub is_looking_for_more: bool,
}

impl LfgListPlayer_LfgUpdateFlag_GroupLeader {
    pub(crate) fn size(&self) -> usize {
        1 // is_looking_for_more: Bool
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct LfgListPlayer_LfgUpdateFlag_GroupGuid {
    pub group: Guid,
}

impl LfgListPlayer_LfgUpdateFlag_GroupGuid {
    pub(crate) fn size(&self) -> usize {
        8 // group: Guid
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct LfgListPlayer_LfgUpdateFlag_Roles {
    pub roles: u8,
}

impl LfgListPlayer_LfgUpdateFlag_Roles {
    pub(crate) fn size(&self) -> usize {
        1 // roles: u8
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct LfgListPlayer_LfgUpdateFlag_Area {
    pub area: Area,
}

impl LfgListPlayer_LfgUpdateFlag_Area {
    pub(crate) fn size(&self) -> usize {
        4 // area: Area
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct LfgListPlayer_LfgUpdateFlag_Status {
    pub unknown1: u8,
}

impl LfgListPlayer_LfgUpdateFlag_Status {
    pub(crate) fn size(&self) -> usize {
        1 // unknown1: u8
    }
}

