/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/social_common.wowm:197`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/social_common.wowm#L197):
/// ```text
/// flag GroupUpdateFlags : u32 {
///     FLAG_NONE = 0x00000000;
///     FLAG_STATUS = 0x00000001;
///     FLAG_CUR_HP = 0x00000002;
///     FLAG_MAX_HP = 0x00000004;
///     FLAG_POWER_TYPE = 0x00000008;
///     FLAG_CUR_POWER = 0x00000010;
///     FLAG_MAX_POWER = 0x00000020;
///     FLAG_LEVEL = 0x00000040;
///     FLAG_ZONE = 0x00000080;
///     FLAG_POSITION = 0x00000100;
///     FLAG_AURAS = 0x00000200;
///     FLAG_AURAS_2 = 0x00000400;
///     FLAG_PET_GUID = 0x00000800;
///     FLAG_PET_NAME = 0x00001000;
///     FLAG_PET_MODEL_ID = 0x00002000;
///     FLAG_PET_CUR_HP = 0x00004000;
///     FLAG_PET_MAX_HP = 0x00008000;
///     FLAG_PET_POWER_TYPE = 0x00010000;
///     FLAG_PET_CUR_POWER = 0x00020000;
///     FLAG_PET_MAX_POWER = 0x00040000;
///     FLAG_PET_AURAS = 0x00080000;
///     FLAG_PET_AURAS_2 = 0x00100000;
///     MODE_OFFLINE = 0x10000000;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
pub struct GroupUpdateFlags {
    inner: u32,
}

impl GroupUpdateFlags {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

