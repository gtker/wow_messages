use std::convert::{TryFrom, TryInto};
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/faction/smsg_set_faction_visible.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/faction/smsg_set_faction_visible.wowm#L3):
/// ```text
/// smsg SMSG_SET_FACTION_VISIBLE = 0x0123 {
///     u32 reputation_list_id;
/// }
/// ```
pub struct SMSG_SET_FACTION_VISIBLE {
    pub reputation_list_id: u32,
}

impl ServerMessage for SMSG_SET_FACTION_VISIBLE {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // reputation_list_id: u32
        w.write_all(&self.reputation_list_id.to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x0123;

    fn size_without_size_or_opcode_fields(&self) -> u16 {
        4
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // reputation_list_id: u32
        let reputation_list_id = crate::util::read_u32_le(r)?;

        Ok(Self {
            reputation_list_id,
        })
    }

}

