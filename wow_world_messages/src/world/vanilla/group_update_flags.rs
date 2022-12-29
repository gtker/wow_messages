/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/social_common.wowm:206`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/social_common.wowm#L206):
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
pub struct GroupUpdateFlags {
    inner: u32,
}

impl GroupUpdateFlags {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

    pub(crate) const NONE: u32 = 0x00;
    pub(crate) const STATUS: u32 = 0x01;
    pub(crate) const CUR_HP: u32 = 0x02;
    pub(crate) const MAX_HP: u32 = 0x04;
    pub(crate) const POWER_TYPE: u32 = 0x08;
    pub(crate) const CUR_POWER: u32 = 0x10;
    pub(crate) const MAX_POWER: u32 = 0x20;
    pub(crate) const LEVEL: u32 = 0x40;
    pub(crate) const ZONE: u32 = 0x80;
    pub(crate) const POSITION: u32 = 0x100;
    pub(crate) const AURAS: u32 = 0x200;
    pub(crate) const AURAS_2: u32 = 0x400;
    pub(crate) const PET_GUID: u32 = 0x800;
    pub(crate) const PET_NAME: u32 = 0x1000;
    pub(crate) const PET_MODEL_ID: u32 = 0x2000;
    pub(crate) const PET_CUR_HP: u32 = 0x4000;
    pub(crate) const PET_MAX_HP: u32 = 0x8000;
    pub(crate) const PET_POWER_TYPE: u32 = 0x10000;
    pub(crate) const PET_CUR_POWER: u32 = 0x20000;
    pub(crate) const PET_MAX_POWER: u32 = 0x40000;
    pub(crate) const PET_AURAS: u32 = 0x80000;
    pub(crate) const PET_AURAS_2: u32 = 0x100000;
    pub(crate) const MODE_OFFLINE: u32 = 0x10000000;

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

    pub const fn is_STATUS(&self) -> bool {
        (self.inner & Self::STATUS) != 0
    }

    /// uint8, enum `GroupMemberOnlineStatus`
    ///
    pub const fn new_STATUS() -> Self {
        Self { inner: Self::STATUS }
    }

    pub fn set_STATUS(&mut self) -> Self {
        self.inner |= Self::STATUS;
        *self
    }

    pub fn clear_STATUS(&mut self) -> Self {
        self.inner &= Self::STATUS.reverse_bits();
        *self
    }

    pub const fn is_CUR_HP(&self) -> bool {
        (self.inner & Self::CUR_HP) != 0
    }

    /// uint16
    ///
    pub const fn new_CUR_HP() -> Self {
        Self { inner: Self::CUR_HP }
    }

    pub fn set_CUR_HP(&mut self) -> Self {
        self.inner |= Self::CUR_HP;
        *self
    }

    pub fn clear_CUR_HP(&mut self) -> Self {
        self.inner &= Self::CUR_HP.reverse_bits();
        *self
    }

    pub const fn is_MAX_HP(&self) -> bool {
        (self.inner & Self::MAX_HP) != 0
    }

    /// uint16
    ///
    pub const fn new_MAX_HP() -> Self {
        Self { inner: Self::MAX_HP }
    }

    pub fn set_MAX_HP(&mut self) -> Self {
        self.inner |= Self::MAX_HP;
        *self
    }

    pub fn clear_MAX_HP(&mut self) -> Self {
        self.inner &= Self::MAX_HP.reverse_bits();
        *self
    }

    pub const fn is_POWER_TYPE(&self) -> bool {
        (self.inner & Self::POWER_TYPE) != 0
    }

    /// uint8, enum Powers
    ///
    pub const fn new_POWER_TYPE() -> Self {
        Self { inner: Self::POWER_TYPE }
    }

    pub fn set_POWER_TYPE(&mut self) -> Self {
        self.inner |= Self::POWER_TYPE;
        *self
    }

    pub fn clear_POWER_TYPE(&mut self) -> Self {
        self.inner &= Self::POWER_TYPE.reverse_bits();
        *self
    }

    pub const fn is_CUR_POWER(&self) -> bool {
        (self.inner & Self::CUR_POWER) != 0
    }

    /// uint16
    ///
    pub const fn new_CUR_POWER() -> Self {
        Self { inner: Self::CUR_POWER }
    }

    pub fn set_CUR_POWER(&mut self) -> Self {
        self.inner |= Self::CUR_POWER;
        *self
    }

    pub fn clear_CUR_POWER(&mut self) -> Self {
        self.inner &= Self::CUR_POWER.reverse_bits();
        *self
    }

    pub const fn is_MAX_POWER(&self) -> bool {
        (self.inner & Self::MAX_POWER) != 0
    }

    /// uint16
    ///
    pub const fn new_MAX_POWER() -> Self {
        Self { inner: Self::MAX_POWER }
    }

    pub fn set_MAX_POWER(&mut self) -> Self {
        self.inner |= Self::MAX_POWER;
        *self
    }

    pub fn clear_MAX_POWER(&mut self) -> Self {
        self.inner &= Self::MAX_POWER.reverse_bits();
        *self
    }

    pub const fn is_LEVEL(&self) -> bool {
        (self.inner & Self::LEVEL) != 0
    }

    /// uint16
    ///
    pub const fn new_LEVEL() -> Self {
        Self { inner: Self::LEVEL }
    }

    pub fn set_LEVEL(&mut self) -> Self {
        self.inner |= Self::LEVEL;
        *self
    }

    pub fn clear_LEVEL(&mut self) -> Self {
        self.inner &= Self::LEVEL.reverse_bits();
        *self
    }

    pub const fn is_ZONE(&self) -> bool {
        (self.inner & Self::ZONE) != 0
    }

    /// uint16
    ///
    pub const fn new_ZONE() -> Self {
        Self { inner: Self::ZONE }
    }

    pub fn set_ZONE(&mut self) -> Self {
        self.inner |= Self::ZONE;
        *self
    }

    pub fn clear_ZONE(&mut self) -> Self {
        self.inner &= Self::ZONE.reverse_bits();
        *self
    }

    pub const fn is_POSITION(&self) -> bool {
        (self.inner & Self::POSITION) != 0
    }

    /// uint16, uint16
    ///
    pub const fn new_POSITION() -> Self {
        Self { inner: Self::POSITION }
    }

    pub fn set_POSITION(&mut self) -> Self {
        self.inner |= Self::POSITION;
        *self
    }

    pub fn clear_POSITION(&mut self) -> Self {
        self.inner &= Self::POSITION.reverse_bits();
        *self
    }

    pub const fn is_AURAS(&self) -> bool {
        (self.inner & Self::AURAS) != 0
    }

    /// uint32 mask, for each bit set uint16 spellid
    ///
    pub const fn new_AURAS() -> Self {
        Self { inner: Self::AURAS }
    }

    pub fn set_AURAS(&mut self) -> Self {
        self.inner |= Self::AURAS;
        *self
    }

    pub fn clear_AURAS(&mut self) -> Self {
        self.inner &= Self::AURAS.reverse_bits();
        *self
    }

    pub const fn is_AURAS_2(&self) -> bool {
        (self.inner & Self::AURAS_2) != 0
    }

    /// uint16 above mask continuation, giving max total of 48 auras possible
    ///
    pub const fn new_AURAS_2() -> Self {
        Self { inner: Self::AURAS_2 }
    }

    pub fn set_AURAS_2(&mut self) -> Self {
        self.inner |= Self::AURAS_2;
        *self
    }

    pub fn clear_AURAS_2(&mut self) -> Self {
        self.inner &= Self::AURAS_2.reverse_bits();
        *self
    }

    pub const fn is_PET_GUID(&self) -> bool {
        (self.inner & Self::PET_GUID) != 0
    }

    /// uint64 pet guid
    ///
    pub const fn new_PET_GUID() -> Self {
        Self { inner: Self::PET_GUID }
    }

    pub fn set_PET_GUID(&mut self) -> Self {
        self.inner |= Self::PET_GUID;
        *self
    }

    pub fn clear_PET_GUID(&mut self) -> Self {
        self.inner &= Self::PET_GUID.reverse_bits();
        *self
    }

    pub const fn is_PET_NAME(&self) -> bool {
        (self.inner & Self::PET_NAME) != 0
    }

    /// pet name, NULL terminated string
    ///
    pub const fn new_PET_NAME() -> Self {
        Self { inner: Self::PET_NAME }
    }

    pub fn set_PET_NAME(&mut self) -> Self {
        self.inner |= Self::PET_NAME;
        *self
    }

    pub fn clear_PET_NAME(&mut self) -> Self {
        self.inner &= Self::PET_NAME.reverse_bits();
        *self
    }

    pub const fn is_PET_MODEL_ID(&self) -> bool {
        (self.inner & Self::PET_MODEL_ID) != 0
    }

    /// uint16, model id
    ///
    pub const fn new_PET_MODEL_ID() -> Self {
        Self { inner: Self::PET_MODEL_ID }
    }

    pub fn set_PET_MODEL_ID(&mut self) -> Self {
        self.inner |= Self::PET_MODEL_ID;
        *self
    }

    pub fn clear_PET_MODEL_ID(&mut self) -> Self {
        self.inner &= Self::PET_MODEL_ID.reverse_bits();
        *self
    }

    pub const fn is_PET_CUR_HP(&self) -> bool {
        (self.inner & Self::PET_CUR_HP) != 0
    }

    /// uint16 pet cur health
    ///
    pub const fn new_PET_CUR_HP() -> Self {
        Self { inner: Self::PET_CUR_HP }
    }

    pub fn set_PET_CUR_HP(&mut self) -> Self {
        self.inner |= Self::PET_CUR_HP;
        *self
    }

    pub fn clear_PET_CUR_HP(&mut self) -> Self {
        self.inner &= Self::PET_CUR_HP.reverse_bits();
        *self
    }

    pub const fn is_PET_MAX_HP(&self) -> bool {
        (self.inner & Self::PET_MAX_HP) != 0
    }

    /// uint16 pet max health
    ///
    pub const fn new_PET_MAX_HP() -> Self {
        Self { inner: Self::PET_MAX_HP }
    }

    pub fn set_PET_MAX_HP(&mut self) -> Self {
        self.inner |= Self::PET_MAX_HP;
        *self
    }

    pub fn clear_PET_MAX_HP(&mut self) -> Self {
        self.inner &= Self::PET_MAX_HP.reverse_bits();
        *self
    }

    pub const fn is_PET_POWER_TYPE(&self) -> bool {
        (self.inner & Self::PET_POWER_TYPE) != 0
    }

    /// uint8 pet power type
    ///
    pub const fn new_PET_POWER_TYPE() -> Self {
        Self { inner: Self::PET_POWER_TYPE }
    }

    pub fn set_PET_POWER_TYPE(&mut self) -> Self {
        self.inner |= Self::PET_POWER_TYPE;
        *self
    }

    pub fn clear_PET_POWER_TYPE(&mut self) -> Self {
        self.inner &= Self::PET_POWER_TYPE.reverse_bits();
        *self
    }

    pub const fn is_PET_CUR_POWER(&self) -> bool {
        (self.inner & Self::PET_CUR_POWER) != 0
    }

    /// uint16 pet cur power
    ///
    pub const fn new_PET_CUR_POWER() -> Self {
        Self { inner: Self::PET_CUR_POWER }
    }

    pub fn set_PET_CUR_POWER(&mut self) -> Self {
        self.inner |= Self::PET_CUR_POWER;
        *self
    }

    pub fn clear_PET_CUR_POWER(&mut self) -> Self {
        self.inner &= Self::PET_CUR_POWER.reverse_bits();
        *self
    }

    pub const fn is_PET_MAX_POWER(&self) -> bool {
        (self.inner & Self::PET_MAX_POWER) != 0
    }

    /// uint16 pet max power
    ///
    pub const fn new_PET_MAX_POWER() -> Self {
        Self { inner: Self::PET_MAX_POWER }
    }

    pub fn set_PET_MAX_POWER(&mut self) -> Self {
        self.inner |= Self::PET_MAX_POWER;
        *self
    }

    pub fn clear_PET_MAX_POWER(&mut self) -> Self {
        self.inner &= Self::PET_MAX_POWER.reverse_bits();
        *self
    }

    pub const fn is_PET_AURAS(&self) -> bool {
        (self.inner & Self::PET_AURAS) != 0
    }

    /// uint32 mask, for each bit set uint16 spellid, pet auras...
    ///
    pub const fn new_PET_AURAS() -> Self {
        Self { inner: Self::PET_AURAS }
    }

    pub fn set_PET_AURAS(&mut self) -> Self {
        self.inner |= Self::PET_AURAS;
        *self
    }

    pub fn clear_PET_AURAS(&mut self) -> Self {
        self.inner &= Self::PET_AURAS.reverse_bits();
        *self
    }

    pub const fn is_PET_AURAS_2(&self) -> bool {
        (self.inner & Self::PET_AURAS_2) != 0
    }

    /// uint16 above mask continuation, giving max total of 48 auras possible
    ///
    pub const fn new_PET_AURAS_2() -> Self {
        Self { inner: Self::PET_AURAS_2 }
    }

    pub fn set_PET_AURAS_2(&mut self) -> Self {
        self.inner |= Self::PET_AURAS_2;
        *self
    }

    pub fn clear_PET_AURAS_2(&mut self) -> Self {
        self.inner &= Self::PET_AURAS_2.reverse_bits();
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

