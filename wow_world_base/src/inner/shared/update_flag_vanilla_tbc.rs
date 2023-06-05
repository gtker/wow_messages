/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/gameobject/smsg_update_object.wowm:14`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/gameobject/smsg_update_object.wowm#L14):
/// ```text
/// flag UpdateFlag : u8 {
///     NONE = 0x00;
///     SELF = 0x01;
///     TRANSPORT = 0x02;
///     MELEE_ATTACKING = 0x04;
///     HIGH_GUID = 0x08;
///     ALL = 0x10;
///     LIVING = 0x20;
///     HAS_POSITION = 0x40;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct UpdateFlag {
    inner: u8,
}

#[cfg(feature = "print-testcase")]
impl UpdateFlag {
    pub fn as_test_case_value(&self) -> String {
        let mut s = String::new();
        let mut first = true;
        if self.is_empty() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "NONE").unwrap();
            first = false;
        }
        if self.is_self() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "SELF").unwrap();
            first = false;
        }
        if self.is_transport() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "TRANSPORT").unwrap();
            first = false;
        }
        if self.is_melee_attacking() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "MELEE_ATTACKING").unwrap();
            first = false;
        }
        if self.is_high_guid() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "HIGH_GUID").unwrap();
            first = false;
        }
        if self.is_all() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "ALL").unwrap();
            first = false;
        }
        if self.is_living() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "LIVING").unwrap();
            first = false;
        }
        if self.is_has_position() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "HAS_POSITION").unwrap();
            first = false;
        }
        s
    }

}

impl UpdateFlag {
    pub const fn new(inner: u8) -> Self {
        Self { inner }
    }

    pub const NONE: u8 = 0x00;
    pub const SELF: u8 = 0x01;
    pub const TRANSPORT: u8 = 0x02;
    pub const MELEE_ATTACKING: u8 = 0x04;
    pub const HIGH_GUID: u8 = 0x08;
    pub const ALL: u8 = 0x10;
    pub const LIVING: u8 = 0x20;
    pub const HAS_POSITION: u8 = 0x40;

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
                | Self::MELEE_ATTACKING
                | Self::HIGH_GUID
                | Self::ALL
                | Self::LIVING
                | Self::HAS_POSITION
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

    pub const fn is_melee_attacking(&self) -> bool {
        (self.inner & Self::MELEE_ATTACKING) != 0
    }

    pub const fn new_melee_attacking() -> Self {
        Self { inner: Self::MELEE_ATTACKING }
    }

    pub fn set_melee_attacking(&mut self) -> Self {
        self.inner |= Self::MELEE_ATTACKING;
        *self
    }

    pub fn clear_melee_attacking(&mut self) -> Self {
        self.inner &= Self::MELEE_ATTACKING.reverse_bits();
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

    pub const fn is_all(&self) -> bool {
        (self.inner & Self::ALL) != 0
    }

    pub const fn new_all() -> Self {
        Self { inner: Self::ALL }
    }

    pub fn set_all(&mut self) -> Self {
        self.inner |= Self::ALL;
        *self
    }

    pub fn clear_all(&mut self) -> Self {
        self.inner &= Self::ALL.reverse_bits();
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

    pub const fn as_int(&self) -> u8 {
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

