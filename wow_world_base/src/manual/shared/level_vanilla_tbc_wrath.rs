use std::fmt::Formatter;
use std::ops::{Add, AddAssign, Sub, SubAssign};

/// Representation of a character or mob level.
///
/// All operations are saturating since a level would never be expected to wrap.
/// So `u8::MAX + 1 == u8::MAX` and `u8::MIN - 1 == u8::MIN`.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Level {
    inner: u8,
}

impl Level {
    pub const fn new(level: u8) -> Self {
        Self { inner: level }
    }

    pub const fn as_int(&self) -> u8 {
        self.inner
    }

    pub const fn new_player() -> Self {
        Self::new(1)
    }

    pub const fn zero() -> Self {
        Self::new(0)
    }

    pub const fn new_absolute_max() -> Self {
        Self { inner: u8::MAX }
    }
}

#[cfg(feature = "vanilla")]
impl Level {
    const VANILLA_MAX_LEVEL: u8 = 60;

    pub const fn new_vanilla_max_level_player() -> Self {
        Self {
            inner: Self::VANILLA_MAX_LEVEL,
        }
    }

    pub const fn new_vanilla_boss() -> Self {
        Self {
            inner: Self::VANILLA_MAX_LEVEL + 3,
        }
    }

    pub const fn is_vanilla_max_level_player(&self) -> bool {
        self.inner == Self::VANILLA_MAX_LEVEL
    }
}

#[cfg(feature = "tbc")]
impl Level {
    const TBC_MAX_LEVEL: u8 = 70;

    pub const fn new_tbc_max_level_player() -> Self {
        Self {
            inner: Self::TBC_MAX_LEVEL,
        }
    }

    pub const fn new_tbc_boss() -> Self {
        Self {
            inner: Self::TBC_MAX_LEVEL + 3,
        }
    }

    pub const fn is_tbc_max_level_player(&self) -> bool {
        self.inner == Self::TBC_MAX_LEVEL
    }
}

#[cfg(feature = "wrath")]
impl Level {
    const WRATH_MAX_LEVEL: u8 = 80;

    pub const fn new_wrath_max_level_player() -> Self {
        Self {
            inner: Self::WRATH_MAX_LEVEL,
        }
    }

    pub const fn new_wrath_boss() -> Self {
        Self {
            inner: Self::WRATH_MAX_LEVEL + 3,
        }
    }

    pub const fn is_wrath_max_level_player(&self) -> bool {
        self.inner == Self::WRATH_MAX_LEVEL
    }
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl From<u8> for Level {
    fn from(value: u8) -> Self {
        Self::new(value)
    }
}
impl From<Level> for u8 {
    fn from(value: Level) -> Self {
        value.as_int()
    }
}

impl Add for Level {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.inner.saturating_add(rhs.inner))
    }
}
impl AddAssign for Level {
    fn add_assign(&mut self, rhs: Self) {
        // Do not use u8::add_assign since it doesn't saturate
        let i = self.inner.saturating_add(rhs.inner);
        self.inner = i;
    }
}
impl Sub for Level {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.inner.saturating_sub(rhs.inner))
    }
}
impl SubAssign for Level {
    fn sub_assign(&mut self, rhs: Self) {
        // Do not use u8::sub_assign since it doesn't saturate
        let i = self.inner.saturating_sub(rhs.inner);
        self.inner = i;
    }
}

#[cfg(test)]
mod test {
    use super::Level;

    #[test]
    fn operators() {
        let s = Level::new(90);
        let mut g = Level::new(1);
        assert_eq!(Level::new(2), g + g);
        assert_eq!(Level::new(91), g + s);

        g += Level::new(90);
        assert_eq!(g, Level::new(91));

        g -= Level::new(90);
        assert_eq!(g, Level::new(1));
    }

    #[test]
    fn overflowing_arithmetic() {
        assert_eq!(Level::new(1) + Level::new(u8::MAX), Level::new(u8::MAX));
        assert_eq!(Level::new(1) - Level::new(u8::MAX), Level::new(u8::MIN));

        let mut g = Level::new(1);
        g += Level::new(u8::MAX);
        assert_eq!(g, Level::new(u8::MAX));

        let mut g = Level::new(1);
        g -= Level::new(u8::MAX);
        assert_eq!(g, Level::new(u8::MIN));
    }
}
