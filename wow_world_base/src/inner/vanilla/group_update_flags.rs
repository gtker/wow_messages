/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/social_common.wowm:379`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/social_common.wowm#L379):
/// ```text
/// flag GroupUpdateFlags : u32 {
///     NONE = 0x00000000;
///     STATUS = 0x00000001;
///     CUR_HP = 0x00000002;
///     MAX_HP = 0x00000004;
///     POWER_TYPE = 0x00000008;
///     CUR_POWER = 0x00000010;
///     MAX_POWER = 0x00000020;
///     LEVEL = 0x00000040;
///     ZONE = 0x00000080;
///     POSITION = 0x00000100;
///     AURAS = 0x00000200;
///     AURAS_2 = 0x00000400;
///     PET_GUID = 0x00000800;
///     PET_NAME = 0x00001000;
///     PET_MODEL_ID = 0x00002000;
///     PET_CUR_HP = 0x00004000;
///     PET_MAX_HP = 0x00008000;
///     PET_POWER_TYPE = 0x00010000;
///     PET_CUR_POWER = 0x00020000;
///     PET_MAX_POWER = 0x00040000;
///     PET_AURAS = 0x00080000;
///     PET_AURAS_2 = 0x00100000;
///     MODE_OFFLINE = 0x10000000;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct GroupUpdateFlags {
    inner: u32,
}

impl GroupUpdateFlags {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