    pub(crate) const FLAG_NONE: u32 = 0x00;
    pub(crate) const FLAG_STATUS: u32 = 0x01;
    pub(crate) const FLAG_CUR_HP: u32 = 0x02;
    pub(crate) const FLAG_MAX_HP: u32 = 0x04;
    pub(crate) const FLAG_POWER_TYPE: u32 = 0x08;
    pub(crate) const FLAG_CUR_POWER: u32 = 0x10;
    pub(crate) const FLAG_MAX_POWER: u32 = 0x20;
    pub(crate) const FLAG_LEVEL: u32 = 0x40;
    pub(crate) const FLAG_ZONE: u32 = 0x80;
    pub(crate) const FLAG_POSITION: u32 = 0x100;
    pub(crate) const FLAG_AURAS: u32 = 0x200;
    pub(crate) const FLAG_AURAS_2: u32 = 0x400;
    pub(crate) const FLAG_PET_GUID: u32 = 0x800;
    pub(crate) const FLAG_PET_NAME: u32 = 0x1000;
    pub(crate) const FLAG_PET_MODEL_ID: u32 = 0x2000;
    pub(crate) const FLAG_PET_CUR_HP: u32 = 0x4000;
    pub(crate) const FLAG_PET_MAX_HP: u32 = 0x8000;
    pub(crate) const FLAG_PET_POWER_TYPE: u32 = 0x10000;
    pub(crate) const FLAG_PET_CUR_POWER: u32 = 0x20000;
    pub(crate) const FLAG_PET_MAX_POWER: u32 = 0x40000;
    pub(crate) const FLAG_PET_AURAS: u32 = 0x80000;
    pub(crate) const FLAG_PET_AURAS_2: u32 = 0x100000;
    pub(crate) const MODE_OFFLINE: u32 = 0x10000000;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::FLAG_NONE
                | Self::FLAG_STATUS
                | Self::FLAG_CUR_HP
                | Self::FLAG_MAX_HP
                | Self::FLAG_POWER_TYPE
                | Self::FLAG_CUR_POWER
                | Self::FLAG_MAX_POWER
                | Self::FLAG_LEVEL
                | Self::FLAG_ZONE
                | Self::FLAG_POSITION
                | Self::FLAG_AURAS
                | Self::FLAG_AURAS_2
                | Self::FLAG_PET_GUID
                | Self::FLAG_PET_NAME
                | Self::FLAG_PET_MODEL_ID
                | Self::FLAG_PET_CUR_HP
                | Self::FLAG_PET_MAX_HP
                | Self::FLAG_PET_POWER_TYPE
                | Self::FLAG_PET_CUR_POWER
                | Self::FLAG_PET_MAX_POWER
                | Self::FLAG_PET_AURAS
                | Self::FLAG_PET_AURAS_2
                | Self::MODE_OFFLINE
        }
    }

    pub const fn is_FLAG_STATUS(&self) -> bool {
        (self.inner & Self::FLAG_STATUS) != 0
    }

    /// uint8, enum `GroupMemberOnlineStatus`
    ///
    pub const fn new_FLAG_STATUS() -> Self {
        Self { inner: Self::FLAG_STATUS }
    }

    pub fn set_FLAG_STATUS(&mut self) -> Self {
        self.inner |= Self::FLAG_STATUS;
        *self
    }

    pub fn clear_FLAG_STATUS(&mut self) -> Self {
        self.inner &= Self::FLAG_STATUS.reverse_bits();
        *self
    }

    pub const fn is_FLAG_CUR_HP(&self) -> bool {
        (self.inner & Self::FLAG_CUR_HP) != 0
    }

    /// uint16
    ///
    pub const fn new_FLAG_CUR_HP() -> Self {
        Self { inner: Self::FLAG_CUR_HP }
    }

    pub fn set_FLAG_CUR_HP(&mut self) -> Self {
        self.inner |= Self::FLAG_CUR_HP;
        *self
    }

    pub fn clear_FLAG_CUR_HP(&mut self) -> Self {
        self.inner &= Self::FLAG_CUR_HP.reverse_bits();
        *self
    }

    pub const fn is_FLAG_MAX_HP(&self) -> bool {
        (self.inner & Self::FLAG_MAX_HP) != 0
    }

    /// uint16
    ///
    pub const fn new_FLAG_MAX_HP() -> Self {
        Self { inner: Self::FLAG_MAX_HP }
    }

    pub fn set_FLAG_MAX_HP(&mut self) -> Self {
        self.inner |= Self::FLAG_MAX_HP;
        *self
    }

    pub fn clear_FLAG_MAX_HP(&mut self) -> Self {
        self.inner &= Self::FLAG_MAX_HP.reverse_bits();
        *self
    }

    pub const fn is_FLAG_POWER_TYPE(&self) -> bool {
        (self.inner & Self::FLAG_POWER_TYPE) != 0
    }

    /// uint8, enum Powers
    ///
    pub const fn new_FLAG_POWER_TYPE() -> Self {
        Self { inner: Self::FLAG_POWER_TYPE }
    }

    pub fn set_FLAG_POWER_TYPE(&mut self) -> Self {
        self.inner |= Self::FLAG_POWER_TYPE;
        *self
    }

    pub fn clear_FLAG_POWER_TYPE(&mut self) -> Self {
        self.inner &= Self::FLAG_POWER_TYPE.reverse_bits();
        *self
    }

    pub const fn is_FLAG_CUR_POWER(&self) -> bool {
        (self.inner & Self::FLAG_CUR_POWER) != 0
    }

    /// uint16
    ///
    pub const fn new_FLAG_CUR_POWER() -> Self {
        Self { inner: Self::FLAG_CUR_POWER }
    }

    pub fn set_FLAG_CUR_POWER(&mut self) -> Self {
        self.inner |= Self::FLAG_CUR_POWER;
        *self
    }

    pub fn clear_FLAG_CUR_POWER(&mut self) -> Self {
        self.inner &= Self::FLAG_CUR_POWER.reverse_bits();
        *self
    }

    pub const fn is_FLAG_MAX_POWER(&self) -> bool {
        (self.inner & Self::FLAG_MAX_POWER) != 0
    }

    /// uint16
    ///
    pub const fn new_FLAG_MAX_POWER() -> Self {
        Self { inner: Self::FLAG_MAX_POWER }
    }

    pub fn set_FLAG_MAX_POWER(&mut self) -> Self {
        self.inner |= Self::FLAG_MAX_POWER;
        *self
    }

    pub fn clear_FLAG_MAX_POWER(&mut self) -> Self {
        self.inner &= Self::FLAG_MAX_POWER.reverse_bits();
        *self
    }

    pub const fn is_FLAG_LEVEL(&self) -> bool {
        (self.inner & Self::FLAG_LEVEL) != 0
    }

    /// uint16
    ///
    pub const fn new_FLAG_LEVEL() -> Self {
        Self { inner: Self::FLAG_LEVEL }
    }

    pub fn set_FLAG_LEVEL(&mut self) -> Self {
        self.inner |= Self::FLAG_LEVEL;
        *self
    }

    pub fn clear_FLAG_LEVEL(&mut self) -> Self {
        self.inner &= Self::FLAG_LEVEL.reverse_bits();
        *self
    }

    pub const fn is_FLAG_ZONE(&self) -> bool {
        (self.inner & Self::FLAG_ZONE) != 0
    }

    /// uint16
    ///
    pub const fn new_FLAG_ZONE() -> Self {
        Self { inner: Self::FLAG_ZONE }
    }

    pub fn set_FLAG_ZONE(&mut self) -> Self {
        self.inner |= Self::FLAG_ZONE;
        *self
    }

    pub fn clear_FLAG_ZONE(&mut self) -> Self {
        self.inner &= Self::FLAG_ZONE.reverse_bits();
        *self
    }

    pub const fn is_FLAG_POSITION(&self) -> bool {
        (self.inner & Self::FLAG_POSITION) != 0
    }

    /// uint16, uint16
    ///
    pub const fn new_FLAG_POSITION() -> Self {
        Self { inner: Self::FLAG_POSITION }
    }

    pub fn set_FLAG_POSITION(&mut self) -> Self {
        self.inner |= Self::FLAG_POSITION;
        *self
    }

    pub fn clear_FLAG_POSITION(&mut self) -> Self {
        self.inner &= Self::FLAG_POSITION.reverse_bits();
        *self
    }

    pub const fn is_FLAG_AURAS(&self) -> bool {
        (self.inner & Self::FLAG_AURAS) != 0
    }

    /// uint32 mask, for each bit set uint16 spellid
    ///
    pub const fn new_FLAG_AURAS() -> Self {
        Self { inner: Self::FLAG_AURAS }
    }

    pub fn set_FLAG_AURAS(&mut self) -> Self {
        self.inner |= Self::FLAG_AURAS;
        *self
    }

    pub fn clear_FLAG_AURAS(&mut self) -> Self {
        self.inner &= Self::FLAG_AURAS.reverse_bits();
        *self
    }

    pub const fn is_FLAG_AURAS_2(&self) -> bool {
        (self.inner & Self::FLAG_AURAS_2) != 0
    }

    /// uint16 above mask continuation, giving max total of 48 auras possible
    ///
    pub const fn new_FLAG_AURAS_2() -> Self {
        Self { inner: Self::FLAG_AURAS_2 }
    }

    pub fn set_FLAG_AURAS_2(&mut self) -> Self {
        self.inner |= Self::FLAG_AURAS_2;
        *self
    }

    pub fn clear_FLAG_AURAS_2(&mut self) -> Self {
        self.inner &= Self::FLAG_AURAS_2.reverse_bits();
        *self
    }

    pub const fn is_FLAG_PET_GUID(&self) -> bool {
        (self.inner & Self::FLAG_PET_GUID) != 0
    }

    /// uint64 pet guid
    ///
    pub const fn new_FLAG_PET_GUID() -> Self {
        Self { inner: Self::FLAG_PET_GUID }
    }

    pub fn set_FLAG_PET_GUID(&mut self) -> Self {
        self.inner |= Self::FLAG_PET_GUID;
        *self
    }

    pub fn clear_FLAG_PET_GUID(&mut self) -> Self {
        self.inner &= Self::FLAG_PET_GUID.reverse_bits();
        *self
    }

    pub const fn is_FLAG_PET_NAME(&self) -> bool {
        (self.inner & Self::FLAG_PET_NAME) != 0
    }

    /// pet name, NULL terminated string
    ///
    pub const fn new_FLAG_PET_NAME() -> Self {
        Self { inner: Self::FLAG_PET_NAME }
    }

    pub fn set_FLAG_PET_NAME(&mut self) -> Self {
        self.inner |= Self::FLAG_PET_NAME;
        *self
    }

    pub fn clear_FLAG_PET_NAME(&mut self) -> Self {
        self.inner &= Self::FLAG_PET_NAME.reverse_bits();
        *self
    }

    pub const fn is_FLAG_PET_MODEL_ID(&self) -> bool {
        (self.inner & Self::FLAG_PET_MODEL_ID) != 0
    }

    /// uint16, model id
    ///
    pub const fn new_FLAG_PET_MODEL_ID() -> Self {
        Self { inner: Self::FLAG_PET_MODEL_ID }
    }

    pub fn set_FLAG_PET_MODEL_ID(&mut self) -> Self {
        self.inner |= Self::FLAG_PET_MODEL_ID;
        *self
    }

    pub fn clear_FLAG_PET_MODEL_ID(&mut self) -> Self {
        self.inner &= Self::FLAG_PET_MODEL_ID.reverse_bits();
        *self
    }

    pub const fn is_FLAG_PET_CUR_HP(&self) -> bool {
        (self.inner & Self::FLAG_PET_CUR_HP) != 0
    }

    /// uint16 pet cur health
    ///
    pub const fn new_FLAG_PET_CUR_HP() -> Self {
        Self { inner: Self::FLAG_PET_CUR_HP }
    }

    pub fn set_FLAG_PET_CUR_HP(&mut self) -> Self {
        self.inner |= Self::FLAG_PET_CUR_HP;
        *self
    }

    pub fn clear_FLAG_PET_CUR_HP(&mut self) -> Self {
        self.inner &= Self::FLAG_PET_CUR_HP.reverse_bits();
        *self
    }

    pub const fn is_FLAG_PET_MAX_HP(&self) -> bool {
        (self.inner & Self::FLAG_PET_MAX_HP) != 0
    }

    /// uint16 pet max health
    ///
    pub const fn new_FLAG_PET_MAX_HP() -> Self {
        Self { inner: Self::FLAG_PET_MAX_HP }
    }

    pub fn set_FLAG_PET_MAX_HP(&mut self) -> Self {
        self.inner |= Self::FLAG_PET_MAX_HP;
        *self
    }

    pub fn clear_FLAG_PET_MAX_HP(&mut self) -> Self {
        self.inner &= Self::FLAG_PET_MAX_HP.reverse_bits();
        *self
    }

    pub const fn is_FLAG_PET_POWER_TYPE(&self) -> bool {
        (self.inner & Self::FLAG_PET_POWER_TYPE) != 0
    }

    /// uint8 pet power type
    ///
    pub const fn new_FLAG_PET_POWER_TYPE() -> Self {
        Self { inner: Self::FLAG_PET_POWER_TYPE }
    }

    pub fn set_FLAG_PET_POWER_TYPE(&mut self) -> Self {
        self.inner |= Self::FLAG_PET_POWER_TYPE;
        *self
    }

    pub fn clear_FLAG_PET_POWER_TYPE(&mut self) -> Self {
        self.inner &= Self::FLAG_PET_POWER_TYPE.reverse_bits();
        *self
    }

    pub const fn is_FLAG_PET_CUR_POWER(&self) -> bool {
        (self.inner & Self::FLAG_PET_CUR_POWER) != 0
    }

    /// uint16 pet cur power
    ///
    pub const fn new_FLAG_PET_CUR_POWER() -> Self {
        Self { inner: Self::FLAG_PET_CUR_POWER }
    }

    pub fn set_FLAG_PET_CUR_POWER(&mut self) -> Self {
        self.inner |= Self::FLAG_PET_CUR_POWER;
        *self
    }

    pub fn clear_FLAG_PET_CUR_POWER(&mut self) -> Self {
        self.inner &= Self::FLAG_PET_CUR_POWER.reverse_bits();
        *self
    }

    pub const fn is_FLAG_PET_MAX_POWER(&self) -> bool {
        (self.inner & Self::FLAG_PET_MAX_POWER) != 0
    }

    /// uint16 pet max power
    ///
    pub const fn new_FLAG_PET_MAX_POWER() -> Self {
        Self { inner: Self::FLAG_PET_MAX_POWER }
    }

    pub fn set_FLAG_PET_MAX_POWER(&mut self) -> Self {
        self.inner |= Self::FLAG_PET_MAX_POWER;
        *self
    }

    pub fn clear_FLAG_PET_MAX_POWER(&mut self) -> Self {
        self.inner &= Self::FLAG_PET_MAX_POWER.reverse_bits();
        *self
    }

    pub const fn is_FLAG_PET_AURAS(&self) -> bool {
        (self.inner & Self::FLAG_PET_AURAS) != 0
    }

    /// uint32 mask, for each bit set uint16 spellid, pet auras...
    ///
    pub const fn new_FLAG_PET_AURAS() -> Self {
        Self { inner: Self::FLAG_PET_AURAS }
    }

    pub fn set_FLAG_PET_AURAS(&mut self) -> Self {
        self.inner |= Self::FLAG_PET_AURAS;
        *self
    }

    pub fn clear_FLAG_PET_AURAS(&mut self) -> Self {
        self.inner &= Self::FLAG_PET_AURAS.reverse_bits();
        *self
    }

    pub const fn is_FLAG_PET_AURAS_2(&self) -> bool {
        (self.inner & Self::FLAG_PET_AURAS_2) != 0
    }

    /// uint16 above mask continuation, giving max total of 48 auras possible
    ///
    pub const fn new_FLAG_PET_AURAS_2() -> Self {
        Self { inner: Self::FLAG_PET_AURAS_2 }
    }

    pub fn set_FLAG_PET_AURAS_2(&mut self) -> Self {
        self.inner |= Self::FLAG_PET_AURAS_2;
        *self
    }

    pub fn clear_FLAG_PET_AURAS_2(&mut self) -> Self {
        self.inner &= Self::FLAG_PET_AURAS_2.reverse_bits();
        *self
    }

    pub const fn is_MODE_OFFLINE(&self) -> bool {
        (self.inner & Self::MODE_OFFLINE) != 0
    }

    pub const fn new_MODE_OFFLINE() -> Self {
        Self { inner: Self::MODE_OFFLINE }
    }

    pub fn set_MODE_OFFLINE(&mut self) -> Self {
        self.inner |= Self::MODE_OFFLINE;
        *self
    }

    pub fn clear_MODE_OFFLINE(&mut self) -> Self {
        self.inner &= Self::MODE_OFFLINE.reverse_bits();
        *self
    }

    pub(crate) const fn as_int(&self) -> u32 {
        self.inner
    }

}

