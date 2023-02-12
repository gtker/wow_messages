/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/common_movement_3_3_5.wowm:74`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/common_movement_3_3_5.wowm#L74):
/// ```text
/// flag ExtraMovementFlags : u16 {
///     NONE = 0x00000000;
///     NO_STRAFE = 0x00000001;
///     NO_JUMPING = 0x00000002;
///     UNK3 = 0x00000004;
///     FULL_SPEED_TURNING = 0x00000008;
///     FULL_SPEED_PITCHING = 0x00000010;
///     ALWAYS_ALLOW_PITCHING = 0x00000020;
///     UNK7 = 0x00000040;
///     UNK8 = 0x00000080;
///     UNK9 = 0x00000100;
///     UNK10 = 0x00000200;
///     INTERPOLATED_MOVEMENT = 0x00000400;
///     INTERPOLATED_TURNING = 0x00000800;
///     INTERPOLATED_PITCHING = 0x00001000;
///     UNK14 = 0x00002000;
///     UNK15 = 0x00004000;
///     UNK16 = 0x00008000;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
pub struct ExtraMovementFlags {
    inner: u16,
}

impl ExtraMovementFlags {
    pub const fn new(inner: u16) -> Self {
        Self { inner }
    }

    pub const NONE: u16 = 0x00;
    pub const NO_STRAFE: u16 = 0x01;
    pub const NO_JUMPING: u16 = 0x02;
    pub const UNK3: u16 = 0x04;
    pub const FULL_SPEED_TURNING: u16 = 0x08;
    pub const FULL_SPEED_PITCHING: u16 = 0x10;
    pub const ALWAYS_ALLOW_PITCHING: u16 = 0x20;
    pub const UNK7: u16 = 0x40;
    pub const UNK8: u16 = 0x80;
    pub const UNK9: u16 = 0x100;
    pub const UNK10: u16 = 0x200;
    pub const INTERPOLATED_MOVEMENT: u16 = 0x400;
    pub const INTERPOLATED_TURNING: u16 = 0x800;
    pub const INTERPOLATED_PITCHING: u16 = 0x1000;
    pub const UNK14: u16 = 0x2000;
    pub const UNK15: u16 = 0x4000;
    pub const UNK16: u16 = 0x8000;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::NONE
                | Self::NO_STRAFE
                | Self::NO_JUMPING
                | Self::UNK3
                | Self::FULL_SPEED_TURNING
                | Self::FULL_SPEED_PITCHING
                | Self::ALWAYS_ALLOW_PITCHING
                | Self::UNK7
                | Self::UNK8
                | Self::UNK9
                | Self::UNK10
                | Self::INTERPOLATED_MOVEMENT
                | Self::INTERPOLATED_TURNING
                | Self::INTERPOLATED_PITCHING
                | Self::UNK14
                | Self::UNK15
                | Self::UNK16
        }
    }

    pub const fn is_NO_STRAFE(&self) -> bool {
        (self.inner & Self::NO_STRAFE) != 0
    }

    pub const fn new_NO_STRAFE() -> Self {
        Self { inner: Self::NO_STRAFE }
    }

    pub fn set_NO_STRAFE(&mut self) -> Self {
        self.inner |= Self::NO_STRAFE;
        *self
    }

    pub fn clear_NO_STRAFE(&mut self) -> Self {
        self.inner &= Self::NO_STRAFE.reverse_bits();
        *self
    }

    pub const fn is_NO_JUMPING(&self) -> bool {
        (self.inner & Self::NO_JUMPING) != 0
    }

    pub const fn new_NO_JUMPING() -> Self {
        Self { inner: Self::NO_JUMPING }
    }

    pub fn set_NO_JUMPING(&mut self) -> Self {
        self.inner |= Self::NO_JUMPING;
        *self
    }

    pub fn clear_NO_JUMPING(&mut self) -> Self {
        self.inner &= Self::NO_JUMPING.reverse_bits();
        *self
    }

    pub const fn is_UNK3(&self) -> bool {
        (self.inner & Self::UNK3) != 0
    }

    /// `AzerothCore`: Overrides various clientside checks
    ///
    pub const fn new_UNK3() -> Self {
        Self { inner: Self::UNK3 }
    }

    pub fn set_UNK3(&mut self) -> Self {
        self.inner |= Self::UNK3;
        *self
    }

    pub fn clear_UNK3(&mut self) -> Self {
        self.inner &= Self::UNK3.reverse_bits();
        *self
    }

    pub const fn is_FULL_SPEED_TURNING(&self) -> bool {
        (self.inner & Self::FULL_SPEED_TURNING) != 0
    }

    pub const fn new_FULL_SPEED_TURNING() -> Self {
        Self { inner: Self::FULL_SPEED_TURNING }
    }

    pub fn set_FULL_SPEED_TURNING(&mut self) -> Self {
        self.inner |= Self::FULL_SPEED_TURNING;
        *self
    }

    pub fn clear_FULL_SPEED_TURNING(&mut self) -> Self {
        self.inner &= Self::FULL_SPEED_TURNING.reverse_bits();
        *self
    }

    pub const fn is_FULL_SPEED_PITCHING(&self) -> bool {
        (self.inner & Self::FULL_SPEED_PITCHING) != 0
    }

    pub const fn new_FULL_SPEED_PITCHING() -> Self {
        Self { inner: Self::FULL_SPEED_PITCHING }
    }

    pub fn set_FULL_SPEED_PITCHING(&mut self) -> Self {
        self.inner |= Self::FULL_SPEED_PITCHING;
        *self
    }

    pub fn clear_FULL_SPEED_PITCHING(&mut self) -> Self {
        self.inner &= Self::FULL_SPEED_PITCHING.reverse_bits();
        *self
    }

    pub const fn is_ALWAYS_ALLOW_PITCHING(&self) -> bool {
        (self.inner & Self::ALWAYS_ALLOW_PITCHING) != 0
    }

    pub const fn new_ALWAYS_ALLOW_PITCHING() -> Self {
        Self { inner: Self::ALWAYS_ALLOW_PITCHING }
    }

    pub fn set_ALWAYS_ALLOW_PITCHING(&mut self) -> Self {
        self.inner |= Self::ALWAYS_ALLOW_PITCHING;
        *self
    }

    pub fn clear_ALWAYS_ALLOW_PITCHING(&mut self) -> Self {
        self.inner &= Self::ALWAYS_ALLOW_PITCHING.reverse_bits();
        *self
    }

    pub const fn is_UNK7(&self) -> bool {
        (self.inner & Self::UNK7) != 0
    }

    pub const fn new_UNK7() -> Self {
        Self { inner: Self::UNK7 }
    }

    pub fn set_UNK7(&mut self) -> Self {
        self.inner |= Self::UNK7;
        *self
    }

    pub fn clear_UNK7(&mut self) -> Self {
        self.inner &= Self::UNK7.reverse_bits();
        *self
    }

    pub const fn is_UNK8(&self) -> bool {
        (self.inner & Self::UNK8) != 0
    }

    pub const fn new_UNK8() -> Self {
        Self { inner: Self::UNK8 }
    }

    pub fn set_UNK8(&mut self) -> Self {
        self.inner |= Self::UNK8;
        *self
    }

    pub fn clear_UNK8(&mut self) -> Self {
        self.inner &= Self::UNK8.reverse_bits();
        *self
    }

    pub const fn is_UNK9(&self) -> bool {
        (self.inner & Self::UNK9) != 0
    }

    pub const fn new_UNK9() -> Self {
        Self { inner: Self::UNK9 }
    }

    pub fn set_UNK9(&mut self) -> Self {
        self.inner |= Self::UNK9;
        *self
    }

    pub fn clear_UNK9(&mut self) -> Self {
        self.inner &= Self::UNK9.reverse_bits();
        *self
    }

    pub const fn is_UNK10(&self) -> bool {
        (self.inner & Self::UNK10) != 0
    }

    pub const fn new_UNK10() -> Self {
        Self { inner: Self::UNK10 }
    }

    pub fn set_UNK10(&mut self) -> Self {
        self.inner |= Self::UNK10;
        *self
    }

    pub fn clear_UNK10(&mut self) -> Self {
        self.inner &= Self::UNK10.reverse_bits();
        *self
    }

    pub const fn is_INTERPOLATED_MOVEMENT(&self) -> bool {
        (self.inner & Self::INTERPOLATED_MOVEMENT) != 0
    }

    pub const fn new_INTERPOLATED_MOVEMENT() -> Self {
        Self { inner: Self::INTERPOLATED_MOVEMENT }
    }

    pub fn set_INTERPOLATED_MOVEMENT(&mut self) -> Self {
        self.inner |= Self::INTERPOLATED_MOVEMENT;
        *self
    }

    pub fn clear_INTERPOLATED_MOVEMENT(&mut self) -> Self {
        self.inner &= Self::INTERPOLATED_MOVEMENT.reverse_bits();
        *self
    }

    pub const fn is_INTERPOLATED_TURNING(&self) -> bool {
        (self.inner & Self::INTERPOLATED_TURNING) != 0
    }

    pub const fn new_INTERPOLATED_TURNING() -> Self {
        Self { inner: Self::INTERPOLATED_TURNING }
    }

    pub fn set_INTERPOLATED_TURNING(&mut self) -> Self {
        self.inner |= Self::INTERPOLATED_TURNING;
        *self
    }

    pub fn clear_INTERPOLATED_TURNING(&mut self) -> Self {
        self.inner &= Self::INTERPOLATED_TURNING.reverse_bits();
        *self
    }

    pub const fn is_INTERPOLATED_PITCHING(&self) -> bool {
        (self.inner & Self::INTERPOLATED_PITCHING) != 0
    }

    pub const fn new_INTERPOLATED_PITCHING() -> Self {
        Self { inner: Self::INTERPOLATED_PITCHING }
    }

    pub fn set_INTERPOLATED_PITCHING(&mut self) -> Self {
        self.inner |= Self::INTERPOLATED_PITCHING;
        *self
    }

    pub fn clear_INTERPOLATED_PITCHING(&mut self) -> Self {
        self.inner &= Self::INTERPOLATED_PITCHING.reverse_bits();
        *self
    }

    pub const fn is_UNK14(&self) -> bool {
        (self.inner & Self::UNK14) != 0
    }

    pub const fn new_UNK14() -> Self {
        Self { inner: Self::UNK14 }
    }

    pub fn set_UNK14(&mut self) -> Self {
        self.inner |= Self::UNK14;
        *self
    }

    pub fn clear_UNK14(&mut self) -> Self {
        self.inner &= Self::UNK14.reverse_bits();
        *self
    }

    pub const fn is_UNK15(&self) -> bool {
        (self.inner & Self::UNK15) != 0
    }

    pub const fn new_UNK15() -> Self {
        Self { inner: Self::UNK15 }
    }

    pub fn set_UNK15(&mut self) -> Self {
        self.inner |= Self::UNK15;
        *self
    }

    pub fn clear_UNK15(&mut self) -> Self {
        self.inner &= Self::UNK15.reverse_bits();
        *self
    }

    pub const fn is_UNK16(&self) -> bool {
        (self.inner & Self::UNK16) != 0
    }

    pub const fn new_UNK16() -> Self {
        Self { inner: Self::UNK16 }
    }

    pub fn set_UNK16(&mut self) -> Self {
        self.inner |= Self::UNK16;
        *self
    }

    pub fn clear_UNK16(&mut self) -> Self {
        self.inner &= Self::UNK16.reverse_bits();
        *self
    }

    pub(crate) const fn as_int(&self) -> u16 {
        self.inner
    }

}

impl std::fmt::UpperHex for ExtraMovementFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for ExtraMovementFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for ExtraMovementFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for ExtraMovementFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

