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

#[cfg(feature = "print-testcase")]
impl CMSG_QUEST_POI_QUERY {
    pub fn to_test_case_string(&self) -> String {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test CMSG_QUEST_POI_QUERY {{").unwrap();
        // Members
        writeln!(s, "    amount_of_pois = {};", self.points_of_interests.len()).unwrap();
        write!(s, "    points_of_interests = [").unwrap();
        for v in self.points_of_interests.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        // Size/Opcode
        let [a, b] = (u16::try_from(self.size() + 6).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 483_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        // Bytes
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_pois");
        for (i, b) in bytes.enumerate() {
            if i == 0 {
                write!(s, "    ").unwrap();
            }
            write!(s, "{b:#04X}, ").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        s
    }

}

impl crate::private::Sealed for CMSG_QUEST_POI_QUERY {}
impl crate::Message for CMSG_QUEST_POI_QUERY {
    const OPCODE: u32 = 0x01e3;

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
    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(4..=10240).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x01E3, size: body_size });
        }

        // amount_of_pois: u32
        let amount_of_pois = crate::util::read_u32_le(&mut r)?;

        // points_of_interests: u32[amount_of_pois]
        let points_of_interests = {
            let mut points_of_interests = Vec::with_capacity(amount_of_pois as usize);
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
#[cfg(feature = "wrath")]
impl crate::wrath::ClientMessage for CMSG_QUEST_POI_QUERY {}

impl CMSG_QUEST_POI_QUERY {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_pois: u32
        + self.points_of_interests.len() * core::mem::size_of::<u32>() // points_of_interests: u32[amount_of_pois]
    }
}

