use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_party_member_stats.wowm:140`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_party_member_stats.wowm#L140):
/// ```text
/// struct Aura {
///     u16 aura;
///     u8 unknown;
/// }
/// ```
pub struct Aura {
    pub aura: u16,
    pub unknown: u8,
}

impl Aura {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // aura: u16
        w.write_all(&self.aura.to_le_bytes())?;

        // unknown: u8
        w.write_all(&self.unknown.to_le_bytes())?;

        Ok(())
    }
}

impl Aura {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, std::io::Error> {
        // aura: u16
        let aura = crate::util::read_u16_le(&mut r)?;

        // unknown: u8
        let unknown = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            aura,
            unknown,
        })
    }

}

