/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/social_common.wowm:427`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/social_common.wowm#L427):
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

#[cfg(feature = "print-testcase")]
impl GroupMemberOnlineStatus {
    #[allow(clippy::missing_const_for_fn)]
    pub fn as_test_case_value(&self) -> String {
        let mut s = String::new();
        let mut first = true;
        if self.is_empty() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "OFFLINE").unwrap();
            first = false;
        }
        if self.is_online() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "ONLINE").unwrap();
            first = false;
        }
        if self.is_pvp() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "PVP").unwrap();
            first = false;
        }
        if self.is_dead() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "DEAD").unwrap();
            first = false;
        }
        if self.is_ghost() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "GHOST").unwrap();
            first = false;
        }
        if self.is_pvp_ffa() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "PVP_FFA").unwrap();
            first = false;
        }
        if self.is_zone_out() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "ZONE_OUT").unwrap();
            first = false;
        }
        if self.is_afk() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "AFK").unwrap();
            first = false;
        }
        if self.is_dnd() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "DND").unwrap();
            first = false;
        }
        s
    }

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

    pub const fn is_online(&self) -> bool {
        (self.inner & Self::ONLINE) != 0
    }

    /// `Lua_UnitIsConnected`
    pub const fn new_online() -> Self {
        Self { inner: Self::ONLINE }
    }

    pub fn set_online(&mut self) -> Self {
        self.inner |= Self::ONLINE;
        *self
    }

    pub fn clear_online(&mut self) -> Self {
        self.inner &= Self::ONLINE.reverse_bits();
        *self
    }

    pub const fn is_pvp(&self) -> bool {
        (self.inner & Self::PVP) != 0
    }

    /// `Lua_UnitIsPVP`
    pub const fn new_pvp() -> Self {
        Self { inner: Self::PVP }
    }

    pub fn set_pvp(&mut self) -> Self {
        self.inner |= Self::PVP;
        *self
    }

    pub fn clear_pvp(&mut self) -> Self {
        self.inner &= Self::PVP.reverse_bits();
        *self
    }

    pub const fn is_dead(&self) -> bool {
        (self.inner & Self::DEAD) != 0
    }

    /// `Lua_UnitIsDead`
    pub const fn new_dead() -> Self {
        Self { inner: Self::DEAD }
    }

    pub fn set_dead(&mut self) -> Self {
        self.inner |= Self::DEAD;
        *self
    }

    pub fn clear_dead(&mut self) -> Self {
        self.inner &= Self::DEAD.reverse_bits();
        *self
    }

    pub const fn is_ghost(&self) -> bool {
        (self.inner & Self::GHOST) != 0
    }

    /// `Lua_UnitIsGhost`
    pub const fn new_ghost() -> Self {
        Self { inner: Self::GHOST }
    }

    pub fn set_ghost(&mut self) -> Self {
        self.inner |= Self::GHOST;
        *self
    }

    pub fn clear_ghost(&mut self) -> Self {
        self.inner &= Self::GHOST.reverse_bits();
        *self
    }

    pub const fn is_pvp_ffa(&self) -> bool {
        (self.inner & Self::PVP_FFA) != 0
    }

    /// `Lua_UnitIsPVPFreeForAll`
    pub const fn new_pvp_ffa() -> Self {
        Self { inner: Self::PVP_FFA }
    }

    pub fn set_pvp_ffa(&mut self) -> Self {
        self.inner |= Self::PVP_FFA;
        *self
    }

    pub fn clear_pvp_ffa(&mut self) -> Self {
        self.inner &= Self::PVP_FFA.reverse_bits();
        *self
    }

    pub const fn is_zone_out(&self) -> bool {
        (self.inner & Self::ZONE_OUT) != 0
    }

    /// used in calls from `Lua_GetPlayerMapPosition`/`Lua_GetBattlefieldFlagPosition`
    pub const fn new_zone_out() -> Self {
        Self { inner: Self::ZONE_OUT }
    }

    pub fn set_zone_out(&mut self) -> Self {
        self.inner |= Self::ZONE_OUT;
        *self
    }

    pub fn clear_zone_out(&mut self) -> Self {
        self.inner &= Self::ZONE_OUT.reverse_bits();
        *self
    }

    pub const fn is_afk(&self) -> bool {
        (self.inner & Self::AFK) != 0
    }

    /// `Lua_UnitIsAFK`
    pub const fn new_afk() -> Self {
        Self { inner: Self::AFK }
    }

    pub fn set_afk(&mut self) -> Self {
        self.inner |= Self::AFK;
        *self
    }

    pub fn clear_afk(&mut self) -> Self {
        self.inner &= Self::AFK.reverse_bits();
        *self
    }

    pub const fn is_dnd(&self) -> bool {
        (self.inner & Self::DND) != 0
    }

    /// `Lua_UnitIsDND`
    pub const fn new_dnd() -> Self {
        Self { inner: Self::DND }
    }

    pub fn set_dnd(&mut self) -> Self {
        self.inner |= Self::DND;
        *self
    }

    pub fn clear_dnd(&mut self) -> Self {
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

impl From<u8> for GroupMemberOnlineStatus {
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}

impl TryFrom<u16> for GroupMemberOnlineStatus {
    type Error = u16;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        let a = TryInto::<u8>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<u32> for GroupMemberOnlineStatus {
    type Error = u32;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let a = TryInto::<u8>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<u64> for GroupMemberOnlineStatus {
    type Error = u64;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let a = TryInto::<u8>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl From<i8> for GroupMemberOnlineStatus {
    fn from(value: i8) -> Self {
        Self::new(u8::from_le_bytes(value.to_le_bytes()))
    }
}

impl TryFrom<i16> for GroupMemberOnlineStatus {
    type Error = i16;
    fn try_from(value: i16) -> Result<Self, Self::Error> {
        let v = u16::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u8>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i32> for GroupMemberOnlineStatus {
    type Error = i32;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let v = u32::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u8>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i64> for GroupMemberOnlineStatus {
    type Error = i64;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let v = u64::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u8>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<usize> for GroupMemberOnlineStatus {
    type Error = usize;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let a = TryInto::<u8>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

