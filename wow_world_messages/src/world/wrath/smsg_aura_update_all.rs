use crate:: {
    Guid,
};
use crate::wrath::AuraUpdate;
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_aura_update_all.wowm:35`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_aura_update_all.wowm#L35):
/// ```text
/// smsg SMSG_AURA_UPDATE_ALL = 0x0495 {
///     PackedGuid unit;
///     AuraUpdate[-] aura_updates;
/// }
/// ```
pub struct SMSG_AURA_UPDATE_ALL {
    pub unit: Guid,
    pub aura_updates: Vec<AuraUpdate>,
}

impl crate::Message for SMSG_AURA_UPDATE_ALL {
    const OPCODE: u32 = 0x0495;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // unit: PackedGuid
        self.unit.write_packed_guid_into_vec(&mut w)?;

        // aura_updates: AuraUpdate[-]
        for i in self.aura_updates.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(2..=65544).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0495, size: body_size as u32 });
        }

        // unit: PackedGuid
        let unit = Guid::read_packed(&mut r)?;

        // aura_updates: AuraUpdate[-]
        let aura_updates = {
            let mut current_size = {
                unit.size() // unit: PackedGuid
            };
            let mut aura_updates = Vec::with_capacity(body_size as usize - current_size);
            while current_size < (body_size as usize) {
                aura_updates.push(AuraUpdate::read(&mut r)?);
                current_size += 1;
            }
            aura_updates
        };

        Ok(Self {
            unit,
            aura_updates,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_AURA_UPDATE_ALL {}

impl SMSG_AURA_UPDATE_ALL {
    pub(crate) fn size(&self) -> usize {
        self.unit.size() // unit: PackedGuid
        + self.aura_updates.iter().fold(0, |acc, x| acc + x.size()) // aura_updates: AuraUpdate[-]
    }
}

