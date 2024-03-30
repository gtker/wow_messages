use std::io::{Read, Write};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_attackerstateupdate.wowm:29`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_attackerstateupdate.wowm#L29):
/// ```text
/// struct DamageInfo {
///     u32 spell_school_mask;
///     f32 damage_float;
///     u32 damage_uint;
///     u32 absorb;
///     u32 resist;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct DamageInfo {
    pub spell_school_mask: u32,
    /// vmangos sends the same data in `damage_uint`.
    pub damage_float: f32,
    /// vmangos sends the same data in `damage_float`.
    pub damage_uint: u32,
    pub absorb: u32,
    pub resist: u32,
}

impl DamageInfo {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // spell_school_mask: u32
        w.write_all(&self.spell_school_mask.to_le_bytes())?;

        // damage_float: f32
        w.write_all(&self.damage_float.to_le_bytes())?;

        // damage_uint: u32
        w.write_all(&self.damage_uint.to_le_bytes())?;

        // absorb: u32
        w.write_all(&self.absorb.to_le_bytes())?;

        // resist: u32
        w.write_all(&self.resist.to_le_bytes())?;

        Ok(())
    }
}

impl DamageInfo {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, std::io::Error> {
        // spell_school_mask: u32
        let spell_school_mask = crate::util::read_u32_le(&mut r)?;

        // damage_float: f32
        let damage_float = crate::util::read_f32_le(&mut r)?;

        // damage_uint: u32
        let damage_uint = crate::util::read_u32_le(&mut r)?;

        // absorb: u32
        let absorb = crate::util::read_u32_le(&mut r)?;

        // resist: u32
        let resist = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            spell_school_mask,
            damage_float,
            damage_uint,
            absorb,
            resist,
        })
    }

}

