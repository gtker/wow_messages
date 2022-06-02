/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/faction/smsg_initialize_factions.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/faction/smsg_initialize_factions.wowm#L3):
/// ```text
/// flag FactionFlag : u8 {
///     VISIBLE = 0x01;
///     AT_WAR = 0x02;
///     HIDDEN = 0x04;
///     INVISIBLE_FORCED = 0x08;
///     PEACE_FORCED = 0x10;
///     INACTIVE = 0x20;
///     RIVAL = 0x40;
/// }

/// ```
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, Default)]
pub struct FactionFlag {
    inner: u8,
}

impl FactionFlag {
    pub const fn new(inner: u8) -> Self {
        Self { inner }
    }

    pub(crate) const VISIBLE: u8 = 0x01;
    pub(crate) const AT_WAR: u8 = 0x02;
    pub(crate) const HIDDEN: u8 = 0x04;
    pub(crate) const INVISIBLE_FORCED: u8 = 0x08;
    pub(crate) const PEACE_FORCED: u8 = 0x10;
    pub(crate) const INACTIVE: u8 = 0x20;
    pub(crate) const RIVAL: u8 = 0x40;

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
        }
    }

    pub const fn is_VISIBLE(&self) -> bool {
        (self.inner & Self::VISIBLE) != 0
    }

    /// # Comment
    ///
    /// makes visible in client (set or can be set at interaction with target of this faction)
    pub const fn new_VISIBLE() -> Self {
        Self { inner: Self::VISIBLE }
    }

    pub fn set_VISIBLE(&mut self) -> Self {
        self.inner |= Self::VISIBLE;
        *self
    }

    pub fn clear_VISIBLE(&mut self) -> Self {
        self.inner &= Self::VISIBLE.reverse_bits();
        *self
    }

    pub const fn is_AT_WAR(&self) -> bool {
        (self.inner & Self::AT_WAR) != 0
    }

    /// # Comment
    ///
    /// enable AtWar-button in client. player controlled (except opposition team always war state), Flag only set on initial creation
    pub const fn new_AT_WAR() -> Self {
        Self { inner: Self::AT_WAR }
    }

    pub fn set_AT_WAR(&mut self) -> Self {
        self.inner |= Self::AT_WAR;
        *self
    }

    pub fn clear_AT_WAR(&mut self) -> Self {
        self.inner &= Self::AT_WAR.reverse_bits();
        *self
    }

    pub const fn is_HIDDEN(&self) -> bool {
        (self.inner & Self::HIDDEN) != 0
    }

    /// # Comment
    ///
    /// hidden faction from reputation pane in client (player can gain reputation, but this update not sent to client)
    pub const fn new_HIDDEN() -> Self {
        Self { inner: Self::HIDDEN }
    }

    pub fn set_HIDDEN(&mut self) -> Self {
        self.inner |= Self::HIDDEN;
        *self
    }

    pub fn clear_HIDDEN(&mut self) -> Self {
        self.inner &= Self::HIDDEN.reverse_bits();
        *self
    }

    pub const fn is_INVISIBLE_FORCED(&self) -> bool {
        (self.inner & Self::INVISIBLE_FORCED) != 0
    }

    /// # Comment
    ///
    /// always overwrite FACTION_FLAG_VISIBLE and hide faction in rep.list, used for hide opposite team factions
    pub const fn new_INVISIBLE_FORCED() -> Self {
        Self { inner: Self::INVISIBLE_FORCED }
    }

    pub fn set_INVISIBLE_FORCED(&mut self) -> Self {
        self.inner |= Self::INVISIBLE_FORCED;
        *self
    }

    pub fn clear_INVISIBLE_FORCED(&mut self) -> Self {
        self.inner &= Self::INVISIBLE_FORCED.reverse_bits();
        *self
    }

    pub const fn is_PEACE_FORCED(&self) -> bool {
        (self.inner & Self::PEACE_FORCED) != 0
    }

    /// # Comment
    ///
    /// always overwrite FACTION_FLAG_AT_WAR, used for prevent war with own team factions
    pub const fn new_PEACE_FORCED() -> Self {
        Self { inner: Self::PEACE_FORCED }
    }

    pub fn set_PEACE_FORCED(&mut self) -> Self {
        self.inner |= Self::PEACE_FORCED;
        *self
    }

    pub fn clear_PEACE_FORCED(&mut self) -> Self {
        self.inner &= Self::PEACE_FORCED.reverse_bits();
        *self
    }

    pub const fn is_INACTIVE(&self) -> bool {
        (self.inner & Self::INACTIVE) != 0
    }

    /// # Comment
    ///
    /// player controlled, state stored in characters.data ( CMSG_SET_FACTION_INACTIVE )
    pub const fn new_INACTIVE() -> Self {
        Self { inner: Self::INACTIVE }
    }

    pub fn set_INACTIVE(&mut self) -> Self {
        self.inner |= Self::INACTIVE;
        *self
    }

    pub fn clear_INACTIVE(&mut self) -> Self {
        self.inner &= Self::INACTIVE.reverse_bits();
        *self
    }

    pub const fn is_RIVAL(&self) -> bool {
        (self.inner & Self::RIVAL) != 0
    }

    /// # Comment
    ///
    /// flag for the two competing outland factions
    pub const fn new_RIVAL() -> Self {
        Self { inner: Self::RIVAL }
    }

    pub fn set_RIVAL(&mut self) -> Self {
        self.inner |= Self::RIVAL;
        *self
    }

    pub fn clear_RIVAL(&mut self) -> Self {
        self.inner &= Self::RIVAL.reverse_bits();
        *self
    }

    pub(crate) const fn as_int(&self) -> u8 {
        self.inner
    }

}

