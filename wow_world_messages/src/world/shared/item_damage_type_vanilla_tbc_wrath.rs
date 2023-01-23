use std::convert::{TryFrom, TryInto};
use wow_world_base::shared::spell_school_vanilla_vanilla_tbc_wrath::SpellSchool;
use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm:53`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm#L53):
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
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // damage_minimum: f32
        w.write_all(&self.damage_minimum.to_le_bytes())?;

        // damage_maximum: f32
        w.write_all(&self.damage_maximum.to_le_bytes())?;

        // school: SpellSchool
        w.write_all(&(self.school.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
}

impl ItemDamageType {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // damage_minimum: f32
        let damage_minimum = crate::util::read_f32_le(r)?;
        // damage_maximum: f32
        let damage_maximum = crate::util::read_f32_le(r)?;
        // school: SpellSchool
        let school: SpellSchool = (crate::util::read_u32_le(r)? as u8).try_into()?;

        Ok(Self {
            damage_minimum,
            damage_maximum,
            school,
        })
    }

}

