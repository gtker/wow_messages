/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/chat/smsg_channel_list.wowm:14`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/chat/smsg_channel_list.wowm#L14):
/// ```text
/// flag ChannelMemberFlags : u8 {
///     NONE = 0x00;
///     OWNER = 0x01;
///     MODERATOR = 0x04;
///     VOICED = 0x08;
///     MUTED = 0x10;
///     CUSTOM = 0x20;
///     MICROPHONE_MUTE = 0x40;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct ChannelMemberFlags {
    inner: u8,
}

#[cfg(feature = "print-testcase")]
impl ChannelMemberFlags {
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
        if self.is_owner() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "OWNER").unwrap();
            first = false;
        }
        if self.is_moderator() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "MODERATOR").unwrap();
            first = false;
        }
        if self.is_voiced() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "VOICED").unwrap();
            first = false;
        }
        if self.is_muted() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "MUTED").unwrap();
            first = false;
        }
        if self.is_custom() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "CUSTOM").unwrap();
            first = false;
        }
        if self.is_microphone_mute() {
            use std::fmt::Write;
            if !first {
                write!(s, "| ").unwrap();
            }
            write!(s, "MICROPHONE_MUTE").unwrap();
            first = false;
        }
        s
    }

}

impl ChannelMemberFlags {
    pub const fn new(inner: u8) -> Self {
        Self { inner }
    }

    pub const NONE: u8 = 0x00;
    pub const OWNER: u8 = 0x01;
    pub const MODERATOR: u8 = 0x04;
    pub const VOICED: u8 = 0x08;
    pub const MUTED: u8 = 0x10;
    pub const CUSTOM: u8 = 0x20;
    pub const MICROPHONE_MUTE: u8 = 0x40;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::NONE
                | Self::OWNER
                | Self::MODERATOR
                | Self::VOICED
                | Self::MUTED
                | Self::CUSTOM
                | Self::MICROPHONE_MUTE
        }
    }

    pub const fn is_owner(&self) -> bool {
        (self.inner & Self::OWNER) != 0
    }

    pub const fn new_owner() -> Self {
        Self { inner: Self::OWNER }
    }

    pub fn set_owner(&mut self) -> Self {
        self.inner |= Self::OWNER;
        *self
    }

    pub fn clear_owner(&mut self) -> Self {
        self.inner &= Self::OWNER.reverse_bits();
        *self
    }

    pub const fn is_moderator(&self) -> bool {
        (self.inner & Self::MODERATOR) != 0
    }

    pub const fn new_moderator() -> Self {
        Self { inner: Self::MODERATOR }
    }

    pub fn set_moderator(&mut self) -> Self {
        self.inner |= Self::MODERATOR;
        *self
    }

    pub fn clear_moderator(&mut self) -> Self {
        self.inner &= Self::MODERATOR.reverse_bits();
        *self
    }

    pub const fn is_voiced(&self) -> bool {
        (self.inner & Self::VOICED) != 0
    }

    pub const fn new_voiced() -> Self {
        Self { inner: Self::VOICED }
    }

    pub fn set_voiced(&mut self) -> Self {
        self.inner |= Self::VOICED;
        *self
    }

    pub fn clear_voiced(&mut self) -> Self {
        self.inner &= Self::VOICED.reverse_bits();
        *self
    }

    pub const fn is_muted(&self) -> bool {
        (self.inner & Self::MUTED) != 0
    }

    pub const fn new_muted() -> Self {
        Self { inner: Self::MUTED }
    }

    pub fn set_muted(&mut self) -> Self {
        self.inner |= Self::MUTED;
        *self
    }

    pub fn clear_muted(&mut self) -> Self {
        self.inner &= Self::MUTED.reverse_bits();
        *self
    }

    pub const fn is_custom(&self) -> bool {
        (self.inner & Self::CUSTOM) != 0
    }

    pub const fn new_custom() -> Self {
        Self { inner: Self::CUSTOM }
    }

    pub fn set_custom(&mut self) -> Self {
        self.inner |= Self::CUSTOM;
        *self
    }

    pub fn clear_custom(&mut self) -> Self {
        self.inner &= Self::CUSTOM.reverse_bits();
        *self
    }

    pub const fn is_microphone_mute(&self) -> bool {
        (self.inner & Self::MICROPHONE_MUTE) != 0
    }

    pub const fn new_microphone_mute() -> Self {
        Self { inner: Self::MICROPHONE_MUTE }
    }

    pub fn set_microphone_mute(&mut self) -> Self {
        self.inner |= Self::MICROPHONE_MUTE;
        *self
    }

    pub fn clear_microphone_mute(&mut self) -> Self {
        self.inner &= Self::MICROPHONE_MUTE.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u8 {
        self.inner
    }

}

impl std::fmt::UpperHex for ChannelMemberFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for ChannelMemberFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for ChannelMemberFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for ChannelMemberFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

impl std::ops::BitAnd for ChannelMemberFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitand(rhs.inner), }
    }
}

impl std::ops::BitAndAssign for ChannelMemberFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner)
    }
}

impl std::ops::BitOr for ChannelMemberFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitor(rhs.inner), }
    }
}

impl std::ops::BitOrAssign for ChannelMemberFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner)
    }
}

impl std::ops::BitXor for ChannelMemberFlags {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitxor(rhs.inner), }
    }
}

impl std::ops::BitXorAssign for ChannelMemberFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner)
    }
}

