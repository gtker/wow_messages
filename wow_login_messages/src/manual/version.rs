use crate::all::Version;

impl Version {
    /// Returns whether the protocol supports PIN two-factor authentication.
    ///
    /// Any version released after `1.12.x` supports this.
    pub const fn supports_pin(&self) -> bool {
        #[expect(clippy::match_like_matches_macro)]
        match (self.major, self.minor, self.patch) {
            (0, _, _) => false,
            (1, 0..=11, _) => false,
            _ => true,
        }
    }

    /// Returns whether the protocol supports matrix card two-factor authentication.
    ///
    /// Any version released after `2.0.1.6180` supports this.
    pub const fn supports_matrix_card(&self) -> bool {
        #[expect(clippy::match_like_matches_macro)]
        match (self.major, self.minor, self.patch) {
            (0..=1, _, _) => false,
            (2, 0, 0..=2) => false,
            _ => true,
        }
    }

    /// Returns whether the protocol supports the TOTP authenticator.
    ///
    /// Any version released after `2.4.0.8089` supports this.
    pub const fn supports_authenticator(&self) -> bool {
        #[expect(clippy::match_like_matches_macro)]
        match (self.major, self.minor, self.patch) {
            (0..=1, _, _) => false,
            (2, 0..=3, _) => false,
            _ => true,
        }
    }
}
