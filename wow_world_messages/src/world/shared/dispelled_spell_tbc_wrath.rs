use std::io::{Read, Write};
use wow_world_base::shared::dispel_method_tbc_wrath::DispelMethod;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spelldispellog.wowm:30`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spelldispellog.wowm#L30):
/// ```text
/// struct DispelledSpell {
///     u32 spell;
///     DispelMethod method;
/// }
/// ```
pub struct DispelledSpell {
    pub spell: u32,
    pub method: DispelMethod,
}

impl DispelledSpell {
    pub(crate) fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // spell: u32
        w.write_all(&self.spell.to_le_bytes())?;

        // method: DispelMethod
        w.write_all(&u8::from(self.method.as_int()).to_le_bytes())?;

        Ok(())
    }
}

impl DispelledSpell {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> std::result::Result<Self, crate::errors::ParseError> {
        // spell: u32
        let spell = crate::util::read_u32_le(&mut r)?;

        // method: DispelMethod
        let method: DispelMethod = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            spell,
            method,
        })
    }

}

