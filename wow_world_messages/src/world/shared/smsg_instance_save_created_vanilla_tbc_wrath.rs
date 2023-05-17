use std::io::{Read, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/smsg_instance_save_created.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_instance_save_created.wowm#L3):
/// ```text
/// smsg SMSG_INSTANCE_SAVE_CREATED = 0x02CB {
///     u32 unknown;
/// }
/// ```
pub struct SMSG_INSTANCE_SAVE_CREATED {
    /// All emulators across all versions set to 0
    ///
    pub unknown: u32,
}

impl crate::private::Sealed for SMSG_INSTANCE_SAVE_CREATED {}
impl crate::Message for SMSG_INSTANCE_SAVE_CREATED {
    const OPCODE: u32 = 0x02cb;

    fn size_without_header(&self) -> u32 {
        4
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // unknown: u32
        w.write_all(&self.unknown.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 4 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02CB, size: body_size });
        }

        // unknown: u32
        let unknown = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            unknown,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_INSTANCE_SAVE_CREATED {}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_INSTANCE_SAVE_CREATED {}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_INSTANCE_SAVE_CREATED {}

