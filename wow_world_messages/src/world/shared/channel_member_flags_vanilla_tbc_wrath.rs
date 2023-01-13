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
pub struct ChannelMemberFlags {
    inner: u8,
}

impl ChannelMemberFlags {
    pub const fn new(inner: u8) -> Self {
        Self { inner }
    }

    pub(crate) const NONE: u8 = 0x00;
    pub(crate) const OWNER: u8 = 0x01;
    pub(crate) const MODERATOR: u8 = 0x04;
    pub(crate) const VOICED: u8 = 0x08;
    pub(crate) const MUTED: u8 = 0x10;
    pub(crate) const CUSTOM: u8 = 0x20;
    pub(crate) const MICROPHONE_MUTE: u8 = 0x40;

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

    pub const fn is_OWNER(&self) -> bool {
        (self.inner & Self::OWNER) != 0
    }

    pub const fn new_OWNER() -> Self {
        Self { inner: Self::OWNER }
    }

    pub fn set_OWNER(&mut self) -> Self {
        self.inner |= Self::OWNER;
        *self
    }

    pub fn clear_OWNER(&mut self) -> Self {
        self.inner &= Self::OWNER.reverse_bits();
        *self
    }

    pub const fn is_MODERATOR(&self) -> bool {
        (self.inner & Self::MODERATOR) != 0
    }

    pub const fn new_MODERATOR() -> Self {
        Self { inner: Self::MODERATOR }
    }

    pub fn set_MODERATOR(&mut self) -> Self {
        self.inner |= Self::MODERATOR;
        *self
    }

    pub fn clear_MODERATOR(&mut self) -> Self {
        self.inner &= Self::MODERATOR.reverse_bits();
        *self
    }

    pub const fn is_VOICED(&self) -> bool {
        (self.inner & Self::VOICED) != 0
    }

    pub const fn new_VOICED() -> Self {
        Self { inner: Self::VOICED }
    }

    pub fn set_VOICED(&mut self) -> Self {
        self.inner |= Self::VOICED;
        *self
    }

    pub fn clear_VOICED(&mut self) -> Self {
        self.inner &= Self::VOICED.reverse_bits();
        *self
    }

    pub const fn is_MUTED(&self) -> bool {
        (self.inner & Self::MUTED) != 0
    }

    pub const fn new_MUTED() -> Self {
        Self { inner: Self::MUTED }
    }

    pub fn set_MUTED(&mut self) -> Self {
        self.inner |= Self::MUTED;
        *self
    }

    pub fn clear_MUTED(&mut self) -> Self {
        self.inner &= Self::MUTED.reverse_bits();
        *self
    }

    pub const fn is_CUSTOM(&self) -> bool {
        (self.inner & Self::CUSTOM) != 0
    }

    pub const fn new_CUSTOM() -> Self {
        Self { inner: Self::CUSTOM }
    }

    pub fn set_CUSTOM(&mut self) -> Self {
        self.inner |= Self::CUSTOM;
        *self
    }

    pub fn clear_CUSTOM(&mut self) -> Self {
        self.inner &= Self::CUSTOM.reverse_bits();
        *self
    }

    pub const fn is_MICROPHONE_MUTE(&self) -> bool {
        (self.inner & Self::MICROPHONE_MUTE) != 0
    }

    pub const fn new_MICROPHONE_MUTE() -> Self {
        Self { inner: Self::MICROPHONE_MUTE }
    }

    pub fn set_MICROPHONE_MUTE(&mut self) -> Self {
        self.inner |= Self::MICROPHONE_MUTE;
        *self
    }

    pub fn clear_MICROPHONE_MUTE(&mut self) -> Self {
        self.inner &= Self::MICROPHONE_MUTE.reverse_bits();
        *self
    }

    pub(crate) const fn as_int(&self) -> u8 {
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

