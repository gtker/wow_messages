use std::io::{Write, Read};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/cinematic/smsg_override_light.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/cinematic/smsg_override_light.wowm#L9):
/// ```text
/// smsg SMSG_OVERRIDE_LIGHT = 0x0412 {
///     u32 default_id;
///     u32 id_override;
///     u32 fade_in_time_in_seconds;
/// }
/// ```
pub struct SMSG_OVERRIDE_LIGHT {
    pub default_id: u32,
    pub id_override: u32,
    pub fade_in_time_in_seconds: u32,
}

impl crate::Message for SMSG_OVERRIDE_LIGHT {
    const OPCODE: u32 = 0x0412;

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl std::io::Write) -> Result<(), std::io::Error> {
        // default_id: u32
        w.write_all(&self.default_id.to_le_bytes())?;

        // id_override: u32
        w.write_all(&self.id_override.to_le_bytes())?;

        // fade_in_time_in_seconds: u32
        w.write_all(&self.fade_in_time_in_seconds.to_le_bytes())?;

        Ok(())
    }
    fn read_body(mut r: &mut &[u8], body_size: u32) -> std::result::Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0412, size: body_size as u32 });
        }

        // default_id: u32
        let default_id = crate::util::read_u32_le(&mut r)?;

        // id_override: u32
        let id_override = crate::util::read_u32_le(&mut r)?;

        // fade_in_time_in_seconds: u32
        let fade_in_time_in_seconds = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            default_id,
            id_override,
            fade_in_time_in_seconds,
        })
    }

}
#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_OVERRIDE_LIGHT {}

