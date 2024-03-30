use std::io::{Read, Write};

use crate::Guid;

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/smsg_petition_show_signatures.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/smsg_petition_show_signatures.wowm#L3):
/// ```text
/// struct PetitionSignature {
///     Guid signer;
///     u32 unknown1;
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct PetitionSignature {
    pub signer: Guid,
    pub unknown1: u32,
}

impl PetitionSignature {
    pub(crate) fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // signer: Guid
        w.write_all(&self.signer.guid().to_le_bytes())?;

        // unknown1: u32
        w.write_all(&self.unknown1.to_le_bytes())?;

        Ok(())
    }
}

impl PetitionSignature {
    pub(crate) fn read<R: std::io::Read>(mut r: R) -> Result<Self, std::io::Error> {
        // signer: Guid
        let signer = crate::util::read_guid(&mut r)?;

        // unknown1: u32
        let unknown1 = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            signer,
            unknown1,
        })
    }

}

