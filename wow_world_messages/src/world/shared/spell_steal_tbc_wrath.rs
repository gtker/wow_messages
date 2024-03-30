use std::io::{Read, Write};

use wow_world_base::shared::spell_steal_action_tbc_wrath::SpellStealAction;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_spellsteallog.wowm:8`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_spellsteallog.wowm#L8):
/// ```text
/// struct SpellSteal {
///     Spell spell;
///     SpellStealAction action;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SpellSteal {
    pub spell: u32,
    pub action: SpellStealAction,
}

impl SpellSteal {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // spell: Spell
        w.write_all(&self.spell.to_le_bytes())?;

        // action: SpellStealAction
        w.write_all(&(self.action.as_int().to_le_bytes()))?;

        Ok(())
    }
}

impl SpellSteal {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, crate::errors::ParseErrorKind> {
        // spell: Spell
        let spell = crate::util::read_u32_le(&mut r)?;

        // action: SpellStealAction
        let action = crate::util::read_u8_le(&mut r)?.try_into()?;

        Ok(Self {
            spell,
            action,
        })
    }

}

