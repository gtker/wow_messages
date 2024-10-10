use crate::all::ProtocolVersion;

impl ProtocolVersion {
    /// Returns whether the protocol supports PIN two-factor authentication.
    ///
    /// Every protocol after [`Self::Two`] supports this.
    pub const fn supports_pin(&self) -> bool {
        match self {
            ProtocolVersion::Two => false,
            ProtocolVersion::Three
            | ProtocolVersion::Five
            | ProtocolVersion::Six
            | ProtocolVersion::Seven
            | ProtocolVersion::Eight => true,
        }
    }

    /// Returns whether the protocol supports matrix card two-factor authentication.
    ///
    /// Every protocol after [`Self::Three`] supports this.
    pub const fn supports_matrix_card(&self) -> bool {
        match self {
            ProtocolVersion::Two | ProtocolVersion::Three => false,
            ProtocolVersion::Five
            | ProtocolVersion::Six
            | ProtocolVersion::Seven
            | ProtocolVersion::Eight => true,
        }
    }

    /// Returns whether the protocol supports the TOTP authenticator.
    ///
    /// Only [`Self::Eight`] supports this.
    pub const fn supports_authenticator(&self) -> bool {
        matches!(self, Self::Eight)
    }
}
