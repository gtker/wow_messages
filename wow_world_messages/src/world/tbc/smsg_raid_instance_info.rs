use std::io::{Read, Write};

use crate::tbc::{
    Map, RaidInfo,
};

/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/raid/smsg_raid_instance_info.wowm:9`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/raid/smsg_raid_instance_info.wowm#L9):
/// ```text
/// smsg SMSG_RAID_INSTANCE_INFO = 0x02CC {
///     u32 amount_of_raid_infos;
///     RaidInfo[amount_of_raid_infos] raid_infos;
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct SMSG_RAID_INSTANCE_INFO {
    pub raid_infos: Vec<RaidInfo>,
}

impl crate::private::Sealed for SMSG_RAID_INSTANCE_INFO {}
impl SMSG_RAID_INSTANCE_INFO {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(4..=65535).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // amount_of_raid_infos: u32
        let amount_of_raid_infos = crate::util::read_u32_le(&mut r)?;

        // raid_infos: RaidInfo[amount_of_raid_infos]
        let raid_infos = {
            let mut raid_infos = Vec::with_capacity(amount_of_raid_infos as usize);

            let allocation_size = u64::from(amount_of_raid_infos) * 16;
            if allocation_size > crate::errors::MAX_ALLOCATION_SIZE {
                return Err(crate::errors::ParseErrorKind::AllocationTooLargeError(allocation_size));
            }

            for _ in 0..amount_of_raid_infos {
                raid_infos.push(RaidInfo::read(&mut r)?);
            }
            raid_infos
        };

        Ok(Self {
            raid_infos,
        })
    }

}

impl crate::Message for SMSG_RAID_INSTANCE_INFO {
    const OPCODE: u32 = 0x02cc;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "SMSG_RAID_INSTANCE_INFO"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_RAID_INSTANCE_INFO {{").unwrap();
        // Members
        writeln!(s, "    amount_of_raid_infos = {};", self.raid_infos.len()).unwrap();
        writeln!(s, "    raid_infos = [").unwrap();
        for v in self.raid_infos.as_slice() {
            writeln!(s, "        {{").unwrap();
            // Members
            writeln!(s, "            map = {};", v.map.as_test_case_value()).unwrap();
            writeln!(s, "            reset_time = {};", v.reset_time).unwrap();
            writeln!(s, "            instance_id = {};", v.instance_id).unwrap();
            writeln!(s, "            index = {};", v.index).unwrap();

            writeln!(s, "        }},").unwrap();
        }
        writeln!(s, "    ];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 716_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_raid_infos", "    ");
        if !self.raid_infos.is_empty() {
            writeln!(s, "    /* raid_infos: RaidInfo[amount_of_raid_infos] start */").unwrap();
            for (i, v) in self.raid_infos.iter().enumerate() {
                writeln!(s, "    /* raid_infos: RaidInfo[amount_of_raid_infos] {i} start */").unwrap();
                crate::util::write_bytes(&mut s, &mut bytes, 4, "map", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "reset_time", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "instance_id", "        ");
                crate::util::write_bytes(&mut s, &mut bytes, 4, "index", "        ");
                writeln!(s, "    /* raid_infos: RaidInfo[amount_of_raid_infos] {i} end */").unwrap();
            }
            writeln!(s, "    /* raid_infos: RaidInfo[amount_of_raid_infos] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("2.4.3".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // amount_of_raid_infos: u32
        w.write_all(&(self.raid_infos.len() as u32).to_le_bytes())?;

        // raid_infos: RaidInfo[amount_of_raid_infos]
        for i in self.raid_infos.iter() {
            i.write_into_vec(&mut w)?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(716, "SMSG_RAID_INSTANCE_INFO", body_size, a))
    }

}

#[cfg(feature = "tbc")]
impl crate::tbc::ServerMessage for SMSG_RAID_INSTANCE_INFO {}

impl SMSG_RAID_INSTANCE_INFO {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_raid_infos: u32
        + self.raid_infos.len() * 16 // raid_infos: RaidInfo[amount_of_raid_infos]
    }
}

