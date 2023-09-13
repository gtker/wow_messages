/// Used in `FactionTemplate.dbc`.
///
/// This type is not sent over the network, but is used in the game in another way.
///
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/external/pvp_flags.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/external/pvp_flags.wowm#L1):
/// ```text
/// flag PvpFlags : u16 {
///     PVP_FLAGGED = 0x800;
///     ATTACK_PVPING_PLAYERS = 0x1000;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct PvpFlags {
    inner: u16,
}

#[cfg(feature = "print-testcase")]
impl PvpFlags {
    #[allow(clippy::missing_const_for_fn)]
    pub fn as_test_case_value(&self) -> String {
        let mut s = String::new();
        let mut first = true;
        if self.is_pvp_flagged() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "PVP_FLAGGED").unwrap();
            first = false;
        }
        if self.is_attack_pvping_players() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "ATTACK_PVPING_PLAYERS").unwrap();
            first = false;
        }
        s
    }

}

impl PvpFlags {
    pub const fn new(inner: u16) -> Self {
        Self { inner }
    }

    pub const PVP_FLAGGED: u16 = 0x800;
    pub const ATTACK_PVPING_PLAYERS: u16 = 0x1000;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::PVP_FLAGGED
                | Self::ATTACK_PVPING_PLAYERS
        }
    }

    pub const fn is_pvp_flagged(&self) -> bool {
        (self.inner & Self::PVP_FLAGGED) != 0
    }

    pub const fn new_pvp_flagged() -> Self {
        Self { inner: Self::PVP_FLAGGED }
    }

    pub fn set_pvp_flagged(&mut self) -> Self {
        self.inner |= Self::PVP_FLAGGED;
        *self
    }

    pub fn clear_pvp_flagged(&mut self) -> Self {
        self.inner &= Self::PVP_FLAGGED.reverse_bits();
        *self
    }

    pub const fn is_attack_pvping_players(&self) -> bool {
        (self.inner & Self::ATTACK_PVPING_PLAYERS) != 0
    }

    pub const fn new_attack_pvping_players() -> Self {
        Self { inner: Self::ATTACK_PVPING_PLAYERS }
    }

    pub fn set_attack_pvping_players(&mut self) -> Self {
        self.inner |= Self::ATTACK_PVPING_PLAYERS;
        *self
    }

    pub fn clear_attack_pvping_players(&mut self) -> Self {
        self.inner &= Self::ATTACK_PVPING_PLAYERS.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u16 {
        self.inner
    }

}

impl std::fmt::UpperHex for PvpFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for PvpFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for PvpFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for PvpFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

impl std::ops::BitAnd for PvpFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitand(rhs.inner), }
    }
}

impl std::ops::BitAndAssign for PvpFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner)
    }
}

impl std::ops::BitOr for PvpFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitor(rhs.inner), }
    }
}

impl std::ops::BitOrAssign for PvpFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner)
    }
}

impl std::ops::BitXor for PvpFlags {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitxor(rhs.inner), }
    }
}

impl std::ops::BitXorAssign for PvpFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner)
    }
}

impl From<u16> for PvpFlags {
    fn from(value: u16) -> Self {
        Self::new(value)
    }
}

impl From<u8> for PvpFlags {
    fn from(value: u8) -> Self {
        Self::new(value.into())
    }
}

impl TryFrom<u32> for PvpFlags {
    type Error = u32;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let a = TryInto::<u16>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<u64> for PvpFlags {
    type Error = u64;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let a = TryInto::<u16>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i8> for PvpFlags {
    type Error = i8;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u16>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl From<i16> for PvpFlags {
    fn from(value: i16) -> Self {
        Self::new(u16::from_le_bytes(value.to_le_bytes()))
    }
}

impl TryFrom<i32> for PvpFlags {
    type Error = i32;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let v = u32::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u16>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i64> for PvpFlags {
    type Error = i64;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let v = u64::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u16>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<usize> for PvpFlags {
    type Error = usize;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let a = TryInto::<u16>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

