use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::AuraMask;
use crate::world::version_1_12::Area;
use crate::world::version_1_12::GroupMemberOnlineStatus;
use crate::world::version_1_12::GroupUpdateFlags;
use crate::world::version_1_12::power::{Power, power_try_from, power_as_int};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_party_member_stats_full.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_party_member_stats_full.wowm#L3):
/// ```text
/// smsg SMSG_PARTY_MEMBER_STATS_FULL = 0x02F2 {
///     PackedGuid player;
///     GroupUpdateFlags mask;
///     if (mask & FLAG_STATUS) {
///         GroupMemberOnlineStatus status;
///     }
///     if (mask & FLAG_CUR_HP) {
///         u16 current_health;
///     }
///     if (mask & FLAG_MAX_HP) {
///         u16 max_health;
///     }
///     if (mask & FLAG_POWER_TYPE) {
///         Power power;
///     }
///     if (mask & FLAG_CUR_POWER) {
///         u16 current_power;
///     }
///     if (mask & FLAG_MAX_POWER) {
///         u16 max_power;
///     }
///     if (mask & FLAG_LEVEL) {
///         u16 level;
///     }
///     if (mask & FLAG_ZONE) {
///         Area area;
///     }
///     if (mask & FLAG_POSITION) {
///         u16 position_x;
///         u16 position_y;
///     }
///     if (mask & FLAG_AURAS) {
///         AuraMask auras;
///     }
///     if (mask & FLAG_PET_NAME) {
///         CString pet_name;
///     }
///     if (mask & FLAG_PET_MODEL_ID) {
///         u16 pet_display_id;
///     }
///     if (mask & FLAG_PET_CUR_HP) {
///         u16 pet_current_health;
///     }
///     if (mask & FLAG_PET_MAX_HP) {
///         u16 pet_max_health;
///     }
///     if (mask & FLAG_PET_POWER_TYPE) {
///         Power pet_power_type;
///     }
///     if (mask & FLAG_PET_CUR_POWER) {
///         u16 pet_current_power;
///     }
///     if (mask & FLAG_PET_MAX_POWER) {
///         u16 pet_max_power;
///     }
///     if (mask & FLAG_PET_AURAS) {
///         AuraMask pet_auras;
///     }
/// }
/// ```
pub struct SMSG_PARTY_MEMBER_STATS_FULL {
    pub player: Guid,
    pub mask: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags,
}

impl ServerMessage for SMSG_PARTY_MEMBER_STATS_FULL {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // player: PackedGuid
        self.player.write_packed_guid_into_vec(w);

        // mask: GroupUpdateFlags
        w.write_all(&(self.mask.as_int() as u32).to_le_bytes())?;

        if let Some(if_statement) = &self.mask.flag_status {
            // status: GroupMemberOnlineStatus
            w.write_all(&(if_statement.status.as_int() as u8).to_le_bytes())?;

        }

