use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/lfg/cmsg_lfg_set_boot_vote.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/lfg/cmsg_lfg_set_boot_vote.wowm#L1):
/// ```text
/// cmsg CMSG_LFG_SET_BOOT_VOTE = 0x036C {
///     Bool agree_to_kick_player;
/// }
/// ```
pub struct CMSG_LFG_SET_BOOT_VOTE {
    pub agree_to_kick_player: bool,
}

impl crate::private::Sealed for CMSG_LFG_SET_BOOT_VOTE {}
impl crate::Message for CMSG_LFG_SET_BOOT_VOTE {
    const OPCODE: u32 = 0x036c;

    fn size_without_header(&self) -> u32 {
        1
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // agree_to_kick_player: Bool
        w.write_all(u8::from(self.agree_to_kick_player).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 1 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x036C, size: body_size });
        }

        // agree_to_kick_player: Bool
        let agree_to_kick_player = crate::util::read_u8_le(&mut r)? != 0;

        Ok(Self {
            agree_to_kick_player,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_LFG_SET_BOOT_VOTE {}

