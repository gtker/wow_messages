use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::{
    Area, AuraMask, GroupMemberOnlineStatus, GroupUpdateFlags, Power,
};
use wow_world_base::shared::level_vanilla_tbc_wrath::Level;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_party_member_stats_full.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_party_member_stats_full.wowm#L1):
/// ```text
/// smsg SMSG_PARTY_MEMBER_STATS_FULL = 0x02F2 {
///     PackedGuid player;
///     GroupUpdateFlags mask;
///     if (mask & STATUS) {
///         GroupMemberOnlineStatus status;
///     }
///     if (mask & CUR_HP) {
///         u16 current_health;
///     }
///     if (mask & MAX_HP) {
///         u16 max_health;
///     }
///     if (mask & POWER_TYPE) {
///         Power power;
///     }
///     if (mask & CUR_POWER) {
///         u16 current_power;
///     }
///     if (mask & MAX_POWER) {
///         u16 max_power;
///     }
///     if (mask & LEVEL) {
///         Level16 level;
///     }
///     if (mask & ZONE) {
///         Area area;
///     }
///     if (mask & POSITION) {
///         u16 position_x;
///         u16 position_y;
///     }
///     if (mask & AURAS) {
///         AuraMask auras;
///     }
///     if (mask & PET_GUID) {
///         Guid pet;
///     }
///     if (mask & PET_NAME) {
///         CString pet_name;
///     }
///     if (mask & PET_MODEL_ID) {
///         u16 pet_display_id;
///     }
///     if (mask & PET_CUR_HP) {
///         u16 pet_current_health;
///     }
///     if (mask & PET_MAX_HP) {
///         u16 pet_max_health;
///     }
///     if (mask & PET_POWER_TYPE) {
///         Power pet_power_type;
///     }
///     if (mask & PET_CUR_POWER) {
///         u16 pet_current_power;
///     }
///     if (mask & PET_MAX_POWER) {
///         u16 pet_max_power;
///     }
///     if (mask & PET_AURAS) {
///         AuraMask pet_auras;
///     }
/// }
/// ```
pub struct SMSG_PARTY_MEMBER_STATS_FULL {
    pub player: Guid,
    pub mask: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags,
}

impl crate::private::Sealed for SMSG_PARTY_MEMBER_STATS_FULL {}
impl crate::Message for SMSG_PARTY_MEMBER_STATS_FULL {
    const OPCODE: u32 = 0x02f2;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // player: PackedGuid
        self.player.write_packed_guid_into_vec(&mut w)?;

        // mask: GroupUpdateFlags
        w.write_all(&(self.mask.as_int().to_le_bytes()))?;

        if let Some(if_statement) = &self.mask.status {
            // status: GroupMemberOnlineStatus
            w.write_all(&(if_statement.status.as_int().to_le_bytes()))?;

        }

