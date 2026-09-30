use std::io::{Read, Write};

use crate::wrath::{
    SplineFlag, Vector3d,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm:31`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/smsg/smsg_monster_move.wowm#L31):
/// ```text
/// struct MonsterMoveData {
///     SplineFlag spline_flags;
///     if (spline_flags & ANIMATION) {
///         u8 animation_id;
///         u32 animation_start_time;
///     }
///     u32 duration;
///     if (spline_flags & PARABOLIC) {
///         f32 vertical_acceleration;
///         u32 effect_start_time;
///     }
///     if (spline_flags & FLYING
///         || spline_flags & CATMULLROM) {
///         FullMonsterMoveSpline full_splines;
///     }
///     else {
///         MonsterMoveSplines splines;
///     }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct MonsterMoveData {
    pub spline_flags: MonsterMoveData_SplineFlag,
    pub duration: u32,
    pub splines: Vec<Vector3d>,
}

impl MonsterMoveData {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // spline_flags: SplineFlag
        w.write_all(&(self.spline_flags.as_int().to_le_bytes()))?;

        if let Some(if_statement) = &self.spline_flags.animation {
            // animation_id: u8
            w.write_all(&if_statement.animation_id.to_le_bytes())?;

            // animation_start_time: u32
            w.write_all(&if_statement.animation_start_time.to_le_bytes())?;

        }

        // duration: u32
        w.write_all(&self.duration.to_le_bytes())?;

        if let Some(if_statement) = &self.spline_flags.parabolic {
            // vertical_acceleration: f32
            w.write_all(&if_statement.vertical_acceleration.to_le_bytes())?;

            // effect_start_time: u32
            w.write_all(&if_statement.effect_start_time.to_le_bytes())?;

        }

        crate::util::write_wrath_monster_move_spline(self.splines.as_slice(), self.spline_flags.get_flying() || self.spline_flags.get_catmullrom(), &mut w)?;

        Ok(())
    }
}

impl MonsterMoveData {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // spline_flags: SplineFlag
        let spline_flags = SplineFlag::new(crate::util::read_u32_le(&mut r)?);

        let spline_flags_animation = if spline_flags.is_animation() {
            // animation_id: u8
            let animation_id = crate::util::read_u8_le(&mut r)?;

            // animation_start_time: u32
            let animation_start_time = crate::util::read_u32_le(&mut r)?;

            Some(MonsterMoveData_SplineFlag_Animation {
                animation_id,
                animation_start_time,
            })
        }
        else {
            None
        };

        // duration: u32
        let duration = crate::util::read_u32_le(&mut r)?;

        let spline_flags_parabolic = if spline_flags.is_parabolic() {
            // vertical_acceleration: f32
            let vertical_acceleration = crate::util::read_f32_le(&mut r)?;

            // effect_start_time: u32
            let effect_start_time = crate::util::read_u32_le(&mut r)?;

            Some(MonsterMoveData_SplineFlag_Parabolic {
                effect_start_time,
                vertical_acceleration,
            })
        }
        else {
            None
        };

        let splines = crate::util::read_wrath_monster_move_spline(&mut r, spline_flags.is_flying() || spline_flags.is_catmullrom())?;

        let spline_flags = MonsterMoveData_SplineFlag {
            inner: spline_flags.as_int(),
            parabolic: spline_flags_parabolic,
            animation: spline_flags_animation,
        };

        Ok(Self {
            spline_flags,
            duration,
            splines,
        })
    }

}

