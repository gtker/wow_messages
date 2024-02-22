use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/cmsg_quest_poi_query.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/cmsg_quest_poi_query.wowm#L1):
/// ```text
/// cmsg CMSG_QUEST_POI_QUERY = 0x01E3 {
///     u32 amount_of_pois;
///     u32[amount_of_pois] points_of_interests;
/// }
/// ```
pub struct CMSG_QUEST_POI_QUERY {
    pub points_of_interests: Vec<u32>,
}

impl crate::private::Sealed for CMSG_QUEST_POI_QUERY {}
impl CMSG_QUEST_POI_QUERY {
    fn read_inner(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseErrorKind> {
        if !(4..=10240).contains(&body_size) {
            return Err(crate::errors::ParseErrorKind::InvalidSize);
        }

        // amount_of_pois: u32
        let amount_of_pois = crate::util::read_u32_le(&mut r)?;

        // points_of_interests: u32[amount_of_pois]
        let points_of_interests = {
            let mut points_of_interests = Vec::with_capacity(amount_of_pois as usize);

            let allocation_size = u64::from(amount_of_pois) * 4;
            if allocation_size > crate::errors::MAX_ALLOCATION_SIZE_WRATH {
                return Err(crate::errors::ParseErrorKind::AllocationTooLargeError(allocation_size));
            }

            for _ in 0..amount_of_pois {
                points_of_interests.push(crate::util::read_u32_le(&mut r)?);
            }
            points_of_interests
        };

        Ok(Self {
            points_of_interests,
        })
    }

}

impl crate::Message for CMSG_QUEST_POI_QUERY {
    const OPCODE: u32 = 0x01e3;

    #[cfg(feature = "print-testcase")]
    fn message_name(&self) -> &'static str {
        "CMSG_QUEST_POI_QUERY"
    }

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_QUEST_POI_QUERY {{").unwrap();
        // Members
        writeln!(s, "    amount_of_pois = {};", self.points_of_interests.len()).unwrap();
        writeln!(s, "    points_of_interests = [").unwrap();
        for v in self.points_of_interests.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "    ];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 4).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b, c, d] = 483_u32.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, {c:#04X}, {d:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_pois", "    ");
        if !self.points_of_interests.is_empty() {
            writeln!(s, "    /* points_of_interests: u32[amount_of_pois] start */").unwrap();
            for (i, v) in self.points_of_interests.iter().enumerate() {
                crate::util::write_bytes(&mut s, &mut bytes, 4, &format!("points_of_interests {i}"), "    ");
            }
            writeln!(s, "    /* points_of_interests: u32[amount_of_pois] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"{}\";", std::env::var("WOWM_TEST_CASE_WORLD_VERSION").unwrap_or("3.3.5".to_string())).unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // amount_of_pois: u32
        w.write_all(&(self.points_of_interests.len() as u32).to_le_bytes())?;

        // points_of_interests: u32[amount_of_pois]
        for i in self.points_of_interests.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        Self::read_inner(r, body_size).map_err(|a| crate::errors::ParseError::new(483, "CMSG_QUEST_POI_QUERY", body_size, a))
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_QUEST_POI_QUERY {}

impl CMSG_QUEST_POI_QUERY {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_pois: u32
        + self.points_of_interests.len() * core::mem::size_of::<u32>() // points_of_interests: u32[amount_of_pois]
    }
}

