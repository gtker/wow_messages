/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/faction/cmsg_set_faction_atwar.wowm:35`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/faction/cmsg_set_faction_atwar.wowm#L35):
/// ```text
/// flag FactionFlag : u8 {
///     VISIBLE = 0x01;
///     AT_WAR = 0x02;
///     HIDDEN = 0x04;
///     INVISIBLE_FORCED = 0x08;
///     PEACE_FORCED = 0x10;
///     INACTIVE = 0x20;
///     RIVAL = 0x40;
///     SPECIAL = 0x80;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct FactionFlag {
    inner: u8,
}

#[cfg(feature = "print-testcase")]
impl FactionFlag {
    pub fn as_test_case_value(&self) -> String {
        let mut s = String::new();
        let mut first = true;
        if self.is_visible() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "VISIBLE").unwrap();
            first = false;
        }
        if self.is_at_war() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "AT_WAR").unwrap();
            first = false;
        }
        if self.is_hidden() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "HIDDEN").unwrap();
            first = false;
        }
        if self.is_invisible_forced() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "INVISIBLE_FORCED").unwrap();
            first = false;
        }
        if self.is_peace_forced() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "PEACE_FORCED").unwrap();
            first = false;
        }
        if self.is_inactive() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "INACTIVE").unwrap();
            first = false;
        }
        if self.is_rival() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "RIVAL").unwrap();
            first = false;
        }
        if self.is_special() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "SPECIAL").unwrap();
            first = false;
        }
        s
    }

}

impl FactionFlag {
    pub const fn new(inner: u8) -> Self {
        Self { inner }
    }

    pub const VISIBLE: u8 = 0x01;
    pub const AT_WAR: u8 = 0x02;
    pub const HIDDEN: u8 = 0x04;
    pub const INVISIBLE_FORCED: u8 = 0x08;
    pub const PEACE_FORCED: u8 = 0x10;
    pub const INACTIVE: u8 = 0x20;
    pub const RIVAL: u8 = 0x40;
    pub const SPECIAL: u8 = 0x80;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::VISIBLE
                | Self::AT_WAR
                | Self::HIDDEN
                | Self::INVISIBLE_FORCED
                | Self::PEACE_FORCED
                | Self::INACTIVE
                | Self::RIVAL
                | Self::SPECIAL
        }
    }

    pub const fn is_visible(&self) -> bool {
        (self.inner & Self::VISIBLE) != 0
    }

    /// makes visible in client (set or can be set at interaction with target of this faction)
    ///
    pub const fn new_visible() -> Self {
        Self { inner: Self::VISIBLE }
    }

    pub fn set_visible(&mut self) -> Self {
        self.inner |= Self::VISIBLE;
        *self
    }

    pub fn clear_visible(&mut self) -> Self {
        self.inner &= Self::VISIBLE.reverse_bits();
        *self
    }

    pub const fn is_at_war(&self) -> bool {
        (self.inner & Self::AT_WAR) != 0
    }

    /// enable AtWar-button in client. player controlled (except opposition team always war state), Flag only set on initial creation
    ///
    pub const fn new_at_war() -> Self {
        Self { inner: Self::AT_WAR }
    }

    pub fn set_at_war(&mut self) -> Self {
        self.inner |= Self::AT_WAR;
        *self
    }

    pub fn clear_at_war(&mut self) -> Self {
        self.inner &= Self::AT_WAR.reverse_bits();
        *self
    }

    pub const fn is_hidden(&self) -> bool {
        (self.inner & Self::HIDDEN) != 0
    }

    /// hidden faction from reputation pane in client (player can gain reputation, but this update not sent to client)
    ///
    pub const fn new_hidden() -> Self {
        Self { inner: Self::HIDDEN }
    }

    pub fn set_hidden(&mut self) -> Self {
        self.inner |= Self::HIDDEN;
        *self
    }

    pub fn clear_hidden(&mut self) -> Self {
        self.inner &= Self::HIDDEN.reverse_bits();
        *self
    }

    pub const fn is_invisible_forced(&self) -> bool {
        (self.inner & Self::INVISIBLE_FORCED) != 0
    }

    /// always overwrite `FACTION_FLAG_VISIBLE` and hide faction in rep.list, used for hide opposite team factions
    ///
    pub const fn new_invisible_forced() -> Self {
        Self { inner: Self::INVISIBLE_FORCED }
    }

    pub fn set_invisible_forced(&mut self) -> Self {
        self.inner |= Self::INVISIBLE_FORCED;
        *self
    }

    pub fn clear_invisible_forced(&mut self) -> Self {
        self.inner &= Self::INVISIBLE_FORCED.reverse_bits();
        *self
    }

    pub const fn is_peace_forced(&self) -> bool {
        (self.inner & Self::PEACE_FORCED) != 0
    }

    /// always overwrite `FACTION_FLAG_AT_WAR`, used for prevent war with own team factions
    ///
    pub const fn new_peace_forced() -> Self {
        Self { inner: Self::PEACE_FORCED }
    }

    pub fn set_peace_forced(&mut self) -> Self {
        self.inner |= Self::PEACE_FORCED;
        *self
    }

    pub fn clear_peace_forced(&mut self) -> Self {
        self.inner &= Self::PEACE_FORCED.reverse_bits();
        *self
    }

    pub const fn is_inactive(&self) -> bool {
        (self.inner & Self::INACTIVE) != 0
    }

    /// player controlled, state stored in characters.data ( `CMSG_SET_FACTION_INACTIVE` )
    ///
    pub const fn new_inactive() -> Self {
        Self { inner: Self::INACTIVE }
    }

    pub fn set_inactive(&mut self) -> Self {
        self.inner |= Self::INACTIVE;
        *self
    }

    pub fn clear_inactive(&mut self) -> Self {
        self.inner &= Self::INACTIVE.reverse_bits();
        *self
    }

    pub const fn is_rival(&self) -> bool {
        (self.inner & Self::RIVAL) != 0
    }

    /// flag for the two competing outland factions
    /// This is also present in vmangos for Vanilla, unsure if it's used.
    ///
    pub const fn new_rival() -> Self {
        Self { inner: Self::RIVAL }
    }

    pub fn set_rival(&mut self) -> Self {
        self.inner |= Self::RIVAL;
        *self
    }

    pub fn clear_rival(&mut self) -> Self {
        self.inner &= Self::RIVAL.reverse_bits();
        *self
    }

    pub const fn is_special(&self) -> bool {
        (self.inner & Self::SPECIAL) != 0
    }

    /// horde and alliance home cities and their northrend allies have this flag
    ///
    pub const fn new_special() -> Self {
        Self { inner: Self::SPECIAL }
    }

    pub fn set_special(&mut self) -> Self {
        self.inner |= Self::SPECIAL;
        *self
    }

    pub fn clear_special(&mut self) -> Self {
        self.inner &= Self::SPECIAL.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u8 {
        self.inner
    }

}

impl std::fmt::UpperHex for FactionFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for FactionFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for FactionFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for FactionFlag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

impl std::ops::BitAnd for FactionFlag {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitand(rhs.inner), }
    }
}

impl std::ops::BitAndAssign for FactionFlag {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner)
    }
}

impl std::ops::BitOr for FactionFlag {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitor(rhs.inner), }
    }
}

impl std::ops::BitOrAssign for FactionFlag {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner)
    }
}

impl std::ops::BitXor for FactionFlag {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitxor(rhs.inner), }
    }
}

impl std::ops::BitXorAssign for FactionFlag {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner)
    }
}

