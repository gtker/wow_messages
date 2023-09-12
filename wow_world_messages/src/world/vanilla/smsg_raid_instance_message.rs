use std::io::{Read, Write};

use crate::vanilla::{
    Map, RaidInstanceMessage,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/smsg_raid_instance_message.wowm:18`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_raid_instance_message.wowm#L18):
/// ```text
/// smsg SMSG_RAID_INSTANCE_MESSAGE = 0x02FA {
///     RaidInstanceMessage message_type;
///     Map map;
///     u32 time_left;
/// }
/// ```
pub struct SMSG_RAID_INSTANCE_MESSAGE {
    pub message_type: RaidInstanceMessage,
    pub map: Map,
    pub time_left: u32,
}

impl crate::private::Sealed for SMSG_RAID_INSTANCE_MESSAGE {}
impl SMSG_RAID_INSTANCE_MESSAGE {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if body_size != 12 {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // message_type: RaidInstanceMessage
        let message_type = crate::util::read_u32_le(&mut r)?.try_into()?;

        // map: Map
        let map = crate::util::read_u32_le(&mut r)?.try_into()?;

        // time_left: u32
        let time_left = crate::util::read_u32_le(&mut r)?;

        Ok(Self {
            message_type,
            map,
            time_left,
        })
    }

}

impl crate::Message for SMSG_RAID_INSTANCE_MESSAGE {
    const OPCODE: u32 = 0x02fa;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_RAID_INSTANCE_MESSAGE"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_RAID_INSTANCE_MESSAGE {{").unwrap();
        // Members
        writeln!(s, "    message_type = {};", self.message_type.as_test_case_value()).unwrap();
        writeln!(s, "    map = {};", self.map.as_test_case_value()).unwrap();
        writeln!(s, "    time_left = {};", self.time_left).unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = 14_u16.to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 762_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "message_type", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "map", "    ");
        crate::util::write_bytes(&mut s, &mut bytes, 4, "time_left", "    ");


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("1.12".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        12
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // message_type: RaidInstanceMessage
        w.write_all(&(self.message_type.as_int().to_le_bytes()))?;

        // map: Map
        w.write_all(&(self.map.as_int().to_le_bytes()))?;

        // time_left: u32
        w.write_all(&self.time_left.to_le_bytes())?;

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(762, "SMSG_RAID_INSTANCE_MESSAGE", body_size, a))
    }

}

#[cfg(feature = "vanilla")]
impl crate::vanilla::ServerMessage for SMSG_RAID_INSTANCE_MESSAGE {}

