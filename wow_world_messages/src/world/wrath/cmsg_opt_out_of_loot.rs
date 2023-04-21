use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/loot/cmsg_opt_out_of_loot.wowm:7`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/loot/cmsg_opt_out_of_loot.wowm#L7):
/// ```text
/// cmsg CMSG_OPT_OUT_OF_LOOT = 0x0409 {
///     Bool32 pass_on_loot;
/// }
/// ```
pub struct CMSG_OPT_OUT_OF_LOOT {
    pub pass_on_loot: bool,
}

impl crate::private::Sealed for CMSG_OPT_OUT_OF_LOOT {}
impl crate::Message for CMSG_OPT_OUT_OF_LOOT {
    const OPCODE: u32 = 0x0409;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // pass_on_loot: Bool32
        w.write_all(u32::from(self.pass_on_loot).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0409, size: body_size as u32 });
        }

        // pass_on_loot: Bool32
        let pass_on_loot = crate::util::read_u32_le(&mut r)? != 0;

        Ok(Self {
            pass_on_loot,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_OPT_OUT_OF_LOOT {}