impl MonsterMoveData {
    pub(crate) const fn size(&self) -> usize {
        self.spline_flags.size() // spline_flags: MonsterMoveData_SplineFlag
        + 4 // duration: u32
        + crate::util::wrath_monster_move_spline_size(self.splines.as_slice(), self.spline_flags.get_flying() || self.spline_flags.get_catmullrom()) // splines: MonsterMoveSplines
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct MonsterMoveData_SplineFlag {
    inner: u32,
    parabolic: Option<MonsterMoveData_SplineFlag_Parabolic>,
    animation: Option<MonsterMoveData_SplineFlag_Animation>,
}

impl MonsterMoveData_SplineFlag {
    pub const fn new(inner: u32, parabolic: Option<MonsterMoveData_SplineFlag_Parabolic>,animation: Option<MonsterMoveData_SplineFlag_Animation>,) -> Self {
        Self {
            inner,
            parabolic, 
            animation, 
        }
    }

    pub const fn empty() -> Self {
        Self {
            inner: 0,
            parabolic: None,
            animation: None,
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
        && self.parabolic.is_none()
        && self.animation.is_none()
    }

    pub const fn new_done() -> Self {
        Self {
            inner: SplineFlag::DONE,
            parabolic: None,
            animation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_done(mut self) -> Self {
        self.inner |= SplineFlag::DONE;
        self
    }

    pub const fn get_done(&self) -> bool {
        (self.inner & SplineFlag::DONE) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_done(mut self) -> Self {
        self.inner &= SplineFlag::DONE.reverse_bits();
        self
    }

    pub const fn new_falling() -> Self {
        Self {
            inner: SplineFlag::FALLING,
            parabolic: None,
            animation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_falling(mut self) -> Self {
        self.inner |= SplineFlag::FALLING;
        self
    }

    pub const fn get_falling(&self) -> bool {
        (self.inner & SplineFlag::FALLING) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_falling(mut self) -> Self {
        self.inner &= SplineFlag::FALLING.reverse_bits();
        self
    }

    pub const fn new_no_spline() -> Self {
        Self {
            inner: SplineFlag::NO_SPLINE,
            parabolic: None,
            animation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_no_spline(mut self) -> Self {
        self.inner |= SplineFlag::NO_SPLINE;
        self
    }

    pub const fn get_no_spline(&self) -> bool {
        (self.inner & SplineFlag::NO_SPLINE) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_no_spline(mut self) -> Self {
        self.inner &= SplineFlag::NO_SPLINE.reverse_bits();
        self
    }

    pub const fn new_parabolic(parabolic: MonsterMoveData_SplineFlag_Parabolic) -> Self {
        Self {
            inner: SplineFlag::PARABOLIC,
            parabolic: Some(parabolic),
            animation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_parabolic(mut self, parabolic: MonsterMoveData_SplineFlag_Parabolic) -> Self {
        self.inner |= SplineFlag::PARABOLIC;
        self.parabolic = Some(parabolic);
        self
    }

    pub const fn get_parabolic(&self) -> Option<&MonsterMoveData_SplineFlag_Parabolic> {
        self.parabolic.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_parabolic(mut self) -> Self {
        self.inner &= SplineFlag::PARABOLIC.reverse_bits();
        self.parabolic = None;
        self
    }

    pub const fn new_walk_mode() -> Self {
        Self {
            inner: SplineFlag::WALK_MODE,
            parabolic: None,
            animation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_walk_mode(mut self) -> Self {
        self.inner |= SplineFlag::WALK_MODE;
        self
    }

    pub const fn get_walk_mode(&self) -> bool {
        (self.inner & SplineFlag::WALK_MODE) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_walk_mode(mut self) -> Self {
        self.inner &= SplineFlag::WALK_MODE.reverse_bits();
        self
    }

    pub const fn new_flying() -> Self {
        Self {
            inner: SplineFlag::FLYING,
            parabolic: None,
            animation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_flying(mut self) -> Self {
        self.inner |= SplineFlag::FLYING;
        self
    }

    pub const fn get_flying(&self) -> bool {
        (self.inner & SplineFlag::FLYING) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_flying(mut self) -> Self {
        self.inner &= SplineFlag::FLYING.reverse_bits();
        self
    }

    pub const fn new_orientation_fixed() -> Self {
        Self {
            inner: SplineFlag::ORIENTATION_FIXED,
            parabolic: None,
            animation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_orientation_fixed(mut self) -> Self {
        self.inner |= SplineFlag::ORIENTATION_FIXED;
        self
    }

    pub const fn get_orientation_fixed(&self) -> bool {
        (self.inner & SplineFlag::ORIENTATION_FIXED) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_orientation_fixed(mut self) -> Self {
        self.inner &= SplineFlag::ORIENTATION_FIXED.reverse_bits();
        self
    }

    pub const fn new_final_point() -> Self {
        Self {
            inner: SplineFlag::FINAL_POINT,
            parabolic: None,
            animation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_final_point(mut self) -> Self {
        self.inner |= SplineFlag::FINAL_POINT;
        self
    }

    pub const fn get_final_point(&self) -> bool {
        (self.inner & SplineFlag::FINAL_POINT) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_final_point(mut self) -> Self {
        self.inner &= SplineFlag::FINAL_POINT.reverse_bits();
        self
    }

    pub const fn new_final_target() -> Self {
        Self {
            inner: SplineFlag::FINAL_TARGET,
            parabolic: None,
            animation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_final_target(mut self) -> Self {
        self.inner |= SplineFlag::FINAL_TARGET;
        self
    }

    pub const fn get_final_target(&self) -> bool {
        (self.inner & SplineFlag::FINAL_TARGET) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_final_target(mut self) -> Self {
        self.inner &= SplineFlag::FINAL_TARGET.reverse_bits();
        self
    }

    pub const fn new_final_angle() -> Self {
        Self {
            inner: SplineFlag::FINAL_ANGLE,
            parabolic: None,
            animation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_final_angle(mut self) -> Self {
        self.inner |= SplineFlag::FINAL_ANGLE;
        self
    }

    pub const fn get_final_angle(&self) -> bool {
        (self.inner & SplineFlag::FINAL_ANGLE) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_final_angle(mut self) -> Self {
        self.inner &= SplineFlag::FINAL_ANGLE.reverse_bits();
        self
    }

    pub const fn new_catmullrom() -> Self {
        Self {
            inner: SplineFlag::CATMULLROM,
            parabolic: None,
            animation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_catmullrom(mut self) -> Self {
        self.inner |= SplineFlag::CATMULLROM;
        self
    }

    pub const fn get_catmullrom(&self) -> bool {
        (self.inner & SplineFlag::CATMULLROM) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_catmullrom(mut self) -> Self {
        self.inner &= SplineFlag::CATMULLROM.reverse_bits();
        self
    }

    pub const fn new_cyclic() -> Self {
        Self {
            inner: SplineFlag::CYCLIC,
            parabolic: None,
            animation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_cyclic(mut self) -> Self {
        self.inner |= SplineFlag::CYCLIC;
        self
    }

    pub const fn get_cyclic(&self) -> bool {
        (self.inner & SplineFlag::CYCLIC) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_cyclic(mut self) -> Self {
        self.inner &= SplineFlag::CYCLIC.reverse_bits();
        self
    }

    pub const fn new_enter_cycle() -> Self {
        Self {
            inner: SplineFlag::ENTER_CYCLE,
            parabolic: None,
            animation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_enter_cycle(mut self) -> Self {
        self.inner |= SplineFlag::ENTER_CYCLE;
        self
    }

    pub const fn get_enter_cycle(&self) -> bool {
        (self.inner & SplineFlag::ENTER_CYCLE) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_enter_cycle(mut self) -> Self {
        self.inner &= SplineFlag::ENTER_CYCLE.reverse_bits();
        self
    }

    pub const fn new_animation(animation: MonsterMoveData_SplineFlag_Animation) -> Self {
        Self {
            inner: SplineFlag::ANIMATION,
            parabolic: None,
            animation: Some(animation),
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_animation(mut self, animation: MonsterMoveData_SplineFlag_Animation) -> Self {
        self.inner |= SplineFlag::ANIMATION;
        self.animation = Some(animation);
        self
    }

    pub const fn get_animation(&self) -> Option<&MonsterMoveData_SplineFlag_Animation> {
        self.animation.as_ref()
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_animation(mut self) -> Self {
        self.inner &= SplineFlag::ANIMATION.reverse_bits();
        self.animation = None;
        self
    }

    pub const fn new_frozen() -> Self {
        Self {
            inner: SplineFlag::FROZEN,
            parabolic: None,
            animation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_frozen(mut self) -> Self {
        self.inner |= SplineFlag::FROZEN;
        self
    }

    pub const fn get_frozen(&self) -> bool {
        (self.inner & SplineFlag::FROZEN) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_frozen(mut self) -> Self {
        self.inner &= SplineFlag::FROZEN.reverse_bits();
        self
    }

    pub const fn new_transport_enter() -> Self {
        Self {
            inner: SplineFlag::TRANSPORT_ENTER,
            parabolic: None,
            animation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_transport_enter(mut self) -> Self {
        self.inner |= SplineFlag::TRANSPORT_ENTER;
        self
    }

    pub const fn get_transport_enter(&self) -> bool {
        (self.inner & SplineFlag::TRANSPORT_ENTER) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_transport_enter(mut self) -> Self {
        self.inner &= SplineFlag::TRANSPORT_ENTER.reverse_bits();
        self
    }

    pub const fn new_transport_exit() -> Self {
        Self {
            inner: SplineFlag::TRANSPORT_EXIT,
            parabolic: None,
            animation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_transport_exit(mut self) -> Self {
        self.inner |= SplineFlag::TRANSPORT_EXIT;
        self
    }

    pub const fn get_transport_exit(&self) -> bool {
        (self.inner & SplineFlag::TRANSPORT_EXIT) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_transport_exit(mut self) -> Self {
        self.inner &= SplineFlag::TRANSPORT_EXIT.reverse_bits();
        self
    }

    pub const fn new_unknown7() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN7,
            parabolic: None,
            animation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown7(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN7;
        self
    }

    pub const fn get_unknown7(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN7) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown7(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN7.reverse_bits();
        self
    }

    pub const fn new_unknown8() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN8,
            parabolic: None,
            animation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown8(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN8;
        self
    }

    pub const fn get_unknown8(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN8) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown8(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN8.reverse_bits();
        self
    }

    pub const fn new_orientation_inversed() -> Self {
        Self {
            inner: SplineFlag::ORIENTATION_INVERSED,
            parabolic: None,
            animation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_orientation_inversed(mut self) -> Self {
        self.inner |= SplineFlag::ORIENTATION_INVERSED;
        self
    }

    pub const fn get_orientation_inversed(&self) -> bool {
        (self.inner & SplineFlag::ORIENTATION_INVERSED) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_orientation_inversed(mut self) -> Self {
        self.inner &= SplineFlag::ORIENTATION_INVERSED.reverse_bits();
        self
    }

    pub const fn new_unknown10() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN10,
            parabolic: None,
            animation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown10(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN10;
        self
    }

    pub const fn get_unknown10(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN10) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown10(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN10.reverse_bits();
        self
    }

    pub const fn new_unknown11() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN11,
            parabolic: None,
            animation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown11(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN11;
        self
    }

    pub const fn get_unknown11(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN11) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown11(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN11.reverse_bits();
        self
    }

    pub const fn new_unknown12() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN12,
            parabolic: None,
            animation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown12(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN12;
        self
    }

    pub const fn get_unknown12(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN12) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown12(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN12.reverse_bits();
        self
    }

    pub const fn new_unknown13() -> Self {
        Self {
            inner: SplineFlag::UNKNOWN13,
            parabolic: None,
            animation: None,
        }
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn set_unknown13(mut self) -> Self {
        self.inner |= SplineFlag::UNKNOWN13;
        self
    }

    pub const fn get_unknown13(&self) -> bool {
        (self.inner & SplineFlag::UNKNOWN13) != 0
    }

    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn clear_unknown13(mut self) -> Self {
        self.inner &= SplineFlag::UNKNOWN13.reverse_bits();
        self
    }

    pub(crate) const fn as_int(&self) -> u32 {
        self.inner
    }

}
impl MonsterMoveData_SplineFlag {
    pub(crate) const fn size(&self) -> usize {
        4 // inner
        + {
            if let Some(s) = &self.parabolic {
                8
            } else {
                0
            }
        }
        + {
            if let Some(s) = &self.animation {
                5
            } else {
                0
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct MonsterMoveData_SplineFlag_Parabolic {
    pub effect_start_time: u32,
    pub vertical_acceleration: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct MonsterMoveData_SplineFlag_Animation {
    pub animation_id: u8,
    pub animation_start_time: u32,
}

