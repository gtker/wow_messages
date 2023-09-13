/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gameobject/smsg_update_object_3_3_5.wowm:12`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gameobject/smsg_update_object_3_3_5.wowm#L12):
/// ```text
/// flag UpdateFlag : u16 {
///     NONE = 0x0000;
///     SELF = 0x0001;
///     TRANSPORT = 0x0002;
///     HAS_ATTACKING_TARGET = 0x0004;
///     LOW_GUID = 0x0008;
///     HIGH_GUID = 0x0010;
///     LIVING = 0x0020;
///     HAS_POSITION = 0x0040;
///     VEHICLE = 0x0080;
///     POSITION = 0x0100;
///     ROTATION = 0x0200;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct UpdateFlag {
    inner: u16,
}

#[cfg(feature = "print-testcase")]
impl UpdateFlag {
    #[allow(clippy::missing_const_for_fn)]
    pub fn as_test_case_value(&self) -> String {
        let mut s = String::new();
        let mut first = true;
        if self.is_empty() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "NONE").unwrap();
            first = false;
        }
        if self.is_self() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "SELF").unwrap();
            first = false;
        }
        if self.is_transport() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "TRANSPORT").unwrap();
            first = false;
        }
        if self.is_has_attacking_target() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "HAS_ATTACKING_TARGET").unwrap();
            first = false;
        }
        if self.is_low_guid() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "LOW_GUID").unwrap();
            first = false;
        }
        if self.is_high_guid() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "HIGH_GUID").unwrap();
            first = false;
        }
        if self.is_living() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "LIVING").unwrap();
            first = false;
        }
        if self.is_has_position() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "HAS_POSITION").unwrap();
            first = false;
        }
        if self.is_vehicle() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "VEHICLE").unwrap();
            first = false;
        }
        if self.is_position() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "POSITION").unwrap();
            first = false;
        }
        if self.is_rotation() {
            use std::fmt::Write;
            if !first {
                write!(s, " | ").unwrap();
            }
            write!(s, "ROTATION").unwrap();
            first = false;
        }
        s
    }

}

impl UpdateFlag {
    pub const fn new(inner: u16) -> Self {
        Self { inner }
    }

