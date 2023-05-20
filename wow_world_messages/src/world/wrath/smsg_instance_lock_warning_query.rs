use std::io::{Read, Write};

use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/queries/smsg_instance_lock_warning_query.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/queries/smsg_instance_lock_warning_query.wowm#L1):
/// ```text
/// smsg SMSG_INSTANCE_LOCK_WARNING_QUERY = 0x0147 {
///     Milliseconds time;
///     u32 encounter_mask;
///     u8 unknown;
/// }
/// ```
pub struct SMSG_INSTANCE_LOCK_WARNING_QUERY {
    pub time: Duration,
    pub encounter_mask: u32,
    pub unknown: u8,
}

impl crate::private::Sealed for SMSG_INSTANCE_LOCK_WARNING_QUERY {}
impl crate::Message for SMSG_INSTANCE_LOCK_WARNING_QUERY {
    const OPCODE: u32 = 0x0147;

    fn size_without_header(&self) -> u32 {
        9
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // time: Milliseconds
        w.write_all((self.time.as_millis() as u32).to_le_bytes().as_slice())?;

        // encounter_mask: u32
        w.write_all(&self.encounter_mask.to_le_bytes())?;

        // unknown: u8
        w.write_all(&self.unknown.to_le_bytes())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 9 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0147, size: body_size });
        }

        // time: Milliseconds
        let time = Duration::from_millis(crate::util::read_u32_le(&mut r)?.into());

        // encounter_mask: u32
        let encounter_mask = crate::util::read_u32_le(&mut r)?;

        // unknown: u8
        let unknown = crate::util::read_u8_le(&mut r)?;

        Ok(Self {
            time,
            encounter_mask,
            unknown,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_INSTANCE_LOCK_WARNING_QUERY {}

