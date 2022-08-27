use std::convert::{TryFrom, TryInto};
use crate::world::version_1_12::GuildEmblemResult;
use crate::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/guild/msg_save_guild_emblem_server.wowm:22`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/guild/msg_save_guild_emblem_server.wowm#L22):
/// ```text
/// smsg MSG_SAVE_GUILD_EMBLEM_Server = 0x01F1 {
///     GuildEmblemResult result;
/// }
/// ```
pub struct MSG_SAVE_GUILD_EMBLEM_Server {
    pub result: GuildEmblemResult,
}

impl ServerMessage for MSG_SAVE_GUILD_EMBLEM_Server {
    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // result: GuildEmblemResult
        w.write_all(&(self.result.as_int() as u32).to_le_bytes())?;

        Ok(())
    }
    const OPCODE: u16 = 0x01f1;

    fn server_size(&self) -> u16 {
        8
    }

    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // result: GuildEmblemResult
        let result: GuildEmblemResult = crate::util::read_u32_le(r)?.try_into()?;

        Ok(Self {
            result,
        })
    }

}

