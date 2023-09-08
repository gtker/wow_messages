use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_resync_runes.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_resync_runes.wowm#L1):
/// ```text
/// struct ResyncRune {
///     u8 current_rune;
///     u8 rune_cooldown;
/// }
/// ```
pub struct ResyncRune {
    pub current_rune: u8,
    pub rune_cooldown: u8,
}

impl ResyncRune {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // current_rune: u8
        w.write_all(&self.current_rune.to_le_bytes())?;

        // rune_cooldown: u8
        w.write_all(&self.rune_cooldown.to_le_bytes())?;

        Ok(())
    }
}

impl ResyncRune {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, std::io::Error> {
        // current_rune: u8
        let current_rune = crate::util::read_u8_le(&mut r)?;

        // rune_cooldown: u8
        let rune_cooldown = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            current_rune,
            rune_cooldown,
        })
    }

}

