use std::convert::{TryFrom, TryInto};
use std::io::{Write, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/unimplemented/vanilla/smsg_send_mail_result.wowm:31`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/unimplemented/vanilla/smsg_send_mail_result.wowm#L31):
/// ```text
/// smsg SMSG_SEND_MAIL_RESULT = 0x0239 {
///     unimplemented
/// }
/// ```
pub struct SMSG_SEND_MAIL_RESULT {
    pub unimplemented: Vec<u8>,
}

impl crate::Message for SMSG_SEND_MAIL_RESULT {
    const OPCODE: u32 = 0x0239;

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, w: &mut Vec<u8>) -> Result<(), std::io::Error> {
        let size_assert_header_size = w.len();
        // unimplemented: u8[-]
        for i in self.unimplemented.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        assert_eq!(self.size() as usize + size_assert_header_size, w.len(), "Mismatch in pre-calculated size and actual written size. This needs investigation as it will cause problems in the game client when sent");
        Ok(())
    }
    fn read_body(r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        // unimplemented: u8[-]
        let mut current_size = {
            0
        };
        let mut unimplemented = Vec::with_capacity(body_size as usize - current_size);
        while current_size < (body_size as usize) {
            unimplemented.push(crate::util::read_u8_le(r)?);
            current_size += 1;
        }

        Ok(Self {
            unimplemented,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::world::vanilla::ServerMessage for SMSG_SEND_MAIL_RESULT {}

impl SMSG_SEND_MAIL_RESULT {
    pub(crate) fn size(&self) -> usize {
        self.unimplemented.len() * core::mem::size_of::<u8>() // unimplemented: u8[-]
    }
}

