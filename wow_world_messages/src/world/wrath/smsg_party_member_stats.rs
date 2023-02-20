use crate::Guid;
use crate::wrath::AuraMask;
use crate::wrath::Area;
use crate::wrath::Power;
use crate::wrath::GroupMemberOnlineStatus;
use crate::wrath::GroupUpdateFlags;
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_party_member_stats.wowm:146`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_party_member_stats.wowm#L146):
/// ```text
/// smsg SMSG_PARTY_MEMBER_STATS = 0x007E {
///     PackedGuid guid;
///     GroupUpdateFlags mask;
///     if (mask & STATUS) {
///         GroupMemberOnlineStatus status;
///     }
///     if (mask & CUR_HP) {
///         u32 current_health;
///     }
///     if (mask & MAX_HP) {
///         u32 max_health;
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
///         u16 level;
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
///         u32 pet_current_health;
///     }
///     if (mask & PET_MAX_HP) {
///         u32 pet_max_health;
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
///     if (mask & VEHICLE_SEAT) {
///         u32 transport;
///     }
/// }
/// ```
pub struct SMSG_PARTY_MEMBER_STATS {
    pub guid: Guid,
    pub mask: SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags,
}

impl crate::Message for SMSG_PARTY_MEMBER_STATS {
    const OPCODE: u32 = 0x007e;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // guid: PackedGuid
        self.guid.write_packed_guid_into_vec(w);

        // mask: GroupUpdateFlags
        w.write_all(&(self.mask.as_int() as u32).to_le_bytes())?;

        if let Some(if_statement) = &self.mask.status {
            // status: GroupMemberOnlineStatus
            w.write_all(&(if_statement.status.as_int() as u8).to_le_bytes())?;

        }

