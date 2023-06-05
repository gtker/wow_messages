use std::io::{Read, Write};

use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/cinematic/smsg_override_light.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/cinematic/smsg_override_light.wowm#L1):
/// ```text
/// smsg SMSG_OVERRIDE_LIGHT = 0x0411 {
///     u32 default_id;
///     u32 id_override;
///     Seconds fade_in_time;
/// }
/// ```
pub struct SMSG_OVERRIDE_LIGHT {
    pub default_id: u32,
    pub id_override: u32,
    pub fade_in_time: Duration,
}

#[cfg(feature = "print-testcase")]
impl SMSG_OVERRIDE_LIGHT {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_OVERRIDE_LIGHT {{").unwrap();
        // Members
        writeln!(s, "    default_id = {};", self.default_id).unwrap();
        writeln!(s, "    id_override = {};", self.id_override).unwrap();
        writeln!(s, "    fade_in_time = {};", self.fade_in_time.as_secs()).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 14_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1041_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "default_id", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "id_override", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "fade_in_time", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"2.4.3\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_OVERRIDE_LIGHT {}
impl crate::Message for SMSG_OVERRIDE_LIGHT {
    const OPCODE: u32 = 0x0411;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_OVERRIDE_LIGHT::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // default_id: u32
        w.write_all(&self.default_id.to_le_bytes())?;

        // id_override: u32
        w.write_all(&self.id_override.to_le_bytes())?;

        // fade_in_time: Seconds
        w.write_all((self.fade_in_time.as_secs() as u32).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 12 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0411, size: body_size });
        }

        // default_id: u32
        let default_id = crate::util::read_u32_le(&mut r)?;

        // id_override: u32
        let id_override = crate::util::read_u32_le(&mut r)?;

        // fade_in_time: Seconds
        let fade_in_time = Duration::from_secs(crate::util::read_u32_le(&mut r)?.into());

        Ok(Self {
            default_id,
            id_override,
            fade_in_time,
        })
    }

}
#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_OVERRIDE_LIGHT {}

