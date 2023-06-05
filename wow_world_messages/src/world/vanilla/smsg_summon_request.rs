use std::io::{Read, Write};

use crate::Guid;
use crate::vanilla::Area;
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/spell/smsg_summon_request.wowm:3`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/spell/smsg_summon_request.wowm#L3):
/// ```text
/// smsg SMSG_SUMMON_REQUEST = 0x02AB {
///     Guid summoner;
///     Area area;
///     Milliseconds auto_decline_time;
/// }
/// ```
pub struct SMSG_SUMMON_REQUEST {
    pub summoner: Guid,
    pub area: Area,
    pub auto_decline_time: Duration,
}

#[cfg(feature = "print-testcase")]
impl SMSG_SUMMON_REQUEST {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_SUMMON_REQUEST {{").unwrap();
        // Members
        writeln!(s, "    summoner = {};", self.summoner.guid()).unwrap();
        writeln!(s, "    area = {};", self.area.as_test_case_value()).unwrap();
        writeln!(s, "    auto_decline_time = {};", self.auto_decline_time.as_millis()).unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = 20_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 683_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 8, "summoner");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"1.12\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for SMSG_SUMMON_REQUEST {}
impl crate::Message for SMSG_SUMMON_REQUEST {
    const OPCODE: u32 = 0x02ab;

    fn size_without_header(&self) -> u32 {
        16
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // summoner: Guid
        w.write_all(&self.summoner.guid().to_le_bytes())?;

        // area: Area
        w.write_all(&(self.area.as_int().to_le_bytes()))?;

        // auto_decline_time: Milliseconds
        w.write_all((self.auto_decline_time.as_millis() as u32).to_le_bytes().as_slice())?;

        Ok(())
    }
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if body_size != 16 {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x02AB, size: body_size });
        }

        // summoner: Guid
        let summoner = crate::util::read_guid(&mut r)?;

        // area: Area
        let area = crate::util::read_u32_le(&mut r)?.try_into()?;

        // auto_decline_time: Milliseconds
        let auto_decline_time = Duration::from_millis(crate::util::read_u32_le(&mut r)?.into());

        Ok(Self {
            summoner,
            area,
            auto_decline_time,
        })
    }

}
#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_SUMMON_REQUEST {}

