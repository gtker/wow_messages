use std::convert::{TryFrom, TryInto};
use std::io::Write;

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm:14`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_item_query_single_response.wowm#L14):
/// ```text
/// struct ItemDamageType {
///     u32 damage_minimum;
///     u32 damage_maximum;
///     u32 damage_type;
/// }
/// ```
pub struct ItemDamageType {
    pub damage_minimum: u32,
    pub damage_maximum: u32,
    /// # Comment
    ///
    /// mangoszero/vmangos/cmangos: id from Resistances.dbc
    pub damage_type: u32,
}

impl ItemDamageType {
    pub(crate) fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // damage_minimum: u32
        w.write_all(&self.damage_minimum.to_le_bytes())?;

        // damage_maximum: u32
        w.write_all(&self.damage_maximum.to_le_bytes())?;

        // damage_type: u32
        w.write_all(&self.damage_type.to_le_bytes())?;

        Ok(())
    }
}

impl ItemDamageType {
    pub(crate) fn read<R: std::io::Read>(r: &mut R) -> std::result::Result<Self, crate::errors::ParseError> {
        // damage_minimum: u32
        let damage_minimum = crate::util::read_u32_le(r)?;

        // damage_maximum: u32
        let damage_maximum = crate::util::read_u32_le(r)?;

        // damage_type: u32
        let damage_type = crate::util::read_u32_le(r)?;

        Ok(Self {
            damage_minimum,
            damage_maximum,
            damage_type,
        })
    }

}

