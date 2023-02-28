use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/battleground/smsg_battlefield_mgr_entered.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/battleground/smsg_battlefield_mgr_entered.wowm#L1):
/// ```text
/// smsg SMSG_BATTLEFIELD_MGR_ENTERED = 0x04E0 {
///     u32 battle_id;
///     u8 unknown1;
///     u8 unknown2;
///     Bool clear_afk;
/// }
/// ```
pub struct SMSG_BATTLEFIELD_MGR_ENTERED {
    pub battle_id: u32,
    pub unknown1: u8,
    pub unknown2: u8,
    pub clear_afk: bool,
}

impl crate::Message for SMSG_BATTLEFIELD_MGR_ENTERED {
    const OPCODE: u32 = 0x04e0;

    fn size_without_header(&self) -> u32 {
        7
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // battle_id: u32
        w.write_all(&self.battle_id.to_le_bytes())?;

        // unknown1: u8
        w.write_all(&self.unknown1.to_le_bytes())?;

        // unknown2: u8
        w.write_all(&self.unknown2.to_le_bytes())?;

        // clear_afk: Bool
        w.write_all(u8::from(self.clear_afk).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 7 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x04E0, size: body_size as u32 });
        }

        // battle_id: u32
        let battle_id = crate::util::read_u32_le(&mut r)?;

        // unknown1: u8
        let unknown1 = crate::util::read_u8_le(&mut r)?;

        // unknown2: u8
        let unknown2 = crate::util::read_u8_le(&mut r)?;

        // clear_afk: Bool
        let clear_afk = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            battle_id,
            unknown1,
            unknown2,
            clear_afk,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_BATTLEFIELD_MGR_ENTERED {}