        if let Some(if_statement) = &self.mask.flag_cur_hp {
            // current_health: u16
            w.write_all(&if_statement.current_health.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.mask.flag_max_hp {
            // max_health: u16
            w.write_all(&if_statement.max_health.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.mask.flag_power_type {
            // power: Power
            w.write_all(&(power_as_int(&if_statement.power) as u8).to_le_bytes())?;

        }

        if let Some(if_statement) = &self.mask.flag_cur_power {
            // current_power: u16
            w.write_all(&if_statement.current_power.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.mask.flag_max_power {
            // max_power: u16
            w.write_all(&if_statement.max_power.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.mask.flag_level {
            // level: u16
            w.write_all(&if_statement.level.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.mask.flag_zone {
            // area: Area
            w.write_all(&(if_statement.area.as_int() as u32).to_le_bytes())?;

        }

        if let Some(if_statement) = &self.mask.flag_position {
            // position_x: u16
            w.write_all(&if_statement.position_x.to_le_bytes())?;

            // position_y: u16
            w.write_all(&if_statement.position_y.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.mask.flag_auras {
            // auras: AuraMask
            if_statement.auras.write_into_vec(w)?;

        }

        if let Some(if_statement) = &self.mask.flag_pet_name {
            // pet_name: CString
            w.write_all(if_statement.pet_name.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

        }

        if let Some(if_statement) = &self.mask.flag_pet_model_id {
            // pet_display_id: u16
            w.write_all(&if_statement.pet_display_id.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.mask.flag_pet_cur_hp {
            // pet_current_health: u16
            w.write_all(&if_statement.pet_current_health.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.mask.flag_pet_max_hp {
            // pet_max_health: u16
            w.write_all(&if_statement.pet_max_health.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.mask.flag_pet_power_type {
            // pet_power_type: Power
            w.write_all(&(power_as_int(&if_statement.pet_power_type) as u8).to_le_bytes())?;

        }

        if let Some(if_statement) = &self.mask.flag_pet_cur_power {
            // pet_current_power: u16
            w.write_all(&if_statement.pet_current_power.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.mask.flag_pet_max_power {
            // pet_max_power: u16
            w.write_all(&if_statement.pet_max_power.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.mask.flag_pet_auras {
            // pet_auras: AuraMask
            if_statement.pet_auras.write_into_vec(w)?;

        }

        Ok(())
    }
    const OPCODE: u16 = 0x02f2;

    fn server_size(&self) -> u16 {
        (self.size() + 4) as u16
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // player: PackedGuid
        let player = Guid::read_packed(r)?;

        // mask: GroupUpdateFlags
        let mask = GroupUpdateFlags::new(crate::util::read_u32_le(r)?);

        let mask_FLAG_STATUS = if mask.is_FLAG_STATUS() {
            // status: GroupMemberOnlineStatus
            let status = GroupMemberOnlineStatus::new(crate::util::read_u8_le(r)?);

            Some(SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_STATUS {
                status,
            })
        }
        else {
            None
        };

        let mask_FLAG_CUR_HP = if mask.is_FLAG_CUR_HP() {
            // current_health: u16
            let current_health = crate::util::read_u16_le(r)?;

            Some(SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_CUR_HP {
                current_health,
            })
        }
        else {
            None
        };

        let mask_FLAG_MAX_HP = if mask.is_FLAG_MAX_HP() {
            // max_health: u16
            let max_health = crate::util::read_u16_le(r)?;

            Some(SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_MAX_HP {
                max_health,
            })
        }
        else {
            None
        };

        let mask_FLAG_POWER_TYPE = if mask.is_FLAG_POWER_TYPE() {
            // power: Power
            let power: Power = power_try_from(crate::util::read_u8_le(r)?)?;

            Some(SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_POWER_TYPE {
                power,
            })
        }
        else {
            None
        };

        let mask_FLAG_CUR_POWER = if mask.is_FLAG_CUR_POWER() {
            // current_power: u16
            let current_power = crate::util::read_u16_le(r)?;

            Some(SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_CUR_POWER {
                current_power,
            })
        }
        else {
            None
        };

        let mask_FLAG_MAX_POWER = if mask.is_FLAG_MAX_POWER() {
            // max_power: u16
            let max_power = crate::util::read_u16_le(r)?;

            Some(SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_MAX_POWER {
                max_power,
            })
        }
        else {
            None
        };

        let mask_FLAG_LEVEL = if mask.is_FLAG_LEVEL() {
            // level: u16
            let level = crate::util::read_u16_le(r)?;

            Some(SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_LEVEL {
                level,
            })
        }
        else {
            None
        };

        let mask_FLAG_ZONE = if mask.is_FLAG_ZONE() {
            // area: Area
            let area: Area = crate::util::read_u32_le(r)?.try_into()?;

            Some(SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_ZONE {
                area,
            })
        }
        else {
            None
        };

        let mask_FLAG_POSITION = if mask.is_FLAG_POSITION() {
            // position_x: u16
            let position_x = crate::util::read_u16_le(r)?;

            // position_y: u16
            let position_y = crate::util::read_u16_le(r)?;

            Some(SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_POSITION {
                position_x,
                position_y,
            })
        }
        else {
            None
        };

        let mask_FLAG_AURAS = if mask.is_FLAG_AURAS() {
            // auras: AuraMask
            let auras = AuraMask::read(r)?;

            Some(SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_AURAS {
                auras,
            })
        }
        else {
            None
        };

        let mask_FLAG_PET_NAME = if mask.is_FLAG_PET_NAME() {
            // pet_name: CString
            let pet_name = crate::util::read_c_string_to_vec(r)?;
            let pet_name = String::from_utf8(pet_name)?;

            Some(SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_NAME {
                pet_name,
            })
        }
        else {
            None
        };

        let mask_FLAG_PET_MODEL_ID = if mask.is_FLAG_PET_MODEL_ID() {
            // pet_display_id: u16
            let pet_display_id = crate::util::read_u16_le(r)?;

            Some(SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_MODEL_ID {
                pet_display_id,
            })
        }
        else {
            None
        };

        let mask_FLAG_PET_CUR_HP = if mask.is_FLAG_PET_CUR_HP() {
            // pet_current_health: u16
            let pet_current_health = crate::util::read_u16_le(r)?;

            Some(SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_CUR_HP {
                pet_current_health,
            })
        }
        else {
            None
        };

        let mask_FLAG_PET_MAX_HP = if mask.is_FLAG_PET_MAX_HP() {
            // pet_max_health: u16
            let pet_max_health = crate::util::read_u16_le(r)?;

            Some(SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_MAX_HP {
                pet_max_health,
            })
        }
        else {
            None
        };

        let mask_FLAG_PET_POWER_TYPE = if mask.is_FLAG_PET_POWER_TYPE() {
            // pet_power_type: Power
            let pet_power_type: Power = power_try_from(crate::util::read_u8_le(r)?)?;

            Some(SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_POWER_TYPE {
                pet_power_type,
            })
        }
        else {
            None
        };

        let mask_FLAG_PET_CUR_POWER = if mask.is_FLAG_PET_CUR_POWER() {
            // pet_current_power: u16
            let pet_current_power = crate::util::read_u16_le(r)?;

            Some(SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_CUR_POWER {
                pet_current_power,
            })
        }
        else {
            None
        };

        let mask_FLAG_PET_MAX_POWER = if mask.is_FLAG_PET_MAX_POWER() {
            // pet_max_power: u16
            let pet_max_power = crate::util::read_u16_le(r)?;

            Some(SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_MAX_POWER {
                pet_max_power,
            })
        }
        else {
            None
        };

        let mask_FLAG_PET_AURAS = if mask.is_FLAG_PET_AURAS() {
            // pet_auras: AuraMask
            let pet_auras = AuraMask::read(r)?;

            Some(SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_AURAS {
                pet_auras,
            })
        }
        else {
            None
        };

        let mask = SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags {
            inner: mask.as_int(),
            flag_status: mask_FLAG_STATUS,
            flag_cur_hp: mask_FLAG_CUR_HP,
            flag_max_hp: mask_FLAG_MAX_HP,
            flag_power_type: mask_FLAG_POWER_TYPE,
            flag_cur_power: mask_FLAG_CUR_POWER,
            flag_max_power: mask_FLAG_MAX_POWER,
            flag_level: mask_FLAG_LEVEL,
            flag_zone: mask_FLAG_ZONE,
            flag_position: mask_FLAG_POSITION,
            flag_auras: mask_FLAG_AURAS,
            flag_pet_name: mask_FLAG_PET_NAME,
            flag_pet_model_id: mask_FLAG_PET_MODEL_ID,
            flag_pet_cur_hp: mask_FLAG_PET_CUR_HP,
            flag_pet_max_hp: mask_FLAG_PET_MAX_HP,
            flag_pet_power_type: mask_FLAG_PET_POWER_TYPE,
            flag_pet_cur_power: mask_FLAG_PET_CUR_POWER,
            flag_pet_max_power: mask_FLAG_PET_MAX_POWER,
            flag_pet_auras: mask_FLAG_PET_AURAS,
        };

        Ok(Self {
            player,
            mask,
        })
    }

}

impl SMSG_PARTY_MEMBER_STATS_FULL {
    pub(crate) fn size(&self) -> usize {
        self.player.size() // player: Guid
        + self.mask.size() // mask: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags
    }
}

#[derive(Default, Debug, PartialEq, Clone)]
pub struct SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags {
    inner: u32,
    flag_status: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_STATUS>,
    flag_cur_hp: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_CUR_HP>,
    flag_max_hp: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_MAX_HP>,
    flag_power_type: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_POWER_TYPE>,
    flag_cur_power: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_CUR_POWER>,
    flag_max_power: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_MAX_POWER>,
    flag_level: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_LEVEL>,
    flag_zone: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_ZONE>,
    flag_position: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_POSITION>,
    flag_auras: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_AURAS>,
    flag_pet_name: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_NAME>,
    flag_pet_model_id: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_MODEL_ID>,
    flag_pet_cur_hp: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_CUR_HP>,
    flag_pet_max_hp: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_MAX_HP>,
    flag_pet_power_type: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_POWER_TYPE>,
    flag_pet_cur_power: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_CUR_POWER>,
    flag_pet_max_power: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_MAX_POWER>,
    flag_pet_auras: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_AURAS>,
}

impl SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags {
    pub const fn empty() -> Self {
        Self {
            inner: 0,
            flag_status: None,
            flag_cur_hp: None,
            flag_max_hp: None,
            flag_power_type: None,
            flag_cur_power: None,
            flag_max_power: None,
            flag_level: None,
            flag_zone: None,
            flag_position: None,
            flag_auras: None,
            flag_pet_name: None,
            flag_pet_model_id: None,
            flag_pet_cur_hp: None,
            flag_pet_max_hp: None,
            flag_pet_power_type: None,
            flag_pet_cur_power: None,
            flag_pet_max_power: None,
            flag_pet_auras: None,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
        && self.flag_status.is_none()
        && self.flag_cur_hp.is_none()
        && self.flag_max_hp.is_none()
        && self.flag_power_type.is_none()
        && self.flag_cur_power.is_none()
        && self.flag_max_power.is_none()
        && self.flag_level.is_none()
        && self.flag_zone.is_none()
        && self.flag_position.is_none()
        && self.flag_auras.is_none()
        && self.flag_pet_name.is_none()
        && self.flag_pet_model_id.is_none()
        && self.flag_pet_cur_hp.is_none()
        && self.flag_pet_max_hp.is_none()
        && self.flag_pet_power_type.is_none()
        && self.flag_pet_cur_power.is_none()
        && self.flag_pet_max_power.is_none()
        && self.flag_pet_auras.is_none()
    }

    pub const fn new_FLAG_STATUS(flag_status: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_STATUS) -> Self {
        Self {
            inner: GroupUpdateFlags::FLAG_STATUS,
            flag_status: Some(flag_status),
            flag_cur_hp: None,
            flag_max_hp: None,
            flag_power_type: None,
            flag_cur_power: None,
            flag_max_power: None,
            flag_level: None,
            flag_zone: None,
            flag_position: None,
            flag_auras: None,
            flag_pet_name: None,
            flag_pet_model_id: None,
            flag_pet_cur_hp: None,
            flag_pet_max_hp: None,
            flag_pet_power_type: None,
            flag_pet_cur_power: None,
            flag_pet_max_power: None,
            flag_pet_auras: None,
        }
    }

    pub fn set_FLAG_STATUS(&mut self, flag_status: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_STATUS) -> Self {
        self.inner |= GroupUpdateFlags::FLAG_STATUS;
        self.flag_status = Some(flag_status);
        self.clone()
    }

    pub const fn get_FLAG_STATUS(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_STATUS> {
        self.flag_status.as_ref()
    }

    pub fn clear_FLAG_STATUS(mut self) -> Self {
        self.inner &= GroupUpdateFlags::FLAG_STATUS.reverse_bits();
        self.flag_status = None;
        self
    }

    pub const fn new_FLAG_CUR_HP(flag_cur_hp: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_CUR_HP) -> Self {
        Self {
            inner: GroupUpdateFlags::FLAG_CUR_HP,
            flag_status: None,
            flag_cur_hp: Some(flag_cur_hp),
            flag_max_hp: None,
            flag_power_type: None,
            flag_cur_power: None,
            flag_max_power: None,
            flag_level: None,
            flag_zone: None,
            flag_position: None,
            flag_auras: None,
            flag_pet_name: None,
            flag_pet_model_id: None,
            flag_pet_cur_hp: None,
            flag_pet_max_hp: None,
            flag_pet_power_type: None,
            flag_pet_cur_power: None,
            flag_pet_max_power: None,
            flag_pet_auras: None,
        }
    }

    pub fn set_FLAG_CUR_HP(&mut self, flag_cur_hp: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_CUR_HP) -> Self {
        self.inner |= GroupUpdateFlags::FLAG_CUR_HP;
        self.flag_cur_hp = Some(flag_cur_hp);
        self.clone()
    }

    pub const fn get_FLAG_CUR_HP(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_CUR_HP> {
        self.flag_cur_hp.as_ref()
    }

    pub fn clear_FLAG_CUR_HP(mut self) -> Self {
        self.inner &= GroupUpdateFlags::FLAG_CUR_HP.reverse_bits();
        self.flag_cur_hp = None;
        self
    }

    pub const fn new_FLAG_MAX_HP(flag_max_hp: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_MAX_HP) -> Self {
        Self {
            inner: GroupUpdateFlags::FLAG_MAX_HP,
            flag_status: None,
            flag_cur_hp: None,
            flag_max_hp: Some(flag_max_hp),
            flag_power_type: None,
            flag_cur_power: None,
            flag_max_power: None,
            flag_level: None,
            flag_zone: None,
            flag_position: None,
            flag_auras: None,
            flag_pet_name: None,
            flag_pet_model_id: None,
            flag_pet_cur_hp: None,
            flag_pet_max_hp: None,
            flag_pet_power_type: None,
            flag_pet_cur_power: None,
            flag_pet_max_power: None,
            flag_pet_auras: None,
        }
    }

    pub fn set_FLAG_MAX_HP(&mut self, flag_max_hp: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_MAX_HP) -> Self {
        self.inner |= GroupUpdateFlags::FLAG_MAX_HP;
        self.flag_max_hp = Some(flag_max_hp);
        self.clone()
    }

    pub const fn get_FLAG_MAX_HP(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_MAX_HP> {
        self.flag_max_hp.as_ref()
    }

    pub fn clear_FLAG_MAX_HP(mut self) -> Self {
        self.inner &= GroupUpdateFlags::FLAG_MAX_HP.reverse_bits();
        self.flag_max_hp = None;
        self
    }

    pub const fn new_FLAG_POWER_TYPE(flag_power_type: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_POWER_TYPE) -> Self {
        Self {
            inner: GroupUpdateFlags::FLAG_POWER_TYPE,
            flag_status: None,
            flag_cur_hp: None,
            flag_max_hp: None,
            flag_power_type: Some(flag_power_type),
            flag_cur_power: None,
            flag_max_power: None,
            flag_level: None,
            flag_zone: None,
            flag_position: None,
            flag_auras: None,
            flag_pet_name: None,
            flag_pet_model_id: None,
            flag_pet_cur_hp: None,
            flag_pet_max_hp: None,
            flag_pet_power_type: None,
            flag_pet_cur_power: None,
            flag_pet_max_power: None,
            flag_pet_auras: None,
        }
    }

    pub fn set_FLAG_POWER_TYPE(&mut self, flag_power_type: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_POWER_TYPE) -> Self {
        self.inner |= GroupUpdateFlags::FLAG_POWER_TYPE;
        self.flag_power_type = Some(flag_power_type);
        self.clone()
    }

    pub const fn get_FLAG_POWER_TYPE(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_POWER_TYPE> {
        self.flag_power_type.as_ref()
    }

    pub fn clear_FLAG_POWER_TYPE(mut self) -> Self {
        self.inner &= GroupUpdateFlags::FLAG_POWER_TYPE.reverse_bits();
        self.flag_power_type = None;
        self
    }

    pub const fn new_FLAG_CUR_POWER(flag_cur_power: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_CUR_POWER) -> Self {
        Self {
            inner: GroupUpdateFlags::FLAG_CUR_POWER,
            flag_status: None,
            flag_cur_hp: None,
            flag_max_hp: None,
            flag_power_type: None,
            flag_cur_power: Some(flag_cur_power),
            flag_max_power: None,
            flag_level: None,
            flag_zone: None,
            flag_position: None,
            flag_auras: None,
            flag_pet_name: None,
            flag_pet_model_id: None,
            flag_pet_cur_hp: None,
            flag_pet_max_hp: None,
            flag_pet_power_type: None,
            flag_pet_cur_power: None,
            flag_pet_max_power: None,
            flag_pet_auras: None,
        }
    }

    pub fn set_FLAG_CUR_POWER(&mut self, flag_cur_power: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_CUR_POWER) -> Self {
        self.inner |= GroupUpdateFlags::FLAG_CUR_POWER;
        self.flag_cur_power = Some(flag_cur_power);
        self.clone()
    }

    pub const fn get_FLAG_CUR_POWER(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_CUR_POWER> {
        self.flag_cur_power.as_ref()
    }

    pub fn clear_FLAG_CUR_POWER(mut self) -> Self {
        self.inner &= GroupUpdateFlags::FLAG_CUR_POWER.reverse_bits();
        self.flag_cur_power = None;
        self
    }

    pub const fn new_FLAG_MAX_POWER(flag_max_power: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_MAX_POWER) -> Self {
        Self {
            inner: GroupUpdateFlags::FLAG_MAX_POWER,
            flag_status: None,
            flag_cur_hp: None,
            flag_max_hp: None,
            flag_power_type: None,
            flag_cur_power: None,
            flag_max_power: Some(flag_max_power),
            flag_level: None,
            flag_zone: None,
            flag_position: None,
            flag_auras: None,
            flag_pet_name: None,
            flag_pet_model_id: None,
            flag_pet_cur_hp: None,
            flag_pet_max_hp: None,
            flag_pet_power_type: None,
            flag_pet_cur_power: None,
            flag_pet_max_power: None,
            flag_pet_auras: None,
        }
    }

    pub fn set_FLAG_MAX_POWER(&mut self, flag_max_power: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_MAX_POWER) -> Self {
        self.inner |= GroupUpdateFlags::FLAG_MAX_POWER;
        self.flag_max_power = Some(flag_max_power);
        self.clone()
    }

    pub const fn get_FLAG_MAX_POWER(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_MAX_POWER> {
        self.flag_max_power.as_ref()
    }

    pub fn clear_FLAG_MAX_POWER(mut self) -> Self {
        self.inner &= GroupUpdateFlags::FLAG_MAX_POWER.reverse_bits();
        self.flag_max_power = None;
        self
    }

    pub const fn new_FLAG_LEVEL(flag_level: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_LEVEL) -> Self {
        Self {
            inner: GroupUpdateFlags::FLAG_LEVEL,
            flag_status: None,
            flag_cur_hp: None,
            flag_max_hp: None,
            flag_power_type: None,
            flag_cur_power: None,
            flag_max_power: None,
            flag_level: Some(flag_level),
            flag_zone: None,
            flag_position: None,
            flag_auras: None,
            flag_pet_name: None,
            flag_pet_model_id: None,
            flag_pet_cur_hp: None,
            flag_pet_max_hp: None,
            flag_pet_power_type: None,
            flag_pet_cur_power: None,
            flag_pet_max_power: None,
            flag_pet_auras: None,
        }
    }

    pub fn set_FLAG_LEVEL(&mut self, flag_level: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_LEVEL) -> Self {
        self.inner |= GroupUpdateFlags::FLAG_LEVEL;
        self.flag_level = Some(flag_level);
        self.clone()
    }

    pub const fn get_FLAG_LEVEL(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_LEVEL> {
        self.flag_level.as_ref()
    }

    pub fn clear_FLAG_LEVEL(mut self) -> Self {
        self.inner &= GroupUpdateFlags::FLAG_LEVEL.reverse_bits();
        self.flag_level = None;
        self
    }

    pub const fn new_FLAG_ZONE(flag_zone: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_ZONE) -> Self {
        Self {
            inner: GroupUpdateFlags::FLAG_ZONE,
            flag_status: None,
            flag_cur_hp: None,
            flag_max_hp: None,
            flag_power_type: None,
            flag_cur_power: None,
            flag_max_power: None,
            flag_level: None,
            flag_zone: Some(flag_zone),
            flag_position: None,
            flag_auras: None,
            flag_pet_name: None,
            flag_pet_model_id: None,
            flag_pet_cur_hp: None,
            flag_pet_max_hp: None,
            flag_pet_power_type: None,
            flag_pet_cur_power: None,
            flag_pet_max_power: None,
            flag_pet_auras: None,
        }
    }

    pub fn set_FLAG_ZONE(&mut self, flag_zone: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_ZONE) -> Self {
        self.inner |= GroupUpdateFlags::FLAG_ZONE;
        self.flag_zone = Some(flag_zone);
        self.clone()
    }

    pub const fn get_FLAG_ZONE(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_ZONE> {
        self.flag_zone.as_ref()
    }

    pub fn clear_FLAG_ZONE(mut self) -> Self {
        self.inner &= GroupUpdateFlags::FLAG_ZONE.reverse_bits();
        self.flag_zone = None;
        self
    }

    pub const fn new_FLAG_POSITION(flag_position: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_POSITION) -> Self {
        Self {
            inner: GroupUpdateFlags::FLAG_POSITION,
            flag_status: None,
            flag_cur_hp: None,
            flag_max_hp: None,
            flag_power_type: None,
            flag_cur_power: None,
            flag_max_power: None,
            flag_level: None,
            flag_zone: None,
            flag_position: Some(flag_position),
            flag_auras: None,
            flag_pet_name: None,
            flag_pet_model_id: None,
            flag_pet_cur_hp: None,
            flag_pet_max_hp: None,
            flag_pet_power_type: None,
            flag_pet_cur_power: None,
            flag_pet_max_power: None,
            flag_pet_auras: None,
        }
    }

    pub fn set_FLAG_POSITION(&mut self, flag_position: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_POSITION) -> Self {
        self.inner |= GroupUpdateFlags::FLAG_POSITION;
        self.flag_position = Some(flag_position);
        self.clone()
    }

    pub const fn get_FLAG_POSITION(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_POSITION> {
        self.flag_position.as_ref()
    }

    pub fn clear_FLAG_POSITION(mut self) -> Self {
        self.inner &= GroupUpdateFlags::FLAG_POSITION.reverse_bits();
        self.flag_position = None;
        self
    }

    pub const fn new_FLAG_AURAS(flag_auras: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_AURAS) -> Self {
        Self {
            inner: GroupUpdateFlags::FLAG_AURAS,
            flag_status: None,
            flag_cur_hp: None,
            flag_max_hp: None,
            flag_power_type: None,
            flag_cur_power: None,
            flag_max_power: None,
            flag_level: None,
            flag_zone: None,
            flag_position: None,
            flag_auras: Some(flag_auras),
            flag_pet_name: None,
            flag_pet_model_id: None,
            flag_pet_cur_hp: None,
            flag_pet_max_hp: None,
            flag_pet_power_type: None,
            flag_pet_cur_power: None,
            flag_pet_max_power: None,
            flag_pet_auras: None,
        }
    }

    pub fn set_FLAG_AURAS(&mut self, flag_auras: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_AURAS) -> Self {
        self.inner |= GroupUpdateFlags::FLAG_AURAS;
        self.flag_auras = Some(flag_auras);
        self.clone()
    }

    pub const fn get_FLAG_AURAS(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_AURAS> {
        self.flag_auras.as_ref()
    }

    pub fn clear_FLAG_AURAS(mut self) -> Self {
        self.inner &= GroupUpdateFlags::FLAG_AURAS.reverse_bits();
        self.flag_auras = None;
        self
    }

    pub const fn new_FLAG_AURAS_2() -> Self {
        Self {
            inner: GroupUpdateFlags::FLAG_AURAS_2,
            flag_status: None,
            flag_cur_hp: None,
            flag_max_hp: None,
            flag_power_type: None,
            flag_cur_power: None,
            flag_max_power: None,
            flag_level: None,
            flag_zone: None,
            flag_position: None,
            flag_auras: None,
            flag_pet_name: None,
            flag_pet_model_id: None,
            flag_pet_cur_hp: None,
            flag_pet_max_hp: None,
            flag_pet_power_type: None,
            flag_pet_cur_power: None,
            flag_pet_max_power: None,
            flag_pet_auras: None,
        }
    }

    pub fn set_FLAG_AURAS_2(&mut self) -> Self {
        self.inner |= GroupUpdateFlags::FLAG_AURAS_2;
        self.clone()
    }

    pub const fn get_FLAG_AURAS_2(&self) -> bool {
        (self.inner & GroupUpdateFlags::FLAG_AURAS_2) != 0
    }

    pub fn clear_FLAG_AURAS_2(mut self) -> Self {
        self.inner &= GroupUpdateFlags::FLAG_AURAS_2.reverse_bits();
        self
    }

    pub const fn new_FLAG_PET_GUID() -> Self {
        Self {
            inner: GroupUpdateFlags::FLAG_PET_GUID,
            flag_status: None,
            flag_cur_hp: None,
            flag_max_hp: None,
            flag_power_type: None,
            flag_cur_power: None,
            flag_max_power: None,
            flag_level: None,
            flag_zone: None,
            flag_position: None,
            flag_auras: None,
            flag_pet_name: None,
            flag_pet_model_id: None,
            flag_pet_cur_hp: None,
            flag_pet_max_hp: None,
            flag_pet_power_type: None,
            flag_pet_cur_power: None,
            flag_pet_max_power: None,
            flag_pet_auras: None,
        }
    }

    pub fn set_FLAG_PET_GUID(&mut self) -> Self {
        self.inner |= GroupUpdateFlags::FLAG_PET_GUID;
        self.clone()
    }

    pub const fn get_FLAG_PET_GUID(&self) -> bool {
        (self.inner & GroupUpdateFlags::FLAG_PET_GUID) != 0
    }

    pub fn clear_FLAG_PET_GUID(mut self) -> Self {
        self.inner &= GroupUpdateFlags::FLAG_PET_GUID.reverse_bits();
        self
    }

    pub const fn new_FLAG_PET_NAME(flag_pet_name: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_NAME) -> Self {
        Self {
            inner: GroupUpdateFlags::FLAG_PET_NAME,
            flag_status: None,
            flag_cur_hp: None,
            flag_max_hp: None,
            flag_power_type: None,
            flag_cur_power: None,
            flag_max_power: None,
            flag_level: None,
            flag_zone: None,
            flag_position: None,
            flag_auras: None,
            flag_pet_name: Some(flag_pet_name),
            flag_pet_model_id: None,
            flag_pet_cur_hp: None,
            flag_pet_max_hp: None,
            flag_pet_power_type: None,
            flag_pet_cur_power: None,
            flag_pet_max_power: None,
            flag_pet_auras: None,
        }
    }

    pub fn set_FLAG_PET_NAME(&mut self, flag_pet_name: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_NAME) -> Self {
        self.inner |= GroupUpdateFlags::FLAG_PET_NAME;
        self.flag_pet_name = Some(flag_pet_name);
        self.clone()
    }

    pub const fn get_FLAG_PET_NAME(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_NAME> {
        self.flag_pet_name.as_ref()
    }

    pub fn clear_FLAG_PET_NAME(mut self) -> Self {
        self.inner &= GroupUpdateFlags::FLAG_PET_NAME.reverse_bits();
        self.flag_pet_name = None;
        self
    }

    pub const fn new_FLAG_PET_MODEL_ID(flag_pet_model_id: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_MODEL_ID) -> Self {
        Self {
            inner: GroupUpdateFlags::FLAG_PET_MODEL_ID,
            flag_status: None,
            flag_cur_hp: None,
            flag_max_hp: None,
            flag_power_type: None,
            flag_cur_power: None,
            flag_max_power: None,
            flag_level: None,
            flag_zone: None,
            flag_position: None,
            flag_auras: None,
            flag_pet_name: None,
            flag_pet_model_id: Some(flag_pet_model_id),
            flag_pet_cur_hp: None,
            flag_pet_max_hp: None,
            flag_pet_power_type: None,
            flag_pet_cur_power: None,
            flag_pet_max_power: None,
            flag_pet_auras: None,
        }
    }

    pub fn set_FLAG_PET_MODEL_ID(&mut self, flag_pet_model_id: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_MODEL_ID) -> Self {
        self.inner |= GroupUpdateFlags::FLAG_PET_MODEL_ID;
        self.flag_pet_model_id = Some(flag_pet_model_id);
        self.clone()
    }

    pub const fn get_FLAG_PET_MODEL_ID(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_MODEL_ID> {
        self.flag_pet_model_id.as_ref()
    }

    pub fn clear_FLAG_PET_MODEL_ID(mut self) -> Self {
        self.inner &= GroupUpdateFlags::FLAG_PET_MODEL_ID.reverse_bits();
        self.flag_pet_model_id = None;
        self
    }

    pub const fn new_FLAG_PET_CUR_HP(flag_pet_cur_hp: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_CUR_HP) -> Self {
        Self {
            inner: GroupUpdateFlags::FLAG_PET_CUR_HP,
            flag_status: None,
            flag_cur_hp: None,
            flag_max_hp: None,
            flag_power_type: None,
            flag_cur_power: None,
            flag_max_power: None,
            flag_level: None,
            flag_zone: None,
            flag_position: None,
            flag_auras: None,
            flag_pet_name: None,
            flag_pet_model_id: None,
            flag_pet_cur_hp: Some(flag_pet_cur_hp),
            flag_pet_max_hp: None,
            flag_pet_power_type: None,
            flag_pet_cur_power: None,
            flag_pet_max_power: None,
            flag_pet_auras: None,
        }
    }

    pub fn set_FLAG_PET_CUR_HP(&mut self, flag_pet_cur_hp: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_CUR_HP) -> Self {
        self.inner |= GroupUpdateFlags::FLAG_PET_CUR_HP;
        self.flag_pet_cur_hp = Some(flag_pet_cur_hp);
        self.clone()
    }

    pub const fn get_FLAG_PET_CUR_HP(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_CUR_HP> {
        self.flag_pet_cur_hp.as_ref()
    }

    pub fn clear_FLAG_PET_CUR_HP(mut self) -> Self {
        self.inner &= GroupUpdateFlags::FLAG_PET_CUR_HP.reverse_bits();
        self.flag_pet_cur_hp = None;
        self
    }

    pub const fn new_FLAG_PET_MAX_HP(flag_pet_max_hp: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_MAX_HP) -> Self {
        Self {
            inner: GroupUpdateFlags::FLAG_PET_MAX_HP,
            flag_status: None,
            flag_cur_hp: None,
            flag_max_hp: None,
            flag_power_type: None,
            flag_cur_power: None,
            flag_max_power: None,
            flag_level: None,
            flag_zone: None,
            flag_position: None,
            flag_auras: None,
            flag_pet_name: None,
            flag_pet_model_id: None,
            flag_pet_cur_hp: None,
            flag_pet_max_hp: Some(flag_pet_max_hp),
            flag_pet_power_type: None,
            flag_pet_cur_power: None,
            flag_pet_max_power: None,
            flag_pet_auras: None,
        }
    }

    pub fn set_FLAG_PET_MAX_HP(&mut self, flag_pet_max_hp: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_MAX_HP) -> Self {
        self.inner |= GroupUpdateFlags::FLAG_PET_MAX_HP;
        self.flag_pet_max_hp = Some(flag_pet_max_hp);
        self.clone()
    }

    pub const fn get_FLAG_PET_MAX_HP(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_MAX_HP> {
        self.flag_pet_max_hp.as_ref()
    }

    pub fn clear_FLAG_PET_MAX_HP(mut self) -> Self {
        self.inner &= GroupUpdateFlags::FLAG_PET_MAX_HP.reverse_bits();
        self.flag_pet_max_hp = None;
        self
    }

    pub const fn new_FLAG_PET_POWER_TYPE(flag_pet_power_type: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_POWER_TYPE) -> Self {
        Self {
            inner: GroupUpdateFlags::FLAG_PET_POWER_TYPE,
            flag_status: None,
            flag_cur_hp: None,
            flag_max_hp: None,
            flag_power_type: None,
            flag_cur_power: None,
            flag_max_power: None,
            flag_level: None,
            flag_zone: None,
            flag_position: None,
            flag_auras: None,
            flag_pet_name: None,
            flag_pet_model_id: None,
            flag_pet_cur_hp: None,
            flag_pet_max_hp: None,
            flag_pet_power_type: Some(flag_pet_power_type),
            flag_pet_cur_power: None,
            flag_pet_max_power: None,
            flag_pet_auras: None,
        }
    }

    pub fn set_FLAG_PET_POWER_TYPE(&mut self, flag_pet_power_type: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_POWER_TYPE) -> Self {
        self.inner |= GroupUpdateFlags::FLAG_PET_POWER_TYPE;
        self.flag_pet_power_type = Some(flag_pet_power_type);
        self.clone()
    }

    pub const fn get_FLAG_PET_POWER_TYPE(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_POWER_TYPE> {
        self.flag_pet_power_type.as_ref()
    }

    pub fn clear_FLAG_PET_POWER_TYPE(mut self) -> Self {
        self.inner &= GroupUpdateFlags::FLAG_PET_POWER_TYPE.reverse_bits();
        self.flag_pet_power_type = None;
        self
    }

    pub const fn new_FLAG_PET_CUR_POWER(flag_pet_cur_power: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_CUR_POWER) -> Self {
        Self {
            inner: GroupUpdateFlags::FLAG_PET_CUR_POWER,
            flag_status: None,
            flag_cur_hp: None,
            flag_max_hp: None,
            flag_power_type: None,
            flag_cur_power: None,
            flag_max_power: None,
            flag_level: None,
            flag_zone: None,
            flag_position: None,
            flag_auras: None,
            flag_pet_name: None,
            flag_pet_model_id: None,
            flag_pet_cur_hp: None,
            flag_pet_max_hp: None,
            flag_pet_power_type: None,
            flag_pet_cur_power: Some(flag_pet_cur_power),
            flag_pet_max_power: None,
            flag_pet_auras: None,
        }
    }

    pub fn set_FLAG_PET_CUR_POWER(&mut self, flag_pet_cur_power: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_CUR_POWER) -> Self {
        self.inner |= GroupUpdateFlags::FLAG_PET_CUR_POWER;
        self.flag_pet_cur_power = Some(flag_pet_cur_power);
        self.clone()
    }

    pub const fn get_FLAG_PET_CUR_POWER(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_CUR_POWER> {
        self.flag_pet_cur_power.as_ref()
    }

    pub fn clear_FLAG_PET_CUR_POWER(mut self) -> Self {
        self.inner &= GroupUpdateFlags::FLAG_PET_CUR_POWER.reverse_bits();
        self.flag_pet_cur_power = None;
        self
    }

    pub const fn new_FLAG_PET_MAX_POWER(flag_pet_max_power: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_MAX_POWER) -> Self {
        Self {
            inner: GroupUpdateFlags::FLAG_PET_MAX_POWER,
            flag_status: None,
            flag_cur_hp: None,
            flag_max_hp: None,
            flag_power_type: None,
            flag_cur_power: None,
            flag_max_power: None,
            flag_level: None,
            flag_zone: None,
            flag_position: None,
            flag_auras: None,
            flag_pet_name: None,
            flag_pet_model_id: None,
            flag_pet_cur_hp: None,
            flag_pet_max_hp: None,
            flag_pet_power_type: None,
            flag_pet_cur_power: None,
            flag_pet_max_power: Some(flag_pet_max_power),
            flag_pet_auras: None,
        }
    }

    pub fn set_FLAG_PET_MAX_POWER(&mut self, flag_pet_max_power: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_MAX_POWER) -> Self {
        self.inner |= GroupUpdateFlags::FLAG_PET_MAX_POWER;
        self.flag_pet_max_power = Some(flag_pet_max_power);
        self.clone()
    }

    pub const fn get_FLAG_PET_MAX_POWER(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_MAX_POWER> {
        self.flag_pet_max_power.as_ref()
    }

    pub fn clear_FLAG_PET_MAX_POWER(mut self) -> Self {
        self.inner &= GroupUpdateFlags::FLAG_PET_MAX_POWER.reverse_bits();
        self.flag_pet_max_power = None;
        self
    }

    pub const fn new_FLAG_PET_AURAS(flag_pet_auras: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_AURAS) -> Self {
        Self {
            inner: GroupUpdateFlags::FLAG_PET_AURAS,
            flag_status: None,
            flag_cur_hp: None,
            flag_max_hp: None,
            flag_power_type: None,
            flag_cur_power: None,
            flag_max_power: None,
            flag_level: None,
            flag_zone: None,
            flag_position: None,
            flag_auras: None,
            flag_pet_name: None,
            flag_pet_model_id: None,
            flag_pet_cur_hp: None,
            flag_pet_max_hp: None,
            flag_pet_power_type: None,
            flag_pet_cur_power: None,
            flag_pet_max_power: None,
            flag_pet_auras: Some(flag_pet_auras),
        }
    }

    pub fn set_FLAG_PET_AURAS(&mut self, flag_pet_auras: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_AURAS) -> Self {
        self.inner |= GroupUpdateFlags::FLAG_PET_AURAS;
        self.flag_pet_auras = Some(flag_pet_auras);
        self.clone()
    }

    pub const fn get_FLAG_PET_AURAS(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_AURAS> {
        self.flag_pet_auras.as_ref()
    }

    pub fn clear_FLAG_PET_AURAS(mut self) -> Self {
        self.inner &= GroupUpdateFlags::FLAG_PET_AURAS.reverse_bits();
        self.flag_pet_auras = None;
        self
    }

    pub const fn new_FLAG_PET_AURAS_2() -> Self {
        Self {
            inner: GroupUpdateFlags::FLAG_PET_AURAS_2,
            flag_status: None,
            flag_cur_hp: None,
            flag_max_hp: None,
            flag_power_type: None,
            flag_cur_power: None,
            flag_max_power: None,
            flag_level: None,
            flag_zone: None,
            flag_position: None,
            flag_auras: None,
            flag_pet_name: None,
            flag_pet_model_id: None,
            flag_pet_cur_hp: None,
            flag_pet_max_hp: None,
            flag_pet_power_type: None,
            flag_pet_cur_power: None,
            flag_pet_max_power: None,
            flag_pet_auras: None,
        }
    }

    pub fn set_FLAG_PET_AURAS_2(&mut self) -> Self {
        self.inner |= GroupUpdateFlags::FLAG_PET_AURAS_2;
        self.clone()
    }

    pub const fn get_FLAG_PET_AURAS_2(&self) -> bool {
        (self.inner & GroupUpdateFlags::FLAG_PET_AURAS_2) != 0
    }

    pub fn clear_FLAG_PET_AURAS_2(mut self) -> Self {
        self.inner &= GroupUpdateFlags::FLAG_PET_AURAS_2.reverse_bits();
        self
    }

    pub const fn new_MODE_OFFLINE() -> Self {
        Self {
            inner: GroupUpdateFlags::MODE_OFFLINE,
            flag_status: None,
            flag_cur_hp: None,
            flag_max_hp: None,
            flag_power_type: None,
            flag_cur_power: None,
            flag_max_power: None,
            flag_level: None,
            flag_zone: None,
            flag_position: None,
            flag_auras: None,
            flag_pet_name: None,
            flag_pet_model_id: None,
            flag_pet_cur_hp: None,
            flag_pet_max_hp: None,
            flag_pet_power_type: None,
            flag_pet_cur_power: None,
            flag_pet_max_power: None,
            flag_pet_auras: None,
        }
    }

    pub fn set_MODE_OFFLINE(&mut self) -> Self {
        self.inner |= GroupUpdateFlags::MODE_OFFLINE;
        self.clone()
    }

    pub const fn get_MODE_OFFLINE(&self) -> bool {
        (self.inner & GroupUpdateFlags::MODE_OFFLINE) != 0
    }

    pub fn clear_MODE_OFFLINE(mut self) -> Self {
        self.inner &= GroupUpdateFlags::MODE_OFFLINE.reverse_bits();
        self
    }

    pub(crate) const fn as_int(&self) -> u32 {
        self.inner
    }

}
impl SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags {
    pub(crate) fn size(&self) -> usize {
        4 // inner
        + {
            if let Some(s) = &self.flag_status {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.flag_cur_hp {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.flag_max_hp {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.flag_power_type {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.flag_cur_power {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.flag_max_power {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.flag_level {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.flag_zone {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.flag_position {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.flag_auras {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.flag_pet_name {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.flag_pet_model_id {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.flag_pet_cur_hp {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.flag_pet_max_hp {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.flag_pet_power_type {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.flag_pet_cur_power {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.flag_pet_max_power {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.flag_pet_auras {
                s.size()
            } else {
                0
            }
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_STATUS {
    pub status: GroupMemberOnlineStatus,
}

impl SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_STATUS {
    pub(crate) fn size(&self) -> usize {
        1 // status: GroupMemberOnlineStatus
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_CUR_HP {
    pub current_health: u16,
}

impl SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_CUR_HP {
    pub(crate) fn size(&self) -> usize {
        2 // current_health: u16
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_MAX_HP {
    pub max_health: u16,
}

impl SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_MAX_HP {
    pub(crate) fn size(&self) -> usize {
        2 // max_health: u16
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_POWER_TYPE {
    pub power: Power,
}

impl SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_POWER_TYPE {
    pub(crate) fn size(&self) -> usize {
        1 // power: Power
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_CUR_POWER {
    pub current_power: u16,
}

impl SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_CUR_POWER {
    pub(crate) fn size(&self) -> usize {
        2 // current_power: u16
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_MAX_POWER {
    pub max_power: u16,
}

impl SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_MAX_POWER {
    pub(crate) fn size(&self) -> usize {
        2 // max_power: u16
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_LEVEL {
    pub level: u16,
}

impl SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_LEVEL {
    pub(crate) fn size(&self) -> usize {
        2 // level: u16
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_ZONE {
    pub area: Area,
}

impl SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_ZONE {
    pub(crate) fn size(&self) -> usize {
        4 // area: Area
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_POSITION {
    pub position_x: u16,
    pub position_y: u16,
}

impl SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_POSITION {
    pub(crate) fn size(&self) -> usize {
        2 // position_x: u16
        + 2 // position_y: u16
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_AURAS {
    pub auras: AuraMask,
}

impl SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_AURAS {
    pub(crate) fn size(&self) -> usize {
        self.auras.size() // auras: AuraMask
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_NAME {
    pub pet_name: String,
}

impl SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_NAME {
    pub(crate) fn size(&self) -> usize {
        self.pet_name.len() + 1 // pet_name: CString
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_MODEL_ID {
    pub pet_display_id: u16,
}

impl SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_MODEL_ID {
    pub(crate) fn size(&self) -> usize {
        2 // pet_display_id: u16
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_CUR_HP {
    pub pet_current_health: u16,
}

impl SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_CUR_HP {
    pub(crate) fn size(&self) -> usize {
        2 // pet_current_health: u16
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_MAX_HP {
    pub pet_max_health: u16,
}

impl SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_MAX_HP {
    pub(crate) fn size(&self) -> usize {
        2 // pet_max_health: u16
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_POWER_TYPE {
    pub pet_power_type: Power,
}

impl SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_POWER_TYPE {
    pub(crate) fn size(&self) -> usize {
        1 // pet_power_type: Power
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_CUR_POWER {
    pub pet_current_power: u16,
}

impl SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_CUR_POWER {
    pub(crate) fn size(&self) -> usize {
        2 // pet_current_power: u16
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_MAX_POWER {
    pub pet_max_power: u16,
}

impl SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_MAX_POWER {
    pub(crate) fn size(&self) -> usize {
        2 // pet_max_power: u16
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_AURAS {
    pub pet_auras: AuraMask,
}

impl SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_FLAG_PET_AURAS {
    pub(crate) fn size(&self) -> usize {
        self.pet_auras.size() // pet_auras: AuraMask
    }
}

