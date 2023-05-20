use std::io::{Read, Write};

use crate::shared::spell_school_vanilla_vanilla_tbc_wrath::SpellSchool;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm:85`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm#L85):
/// ```text
/// struct ItemDamageType {
///     f32 damage_minimum;
///     f32 damage_maximum;
///     (u32)SpellSchool school;
/// }
/// ```
pub struct ItemDamageType {
    pub damage_minimum: f32,
    pub damage_maximum: f32,
    pub school: SpellSchool,
}

impl ItemDamageType {
    pub fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // damage_minimum: f32
        w.write_all(&self.damage_minimum.to_le_bytes())?;

        // damage_maximum: f32
        w.write_all(&self.damage_maximum.to_le_bytes())?;

        // school: SpellSchool
        w.write_all(&u32::from(self.school.as_int()).to_le_bytes())?;

        Ok(())
    }
}

impl ItemDamageType {
    pub fn read<R: Read>(mut r: R) -> Result<Self, crate::errors::ParseError> {
        // damage_minimum: f32
        let damage_minimum = crate::util::read_f32_le(&mut r)?;

        // damage_maximum: f32
        let damage_maximum = crate::util::read_f32_le(&mut r)?;

        // school: SpellSchool
        let school: SpellSchool = (crate::util::read_u32_le(&mut r)? as u8).try_into()?;

        Ok(Self {
            damage_minimum,
            damage_maximum,
            school,
        })
    }

}

