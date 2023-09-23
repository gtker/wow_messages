use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/combat/smsg_attackerstateupdate_3_3_5.wowm:41`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/combat/smsg_attackerstateupdate_3_3_5.wowm#L41):
/// ```text
/// struct DamageInfo {
///     u32 spell_school_mask;
///     f32 damage_float;
///     u32 damage_uint;
/// }
/// ```
pub struct DamageInfo {
    pub spell_school_mask: u32,
    /// arcemu sends the same data in `damage_uint`.
    pub damage_float: f32,
    /// arcemu sends the same data in `damage_float`.
    pub damage_uint: u32,
}

impl DamageInfo {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // spell_school_mask: u32
        w.write_all(&self.spell_school_mask.to_le_bytes())?;

        // damage_float: f32
        w.write_all(&self.damage_float.to_le_bytes())?;

        // damage_uint: u32
        w.write_all(&self.damage_uint.to_le_bytes())?;

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

        Ok(Self {
            spell_school_mask,
            damage_float,
            damage_uint,
        })
    }

}

