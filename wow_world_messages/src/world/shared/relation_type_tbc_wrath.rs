/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_contact_list.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_contact_list.wowm#L1):
/// ```text
/// flag RelationType : u32 {
///     NONE = 0x00;
///     FRIEND = 0x01;
///     IGNORED = 0x02;
///     MUTED = 0x04;
///     RECRUITAFRIEND = 0x08;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
pub struct RelationType {
    inner: u32,
}

impl RelationType {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

    pub(crate) const NONE: u32 = 0x00;
    pub(crate) const FRIEND: u32 = 0x01;
    pub(crate) const IGNORED: u32 = 0x02;
    pub(crate) const MUTED: u32 = 0x04;
    pub(crate) const RECRUITAFRIEND: u32 = 0x08;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::NONE
                | Self::FRIEND
                | Self::IGNORED
                | Self::MUTED
                | Self::RECRUITAFRIEND
        }
    }

    pub const fn is_FRIEND(&self) -> bool {
        (self.inner & Self::FRIEND) != 0
    }

    pub const fn new_FRIEND() -> Self {
        Self { inner: Self::FRIEND }
    }

    pub fn set_FRIEND(&mut self) -> Self {
        self.inner |= Self::FRIEND;
        *self
    }

    pub fn clear_FRIEND(&mut self) -> Self {
        self.inner &= Self::FRIEND.reverse_bits();
        *self
    }

    pub const fn is_IGNORED(&self) -> bool {
        (self.inner & Self::IGNORED) != 0
    }

    pub const fn new_IGNORED() -> Self {
        Self { inner: Self::IGNORED }
    }

    pub fn set_IGNORED(&mut self) -> Self {
        self.inner |= Self::IGNORED;
        *self
    }

    pub fn clear_IGNORED(&mut self) -> Self {
        self.inner &= Self::IGNORED.reverse_bits();
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

    pub const fn is_RECRUITAFRIEND(&self) -> bool {
        (self.inner & Self::RECRUITAFRIEND) != 0
    }

    pub const fn new_RECRUITAFRIEND() -> Self {
        Self { inner: Self::RECRUITAFRIEND }
    }

    pub fn set_RECRUITAFRIEND(&mut self) -> Self {
        self.inner |= Self::RECRUITAFRIEND;
        *self
    }

    pub fn clear_RECRUITAFRIEND(&mut self) -> Self {
        self.inner &= Self::RECRUITAFRIEND.reverse_bits();
        *self
    }

    pub(crate) const fn as_int(&self) -> u32 {
        self.inner
    }

}

impl std::fmt::UpperHex for RelationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for RelationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for RelationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for RelationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

