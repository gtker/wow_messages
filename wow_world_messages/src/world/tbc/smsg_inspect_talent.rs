use crate:: {
    Guid,
};
use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_inspect_talent.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_inspect_talent.wowm#L1):
/// ```text
/// smsg SMSG_INSPECT_TALENT = 0x03F3 {
///     PackedGuid player;
///     u8[-] talent_data;
/// }
/// ```
pub struct SMSG_INSPECT_TALENT {
    pub player: Guid,
    pub talent_data: Vec<u8>,
}

impl crate::Message for SMSG_INSPECT_TALENT {
    const OPCODE: u32 = 0x03f3;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // player: PackedGuid
        self.player.write_packed_guid_into_vec(&mut w)?;

        // talent_data: u8[-]
        for i in self.talent_data.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if !(2..=65544).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x03F3, size: body_size as u32 });
        }

        // player: PackedGuid
        let player = Guid::read_packed(&mut r)?;

        // talent_data: u8[-]
        let talent_data = {
            let mut current_size = {
                player.size() // player: Guid
            };
            let mut talent_data = Vec::with_capacity(body_size as usize - current_size);
            while current_size < (body_size as usize) {
                talent_data.push(crate::util::read_u8_le(&mut r)?);
                current_size += 1;
            }
            talent_data
        };

        Ok(Self {
            player,
            talent_data,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_INSPECT_TALENT {}

impl SMSG_INSPECT_TALENT {
    pub(crate) fn size(&self) -> usize {
        self.player.size() // player: Guid
        + self.talent_data.len() * core::mem::size_of::<u8>() // talent_data: u8[-]
    }
}