        if let Some(if_statement) = &self.mask.cur_hp {
            // current_health: u16
            w.write_all(&if_statement.current_health.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.mask.max_hp {
            // max_health: u16
            w.write_all(&if_statement.max_health.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.mask.power_type {
            // power: Power
            w.write_all(&(if_statement.power.as_int().to_le_bytes()))?;

        }

        if let Some(if_statement) = &self.mask.cur_power {
            // current_power: u16
            w.write_all(&if_statement.current_power.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.mask.max_power {
            // max_power: u16
            w.write_all(&if_statement.max_power.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.mask.level {
            // level: Level16
            w.write_all(&u16::from(if_statement.level.as_int()).to_le_bytes())?;

        }

        if let Some(if_statement) = &self.mask.zone {
            // area: Area
            w.write_all(&(if_statement.area.as_int().to_le_bytes()))?;

        }

        if let Some(if_statement) = &self.mask.position {
            // position_x: u16
            w.write_all(&if_statement.position_x.to_le_bytes())?;

            // position_y: u16
            w.write_all(&if_statement.position_y.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.mask.auras {
            // auras: AuraMask
            if_statement.auras.write_into_vec(&mut w)?;

        }

        if let Some(if_statement) = &self.mask.pet_guid {
            // pet: Guid
            w.write_all(&if_statement.pet.guid().to_le_bytes())?;

        }

        if let Some(if_statement) = &self.mask.pet_name {
            // pet_name: CString
            // TODO: Guard against strings that are already null-terminated
            assert_ne!(if_statement.pet_name.as_bytes().iter().rev().next(), Some(&0_u8), "String `pet_name` must not be null-terminated.");
            w.write_all(if_statement.pet_name.as_bytes())?;
            // Null terminator
            w.write_all(&[0])?;

        }

        if let Some(if_statement) = &self.mask.pet_model_id {
            // pet_display_id: u16
            w.write_all(&if_statement.pet_display_id.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.mask.pet_cur_hp {
            // pet_current_health: u16
            w.write_all(&if_statement.pet_current_health.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.mask.pet_max_hp {
            // pet_max_health: u16
            w.write_all(&if_statement.pet_max_health.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.mask.pet_power_type {
            // pet_power_type: Power
            w.write_all(&(if_statement.pet_power_type.as_int().to_le_bytes()))?;

        }

        if let Some(if_statement) = &self.mask.pet_cur_power {
            // pet_current_power: u16
            w.write_all(&if_statement.pet_current_power.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.mask.pet_max_power {
            // pet_max_power: u16
            w.write_all(&if_statement.pet_max_power.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.mask.pet_auras {
            // pet_auras: AuraMask
            if_statement.pet_auras.write_into_vec(&mut w)?;

        }

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(6..=572).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02F2, size: body_size });
        }

        // player: PackedGuid
        let player = Guid::read_packed(&mut r)?;

        // mask: GroupUpdateFlags
        let mask = GroupUpdateFlags::new(crate::util::read_u32_le(&mut r)?);

        let mask_status = if mask.is_status() {
            // status: GroupMemberOnlineStatus
            let status = GroupMemberOnlineStatus::new(crate::util::read_u8_le(&mut r)?);

            Some(SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_Status {
                status,
            })
        }
        else {
            None
        };

        let mask_cur_hp = if mask.is_cur_hp() {
            // current_health: u16
            let current_health = crate::util::read_u16_le(&mut r)?;

            Some(SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_CurHp {
                current_health,
            })
        }
        else {
            None
        };

        let mask_max_hp = if mask.is_max_hp() {
            // max_health: u16
            let max_health = crate::util::read_u16_le(&mut r)?;

            Some(SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_MaxHp {
                max_health,
            })
        }
        else {
            None
        };

        let mask_power_type = if mask.is_power_type() {
            // power: Power
            let power: Power = crate::util::read_u8_le(&mut r)?.try_into()?;

            Some(SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PowerType {
                power,
            })
        }
        else {
            None
        };

        let mask_cur_power = if mask.is_cur_power() {
            // current_power: u16
            let current_power = crate::util::read_u16_le(&mut r)?;

            Some(SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_CurPower {
                current_power,
            })
        }
        else {
            None
        };

        let mask_max_power = if mask.is_max_power() {
            // max_power: u16
            let max_power = crate::util::read_u16_le(&mut r)?;

            Some(SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_MaxPower {
                max_power,
            })
        }
        else {
            None
        };

        let mask_level = if mask.is_level() {
            // level: Level16
            let level = Level::new(crate::util::read_u16_le(&mut r)? as u8);

            Some(SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_Level {
                level,
            })
        }
        else {
            None
        };

        let mask_zone = if mask.is_zone() {
            // area: Area
            let area: Area = crate::util::read_u32_le(&mut r)?.try_into()?;

            Some(SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_Zone {
                area,
            })
        }
        else {
            None
        };

        let mask_position = if mask.is_position() {
            // position_x: u16
            let position_x = crate::util::read_u16_le(&mut r)?;

            // position_y: u16
            let position_y = crate::util::read_u16_le(&mut r)?;

            Some(SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_Position {
                position_x,
                position_y,
            })
        }
        else {
            None
        };

        let mask_auras = if mask.is_auras() {
            // auras: AuraMask
            let auras = AuraMask::read(&mut r)?;

            Some(SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_Auras {
                auras,
            })
        }
        else {
            None
        };

        let mask_pet_guid = if mask.is_pet_guid() {
            // pet: Guid
            let pet = Guid::read(&mut r)?;

            Some(SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetGuid {
                pet,
            })
        }
        else {
            None
        };

        let mask_pet_name = if mask.is_pet_name() {
            // pet_name: CString
            let pet_name = {
                let pet_name = crate::util::read_c_string_to_vec(&mut r)?;
                String::from_utf8(pet_name)?
            };

            Some(SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetName {
                pet_name,
            })
        }
        else {
            None
        };

        let mask_pet_model_id = if mask.is_pet_model_id() {
            // pet_display_id: u16
            let pet_display_id = crate::util::read_u16_le(&mut r)?;

            Some(SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetModelId {
                pet_display_id,
            })
        }
        else {
            None
        };

        let mask_pet_cur_hp = if mask.is_pet_cur_hp() {
            // pet_current_health: u16
            let pet_current_health = crate::util::read_u16_le(&mut r)?;

            Some(SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetCurHp {
                pet_current_health,
            })
        }
        else {
            None
        };

        let mask_pet_max_hp = if mask.is_pet_max_hp() {
            // pet_max_health: u16
            let pet_max_health = crate::util::read_u16_le(&mut r)?;

            Some(SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetMaxHp {
                pet_max_health,
            })
        }
        else {
            None
        };

        let mask_pet_power_type = if mask.is_pet_power_type() {
            // pet_power_type: Power
            let pet_power_type: Power = crate::util::read_u8_le(&mut r)?.try_into()?;

            Some(SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetPowerType {
                pet_power_type,
            })
        }
        else {
            None
        };

        let mask_pet_cur_power = if mask.is_pet_cur_power() {
            // pet_current_power: u16
            let pet_current_power = crate::util::read_u16_le(&mut r)?;

            Some(SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetCurPower {
                pet_current_power,
            })
        }
        else {
            None
        };

        let mask_pet_max_power = if mask.is_pet_max_power() {
            // pet_max_power: u16
            let pet_max_power = crate::util::read_u16_le(&mut r)?;

            Some(SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetMaxPower {
                pet_max_power,
            })
        }
        else {
            None
        };

        let mask_pet_auras = if mask.is_pet_auras() {
            // pet_auras: AuraMask
            let pet_auras = AuraMask::read(&mut r)?;

            Some(SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetAuras {
                pet_auras,
            })
        }
        else {
            None
        };

        let mask = SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags {
            inner: mask.as_int(),
            status: mask_status,
            cur_hp: mask_cur_hp,
            max_hp: mask_max_hp,
            power_type: mask_power_type,
            cur_power: mask_cur_power,
            max_power: mask_max_power,
            level: mask_level,
            zone: mask_zone,
            position: mask_position,
            auras: mask_auras,
            pet_guid: mask_pet_guid,
            pet_name: mask_pet_name,
            pet_model_id: mask_pet_model_id,
            pet_cur_hp: mask_pet_cur_hp,
            pet_max_hp: mask_pet_max_hp,
            pet_power_type: mask_pet_power_type,
            pet_cur_power: mask_pet_cur_power,
            pet_max_power: mask_pet_max_power,
            pet_auras: mask_pet_auras,
        };

        Ok(Self {
            player,
            mask,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_PARTY_MEMBER_STATS_FULL {}

impl SMSG_PARTY_MEMBER_STATS_FULL {
    pub(crate) fn size(&self) -> usize {
        self.player.size() // player: PackedGuid
        + self.mask.size() // mask: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags {
    inner: u32,
    status: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_Status>,
    cur_hp: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_CurHp>,
    max_hp: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_MaxHp>,
    power_type: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PowerType>,
    cur_power: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_CurPower>,
    max_power: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_MaxPower>,
    level: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_Level>,
    zone: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_Zone>,
    position: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_Position>,
    auras: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_Auras>,
    pet_guid: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetGuid>,
    pet_name: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetName>,
    pet_model_id: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetModelId>,
    pet_cur_hp: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetCurHp>,
    pet_max_hp: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetMaxHp>,
    pet_power_type: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetPowerType>,
    pet_cur_power: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetCurPower>,
    pet_max_power: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetMaxPower>,
    pet_auras: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetAuras>,
}

impl SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags {
    pub const fn new(inner: u32, status: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_Status>,cur_hp: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_CurHp>,max_hp: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_MaxHp>,power_type: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PowerType>,cur_power: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_CurPower>,max_power: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_MaxPower>,level: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_Level>,zone: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_Zone>,position: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_Position>,auras: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_Auras>,pet_guid: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetGuid>,pet_name: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetName>,pet_model_id: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetModelId>,pet_cur_hp: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetCurHp>,pet_max_hp: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetMaxHp>,pet_power_type: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetPowerType>,pet_cur_power: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetCurPower>,pet_max_power: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetMaxPower>,pet_auras: Option<SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetAuras>,) -> Self {
        Self {
            inner,
            status, 
            cur_hp, 
            max_hp, 
            power_type, 
            cur_power, 
            max_power, 
            level, 
            zone, 
            position, 
            auras, 
            pet_guid, 
            pet_name, 
            pet_model_id, 
            pet_cur_hp, 
            pet_max_hp, 
            pet_power_type, 
            pet_cur_power, 
            pet_max_power, 
            pet_auras, 
        }
    }

    pub const fn empty() -> Self {
        Self {
            inner: 0,
            status: None,
            cur_hp: None,
            max_hp: None,
            power_type: None,
            cur_power: None,
            max_power: None,
            level: None,
            zone: None,
            position: None,
            auras: None,
            pet_guid: None,
            pet_name: None,
            pet_model_id: None,
            pet_cur_hp: None,
            pet_max_hp: None,
            pet_power_type: None,
            pet_cur_power: None,
            pet_max_power: None,
            pet_auras: None,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
        && self.status.is_none()
        && self.cur_hp.is_none()
        && self.max_hp.is_none()
        && self.power_type.is_none()
        && self.cur_power.is_none()
        && self.max_power.is_none()
        && self.level.is_none()
        && self.zone.is_none()
        && self.position.is_none()
        && self.auras.is_none()
        && self.pet_guid.is_none()
        && self.pet_name.is_none()
        && self.pet_model_id.is_none()
        && self.pet_cur_hp.is_none()
        && self.pet_max_hp.is_none()
        && self.pet_power_type.is_none()
        && self.pet_cur_power.is_none()
        && self.pet_max_power.is_none()
        && self.pet_auras.is_none()
    }

    pub const fn new_status(status: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_Status) -> Self {
        Self {
            inner: GroupUpdateFlags::STATUS,
            status: Some(status),
            cur_hp: None,
            max_hp: None,
            power_type: None,
            cur_power: None,
            max_power: None,
            level: None,
            zone: None,
            position: None,
            auras: None,
            pet_guid: None,
            pet_name: None,
            pet_model_id: None,
            pet_cur_hp: None,
            pet_max_hp: None,
            pet_power_type: None,
            pet_cur_power: None,
            pet_max_power: None,
            pet_auras: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_status(mut self, status: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_Status) -> Self {
        self.inner |= GroupUpdateFlags::STATUS;
        self.status = Some(status);
        self
    }

    pub const fn get_status(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_Status> {
        self.status.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_status(mut self) -> Self {
        self.inner &= GroupUpdateFlags::STATUS.reverse_bits();
        self.status = None;
        self
    }

    pub const fn new_cur_hp(cur_hp: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_CurHp) -> Self {
        Self {
            inner: GroupUpdateFlags::CUR_HP,
            status: None,
            cur_hp: Some(cur_hp),
            max_hp: None,
            power_type: None,
            cur_power: None,
            max_power: None,
            level: None,
            zone: None,
            position: None,
            auras: None,
            pet_guid: None,
            pet_name: None,
            pet_model_id: None,
            pet_cur_hp: None,
            pet_max_hp: None,
            pet_power_type: None,
            pet_cur_power: None,
            pet_max_power: None,
            pet_auras: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_cur_hp(mut self, cur_hp: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_CurHp) -> Self {
        self.inner |= GroupUpdateFlags::CUR_HP;
        self.cur_hp = Some(cur_hp);
        self
    }

    pub const fn get_cur_hp(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_CurHp> {
        self.cur_hp.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_cur_hp(mut self) -> Self {
        self.inner &= GroupUpdateFlags::CUR_HP.reverse_bits();
        self.cur_hp = None;
        self
    }

    pub const fn new_max_hp(max_hp: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_MaxHp) -> Self {
        Self {
            inner: GroupUpdateFlags::MAX_HP,
            status: None,
            cur_hp: None,
            max_hp: Some(max_hp),
            power_type: None,
            cur_power: None,
            max_power: None,
            level: None,
            zone: None,
            position: None,
            auras: None,
            pet_guid: None,
            pet_name: None,
            pet_model_id: None,
            pet_cur_hp: None,
            pet_max_hp: None,
            pet_power_type: None,
            pet_cur_power: None,
            pet_max_power: None,
            pet_auras: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_max_hp(mut self, max_hp: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_MaxHp) -> Self {
        self.inner |= GroupUpdateFlags::MAX_HP;
        self.max_hp = Some(max_hp);
        self
    }

    pub const fn get_max_hp(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_MaxHp> {
        self.max_hp.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_max_hp(mut self) -> Self {
        self.inner &= GroupUpdateFlags::MAX_HP.reverse_bits();
        self.max_hp = None;
        self
    }

    pub const fn new_power_type(power_type: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PowerType) -> Self {
        Self {
            inner: GroupUpdateFlags::POWER_TYPE,
            status: None,
            cur_hp: None,
            max_hp: None,
            power_type: Some(power_type),
            cur_power: None,
            max_power: None,
            level: None,
            zone: None,
            position: None,
            auras: None,
            pet_guid: None,
            pet_name: None,
            pet_model_id: None,
            pet_cur_hp: None,
            pet_max_hp: None,
            pet_power_type: None,
            pet_cur_power: None,
            pet_max_power: None,
            pet_auras: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_power_type(mut self, power_type: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PowerType) -> Self {
        self.inner |= GroupUpdateFlags::POWER_TYPE;
        self.power_type = Some(power_type);
        self
    }

    pub const fn get_power_type(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PowerType> {
        self.power_type.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_power_type(mut self) -> Self {
        self.inner &= GroupUpdateFlags::POWER_TYPE.reverse_bits();
        self.power_type = None;
        self
    }

    pub const fn new_cur_power(cur_power: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_CurPower) -> Self {
        Self {
            inner: GroupUpdateFlags::CUR_POWER,
            status: None,
            cur_hp: None,
            max_hp: None,
            power_type: None,
            cur_power: Some(cur_power),
            max_power: None,
            level: None,
            zone: None,
            position: None,
            auras: None,
            pet_guid: None,
            pet_name: None,
            pet_model_id: None,
            pet_cur_hp: None,
            pet_max_hp: None,
            pet_power_type: None,
            pet_cur_power: None,
            pet_max_power: None,
            pet_auras: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_cur_power(mut self, cur_power: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_CurPower) -> Self {
        self.inner |= GroupUpdateFlags::CUR_POWER;
        self.cur_power = Some(cur_power);
        self
    }

    pub const fn get_cur_power(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_CurPower> {
        self.cur_power.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_cur_power(mut self) -> Self {
        self.inner &= GroupUpdateFlags::CUR_POWER.reverse_bits();
        self.cur_power = None;
        self
    }

    pub const fn new_max_power(max_power: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_MaxPower) -> Self {
        Self {
            inner: GroupUpdateFlags::MAX_POWER,
            status: None,
            cur_hp: None,
            max_hp: None,
            power_type: None,
            cur_power: None,
            max_power: Some(max_power),
            level: None,
            zone: None,
            position: None,
            auras: None,
            pet_guid: None,
            pet_name: None,
            pet_model_id: None,
            pet_cur_hp: None,
            pet_max_hp: None,
            pet_power_type: None,
            pet_cur_power: None,
            pet_max_power: None,
            pet_auras: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_max_power(mut self, max_power: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_MaxPower) -> Self {
        self.inner |= GroupUpdateFlags::MAX_POWER;
        self.max_power = Some(max_power);
        self
    }

    pub const fn get_max_power(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_MaxPower> {
        self.max_power.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_max_power(mut self) -> Self {
        self.inner &= GroupUpdateFlags::MAX_POWER.reverse_bits();
        self.max_power = None;
        self
    }

    pub const fn new_level(level: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_Level) -> Self {
        Self {
            inner: GroupUpdateFlags::LEVEL,
            status: None,
            cur_hp: None,
            max_hp: None,
            power_type: None,
            cur_power: None,
            max_power: None,
            level: Some(level),
            zone: None,
            position: None,
            auras: None,
            pet_guid: None,
            pet_name: None,
            pet_model_id: None,
            pet_cur_hp: None,
            pet_max_hp: None,
            pet_power_type: None,
            pet_cur_power: None,
            pet_max_power: None,
            pet_auras: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_level(mut self, level: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_Level) -> Self {
        self.inner |= GroupUpdateFlags::LEVEL;
        self.level = Some(level);
        self
    }

    pub const fn get_level(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_Level> {
        self.level.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_level(mut self) -> Self {
        self.inner &= GroupUpdateFlags::LEVEL.reverse_bits();
        self.level = None;
        self
    }

    pub const fn new_zone(zone: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_Zone) -> Self {
        Self {
            inner: GroupUpdateFlags::ZONE,
            status: None,
            cur_hp: None,
            max_hp: None,
            power_type: None,
            cur_power: None,
            max_power: None,
            level: None,
            zone: Some(zone),
            position: None,
            auras: None,
            pet_guid: None,
            pet_name: None,
            pet_model_id: None,
            pet_cur_hp: None,
            pet_max_hp: None,
            pet_power_type: None,
            pet_cur_power: None,
            pet_max_power: None,
            pet_auras: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_zone(mut self, zone: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_Zone) -> Self {
        self.inner |= GroupUpdateFlags::ZONE;
        self.zone = Some(zone);
        self
    }

    pub const fn get_zone(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_Zone> {
        self.zone.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_zone(mut self) -> Self {
        self.inner &= GroupUpdateFlags::ZONE.reverse_bits();
        self.zone = None;
        self
    }

    pub const fn new_position(position: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_Position) -> Self {
        Self {
            inner: GroupUpdateFlags::POSITION,
            status: None,
            cur_hp: None,
            max_hp: None,
            power_type: None,
            cur_power: None,
            max_power: None,
            level: None,
            zone: None,
            position: Some(position),
            auras: None,
            pet_guid: None,
            pet_name: None,
            pet_model_id: None,
            pet_cur_hp: None,
            pet_max_hp: None,
            pet_power_type: None,
            pet_cur_power: None,
            pet_max_power: None,
            pet_auras: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_position(mut self, position: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_Position) -> Self {
        self.inner |= GroupUpdateFlags::POSITION;
        self.position = Some(position);
        self
    }

    pub const fn get_position(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_Position> {
        self.position.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_position(mut self) -> Self {
        self.inner &= GroupUpdateFlags::POSITION.reverse_bits();
        self.position = None;
        self
    }

    pub const fn new_auras(auras: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_Auras) -> Self {
        Self {
            inner: GroupUpdateFlags::AURAS,
            status: None,
            cur_hp: None,
            max_hp: None,
            power_type: None,
            cur_power: None,
            max_power: None,
            level: None,
            zone: None,
            position: None,
            auras: Some(auras),
            pet_guid: None,
            pet_name: None,
            pet_model_id: None,
            pet_cur_hp: None,
            pet_max_hp: None,
            pet_power_type: None,
            pet_cur_power: None,
            pet_max_power: None,
            pet_auras: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_auras(mut self, auras: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_Auras) -> Self {
        self.inner |= GroupUpdateFlags::AURAS;
        self.auras = Some(auras);
        self
    }

    pub const fn get_auras(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_Auras> {
        self.auras.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_auras(mut self) -> Self {
        self.inner &= GroupUpdateFlags::AURAS.reverse_bits();
        self.auras = None;
        self
    }

    pub const fn new_auras_2() -> Self {
        Self {
            inner: GroupUpdateFlags::AURAS_2,
            status: None,
            cur_hp: None,
            max_hp: None,
            power_type: None,
            cur_power: None,
            max_power: None,
            level: None,
            zone: None,
            position: None,
            auras: None,
            pet_guid: None,
            pet_name: None,
            pet_model_id: None,
            pet_cur_hp: None,
            pet_max_hp: None,
            pet_power_type: None,
            pet_cur_power: None,
            pet_max_power: None,
            pet_auras: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_auras_2(mut self) -> Self {
        self.inner |= GroupUpdateFlags::AURAS_2;
        self
    }

    pub const fn get_auras_2(&self) -> bool {
        (self.inner & GroupUpdateFlags::AURAS_2) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_auras_2(mut self) -> Self {
        self.inner &= GroupUpdateFlags::AURAS_2.reverse_bits();
        self
    }

    pub const fn new_pet_guid(pet_guid: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetGuid) -> Self {
        Self {
            inner: GroupUpdateFlags::PET_GUID,
            status: None,
            cur_hp: None,
            max_hp: None,
            power_type: None,
            cur_power: None,
            max_power: None,
            level: None,
            zone: None,
            position: None,
            auras: None,
            pet_guid: Some(pet_guid),
            pet_name: None,
            pet_model_id: None,
            pet_cur_hp: None,
            pet_max_hp: None,
            pet_power_type: None,
            pet_cur_power: None,
            pet_max_power: None,
            pet_auras: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_pet_guid(mut self, pet_guid: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetGuid) -> Self {
        self.inner |= GroupUpdateFlags::PET_GUID;
        self.pet_guid = Some(pet_guid);
        self
    }

    pub const fn get_pet_guid(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetGuid> {
        self.pet_guid.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_pet_guid(mut self) -> Self {
        self.inner &= GroupUpdateFlags::PET_GUID.reverse_bits();
        self.pet_guid = None;
        self
    }

    pub const fn new_pet_name(pet_name: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetName) -> Self {
        Self {
            inner: GroupUpdateFlags::PET_NAME,
            status: None,
            cur_hp: None,
            max_hp: None,
            power_type: None,
            cur_power: None,
            max_power: None,
            level: None,
            zone: None,
            position: None,
            auras: None,
            pet_guid: None,
            pet_name: Some(pet_name),
            pet_model_id: None,
            pet_cur_hp: None,
            pet_max_hp: None,
            pet_power_type: None,
            pet_cur_power: None,
            pet_max_power: None,
            pet_auras: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_pet_name(mut self, pet_name: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetName) -> Self {
        self.inner |= GroupUpdateFlags::PET_NAME;
        self.pet_name = Some(pet_name);
        self
    }

    pub const fn get_pet_name(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetName> {
        self.pet_name.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_pet_name(mut self) -> Self {
        self.inner &= GroupUpdateFlags::PET_NAME.reverse_bits();
        self.pet_name = None;
        self
    }

    pub const fn new_pet_model_id(pet_model_id: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetModelId) -> Self {
        Self {
            inner: GroupUpdateFlags::PET_MODEL_ID,
            status: None,
            cur_hp: None,
            max_hp: None,
            power_type: None,
            cur_power: None,
            max_power: None,
            level: None,
            zone: None,
            position: None,
            auras: None,
            pet_guid: None,
            pet_name: None,
            pet_model_id: Some(pet_model_id),
            pet_cur_hp: None,
            pet_max_hp: None,
            pet_power_type: None,
            pet_cur_power: None,
            pet_max_power: None,
            pet_auras: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_pet_model_id(mut self, pet_model_id: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetModelId) -> Self {
        self.inner |= GroupUpdateFlags::PET_MODEL_ID;
        self.pet_model_id = Some(pet_model_id);
        self
    }

    pub const fn get_pet_model_id(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetModelId> {
        self.pet_model_id.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_pet_model_id(mut self) -> Self {
        self.inner &= GroupUpdateFlags::PET_MODEL_ID.reverse_bits();
        self.pet_model_id = None;
        self
    }

    pub const fn new_pet_cur_hp(pet_cur_hp: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetCurHp) -> Self {
        Self {
            inner: GroupUpdateFlags::PET_CUR_HP,
            status: None,
            cur_hp: None,
            max_hp: None,
            power_type: None,
            cur_power: None,
            max_power: None,
            level: None,
            zone: None,
            position: None,
            auras: None,
            pet_guid: None,
            pet_name: None,
            pet_model_id: None,
            pet_cur_hp: Some(pet_cur_hp),
            pet_max_hp: None,
            pet_power_type: None,
            pet_cur_power: None,
            pet_max_power: None,
            pet_auras: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_pet_cur_hp(mut self, pet_cur_hp: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetCurHp) -> Self {
        self.inner |= GroupUpdateFlags::PET_CUR_HP;
        self.pet_cur_hp = Some(pet_cur_hp);
        self
    }

    pub const fn get_pet_cur_hp(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetCurHp> {
        self.pet_cur_hp.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_pet_cur_hp(mut self) -> Self {
        self.inner &= GroupUpdateFlags::PET_CUR_HP.reverse_bits();
        self.pet_cur_hp = None;
        self
    }

    pub const fn new_pet_max_hp(pet_max_hp: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetMaxHp) -> Self {
        Self {
            inner: GroupUpdateFlags::PET_MAX_HP,
            status: None,
            cur_hp: None,
            max_hp: None,
            power_type: None,
            cur_power: None,
            max_power: None,
            level: None,
            zone: None,
            position: None,
            auras: None,
            pet_guid: None,
            pet_name: None,
            pet_model_id: None,
            pet_cur_hp: None,
            pet_max_hp: Some(pet_max_hp),
            pet_power_type: None,
            pet_cur_power: None,
            pet_max_power: None,
            pet_auras: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_pet_max_hp(mut self, pet_max_hp: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetMaxHp) -> Self {
        self.inner |= GroupUpdateFlags::PET_MAX_HP;
        self.pet_max_hp = Some(pet_max_hp);
        self
    }

    pub const fn get_pet_max_hp(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetMaxHp> {
        self.pet_max_hp.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_pet_max_hp(mut self) -> Self {
        self.inner &= GroupUpdateFlags::PET_MAX_HP.reverse_bits();
        self.pet_max_hp = None;
        self
    }

    pub const fn new_pet_power_type(pet_power_type: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetPowerType) -> Self {
        Self {
            inner: GroupUpdateFlags::PET_POWER_TYPE,
            status: None,
            cur_hp: None,
            max_hp: None,
            power_type: None,
            cur_power: None,
            max_power: None,
            level: None,
            zone: None,
            position: None,
            auras: None,
            pet_guid: None,
            pet_name: None,
            pet_model_id: None,
            pet_cur_hp: None,
            pet_max_hp: None,
            pet_power_type: Some(pet_power_type),
            pet_cur_power: None,
            pet_max_power: None,
            pet_auras: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_pet_power_type(mut self, pet_power_type: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetPowerType) -> Self {
        self.inner |= GroupUpdateFlags::PET_POWER_TYPE;
        self.pet_power_type = Some(pet_power_type);
        self
    }

    pub const fn get_pet_power_type(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetPowerType> {
        self.pet_power_type.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_pet_power_type(mut self) -> Self {
        self.inner &= GroupUpdateFlags::PET_POWER_TYPE.reverse_bits();
        self.pet_power_type = None;
        self
    }

    pub const fn new_pet_cur_power(pet_cur_power: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetCurPower) -> Self {
        Self {
            inner: GroupUpdateFlags::PET_CUR_POWER,
            status: None,
            cur_hp: None,
            max_hp: None,
            power_type: None,
            cur_power: None,
            max_power: None,
            level: None,
            zone: None,
            position: None,
            auras: None,
            pet_guid: None,
            pet_name: None,
            pet_model_id: None,
            pet_cur_hp: None,
            pet_max_hp: None,
            pet_power_type: None,
            pet_cur_power: Some(pet_cur_power),
            pet_max_power: None,
            pet_auras: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_pet_cur_power(mut self, pet_cur_power: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetCurPower) -> Self {
        self.inner |= GroupUpdateFlags::PET_CUR_POWER;
        self.pet_cur_power = Some(pet_cur_power);
        self
    }

    pub const fn get_pet_cur_power(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetCurPower> {
        self.pet_cur_power.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_pet_cur_power(mut self) -> Self {
        self.inner &= GroupUpdateFlags::PET_CUR_POWER.reverse_bits();
        self.pet_cur_power = None;
        self
    }

    pub const fn new_pet_max_power(pet_max_power: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetMaxPower) -> Self {
        Self {
            inner: GroupUpdateFlags::PET_MAX_POWER,
            status: None,
            cur_hp: None,
            max_hp: None,
            power_type: None,
            cur_power: None,
            max_power: None,
            level: None,
            zone: None,
            position: None,
            auras: None,
            pet_guid: None,
            pet_name: None,
            pet_model_id: None,
            pet_cur_hp: None,
            pet_max_hp: None,
            pet_power_type: None,
            pet_cur_power: None,
            pet_max_power: Some(pet_max_power),
            pet_auras: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_pet_max_power(mut self, pet_max_power: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetMaxPower) -> Self {
        self.inner |= GroupUpdateFlags::PET_MAX_POWER;
        self.pet_max_power = Some(pet_max_power);
        self
    }

    pub const fn get_pet_max_power(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetMaxPower> {
        self.pet_max_power.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_pet_max_power(mut self) -> Self {
        self.inner &= GroupUpdateFlags::PET_MAX_POWER.reverse_bits();
        self.pet_max_power = None;
        self
    }

    pub const fn new_pet_auras(pet_auras: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetAuras) -> Self {
        Self {
            inner: GroupUpdateFlags::PET_AURAS,
            status: None,
            cur_hp: None,
            max_hp: None,
            power_type: None,
            cur_power: None,
            max_power: None,
            level: None,
            zone: None,
            position: None,
            auras: None,
            pet_guid: None,
            pet_name: None,
            pet_model_id: None,
            pet_cur_hp: None,
            pet_max_hp: None,
            pet_power_type: None,
            pet_cur_power: None,
            pet_max_power: None,
            pet_auras: Some(pet_auras),
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_pet_auras(mut self, pet_auras: SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetAuras) -> Self {
        self.inner |= GroupUpdateFlags::PET_AURAS;
        self.pet_auras = Some(pet_auras);
        self
    }

    pub const fn get_pet_auras(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetAuras> {
        self.pet_auras.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_pet_auras(mut self) -> Self {
        self.inner &= GroupUpdateFlags::PET_AURAS.reverse_bits();
        self.pet_auras = None;
        self
    }

    pub const fn new_pet_auras_2() -> Self {
        Self {
            inner: GroupUpdateFlags::PET_AURAS_2,
            status: None,
            cur_hp: None,
            max_hp: None,
            power_type: None,
            cur_power: None,
            max_power: None,
            level: None,
            zone: None,
            position: None,
            auras: None,
            pet_guid: None,
            pet_name: None,
            pet_model_id: None,
            pet_cur_hp: None,
            pet_max_hp: None,
            pet_power_type: None,
            pet_cur_power: None,
            pet_max_power: None,
            pet_auras: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_pet_auras_2(mut self) -> Self {
        self.inner |= GroupUpdateFlags::PET_AURAS_2;
        self
    }

    pub const fn get_pet_auras_2(&self) -> bool {
        (self.inner & GroupUpdateFlags::PET_AURAS_2) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_pet_auras_2(mut self) -> Self {
        self.inner &= GroupUpdateFlags::PET_AURAS_2.reverse_bits();
        self
    }

    pub const fn new_mode_offline() -> Self {
        Self {
            inner: GroupUpdateFlags::MODE_OFFLINE,
            status: None,
            cur_hp: None,
            max_hp: None,
            power_type: None,
            cur_power: None,
            max_power: None,
            level: None,
            zone: None,
            position: None,
            auras: None,
            pet_guid: None,
            pet_name: None,
            pet_model_id: None,
            pet_cur_hp: None,
            pet_max_hp: None,
            pet_power_type: None,
            pet_cur_power: None,
            pet_max_power: None,
            pet_auras: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_mode_offline(mut self) -> Self {
        self.inner |= GroupUpdateFlags::MODE_OFFLINE;
        self
    }

    pub const fn get_mode_offline(&self) -> bool {
        (self.inner & GroupUpdateFlags::MODE_OFFLINE) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_mode_offline(mut self) -> Self {
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
            if let Some(s) = &self.status {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.cur_hp {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.max_hp {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.power_type {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.cur_power {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.max_power {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.level {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.zone {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.position {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.auras {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.pet_guid {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.pet_name {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.pet_model_id {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.pet_cur_hp {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.pet_max_hp {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.pet_power_type {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.pet_cur_power {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.pet_max_power {
                s.size()
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.pet_auras {
                s.size()
            } else {
                0
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_Status {
    pub status: GroupMemberOnlineStatus,
}

impl SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_Status {
    pub(crate) const fn size(&self) -> usize {
        1 // status: GroupMemberOnlineStatus
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_CurHp {
    pub current_health: u16,
}

impl SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_CurHp {
    pub(crate) const fn size(&self) -> usize {
        2 // current_health: u16
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_MaxHp {
    pub max_health: u16,
}

impl SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_MaxHp {
    pub(crate) const fn size(&self) -> usize {
        2 // max_health: u16
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PowerType {
    pub power: Power,
}

impl SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PowerType {
    pub(crate) const fn size(&self) -> usize {
        1 // power: Power
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_CurPower {
    pub current_power: u16,
}

impl SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_CurPower {
    pub(crate) const fn size(&self) -> usize {
        2 // current_power: u16
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_MaxPower {
    pub max_power: u16,
}

impl SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_MaxPower {
    pub(crate) const fn size(&self) -> usize {
        2 // max_power: u16
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_Level {
    pub level: Level,
}

impl SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_Level {
    pub(crate) const fn size(&self) -> usize {
        2 // level: Level16
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_Zone {
    pub area: Area,
}

impl SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_Zone {
    pub(crate) const fn size(&self) -> usize {
        4 // area: Area
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_Position {
    pub position_x: u16,
    pub position_y: u16,
}

impl SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_Position {
    pub(crate) const fn size(&self) -> usize {
        2 // position_x: u16
        + 2 // position_y: u16
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_Auras {
    pub auras: AuraMask,
}

impl SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_Auras {
    pub(crate) const fn size(&self) -> usize {
        self.auras.size() // auras: AuraMask
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetGuid {
    pub pet: Guid,
}

impl SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetGuid {
    pub(crate) const fn size(&self) -> usize {
        8 // pet: Guid
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetName {
    pub pet_name: String,
}

impl SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetName {
    pub(crate) fn size(&self) -> usize {
        self.pet_name.len() + 1 // pet_name: CString
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetModelId {
    pub pet_display_id: u16,
}

impl SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetModelId {
    pub(crate) const fn size(&self) -> usize {
        2 // pet_display_id: u16
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetCurHp {
    pub pet_current_health: u16,
}

impl SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetCurHp {
    pub(crate) const fn size(&self) -> usize {
        2 // pet_current_health: u16
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetMaxHp {
    pub pet_max_health: u16,
}

impl SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetMaxHp {
    pub(crate) const fn size(&self) -> usize {
        2 // pet_max_health: u16
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetPowerType {
    pub pet_power_type: Power,
}

impl SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetPowerType {
    pub(crate) const fn size(&self) -> usize {
        1 // pet_power_type: Power
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetCurPower {
    pub pet_current_power: u16,
}

impl SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetCurPower {
    pub(crate) const fn size(&self) -> usize {
        2 // pet_current_power: u16
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetMaxPower {
    pub pet_max_power: u16,
}

impl SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetMaxPower {
    pub(crate) const fn size(&self) -> usize {
        2 // pet_max_power: u16
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetAuras {
    pub pet_auras: AuraMask,
}

impl SMSG_PARTY_MEMBER_STATS_FULL_GroupUpdateFlags_PetAuras {
    pub(crate) const fn size(&self) -> usize {
        self.pet_auras.size() // pet_auras: AuraMask
    }
}