    pub const NONE: u32 = 0x00;
    pub const STATUS: u32 = 0x01;
    pub const CUR_HP: u32 = 0x02;
    pub const MAX_HP: u32 = 0x04;
    pub const POWER_TYPE: u32 = 0x08;
    pub const CUR_POWER: u32 = 0x10;
    pub const MAX_POWER: u32 = 0x20;
    pub const LEVEL: u32 = 0x40;
    pub const ZONE: u32 = 0x80;
    pub const POSITION: u32 = 0x100;
    pub const AURAS: u32 = 0x200;
    pub const AURAS_2: u32 = 0x400;
    pub const PET_GUID: u32 = 0x800;
    pub const PET_NAME: u32 = 0x1000;
    pub const PET_MODEL_ID: u32 = 0x2000;
    pub const PET_CUR_HP: u32 = 0x4000;
    pub const PET_MAX_HP: u32 = 0x8000;
    pub const PET_POWER_TYPE: u32 = 0x10000;
    pub const PET_CUR_POWER: u32 = 0x20000;
    pub const PET_MAX_POWER: u32 = 0x40000;
    pub const PET_AURAS: u32 = 0x80000;
    pub const PET_AURAS_2: u32 = 0x100000;
    pub const MODE_OFFLINE: u32 = 0x10000000;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::NONE
                | Self::STATUS
                | Self::CUR_HP
                | Self::MAX_HP
                | Self::POWER_TYPE
                | Self::CUR_POWER
                | Self::MAX_POWER
                | Self::LEVEL
                | Self::ZONE
                | Self::POSITION
                | Self::AURAS
                | Self::AURAS_2
                | Self::PET_GUID
                | Self::PET_NAME
                | Self::PET_MODEL_ID
                | Self::PET_CUR_HP
                | Self::PET_MAX_HP
                | Self::PET_POWER_TYPE
                | Self::PET_CUR_POWER
                | Self::PET_MAX_POWER
                | Self::PET_AURAS
                | Self::PET_AURAS_2
                | Self::MODE_OFFLINE
        }
    }

    pub const fn is_status(&self) -> bool {
        (self.inner & Self::STATUS) != 0
    }

    /// uint8, enum `GroupMemberOnlineStatus`
    ///
    pub const fn new_status() -> Self {
        Self { inner: Self::STATUS }
    }

    pub fn set_status(&mut self) -> Self {
        self.inner |= Self::STATUS;
        *self
    }

    pub fn clear_status(&mut self) -> Self {
        self.inner &= Self::STATUS.reverse_bits();
        *self
    }

    pub const fn is_cur_hp(&self) -> bool {
        (self.inner & Self::CUR_HP) != 0
    }

    /// uint16
    ///
    pub const fn new_cur_hp() -> Self {
        Self { inner: Self::CUR_HP }
    }

    pub fn set_cur_hp(&mut self) -> Self {
        self.inner |= Self::CUR_HP;
        *self
    }

    pub fn clear_cur_hp(&mut self) -> Self {
        self.inner &= Self::CUR_HP.reverse_bits();
        *self
    }

    pub const fn is_max_hp(&self) -> bool {
        (self.inner & Self::MAX_HP) != 0
    }

    /// uint16
    ///
    pub const fn new_max_hp() -> Self {
        Self { inner: Self::MAX_HP }
    }

    pub fn set_max_hp(&mut self) -> Self {
        self.inner |= Self::MAX_HP;
        *self
    }

    pub fn clear_max_hp(&mut self) -> Self {
        self.inner &= Self::MAX_HP.reverse_bits();
        *self
    }

    pub const fn is_power_type(&self) -> bool {
        (self.inner & Self::POWER_TYPE) != 0
    }

    /// uint8, enum Powers
    ///
    pub const fn new_power_type() -> Self {
        Self { inner: Self::POWER_TYPE }
    }

    pub fn set_power_type(&mut self) -> Self {
        self.inner |= Self::POWER_TYPE;
        *self
    }

    pub fn clear_power_type(&mut self) -> Self {
        self.inner &= Self::POWER_TYPE.reverse_bits();
        *self
    }

    pub const fn is_cur_power(&self) -> bool {
        (self.inner & Self::CUR_POWER) != 0
    }

    /// uint16
    ///
    pub const fn new_cur_power() -> Self {
        Self { inner: Self::CUR_POWER }
    }

    pub fn set_cur_power(&mut self) -> Self {
        self.inner |= Self::CUR_POWER;
        *self
    }

    pub fn clear_cur_power(&mut self) -> Self {
        self.inner &= Self::CUR_POWER.reverse_bits();
        *self
    }

    pub const fn is_max_power(&self) -> bool {
        (self.inner & Self::MAX_POWER) != 0
    }

    /// uint16
    ///
    pub const fn new_max_power() -> Self {
        Self { inner: Self::MAX_POWER }
    }

    pub fn set_max_power(&mut self) -> Self {
        self.inner |= Self::MAX_POWER;
        *self
    }

    pub fn clear_max_power(&mut self) -> Self {
        self.inner &= Self::MAX_POWER.reverse_bits();
        *self
    }

    pub const fn is_level(&self) -> bool {
        (self.inner & Self::LEVEL) != 0
    }

    /// uint16
    ///
    pub const fn new_level() -> Self {
        Self { inner: Self::LEVEL }
    }

    pub fn set_level(&mut self) -> Self {
        self.inner |= Self::LEVEL;
        *self
    }

    pub fn clear_level(&mut self) -> Self {
        self.inner &= Self::LEVEL.reverse_bits();
        *self
    }

    pub const fn is_zone(&self) -> bool {
        (self.inner & Self::ZONE) != 0
    }

    /// uint16
    ///
    pub const fn new_zone() -> Self {
        Self { inner: Self::ZONE }
    }

    pub fn set_zone(&mut self) -> Self {
        self.inner |= Self::ZONE;
        *self
    }

    pub fn clear_zone(&mut self) -> Self {
        self.inner &= Self::ZONE.reverse_bits();
        *self
    }

    pub const fn is_position(&self) -> bool {
        (self.inner & Self::POSITION) != 0
    }

    /// uint16, uint16
    ///
    pub const fn new_position() -> Self {
        Self { inner: Self::POSITION }
    }

    pub fn set_position(&mut self) -> Self {
        self.inner |= Self::POSITION;
        *self
    }

    pub fn clear_position(&mut self) -> Self {
        self.inner &= Self::POSITION.reverse_bits();
        *self
    }

    pub const fn is_auras(&self) -> bool {
        (self.inner & Self::AURAS) != 0
    }

    /// uint32 mask, for each bit set uint16 spellid
    ///
    pub const fn new_auras() -> Self {
        Self { inner: Self::AURAS }
    }

    pub fn set_auras(&mut self) -> Self {
        self.inner |= Self::AURAS;
        *self
    }

    pub fn clear_auras(&mut self) -> Self {
        self.inner &= Self::AURAS.reverse_bits();
        *self
    }

    pub const fn is_auras_2(&self) -> bool {
        (self.inner & Self::AURAS_2) != 0
    }

    /// uint16 above mask continuation, giving max total of 48 auras possible
    ///
    pub const fn new_auras_2() -> Self {
        Self { inner: Self::AURAS_2 }
    }

    pub fn set_auras_2(&mut self) -> Self {
        self.inner |= Self::AURAS_2;
        *self
    }

    pub fn clear_auras_2(&mut self) -> Self {
        self.inner &= Self::AURAS_2.reverse_bits();
        *self
    }

    pub const fn is_pet_guid(&self) -> bool {
        (self.inner & Self::PET_GUID) != 0
    }

    /// uint64 pet guid
    ///
    pub const fn new_pet_guid() -> Self {
        Self { inner: Self::PET_GUID }
    }

    pub fn set_pet_guid(&mut self) -> Self {
        self.inner |= Self::PET_GUID;
        *self
    }

    pub fn clear_pet_guid(&mut self) -> Self {
        self.inner &= Self::PET_GUID.reverse_bits();
        *self
    }

    pub const fn is_pet_name(&self) -> bool {
        (self.inner & Self::PET_NAME) != 0
    }

    /// pet name, NULL terminated string
    ///
    pub const fn new_pet_name() -> Self {
        Self { inner: Self::PET_NAME }
    }

    pub fn set_pet_name(&mut self) -> Self {
        self.inner |= Self::PET_NAME;
        *self
    }

    pub fn clear_pet_name(&mut self) -> Self {
        self.inner &= Self::PET_NAME.reverse_bits();
        *self
    }

    pub const fn is_pet_model_id(&self) -> bool {
        (self.inner & Self::PET_MODEL_ID) != 0
    }

    /// uint16, model id
    ///
    pub const fn new_pet_model_id() -> Self {
        Self { inner: Self::PET_MODEL_ID }
    }

    pub fn set_pet_model_id(&mut self) -> Self {
        self.inner |= Self::PET_MODEL_ID;
        *self
    }

    pub fn clear_pet_model_id(&mut self) -> Self {
        self.inner &= Self::PET_MODEL_ID.reverse_bits();
        *self
    }

    pub const fn is_pet_cur_hp(&self) -> bool {
        (self.inner & Self::PET_CUR_HP) != 0
    }

    /// uint16 pet cur health
    ///
    pub const fn new_pet_cur_hp() -> Self {
        Self { inner: Self::PET_CUR_HP }
    }

    pub fn set_pet_cur_hp(&mut self) -> Self {
        self.inner |= Self::PET_CUR_HP;
        *self
    }

    pub fn clear_pet_cur_hp(&mut self) -> Self {
        self.inner &= Self::PET_CUR_HP.reverse_bits();
        *self
    }

    pub const fn is_pet_max_hp(&self) -> bool {
        (self.inner & Self::PET_MAX_HP) != 0
    }

    /// uint16 pet max health
    ///
    pub const fn new_pet_max_hp() -> Self {
        Self { inner: Self::PET_MAX_HP }
    }

    pub fn set_pet_max_hp(&mut self) -> Self {
        self.inner |= Self::PET_MAX_HP;
        *self
    }

    pub fn clear_pet_max_hp(&mut self) -> Self {
        self.inner &= Self::PET_MAX_HP.reverse_bits();
        *self
    }

    pub const fn is_pet_power_type(&self) -> bool {
        (self.inner & Self::PET_POWER_TYPE) != 0
    }

    /// uint8 pet power type
    ///
    pub const fn new_pet_power_type() -> Self {
        Self { inner: Self::PET_POWER_TYPE }
    }

    pub fn set_pet_power_type(&mut self) -> Self {
        self.inner |= Self::PET_POWER_TYPE;
        *self
    }

    pub fn clear_pet_power_type(&mut self) -> Self {
        self.inner &= Self::PET_POWER_TYPE.reverse_bits();
        *self
    }

    pub const fn is_pet_cur_power(&self) -> bool {
        (self.inner & Self::PET_CUR_POWER) != 0
    }

    /// uint16 pet cur power
    ///
    pub const fn new_pet_cur_power() -> Self {
        Self { inner: Self::PET_CUR_POWER }
    }

    pub fn set_pet_cur_power(&mut self) -> Self {
        self.inner |= Self::PET_CUR_POWER;
        *self
    }

    pub fn clear_pet_cur_power(&mut self) -> Self {
        self.inner &= Self::PET_CUR_POWER.reverse_bits();
        *self
    }

    pub const fn is_pet_max_power(&self) -> bool {
        (self.inner & Self::PET_MAX_POWER) != 0
    }

    /// uint16 pet max power
    ///
    pub const fn new_pet_max_power() -> Self {
        Self { inner: Self::PET_MAX_POWER }
    }

    pub fn set_pet_max_power(&mut self) -> Self {
        self.inner |= Self::PET_MAX_POWER;
        *self
    }

    pub fn clear_pet_max_power(&mut self) -> Self {
        self.inner &= Self::PET_MAX_POWER.reverse_bits();
        *self
    }

    pub const fn is_pet_auras(&self) -> bool {
        (self.inner & Self::PET_AURAS) != 0
    }

    /// uint32 mask, for each bit set uint16 spellid, pet auras...
    ///
    pub const fn new_pet_auras() -> Self {
        Self { inner: Self::PET_AURAS }
    }

    pub fn set_pet_auras(&mut self) -> Self {
        self.inner |= Self::PET_AURAS;
        *self
    }

    pub fn clear_pet_auras(&mut self) -> Self {
        self.inner &= Self::PET_AURAS.reverse_bits();
        *self
    }

    pub const fn is_pet_auras_2(&self) -> bool {
        (self.inner & Self::PET_AURAS_2) != 0
    }

    /// uint16 above mask continuation, giving max total of 48 auras possible
    ///
    pub const fn new_pet_auras_2() -> Self {
        Self { inner: Self::PET_AURAS_2 }
    }

    pub fn set_pet_auras_2(&mut self) -> Self {
        self.inner |= Self::PET_AURAS_2;
        *self
    }

    pub fn clear_pet_auras_2(&mut self) -> Self {
        self.inner &= Self::PET_AURAS_2.reverse_bits();
        *self
    }

    pub const fn is_mode_offline(&self) -> bool {
        (self.inner & Self::MODE_OFFLINE) != 0
    }

    pub const fn new_mode_offline() -> Self {
        Self { inner: Self::MODE_OFFLINE }
    }

    pub fn set_mode_offline(&mut self) -> Self {
        self.inner |= Self::MODE_OFFLINE;
        *self
    }

    pub fn clear_mode_offline(&mut self) -> Self {
        self.inner &= Self::MODE_OFFLINE.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u32 {
        self.inner
    }

}

impl std::fmt::UpperHex for GroupUpdateFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for GroupUpdateFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for GroupUpdateFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for GroupUpdateFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

impl std::ops::BitAnd for GroupUpdateFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitand(rhs.inner), }
    }
}

impl std::ops::BitAndAssign for GroupUpdateFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner)
    }
}

impl std::ops::BitOr for GroupUpdateFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitor(rhs.inner), }
    }
}

impl std::ops::BitOrAssign for GroupUpdateFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner)
    }
}

impl std::ops::BitXor for GroupUpdateFlags {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitxor(rhs.inner), }
    }
}

impl std::ops::BitXorAssign for GroupUpdateFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner)
    }
}

