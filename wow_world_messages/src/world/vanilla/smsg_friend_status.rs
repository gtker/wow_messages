use std::convert::{TryFrom, TryInto};
use crate::Guid;
use crate::world::vanilla::FriendResult;
use crate::world::vanilla::ServerMessage;
use wow_srp::header_crypto::Encrypter;
use std::io::{Write, Read};

#[derive(Debug, PartialEq, Clone, Default)]
#[derive(Copy)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/social/smsg_friend_status.wowm:33`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/social/smsg_friend_status.wowm#L33):
/// ```text
/// smsg SMSG_FRIEND_STATUS = 0x0068 {
///     FriendResult result;
///     Guid guid;
/// }
/// ```
pub struct SMSG_FRIEND_STATUS {
    pub result: FriendResult,
    pub guid: Guid,
}

impl crate::Message for SMSG_FRIEND_STATUS {
    const OPCODE: u32 = 0x0068;

    fn size_without_header(&self) -> u32 {
        9
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        // result: FriendResult
        w.write_all(&(self.result.as_int() as u8).to_le_bytes())?;

        // guid: Guid
        w.write_all(&self.guid.guid().to_le_bytes())?;

        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 9 {
            return Err(crate::errors::ParseError::InvalidSize(body_size as u32));
        }

        // result: FriendResult
        let result: FriendResult = crate::util::read_u8_le(r)?.try_into()?;

        // guid: Guid
        let guid = Guid::read(r)?;

        Ok(Self {
            result,
            guid,
        })
    }

}
impl ServerMessage for SMSG_FRIEND_STATUS {}

