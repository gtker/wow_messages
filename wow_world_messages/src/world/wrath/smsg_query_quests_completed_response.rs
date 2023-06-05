use std::io::{Read, Write};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/quest/smsg_query_quests_completed_response.wowm:1`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/quest/smsg_query_quests_completed_response.wowm#L1):
/// ```text
/// smsg SMSG_QUERY_QUESTS_COMPLETED_RESPONSE = 0x0501 {
///     u32 amount_of_reward_quests;
///     u32[amount_of_reward_quests] reward_quests;
/// }
/// ```
pub struct SMSG_QUERY_QUESTS_COMPLETED_RESPONSE {
    pub reward_quests: Vec<u32>,
}

#[cfg(feature = "print-testcase")]
impl SMSG_QUERY_QUESTS_COMPLETED_RESPONSE {
    pub fn to_test_case_string(&self) -> Option<String> {
        use std::fmt::Write;
        use crate::traits::Message;

        let mut s = String::new();

        writeln!(s, "test SMSG_QUERY_QUESTS_COMPLETED_RESPONSE {{").unwrap();
        // Members
        writeln!(s, "    amount_of_reward_quests = {};", self.reward_quests.len()).unwrap();
        write!(s, "    reward_quests = [").unwrap();
        for v in self.reward_quests.as_slice() {
            write!(s, "{v:#04X}, ").unwrap();
        }
        writeln!(s, "];").unwrap();

        writeln!(s, "}} [").unwrap();

        let [a, b] = (u16::try_from(self.size() + 2).unwrap()).to_be_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* size */").unwrap();
        let [a, b] = 1281_u16.to_le_bytes();
        writeln!(s, "    {a:#04X}, {b:#04X}, /* opcode */").unwrap();
        let mut bytes: Vec<u8> = Vec::new();
        self.write_into_vec(&mut bytes).unwrap();
        let mut bytes = bytes.into_iter();

        crate::util::write_bytes(&mut s, &mut bytes, 4, "amount_of_reward_quests", "    ");
        if !self.reward_quests.is_empty() {
            writeln!(s, "    /* reward_quests: u32[amount_of_reward_quests] start */").unwrap();
            for (i, v) in self.reward_quests.iter().enumerate() {
                crate::util::write_bytes(&mut s, &mut bytes, 4, &format!("reward_quests {i}"), "    ");
            }
            writeln!(s, "    /* reward_quests: u32[amount_of_reward_quests] end */").unwrap();
        }


        writeln!(s, "] {{").unwrap();
        writeln!(s, "    versions = \"3.3.5\";").unwrap();
        writeln!(s, "}}\n").unwrap();

        Some(s)
    }

}

impl crate::private::Sealed for SMSG_QUERY_QUESTS_COMPLETED_RESPONSE {}
impl crate::Message for SMSG_QUERY_QUESTS_COMPLETED_RESPONSE {
    const OPCODE: u32 = 0x0501;

    #[cfg(feature = "print-testcase")]
    fn to_test_case_string(&self) -> Option<String> {
        SMSG_QUERY_QUESTS_COMPLETED_RESPONSE::to_test_case_string(self)
    }

    fn size_without_header(&self) -> u32 {
        self.size() as u32
    }

    fn write_into_vec(&self, mut w: impl Write) -> Result<(), std::io::Error> {
        // amount_of_reward_quests: u32
        w.write_all(&(self.reward_quests.len() as u32).to_le_bytes())?;

        // reward_quests: u32[amount_of_reward_quests]
        for i in self.reward_quests.iter() {
            w.write_all(&i.to_le_bytes())?;
        }

        Ok(())
    }

    fn read_body<S: crate::private::Sealed>(mut r: &mut &[u8], body_size: u32) -> Result<Self, crate::errors::ParseError> {
        if !(4..=16777215).contains(&body_size) {
            return Err(crate::errors::ParseError::InvalidSize { opcode: 0x0501, size: body_size });
        }

        // amount_of_reward_quests: u32
        let amount_of_reward_quests = crate::util::read_u32_le(&mut r)?;

        // reward_quests: u32[amount_of_reward_quests]
        let reward_quests = {
            let mut reward_quests = Vec::with_capacity(amount_of_reward_quests as usize);
            for _ in 0..amount_of_reward_quests {
                reward_quests.push(crate::util::read_u32_le(&mut r)?);
            }
            reward_quests
        };

        Ok(Self {
            reward_quests,
        })
    }

}

#[cfg(feature = "wrath")]
impl crate::wrath::ServerMessage for SMSG_QUERY_QUESTS_COMPLETED_RESPONSE {}

impl SMSG_QUERY_QUESTS_COMPLETED_RESPONSE {
    pub(crate) fn size(&self) -> usize {
        4 // amount_of_reward_quests: u32
        + self.reward_quests.len() * core::mem::size_of::<u32>() // reward_quests: u32[amount_of_reward_quests]
    }
}

