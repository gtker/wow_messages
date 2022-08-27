/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/character_screen/smsg_char_enum.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/character_screen/smsg_char_enum.wowm#L3):
/// ```text
/// flag CharacterFlags : u32 {
///     NONE = 0x00;
///     LOCKED_FOR_TRANSFER = 0x04;
///     HIDE_HELM = 0x400;
///     HIDE_CLOAK = 0x800;
///     GHOST = 0x2000;
///     RENAME = 0x4000;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
pub struct CharacterFlags {
    inner: u32,
}

impl CharacterFlags {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

    pub(crate) const NONE: u32 = 0x00;
    pub(crate) const LOCKED_FOR_TRANSFER: u32 = 0x04;
    pub(crate) const HIDE_HELM: u32 = 0x400;
    pub(crate) const HIDE_CLOAK: u32 = 0x800;
    pub(crate) const GHOST: u32 = 0x2000;
    pub(crate) const RENAME: u32 = 0x4000;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::NONE
                | Self::LOCKED_FOR_TRANSFER
                | Self::HIDE_HELM
                | Self::HIDE_CLOAK
                | Self::GHOST
                | Self::RENAME
        }
    }

    pub const fn is_LOCKED_FOR_TRANSFER(&self) -> bool {
        (self.inner & Self::LOCKED_FOR_TRANSFER) != 0
    }

    pub const fn new_LOCKED_FOR_TRANSFER() -> Self {
        Self { inner: Self::LOCKED_FOR_TRANSFER }
    }

    pub fn set_LOCKED_FOR_TRANSFER(&mut self) -> Self {
        self.inner |= Self::LOCKED_FOR_TRANSFER;
        *self
    }

    pub fn clear_LOCKED_FOR_TRANSFER(&mut self) -> Self {
        self.inner &= Self::LOCKED_FOR_TRANSFER.reverse_bits();
        *self
    }

    pub const fn is_HIDE_HELM(&self) -> bool {
        (self.inner & Self::HIDE_HELM) != 0
    }

    pub const fn new_HIDE_HELM() -> Self {
        Self { inner: Self::HIDE_HELM }
    }

    pub fn set_HIDE_HELM(&mut self) -> Self {
        self.inner |= Self::HIDE_HELM;
        *self
    }

    pub fn clear_HIDE_HELM(&mut self) -> Self {
        self.inner &= Self::HIDE_HELM.reverse_bits();
        *self
    }

    pub const fn is_HIDE_CLOAK(&self) -> bool {
        (self.inner & Self::HIDE_CLOAK) != 0
    }

    pub const fn new_HIDE_CLOAK() -> Self {
        Self { inner: Self::HIDE_CLOAK }
    }

    pub fn set_HIDE_CLOAK(&mut self) -> Self {
        self.inner |= Self::HIDE_CLOAK;
        *self
    }

    pub fn clear_HIDE_CLOAK(&mut self) -> Self {
        self.inner &= Self::HIDE_CLOAK.reverse_bits();
        *self
    }

    pub const fn is_GHOST(&self) -> bool {
        (self.inner & Self::GHOST) != 0
    }

    pub const fn new_GHOST() -> Self {
        Self { inner: Self::GHOST }
    }

    pub fn set_GHOST(&mut self) -> Self {
        self.inner |= Self::GHOST;
        *self
    }

    pub fn clear_GHOST(&mut self) -> Self {
        self.inner &= Self::GHOST.reverse_bits();
        *self
    }

    pub const fn is_RENAME(&self) -> bool {
        (self.inner & Self::RENAME) != 0
    }

    pub const fn new_RENAME() -> Self {
        Self { inner: Self::RENAME }
    }

    pub fn set_RENAME(&mut self) -> Self {
        self.inner |= Self::RENAME;
        *self
    }

    pub fn clear_RENAME(&mut self) -> Self {
        self.inner &= Self::RENAME.reverse_bits();
        *self
    }

    pub(crate) const fn as_int(&self) -> u32 {
        self.inner
    }

}