    pub const NONE: u16 = 0x00;
    pub const SELF: u16 = 0x01;
    pub const TRANSPORT: u16 = 0x02;
    pub const HAS_ATTACKING_TARGET: u16 = 0x04;
    pub const LOW_GUID: u16 = 0x08;
    pub const HIGH_GUID: u16 = 0x10;
    pub const LIVING: u16 = 0x20;
    pub const HAS_POSITION: u16 = 0x40;
    pub const VEHICLE: u16 = 0x80;
    pub const POSITION: u16 = 0x100;
    pub const ROTATION: u16 = 0x200;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::NONE
                | Self::SELF
                | Self::TRANSPORT
                | Self::HAS_ATTACKING_TARGET
                | Self::LOW_GUID
                | Self::HIGH_GUID
                | Self::LIVING
                | Self::HAS_POSITION
                | Self::VEHICLE
                | Self::POSITION
                | Self::ROTATION
        }
    }

    pub const fn is_self(&self) -> bool {
        (self.inner & Self::SELF) != 0
    }

    pub const fn new_self() -> Self {
        Self { inner: Self::SELF }
    }

    pub fn set_self(&mut self) -> Self {
        self.inner |= Self::SELF;
        *self
    }

    pub fn clear_self(&mut self) -> Self {
        self.inner &= Self::SELF.reverse_bits();
        *self
    }

    pub const fn is_transport(&self) -> bool {
        (self.inner & Self::TRANSPORT) != 0
    }

    pub const fn new_transport() -> Self {
        Self { inner: Self::TRANSPORT }
    }

    pub fn set_transport(&mut self) -> Self {
        self.inner |= Self::TRANSPORT;
        *self
    }

    pub fn clear_transport(&mut self) -> Self {
        self.inner &= Self::TRANSPORT.reverse_bits();
        *self
    }

    pub const fn is_has_attacking_target(&self) -> bool {
        (self.inner & Self::HAS_ATTACKING_TARGET) != 0
    }

    pub const fn new_has_attacking_target() -> Self {
        Self { inner: Self::HAS_ATTACKING_TARGET }
    }

    pub fn set_has_attacking_target(&mut self) -> Self {
        self.inner |= Self::HAS_ATTACKING_TARGET;
        *self
    }

    pub fn clear_has_attacking_target(&mut self) -> Self {
        self.inner &= Self::HAS_ATTACKING_TARGET.reverse_bits();
        *self
    }

    pub const fn is_low_guid(&self) -> bool {
        (self.inner & Self::LOW_GUID) != 0
    }

    pub const fn new_low_guid() -> Self {
        Self { inner: Self::LOW_GUID }
    }

    pub fn set_low_guid(&mut self) -> Self {
        self.inner |= Self::LOW_GUID;
        *self
    }

    pub fn clear_low_guid(&mut self) -> Self {
        self.inner &= Self::LOW_GUID.reverse_bits();
        *self
    }

    pub const fn is_high_guid(&self) -> bool {
        (self.inner & Self::HIGH_GUID) != 0
    }

    pub const fn new_high_guid() -> Self {
        Self { inner: Self::HIGH_GUID }
    }

    pub fn set_high_guid(&mut self) -> Self {
        self.inner |= Self::HIGH_GUID;
        *self
    }

    pub fn clear_high_guid(&mut self) -> Self {
        self.inner &= Self::HIGH_GUID.reverse_bits();
        *self
    }

    pub const fn is_living(&self) -> bool {
        (self.inner & Self::LIVING) != 0
    }

    pub const fn new_living() -> Self {
        Self { inner: Self::LIVING }
    }

    pub fn set_living(&mut self) -> Self {
        self.inner |= Self::LIVING;
        *self
    }

    pub fn clear_living(&mut self) -> Self {
        self.inner &= Self::LIVING.reverse_bits();
        *self
    }

    pub const fn is_has_position(&self) -> bool {
        (self.inner & Self::HAS_POSITION) != 0
    }

    pub const fn new_has_position() -> Self {
        Self { inner: Self::HAS_POSITION }
    }

    pub fn set_has_position(&mut self) -> Self {
        self.inner |= Self::HAS_POSITION;
        *self
    }

    pub fn clear_has_position(&mut self) -> Self {
        self.inner &= Self::HAS_POSITION.reverse_bits();
        *self
    }

    pub const fn is_vehicle(&self) -> bool {
        (self.inner & Self::VEHICLE) != 0
    }

    pub const fn new_vehicle() -> Self {
        Self { inner: Self::VEHICLE }
    }

    pub fn set_vehicle(&mut self) -> Self {
        self.inner |= Self::VEHICLE;
        *self
    }

    pub fn clear_vehicle(&mut self) -> Self {
        self.inner &= Self::VEHICLE.reverse_bits();
        *self
    }

    pub const fn is_position(&self) -> bool {
        (self.inner & Self::POSITION) != 0
    }

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

    pub const fn is_rotation(&self) -> bool {
        (self.inner & Self::ROTATION) != 0
    }

    pub const fn new_rotation() -> Self {
        Self { inner: Self::ROTATION }
    }

    pub fn set_rotation(&mut self) -> Self {
        self.inner |= Self::ROTATION;
        *self
    }

    pub fn clear_rotation(&mut self) -> Self {
        self.inner &= Self::ROTATION.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u16 {
        self.inner
    }

}

impl std::fmt::UpperHex for UpdateFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for UpdateFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for UpdateFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for UpdateFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

impl std::ops::BitAnd for UpdateFlag {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitand(rhs.inner), }
    }
}

impl std::ops::BitAndAssign for UpdateFlag {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner)
    }
}

impl std::ops::BitOr for UpdateFlag {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitor(rhs.inner), }
    }
}

impl std::ops::BitOrAssign for UpdateFlag {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner)
    }
}

impl std::ops::BitXor for UpdateFlag {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitxor(rhs.inner), }
    }
}

impl std::ops::BitXorAssign for UpdateFlag {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner)
    }
}

impl From<u16> for UpdateFlag {
    fn from(value: u16) -> Self {
        Self::new(value)
    }
}

impl From<u8> for UpdateFlag {
    fn from(value: u8) -> Self {
        Self::new(value.into())
    }
}

impl TryFrom<u32> for UpdateFlag {
    type Error = u32;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        let a = TryInto::<u16>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<u64> for UpdateFlag {
    type Error = u64;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        let a = TryInto::<u16>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i8> for UpdateFlag {
    type Error = i8;
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        let v = u8::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u16>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl From<i16> for UpdateFlag {
    fn from(value: i16) -> Self {
        Self::new(u16::from_le_bytes(value.to_le_bytes()))
    }
}

impl TryFrom<i32> for UpdateFlag {
    type Error = i32;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let v = u32::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u16>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<i64> for UpdateFlag {
    type Error = i64;
    fn try_from(value: i64) -> Result<Self, Self::Error> {
        let v = u64::from_le_bytes(value.to_le_bytes());
        let a = TryInto::<u16>::try_into(v).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

impl TryFrom<usize> for UpdateFlag {
    type Error = usize;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let a = TryInto::<u16>::try_into(value).ok().ok_or(value)?;
        Ok(Self::new(a))
    }
}

