use std::fmt::Formatter;

#[derive(std::fmt::Debug, Ord, PartialOrd, Eq, PartialEq, Default, Hash, Copy, Clone)]
pub struct Guid {
    guid: u64,
}

impl Guid {
    pub const fn new(guid: u64) -> Self {
        Self { guid }
    }

    /// Guid with 0 value.
    ///
    /// Client uses this to mean different things, including things like no target selected.
    pub const fn zero() -> Self {
        Self::new(0)
    }

    pub const fn is_zero(&self) -> bool {
        self.guid == 0
    }

    pub const fn guid(&self) -> u64 {
        self.guid
    }

    /// Returns the guid separated into the low and high u32s.
    ///
    /// Returns a tuple containing `(low, high)`.
    pub const fn to_u32s(&self) -> (u32, u32) {
        let lower = self.guid as u32;
        let upper = (self.guid >> 32) as u32;

        (lower, upper)
    }

    /// Creates guid from low and high u32s.
    pub const fn from_u32s(lower: u32, upper: u32) -> Self {
        let lower = lower as u64;
        let upper = upper as u64;
        Self {
            guid: upper << 32 | lower,
        }
    }
}

impl From<u64> for Guid {
    fn from(v: u64) -> Self {
        Self::new(v)
    }
}

impl std::fmt::Display for Guid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.guid().fmt(f)
    }
}

impl std::fmt::LowerHex for Guid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.guid().fmt(f)
    }
}

impl std::fmt::UpperHex for Guid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.guid().fmt(f)
    }
}

impl std::fmt::Octal for Guid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.guid().fmt(f)
    }
}

impl std::fmt::Binary for Guid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.guid().fmt(f)
    }
}

#[cfg(test)]
mod test {
    use crate::vanilla::Guid;

    #[test]
    fn packed() {
        const GUID: u64 = 0xDEADBEEFFACADE;
        const LOW: u32 = 0xEFFACADE;
        const HIGH: u32 = 0xDEADBE;

        let guid = Guid::new(GUID);
        assert_eq!(GUID, guid.guid());
        assert!(!guid.is_zero());

        let zero = Guid::zero();
        assert_eq!(0, zero.guid());
        assert!(zero.is_zero());

        let from_parts = Guid::from_u32s(LOW, HIGH);
        assert_eq!(guid, from_parts);
        assert_eq!(guid.guid(), from_parts.guid());
        assert!(!from_parts.is_zero());

        let (low, high) = from_parts.to_u32s();
        assert_eq!(low, LOW);
        assert_eq!(high, HIGH);
    }
}
