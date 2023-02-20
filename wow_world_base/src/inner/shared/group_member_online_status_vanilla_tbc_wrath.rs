/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/social_common.wowm:277`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/social_common.wowm#L277):
/// ```text
/// flag GroupMemberOnlineStatus : u8 {
///     OFFLINE = 0x0000;
///     ONLINE = 0x0001;
///     PVP = 0x0002;
///     DEAD = 0x0004;
///     GHOST = 0x0008;
///     PVP_FFA = 0x0010;
///     ZONE_OUT = 0x0020;
///     AFK = 0x0040;
///     DND = 0x0080;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct GroupMemberOnlineStatus {
    inner: u8,
}

impl GroupMemberOnlineStatus {
    pub const fn new(inner: u8) -> Self {
        Self { inner }
    }

    pub const OFFLINE: u8 = 0x00;
    pub const ONLINE: u8 = 0x01;
    pub const PVP: u8 = 0x02;
    pub const DEAD: u8 = 0x04;
    pub const GHOST: u8 = 0x08;
    pub const PVP_FFA: u8 = 0x10;
    pub const ZONE_OUT: u8 = 0x20;
    pub const AFK: u8 = 0x40;
    pub const DND: u8 = 0x80;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::OFFLINE
                | Self::ONLINE
                | Self::PVP
                | Self::DEAD
                | Self::GHOST
                | Self::PVP_FFA
                | Self::ZONE_OUT
                | Self::AFK
                | Self::DND
        }
    }

    pub const fn is_ONLINE(&self) -> bool {
        (self.inner & Self::ONLINE) != 0
    }

    /// `Lua_UnitIsConnected`
    ///
    pub const fn new_ONLINE() -> Self {
        Self { inner: Self::ONLINE }
    }

    pub fn set_ONLINE(&mut self) -> Self {
        self.inner |= Self::ONLINE;
        *self
    }

    pub fn clear_ONLINE(&mut self) -> Self {
        self.inner &= Self::ONLINE.reverse_bits();
        *self
    }

    pub const fn is_PVP(&self) -> bool {
        (self.inner & Self::PVP) != 0
    }

    /// `Lua_UnitIsPVP`
    ///
    pub const fn new_PVP() -> Self {
        Self { inner: Self::PVP }
    }

    pub fn set_PVP(&mut self) -> Self {
        self.inner |= Self::PVP;
        *self
    }

    pub fn clear_PVP(&mut self) -> Self {
        self.inner &= Self::PVP.reverse_bits();
        *self
    }

    pub const fn is_DEAD(&self) -> bool {
        (self.inner & Self::DEAD) != 0
    }

    /// `Lua_UnitIsDead`
    ///
    pub const fn new_DEAD() -> Self {
        Self { inner: Self::DEAD }
    }

    pub fn set_DEAD(&mut self) -> Self {
        self.inner |= Self::DEAD;
        *self
    }

    pub fn clear_DEAD(&mut self) -> Self {
        self.inner &= Self::DEAD.reverse_bits();
        *self
    }

    pub const fn is_GHOST(&self) -> bool {
        (self.inner & Self::GHOST) != 0
    }

    /// `Lua_UnitIsGhost`
    ///
    pub const fn new_GHOST() -> Self {
        Self { inner: Self::GHOST }
    }

    pub fn set_GHOST(&mut self) -> Self {
        self.inner |= Self::GHOST;
        *self
    }

    pub fn clear_GHOST(&mut self) -> Self {
        self.inner &= Self::GHOST.reverse_bits();
        *self
    }

    pub const fn is_PVP_FFA(&self) -> bool {
        (self.inner & Self::PVP_FFA) != 0
    }

    /// `Lua_UnitIsPVPFreeForAll`
    ///
    pub const fn new_PVP_FFA() -> Self {
        Self { inner: Self::PVP_FFA }
    }

    pub fn set_PVP_FFA(&mut self) -> Self {
        self.inner |= Self::PVP_FFA;
        *self
    }

    pub fn clear_PVP_FFA(&mut self) -> Self {
        self.inner &= Self::PVP_FFA.reverse_bits();
        *self
    }

    pub const fn is_ZONE_OUT(&self) -> bool {
        (self.inner & Self::ZONE_OUT) != 0
    }

    /// used in calls from `Lua_GetPlayerMapPosition`/`Lua_GetBattlefieldFlagPosition`
    ///
    pub const fn new_ZONE_OUT() -> Self {
        Self { inner: Self::ZONE_OUT }
    }

    pub fn set_ZONE_OUT(&mut self) -> Self {
        self.inner |= Self::ZONE_OUT;
        *self
    }

    pub fn clear_ZONE_OUT(&mut self) -> Self {
        self.inner &= Self::ZONE_OUT.reverse_bits();
        *self
    }

    pub const fn is_AFK(&self) -> bool {
        (self.inner & Self::AFK) != 0
    }

    /// `Lua_UnitIsAFK`
    ///
    pub const fn new_AFK() -> Self {
        Self { inner: Self::AFK }
    }

    pub fn set_AFK(&mut self) -> Self {
        self.inner |= Self::AFK;
        *self
    }

    pub fn clear_AFK(&mut self) -> Self {
        self.inner &= Self::AFK.reverse_bits();
        *self
    }

    pub const fn is_DND(&self) -> bool {
        (self.inner & Self::DND) != 0
    }

    /// `Lua_UnitIsDND`
    ///
    pub const fn new_DND() -> Self {
        Self { inner: Self::DND }
    }

    pub fn set_DND(&mut self) -> Self {
        self.inner |= Self::DND;
        *self
    }

    pub fn clear_DND(&mut self) -> Self {
        self.inner &= Self::DND.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u8 {
        self.inner
    }

}

impl std::fmt::UpperHex for GroupMemberOnlineStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for GroupMemberOnlineStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for GroupMemberOnlineStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for GroupMemberOnlineStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

impl std::ops::BitAnd for GroupMemberOnlineStatus {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitand(rhs.inner), }
    }
}

impl std::ops::BitAndAssign for GroupMemberOnlineStatus {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner)
    }
}

impl std::ops::BitOr for GroupMemberOnlineStatus {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitor(rhs.inner), }
    }
}

impl std::ops::BitOrAssign for GroupMemberOnlineStatus {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner)
    }
}

impl std::ops::BitXor for GroupMemberOnlineStatus {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitxor(rhs.inner), }
    }
}

impl std::ops::BitXorAssign for GroupMemberOnlineStatus {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner)
    }
}

