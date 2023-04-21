use std::fmt::Formatter;
use std::ops::{Add, AddAssign, Sub, SubAssign};

/// Representation of money with helper methods.
///
/// All operations are saturating since money would never be expected to wrap.
/// So `u32::MAX + 1 == u32::MAX` and `u32::MIN - 1 == u32::MIN`.
#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Gold {
    inner: u32,
}

impl Gold {
    const COPPER_PER_SILVER: u32 = 100;
    const COPPER_PER_GOLD: u32 = 10_000;

    /// Creates a new object from amount of coppers.
    ///
    /// 100 would be one silver and `10_000` would be one gold.
    ///
    /// You should probably use either [`Self::from_parts`], [`Self::from_copper`],
    /// [`Self::from_silver`], or [`Self::from_gold`] instead.
    ///
    /// Same as [`Self::from_copper`].
    pub const fn new(value: u32) -> Self {
        Self { inner: value }
    }

    /// Creates a new object from separate parts.
    ///
    /// Variables will roll over, so supplying 100 `silver` will equal to 1 `gold`.
    /// Supplying 1 `gold` and and 100 `silver` will equal 2 `gold`.
    ///
    /// Get the values with [`Self::as_parts`].
    pub const fn from_parts(gold: u32, silver: u32, copper: u32) -> Self {
        let gold = gold.saturating_mul(Self::COPPER_PER_GOLD);
        let silver = silver.saturating_mul(Self::COPPER_PER_SILVER);

        Self::new(copper.saturating_add(silver).saturating_add(gold))
    }

    /// Convenience function for creation.
    pub const fn from_copper(value: u32) -> Self {
        Self::new(value)
    }

    /// Convenience function for creation.
    pub const fn from_silver(value: u32) -> Self {
        Self::new(value.saturating_mul(Self::COPPER_PER_SILVER))
    }

    /// Convenience function for creation.
    pub const fn from_gold(value: u32) -> Self {
        Self::new(value.saturating_mul(Self::COPPER_PER_GOLD))
    }

    /// Creates a new object with value zero.
    pub const fn zero() -> Self {
        Self::new(0)
    }

    /// Returns the underlying integer type.
    pub const fn as_int(&self) -> u32 {
        self.inner
    }

    /// Returns a tuple with values in a `(`[`Self::gold`], [`Self::silver`], [`Self::copper`]`)` tuple.
    pub const fn as_parts(&self) -> (u32, u32, u32) {
        (self.gold(), self.silver(), self.copper())
    }

    /// Returns the amount of gold.
    pub const fn gold(&self) -> u32 {
        self.inner / Self::COPPER_PER_GOLD
    }

    /// Returns the amount of silver in the range of 0 to 99.
    pub const fn silver(&self) -> u32 {
        (self.inner / Self::COPPER_PER_SILVER) % Self::COPPER_PER_SILVER
    }

    /// Returns the amount of copper in the range of 0 to 99.
    pub const fn copper(&self) -> u32 {
        self.inner % Self::COPPER_PER_SILVER
    }

    /// Explicitly checked function for when you don't want saturating.
    pub const fn checked_add(self, rhs: Self) -> Option<Self> {
        if let Some(v) = self.inner.checked_add(rhs.inner) {
            Some(Self::new(v))
        } else {
            None
        }
    }

    /// Explicitly checked function for when you don't want saturating.
    pub const fn checked_sub(self, rhs: Self) -> Option<Self> {
        if let Some(v) = self.inner.checked_sub(rhs.inner) {
            Some(Self::new(v))
        } else {
            None
        }
    }
}

impl std::fmt::Display for Gold {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (gold, silver, copper) = self.as_parts();
        write!(f, "{}g {}s {}c", gold, silver, copper)
    }
}

impl Add for Gold {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.inner.saturating_add(rhs.inner))
    }
}

impl AddAssign for Gold {
    fn add_assign(&mut self, rhs: Self) {
        // Do not use u32::add_assign since it doesn't saturate
        let i = self.inner.saturating_add(rhs.inner);
        self.inner = i;
    }
}

impl Sub for Gold {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.inner.saturating_sub(rhs.inner))
    }
}

impl SubAssign for Gold {
    fn sub_assign(&mut self, rhs: Self) {
        // Do not use u32::sub_assign since it doesn't saturate
        let i = self.inner.saturating_sub(rhs.inner);
        self.inner = i;
    }
}

impl From<u8> for Gold {
    fn from(value: u8) -> Self {
        Self::new(value.into())
    }
}

impl From<u16> for Gold {
    fn from(value: u16) -> Self {
        Self::new(value.into())
    }
}

impl From<u32> for Gold {
    fn from(value: u32) -> Self {
        Self::new(value)
    }
}

impl From<Gold> for u32 {
    fn from(value: Gold) -> Self {
        value.as_int()
    }
}

#[cfg(test)]
mod test {
    use crate::shared::gold_vanilla_tbc_wrath::Gold;

    #[test]
    fn as_parts() {
        let tests = [
            (0, (0, 0, 0)),
            (1_00, (0, 1, 0)),
            (1_27, (0, 1, 27)),
            (1_01, (0, 1, 1)),
            (1_99, (0, 1, 99)),
            (2_00, (0, 2, 0)),
            (10_00, (0, 10, 0)),
            (1_00_00, (1, 0, 0)),
            (1_00_03, (1, 0, 3)),
            (1_01_03, (1, 1, 3)),
            (1_327_621, (132, 76, 21)),
            (1_320_000, (132, 0, 0)),
        ];

        for (inner, parts) in tests {
            let g = Gold::new(inner);
            assert_eq!(g.as_parts(), parts);
        }
    }

    #[test]
    fn operators() {
        let s = Gold::from_silver(90);
        let mut g = Gold::from_gold(1);
        assert_eq!(Gold::from_gold(2), g + g);
        assert_eq!(Gold::from_parts(1, 90, 0), g + s);

        g += Gold::from_silver(90);
        assert_eq!(g, Gold::from_parts(1, 90, 0));

        g -= Gold::from_silver(90);
        assert_eq!(g, Gold::from_parts(1, 0, 0));
    }

    #[test]
    fn overflowing_arithmetic() {
        assert_eq!(Gold::from_silver(u32::MAX), Gold::new(u32::MAX));
        assert_eq!(Gold::from_gold(u32::MAX), Gold::new(u32::MAX));
        assert_eq!(Gold::from_copper(u32::MAX), Gold::new(u32::MAX));
        assert_eq!(
            Gold::from_parts(u32::MAX, u32::MAX, u32::MAX),
            Gold::new(u32::MAX)
        );

        assert_eq!(
            Gold::from_gold(1) + Gold::from_gold(u32::MAX),
            Gold::new(u32::MAX)
        );
        assert_eq!(
            Gold::from_gold(1) - Gold::from_gold(u32::MAX),
            Gold::new(u32::MIN)
        );

        let mut g = Gold::from_gold(1);
        g += Gold::new(u32::MAX);
        assert_eq!(g, Gold::new(u32::MAX));

        let mut g = Gold::from_gold(1);
        g -= Gold::new(u32::MAX);
        assert_eq!(g, Gold::new(u32::MIN));
    }
}