        if let Some(if_statement) = &self.mask.cur_hp {
            // current_health: u32
            w.write_all(&if_statement.current_health.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.mask.max_hp {
            // max_health: u32
            w.write_all(&if_statement.max_health.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.mask.power_type {
            // power: Power
            w.write_all(&(if_statement.power.as_int() as u8).to_le_bytes())?;

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
            // level: u16
            w.write_all(&if_statement.level.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.mask.zone {
            // area: Area
            w.write_all(&(if_statement.area.as_int() as u32).to_le_bytes())?;

        }

        if let Some(if_statement) = &self.mask.position {
            // position_x: u16
            w.write_all(&if_statement.position_x.to_le_bytes())?;

            // position_y: u16
            w.write_all(&if_statement.position_y.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.mask.auras {
            // auras: AuraMask
            if_statement.auras.write_into_vec(w)?;

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
            // pet_current_health: u32
            w.write_all(&if_statement.pet_current_health.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.mask.pet_max_hp {
            // pet_max_health: u32
            w.write_all(&if_statement.pet_max_health.to_le_bytes())?;

        }

        if let Some(if_statement) = &self.mask.pet_power_type {
            // pet_power_type: Power
            w.write_all(&(if_statement.pet_power_type.as_int() as u8).to_le_bytes())?;

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
            if_statement.pet_auras.write_into_vec(w)?;

        }

        if let Some(if_statement) = &self.mask.vehicle_seat {
            // transport: u32
            w.write_all(&if_statement.transport.to_le_bytes())?;

        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(6..=584).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x007E, size: body_size as u32 });
        }

        // guid: PackedGuid
        let guid = Guid::read_packed(r)?;

        // mask: GroupUpdateFlags
        let mask = GroupUpdateFlags::new(crate::util::read_u32_le(r)?);

        let mask_STATUS = if mask.is_STATUS() {
            // status: GroupMemberOnlineStatus
            let status = GroupMemberOnlineStatus::new(crate::util::read_u8_le(r)?);

            Some(SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_Status {
                status,
            })
        }
        else {
            None
        };

        let mask_CUR_HP = if mask.is_CUR_HP() {
            // current_health: u32
            let current_health = crate::util::read_u32_le(r)?;

            Some(SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_CurHp {
                current_health,
            })
        }
        else {
            None
        };

        let mask_MAX_HP = if mask.is_MAX_HP() {
            // max_health: u32
            let max_health = crate::util::read_u32_le(r)?;

            Some(SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_MaxHp {
                max_health,
            })
        }
        else {
            None
        };

        let mask_POWER_TYPE = if mask.is_POWER_TYPE() {
            // power: Power
            let power: Power = crate::util::read_u8_le(r)?.try_into()?;

            Some(SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PowerType {
                power,
            })
        }
        else {
            None
        };

        let mask_CUR_POWER = if mask.is_CUR_POWER() {
            // current_power: u16
            let current_power = crate::util::read_u16_le(r)?;

            Some(SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_CurPower {
                current_power,
            })
        }
        else {
            None
        };

        let mask_MAX_POWER = if mask.is_MAX_POWER() {
            // max_power: u16
            let max_power = crate::util::read_u16_le(r)?;

            Some(SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_MaxPower {
                max_power,
            })
        }
        else {
            None
        };

        let mask_LEVEL = if mask.is_LEVEL() {
            // level: u16
            let level = crate::util::read_u16_le(r)?;

            Some(SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_Level {
                level,
            })
        }
        else {
            None
        };

        let mask_ZONE = if mask.is_ZONE() {
            // area: Area
            let area: Area = crate::util::read_u32_le(r)?.try_into()?;

            Some(SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_Zone {
                area,
            })
        }
        else {
            None
        };

        let mask_POSITION = if mask.is_POSITION() {
            // position_x: u16
            let position_x = crate::util::read_u16_le(r)?;

            // position_y: u16
            let position_y = crate::util::read_u16_le(r)?;

            Some(SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_Position {
                position_x,
                position_y,
            })
        }
        else {
            None
        };

        let mask_AURAS = if mask.is_AURAS() {
            // auras: AuraMask
            let auras = AuraMask::read(r)?;

            Some(SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_Auras {
                auras,
            })
        }
        else {
            None
        };

        let mask_PET_GUID = if mask.is_PET_GUID() {
            // pet: Guid
            let pet = Guid::read(r)?;

            Some(SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetGuid {
                pet,
            })
        }
        else {
            None
        };

        let mask_PET_NAME = if mask.is_PET_NAME() {
            // pet_name: CString
            let pet_name = {
                let pet_name = crate::util::read_c_string_to_vec(r)?;
                String::from_utf8(pet_name)?
            };

            Some(SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetName {
                pet_name,
            })
        }
        else {
            None
        };

        let mask_PET_MODEL_ID = if mask.is_PET_MODEL_ID() {
            // pet_display_id: u16
            let pet_display_id = crate::util::read_u16_le(r)?;

            Some(SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetModelId {
                pet_display_id,
            })
        }
        else {
            None
        };

        let mask_PET_CUR_HP = if mask.is_PET_CUR_HP() {
            // pet_current_health: u32
            let pet_current_health = crate::util::read_u32_le(r)?;

            Some(SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetCurHp {
                pet_current_health,
            })
        }
        else {
            None
        };

        let mask_PET_MAX_HP = if mask.is_PET_MAX_HP() {
            // pet_max_health: u32
            let pet_max_health = crate::util::read_u32_le(r)?;

            Some(SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetMaxHp {
                pet_max_health,
            })
        }
        else {
            None
        };

        let mask_PET_POWER_TYPE = if mask.is_PET_POWER_TYPE() {
            // pet_power_type: Power
            let pet_power_type: Power = crate::util::read_u8_le(r)?.try_into()?;

            Some(SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetPowerType {
                pet_power_type,
            })
        }
        else {
            None
        };

        let mask_PET_CUR_POWER = if mask.is_PET_CUR_POWER() {
            // pet_current_power: u16
            let pet_current_power = crate::util::read_u16_le(r)?;

            Some(SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetCurPower {
                pet_current_power,
            })
        }
        else {
            None
        };

        let mask_PET_MAX_POWER = if mask.is_PET_MAX_POWER() {
            // pet_max_power: u16
            let pet_max_power = crate::util::read_u16_le(r)?;

            Some(SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetMaxPower {
                pet_max_power,
            })
        }
        else {
            None
        };

        let mask_PET_AURAS = if mask.is_PET_AURAS() {
            // pet_auras: AuraMask
            let pet_auras = AuraMask::read(r)?;

            Some(SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetAuras {
                pet_auras,
            })
        }
        else {
            None
        };

        let mask_VEHICLE_SEAT = if mask.is_VEHICLE_SEAT() {
            // transport: u32
            let transport = crate::util::read_u32_le(r)?;

            Some(SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_VehicleSeat {
                transport,
            })
        }
        else {
            None
        };

        let mask = SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags {
            inner: mask.as_int(),
            status: mask_STATUS,
            cur_hp: mask_CUR_HP,
            max_hp: mask_MAX_HP,
            power_type: mask_POWER_TYPE,
            cur_power: mask_CUR_POWER,
            max_power: mask_MAX_POWER,
            level: mask_LEVEL,
            zone: mask_ZONE,
            position: mask_POSITION,
            auras: mask_AURAS,
            pet_guid: mask_PET_GUID,
            pet_name: mask_PET_NAME,
            pet_model_id: mask_PET_MODEL_ID,
            pet_cur_hp: mask_PET_CUR_HP,
            pet_max_hp: mask_PET_MAX_HP,
            pet_power_type: mask_PET_POWER_TYPE,
            pet_cur_power: mask_PET_CUR_POWER,
            pet_max_power: mask_PET_MAX_POWER,
            pet_auras: mask_PET_AURAS,
            vehicle_seat: mask_VEHICLE_SEAT,
        };

        Ok(Self {
            guid,
            mask,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_PARTY_MEMBER_STATS {}

impl SMSG_PARTY_MEMBER_STATS {
    pub(crate) fn size(&self) -> usize {
        self.guid.size() // guid: Guid
        + self.mask.size() // mask: SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags {
    inner: u32,
    status: Option<SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_Status>,
    cur_hp: Option<SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_CurHp>,
    max_hp: Option<SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_MaxHp>,
    power_type: Option<SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PowerType>,
    cur_power: Option<SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_CurPower>,
    max_power: Option<SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_MaxPower>,
    level: Option<SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_Level>,
    zone: Option<SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_Zone>,
    position: Option<SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_Position>,
    auras: Option<SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_Auras>,
    pet_guid: Option<SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetGuid>,
    pet_name: Option<SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetName>,
    pet_model_id: Option<SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetModelId>,
    pet_cur_hp: Option<SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetCurHp>,
    pet_max_hp: Option<SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetMaxHp>,
    pet_power_type: Option<SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetPowerType>,
    pet_cur_power: Option<SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetCurPower>,
    pet_max_power: Option<SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetMaxPower>,
    pet_auras: Option<SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetAuras>,
    vehicle_seat: Option<SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_VehicleSeat>,
}

impl SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags {
    pub const fn new(inner: u32, status: Option<SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_Status>,cur_hp: Option<SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_CurHp>,max_hp: Option<SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_MaxHp>,power_type: Option<SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PowerType>,cur_power: Option<SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_CurPower>,max_power: Option<SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_MaxPower>,level: Option<SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_Level>,zone: Option<SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_Zone>,position: Option<SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_Position>,auras: Option<SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_Auras>,pet_guid: Option<SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetGuid>,pet_name: Option<SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetName>,pet_model_id: Option<SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetModelId>,pet_cur_hp: Option<SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetCurHp>,pet_max_hp: Option<SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetMaxHp>,pet_power_type: Option<SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetPowerType>,pet_cur_power: Option<SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetCurPower>,pet_max_power: Option<SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetMaxPower>,pet_auras: Option<SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetAuras>,vehicle_seat: Option<SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_VehicleSeat>,) -> Self {
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
            vehicle_seat, 
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
            vehicle_seat: None,
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
        && self.vehicle_seat.is_none()
    }

    pub const fn new_STATUS(status: SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_Status) -> Self {
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
            vehicle_seat: None,
        }
    }

    pub fn set_STATUS(mut self, status: SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_Status) -> Self {
        self.inner |= GroupUpdateFlags::STATUS;
        self.status = Some(status);
        self
    }

    pub const fn get_STATUS(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_Status> {
        self.status.as_ref()
    }

    pub fn clear_STATUS(mut self) -> Self {
        self.inner &= GroupUpdateFlags::STATUS.reverse_bits();
        self.status = None;
        self
    }

    pub const fn new_CUR_HP(cur_hp: SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_CurHp) -> Self {
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
            vehicle_seat: None,
        }
    }

    pub fn set_CUR_HP(mut self, cur_hp: SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_CurHp) -> Self {
        self.inner |= GroupUpdateFlags::CUR_HP;
        self.cur_hp = Some(cur_hp);
        self
    }

    pub const fn get_CUR_HP(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_CurHp> {
        self.cur_hp.as_ref()
    }

    pub fn clear_CUR_HP(mut self) -> Self {
        self.inner &= GroupUpdateFlags::CUR_HP.reverse_bits();
        self.cur_hp = None;
        self
    }

    pub const fn new_MAX_HP(max_hp: SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_MaxHp) -> Self {
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
            vehicle_seat: None,
        }
    }

    pub fn set_MAX_HP(mut self, max_hp: SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_MaxHp) -> Self {
        self.inner |= GroupUpdateFlags::MAX_HP;
        self.max_hp = Some(max_hp);
        self
    }

    pub const fn get_MAX_HP(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_MaxHp> {
        self.max_hp.as_ref()
    }

    pub fn clear_MAX_HP(mut self) -> Self {
        self.inner &= GroupUpdateFlags::MAX_HP.reverse_bits();
        self.max_hp = None;
        self
    }

    pub const fn new_POWER_TYPE(power_type: SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PowerType) -> Self {
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
            vehicle_seat: None,
        }
    }

    pub fn set_POWER_TYPE(mut self, power_type: SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PowerType) -> Self {
        self.inner |= GroupUpdateFlags::POWER_TYPE;
        self.power_type = Some(power_type);
        self
    }

    pub const fn get_POWER_TYPE(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PowerType> {
        self.power_type.as_ref()
    }

    pub fn clear_POWER_TYPE(mut self) -> Self {
        self.inner &= GroupUpdateFlags::POWER_TYPE.reverse_bits();
        self.power_type = None;
        self
    }

    pub const fn new_CUR_POWER(cur_power: SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_CurPower) -> Self {
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
            vehicle_seat: None,
        }
    }

    pub fn set_CUR_POWER(mut self, cur_power: SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_CurPower) -> Self {
        self.inner |= GroupUpdateFlags::CUR_POWER;
        self.cur_power = Some(cur_power);
        self
    }

    pub const fn get_CUR_POWER(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_CurPower> {
        self.cur_power.as_ref()
    }

    pub fn clear_CUR_POWER(mut self) -> Self {
        self.inner &= GroupUpdateFlags::CUR_POWER.reverse_bits();
        self.cur_power = None;
        self
    }

    pub const fn new_MAX_POWER(max_power: SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_MaxPower) -> Self {
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
            vehicle_seat: None,
        }
    }

    pub fn set_MAX_POWER(mut self, max_power: SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_MaxPower) -> Self {
        self.inner |= GroupUpdateFlags::MAX_POWER;
        self.max_power = Some(max_power);
        self
    }

    pub const fn get_MAX_POWER(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_MaxPower> {
        self.max_power.as_ref()
    }

    pub fn clear_MAX_POWER(mut self) -> Self {
        self.inner &= GroupUpdateFlags::MAX_POWER.reverse_bits();
        self.max_power = None;
        self
    }

    pub const fn new_LEVEL(level: SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_Level) -> Self {
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
            vehicle_seat: None,
        }
    }

    pub fn set_LEVEL(mut self, level: SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_Level) -> Self {
        self.inner |= GroupUpdateFlags::LEVEL;
        self.level = Some(level);
        self
    }

    pub const fn get_LEVEL(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_Level> {
        self.level.as_ref()
    }

    pub fn clear_LEVEL(mut self) -> Self {
        self.inner &= GroupUpdateFlags::LEVEL.reverse_bits();
        self.level = None;
        self
    }

    pub const fn new_ZONE(zone: SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_Zone) -> Self {
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
            vehicle_seat: None,
        }
    }

    pub fn set_ZONE(mut self, zone: SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_Zone) -> Self {
        self.inner |= GroupUpdateFlags::ZONE;
        self.zone = Some(zone);
        self
    }

    pub const fn get_ZONE(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_Zone> {
        self.zone.as_ref()
    }

    pub fn clear_ZONE(mut self) -> Self {
        self.inner &= GroupUpdateFlags::ZONE.reverse_bits();
        self.zone = None;
        self
    }

    pub const fn new_POSITION(position: SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_Position) -> Self {
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
            vehicle_seat: None,
        }
    }

    pub fn set_POSITION(mut self, position: SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_Position) -> Self {
        self.inner |= GroupUpdateFlags::POSITION;
        self.position = Some(position);
        self
    }

    pub const fn get_POSITION(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_Position> {
        self.position.as_ref()
    }

    pub fn clear_POSITION(mut self) -> Self {
        self.inner &= GroupUpdateFlags::POSITION.reverse_bits();
        self.position = None;
        self
    }

    pub const fn new_AURAS(auras: SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_Auras) -> Self {
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
            vehicle_seat: None,
        }
    }

    pub fn set_AURAS(mut self, auras: SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_Auras) -> Self {
        self.inner |= GroupUpdateFlags::AURAS;
        self.auras = Some(auras);
        self
    }

    pub const fn get_AURAS(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_Auras> {
        self.auras.as_ref()
    }

    pub fn clear_AURAS(mut self) -> Self {
        self.inner &= GroupUpdateFlags::AURAS.reverse_bits();
        self.auras = None;
        self
    }

    pub const fn new_PET_GUID(pet_guid: SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetGuid) -> Self {
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
            vehicle_seat: None,
        }
    }

    pub fn set_PET_GUID(mut self, pet_guid: SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetGuid) -> Self {
        self.inner |= GroupUpdateFlags::PET_GUID;
        self.pet_guid = Some(pet_guid);
        self
    }

    pub const fn get_PET_GUID(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetGuid> {
        self.pet_guid.as_ref()
    }

    pub fn clear_PET_GUID(mut self) -> Self {
        self.inner &= GroupUpdateFlags::PET_GUID.reverse_bits();
        self.pet_guid = None;
        self
    }

    pub const fn new_PET_NAME(pet_name: SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetName) -> Self {
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
            vehicle_seat: None,
        }
    }

    pub fn set_PET_NAME(mut self, pet_name: SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetName) -> Self {
        self.inner |= GroupUpdateFlags::PET_NAME;
        self.pet_name = Some(pet_name);
        self
    }

    pub const fn get_PET_NAME(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetName> {
        self.pet_name.as_ref()
    }

    pub fn clear_PET_NAME(mut self) -> Self {
        self.inner &= GroupUpdateFlags::PET_NAME.reverse_bits();
        self.pet_name = None;
        self
    }

    pub const fn new_PET_MODEL_ID(pet_model_id: SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetModelId) -> Self {
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
            vehicle_seat: None,
        }
    }

    pub fn set_PET_MODEL_ID(mut self, pet_model_id: SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetModelId) -> Self {
        self.inner |= GroupUpdateFlags::PET_MODEL_ID;
        self.pet_model_id = Some(pet_model_id);
        self
    }

    pub const fn get_PET_MODEL_ID(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetModelId> {
        self.pet_model_id.as_ref()
    }

    pub fn clear_PET_MODEL_ID(mut self) -> Self {
        self.inner &= GroupUpdateFlags::PET_MODEL_ID.reverse_bits();
        self.pet_model_id = None;
        self
    }

    pub const fn new_PET_CUR_HP(pet_cur_hp: SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetCurHp) -> Self {
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
            vehicle_seat: None,
        }
    }

    pub fn set_PET_CUR_HP(mut self, pet_cur_hp: SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetCurHp) -> Self {
        self.inner |= GroupUpdateFlags::PET_CUR_HP;
        self.pet_cur_hp = Some(pet_cur_hp);
        self
    }

    pub const fn get_PET_CUR_HP(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetCurHp> {
        self.pet_cur_hp.as_ref()
    }

    pub fn clear_PET_CUR_HP(mut self) -> Self {
        self.inner &= GroupUpdateFlags::PET_CUR_HP.reverse_bits();
        self.pet_cur_hp = None;
        self
    }

    pub const fn new_PET_MAX_HP(pet_max_hp: SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetMaxHp) -> Self {
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
            vehicle_seat: None,
        }
    }

    pub fn set_PET_MAX_HP(mut self, pet_max_hp: SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetMaxHp) -> Self {
        self.inner |= GroupUpdateFlags::PET_MAX_HP;
        self.pet_max_hp = Some(pet_max_hp);
        self
    }

    pub const fn get_PET_MAX_HP(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetMaxHp> {
        self.pet_max_hp.as_ref()
    }

    pub fn clear_PET_MAX_HP(mut self) -> Self {
        self.inner &= GroupUpdateFlags::PET_MAX_HP.reverse_bits();
        self.pet_max_hp = None;
        self
    }

    pub const fn new_PET_POWER_TYPE(pet_power_type: SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetPowerType) -> Self {
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
            vehicle_seat: None,
        }
    }

    pub fn set_PET_POWER_TYPE(mut self, pet_power_type: SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetPowerType) -> Self {
        self.inner |= GroupUpdateFlags::PET_POWER_TYPE;
        self.pet_power_type = Some(pet_power_type);
        self
    }

    pub const fn get_PET_POWER_TYPE(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetPowerType> {
        self.pet_power_type.as_ref()
    }

    pub fn clear_PET_POWER_TYPE(mut self) -> Self {
        self.inner &= GroupUpdateFlags::PET_POWER_TYPE.reverse_bits();
        self.pet_power_type = None;
        self
    }

    pub const fn new_PET_CUR_POWER(pet_cur_power: SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetCurPower) -> Self {
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
            vehicle_seat: None,
        }
    }

    pub fn set_PET_CUR_POWER(mut self, pet_cur_power: SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetCurPower) -> Self {
        self.inner |= GroupUpdateFlags::PET_CUR_POWER;
        self.pet_cur_power = Some(pet_cur_power);
        self
    }

    pub const fn get_PET_CUR_POWER(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetCurPower> {
        self.pet_cur_power.as_ref()
    }

    pub fn clear_PET_CUR_POWER(mut self) -> Self {
        self.inner &= GroupUpdateFlags::PET_CUR_POWER.reverse_bits();
        self.pet_cur_power = None;
        self
    }

    pub const fn new_PET_MAX_POWER(pet_max_power: SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetMaxPower) -> Self {
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
            vehicle_seat: None,
        }
    }

    pub fn set_PET_MAX_POWER(mut self, pet_max_power: SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetMaxPower) -> Self {
        self.inner |= GroupUpdateFlags::PET_MAX_POWER;
        self.pet_max_power = Some(pet_max_power);
        self
    }

    pub const fn get_PET_MAX_POWER(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetMaxPower> {
        self.pet_max_power.as_ref()
    }

    pub fn clear_PET_MAX_POWER(mut self) -> Self {
        self.inner &= GroupUpdateFlags::PET_MAX_POWER.reverse_bits();
        self.pet_max_power = None;
        self
    }

    pub const fn new_PET_AURAS(pet_auras: SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetAuras) -> Self {
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
            vehicle_seat: None,
        }
    }

    pub fn set_PET_AURAS(mut self, pet_auras: SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetAuras) -> Self {
        self.inner |= GroupUpdateFlags::PET_AURAS;
        self.pet_auras = Some(pet_auras);
        self
    }

    pub const fn get_PET_AURAS(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetAuras> {
        self.pet_auras.as_ref()
    }

    pub fn clear_PET_AURAS(mut self) -> Self {
        self.inner &= GroupUpdateFlags::PET_AURAS.reverse_bits();
        self.pet_auras = None;
        self
    }

    pub const fn new_VEHICLE_SEAT(vehicle_seat: SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_VehicleSeat) -> Self {
        Self {
            inner: GroupUpdateFlags::VEHICLE_SEAT,
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
            vehicle_seat: Some(vehicle_seat),
        }
    }

    pub fn set_VEHICLE_SEAT(mut self, vehicle_seat: SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_VehicleSeat) -> Self {
        self.inner |= GroupUpdateFlags::VEHICLE_SEAT;
        self.vehicle_seat = Some(vehicle_seat);
        self
    }

    pub const fn get_VEHICLE_SEAT(&self) -> Option<&SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_VehicleSeat> {
        self.vehicle_seat.as_ref()
    }

    pub fn clear_VEHICLE_SEAT(mut self) -> Self {
        self.inner &= GroupUpdateFlags::VEHICLE_SEAT.reverse_bits();
        self.vehicle_seat = None;
        self
    }

    pub(crate) const fn as_int(&self) -> u32 {
        self.inner
    }

}
impl SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags {
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
        + {
            if let Some(s) = &self.vehicle_seat {
                s.size()
            } else {
                0
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_Status {
    pub status: GroupMemberOnlineStatus,
}

impl SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_Status {
    pub(crate) fn size(&self) -> usize {
        1 // status: GroupMemberOnlineStatus
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_CurHp {
    pub current_health: u32,
}

impl SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_CurHp {
    pub(crate) fn size(&self) -> usize {
        4 // current_health: u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_MaxHp {
    pub max_health: u32,
}

impl SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_MaxHp {
    pub(crate) fn size(&self) -> usize {
        4 // max_health: u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PowerType {
    pub power: Power,
}

impl SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PowerType {
    pub(crate) fn size(&self) -> usize {
        1 // power: Power
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_CurPower {
    pub current_power: u16,
}

impl SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_CurPower {
    pub(crate) fn size(&self) -> usize {
        2 // current_power: u16
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_MaxPower {
    pub max_power: u16,
}

impl SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_MaxPower {
    pub(crate) fn size(&self) -> usize {
        2 // max_power: u16
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_Level {
    pub level: u16,
}

impl SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_Level {
    pub(crate) fn size(&self) -> usize {
        2 // level: u16
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_Zone {
    pub area: Area,
}

impl SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_Zone {
    pub(crate) fn size(&self) -> usize {
        4 // area: Area
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_Position {
    pub position_x: u16,
    pub position_y: u16,
}

impl SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_Position {
    pub(crate) fn size(&self) -> usize {
        2 // position_x: u16
        + 2 // position_y: u16
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_Auras {
    pub auras: AuraMask,
}

impl SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_Auras {
    pub(crate) fn size(&self) -> usize {
        self.auras.size() // auras: AuraMask
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetGuid {
    pub pet: Guid,
}

impl SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetGuid {
    pub(crate) fn size(&self) -> usize {
        8 // pet: Guid
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetName {
    pub pet_name: String,
}

impl SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetName {
    pub(crate) fn size(&self) -> usize {
        self.pet_name.len() + 1 // pet_name: CString
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetModelId {
    pub pet_display_id: u16,
}

impl SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetModelId {
    pub(crate) fn size(&self) -> usize {
        2 // pet_display_id: u16
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetCurHp {
    pub pet_current_health: u32,
}

impl SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetCurHp {
    pub(crate) fn size(&self) -> usize {
        4 // pet_current_health: u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetMaxHp {
    pub pet_max_health: u32,
}

impl SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetMaxHp {
    pub(crate) fn size(&self) -> usize {
        4 // pet_max_health: u32
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetPowerType {
    pub pet_power_type: Power,
}

impl SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetPowerType {
    pub(crate) fn size(&self) -> usize {
        1 // pet_power_type: Power
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetCurPower {
    pub pet_current_power: u16,
}

impl SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetCurPower {
    pub(crate) fn size(&self) -> usize {
        2 // pet_current_power: u16
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetMaxPower {
    pub pet_max_power: u16,
}

impl SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetMaxPower {
    pub(crate) fn size(&self) -> usize {
        2 // pet_max_power: u16
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetAuras {
    pub pet_auras: AuraMask,
}

impl SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_PetAuras {
    pub(crate) fn size(&self) -> usize {
        self.pet_auras.size() // pet_auras: AuraMask
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_VehicleSeat {
    pub transport: u32,
}

impl SMSG_PARTY_MEMBER_STATS_GroupUpdateFlags_VehicleSeat {
    pub(crate) fn size(&self) -> usize {
        4 // transport: u32
    }
}

